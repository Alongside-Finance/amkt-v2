use crate::truth::{self, ISSUANCE_ADDRESS, UNISWAP_V3_ROUTER_ADDRESS};
use crate::utils::run_with_shutdown;
use amkt_bindings::{erc20::ERC20, issuance::Issuance};
use ethers::contract::ContractError;
use ethers::middleware::SignerMiddleware;
use ethers::types::Bytes;
use ethers::{
    providers::Middleware,
    signers::Signer,
    types::{Address, Transaction, U256},
};
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};
use v3::pool::FeeTier;

ethers::contract::abigen! {
    ISwapRouter,
    "src/abi/ISwapRouter.json",
}

/// A handle and convience wrapper to interacting with the executor
#[derive(Clone)]
pub struct ExecutorHandle {
    pub tx: mpsc::Sender<Command>,
    pub address: Address,
}

impl ExecutorHandle {
    pub async fn estimate_gas(
        &self,
        txs: Vec<TxType>,
    ) -> anyhow::Result<oneshot::Receiver<anyhow::Result<U256>>> {
        let (sender, receiver) = oneshot::channel::<anyhow::Result<U256>>();

        let _ = self.tx.send(Command::EstimateGas(sender, txs)).await?;

        Ok(receiver)
    }

    pub async fn execute(
        &self,
        txs: Vec<TxType>,
    ) -> anyhow::Result<oneshot::Receiver<anyhow::Result<()>>> {
        let (send, rx) = oneshot::channel::<anyhow::Result<()>>();

        let _ = self.tx.send(Command::Execute(send, txs)).await?;

        Ok(rx)
    }
}

type FunctionCall<D, M, S> =
    ethers::contract::FunctionCall<Arc<SignerMiddleware<M, S>>, SignerMiddleware<M, S>, D>;

#[derive(Clone, Copy)]
enum Mode {
    EstimateGas,
    Execute,
}

impl Mode {
    fn is_execute(&self) -> bool {
        match self {
            Mode::EstimateGas => false,
            Mode::Execute => true,
        }
    }
}

pub enum Command {
    EstimateGas(oneshot::Sender<anyhow::Result<U256>>, Vec<TxType>),
    Execute(oneshot::Sender<anyhow::Result<()>>, Vec<TxType>),
}

#[derive(Clone)]
pub enum TxType {
    ExactOutput {
        input: Address,
        output: Address,
        max_in: U256,
        amount: U256,
        fee: FeeTier,
    },
    ExactInput {
        input: Address,
        output: Address,
        min_out: U256,
        amount: U256,
        fee: FeeTier,
    },
    Approve {
        token: Address,
        spender: Address,
        amount: U256,
    },
    Mint {
        amount: U256,
    },
    Burn {
        amount: U256,
    },
    Signed {
        tx: Transaction,
    },
}

/// A single threaded Ethereum transaction executor.
///
/// A Simple wrapper over a few predefined operations relevant to the strategies
/// that awaits confirmations in the order they are receieved
pub struct EthExecutor<S, M>
where
    S: Signer + 'static,
    M: Middleware + 'static,
{
    signer_middleware: Arc<SignerMiddleware<M, S>>,
    rx: mpsc::Receiver<Command>,
    shutdown: Option<oneshot::Sender<()>>,
    issuance: Issuance<SignerMiddleware<M, S>>,
    router: ISwapRouter<SignerMiddleware<M, S>>,
    running_sum: Option<U256>,
    confirmations: usize,
    /// errors that dont cost money
    retry: usize,
    /// errors that cost money
    reverts: usize,
}

impl<S, M> EthExecutor<S, M>
where
    S: Signer + 'static,
    M: Middleware + 'static,
{
    /// creates a new thread and returns a cloneable sender to send commands to it
    /// also returns a oneshot reciver that will be sent a message if the executor errors
    pub fn spawn(
        middleware: M,
        signer: S,
        confirmations: usize,
        retry: usize,
        reverts: usize,
        should_shutdown: bool,
    ) -> (ExecutorHandle, Option<oneshot::Receiver<()>>) {
        let (tx, rx) = mpsc::channel(20);
        let (shutdown, shutdown_recv) = if should_shutdown {
            let (tx, rx) = oneshot::channel::<()>();
            (Some(tx), Some(rx))
        } else {
            (None, None)
        };

        let signer_middleware = Arc::new(ethers::middleware::SignerMiddleware::new(
            middleware, signer,
        ));

        let address = signer_middleware.address();

        let _ = std::thread::spawn(move || {
            let issuance = Issuance::new(*ISSUANCE_ADDRESS, signer_middleware.clone());
            let router = ISwapRouter::new(*UNISWAP_V3_ROUTER_ADDRESS, signer_middleware.clone());

            let executor = Self {
                signer_middleware,
                issuance,
                router,
                rx,
                confirmations,
                retry,
                shutdown,
                reverts,
                running_sum: None,
            };

            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Couldnt build EthExecutor runtime");

            rt.block_on(async move {
                run_with_shutdown(executor.handle_commands()).await;
            });
        });

        (ExecutorHandle { address, tx }, shutdown_recv)
    }

    /// blocks the runtime thread while it waits for message
    /// will return an Err if the channel is closed
    /// but this should never happen during runtime
    ///
    /// doesnt spawn a task becuase we want them to be serially executed
    async fn handle_commands(mut self) {
        while let Some(command) = self.rx.recv().await {
            let (mode, txs, mut maybe_gas_sender, mut maybe_status_sender) = match command {
                Command::EstimateGas(sender, txs) => {
                    self.running_sum = Some(U256::zero());
                    (Mode::EstimateGas, txs, Some(sender), None)
                }
                Command::Execute(sender, txs) => (Mode::Execute, txs, None, Some(sender)),
            };

            for tx in txs.into_iter() {
                let res = match tx {
                    TxType::ExactOutput {
                        input,
                        output,
                        amount,
                        fee,
                        max_in,
                    } => {
                        tracing::debug!("Executing Exact Output");
                        self.handle_exact_output(mode, input, output, amount, max_in, fee)
                            .await
                    }
                    TxType::ExactInput {
                        input,
                        output,
                        min_out,
                        amount,
                        fee,
                    } => {
                        tracing::debug!("Executing Exact Input");
                        self.handle_exact_input(mode, input, output, amount, min_out, fee)
                            .await
                    }
                    TxType::Approve {
                        token,
                        spender,
                        amount,
                    } => {
                        tracing::debug!("Executing Approval");
                        self.handle_approve(mode, token, spender, amount).await
                    }
                    TxType::Mint { amount } => {
                        tracing::debug!("Executing Mint");
                        self.handle_mint(mode, amount).await
                    }
                    TxType::Burn { amount } => {
                        tracing::debug!("Executing Burn");
                        self.handle_burn(mode, amount).await
                    }
                    TxType::Signed { tx } => {
                        tracing::debug!("Executing Signed");
                        self.handle_signed(mode, tx).await
                    }
                };

                // if weve reached this point it means that weve either exceeded the retry or revert limit
                // or it means gas estimation failed
                // we condintally shut down based on runtime command
                if let Err(e) = res {
                    tracing::error!("Executor Error: {}", e);

                    if mode.is_execute() {
                        let sender = maybe_status_sender.take().expect("A sender to be here");
                        let _ = sender.send(Err(e));

                        match self.shutdown {
                            Some(shutdown) => {
                                let _ = shutdown.send(());
                                return;
                            }
                            None => {}
                        }
                    } else {
                        tracing::error!("Gas Estimation failed");
                        let sender = maybe_gas_sender.take().expect("A sender to be here");

                        let _ = sender.send(Err(anyhow::anyhow!(
                            "Executor doesnt retry on gas estimation {}",
                            e
                        )));
                    }

                    break;
                }
            }

            if mode.is_execute() {
                match maybe_status_sender {
                    Some(sender) => {
                        let _ = sender.send(Ok(()));
                    }
                    // If there is no sender than we had an error and its already been sent
                    None => {}
                }
            } else {
                match maybe_gas_sender {
                    Some(sender) => {
                        let _ = sender.send(Ok(self
                            .running_sum
                            .expect("Running sum to be here on estimate gas")));
                    }
                    // If there is no sender than we had an error and its already been sent
                    None => {}
                }
            }
        }
    }

    async fn handle_approve(
        &mut self,
        mode: Mode,
        token: Address,
        spender: Address,
        amount: U256,
    ) -> anyhow::Result<()> {
        let erc20 = ERC20::new(token, self.signer_middleware.clone());
        let tx = erc20.approve(spender, amount);

        match mode {
            Mode::EstimateGas => {
                let gas = tx.estimate_gas().await?;
                self.running_sum = Some(
                    self.running_sum
                        .expect("Running sum to be here on estimate gas")
                        + gas,
                );
            }
            Mode::Execute => {
                if *truth::EXECUTE {
                    Self::execute_tx(tx, self.confirmations, self.retry, self.reverts, "Approve")
                        .await?;
                }
            }
        }

        Ok(())
    }

    async fn handle_mint(&mut self, mode: Mode, amount: U256) -> anyhow::Result<()> {
        let tx = self.issuance.issue(amount);

        match mode {
            Mode::EstimateGas => {
                let gas = tx.estimate_gas().await?;
                self.running_sum = Some(
                    self.running_sum
                        .expect("Running sum to be here on estimate gas")
                        + gas,
                );
            }
            Mode::Execute => {
                if *truth::EXECUTE {
                    Self::execute_tx(tx, self.confirmations, self.retry, self.reverts, "Mint")
                        .await?;
                }
            }
        }

        Ok(())
    }

    async fn handle_burn(&mut self, mode: Mode, amount: U256) -> anyhow::Result<()> {
        let tx = self.issuance.redeem(amount);

        match mode {
            Mode::EstimateGas => {
                let gas = tx.estimate_gas().await?;
                self.running_sum = Some(
                    self.running_sum
                        .expect("Running sum to be here on estimate gas")
                        + gas,
                );
            }
            Mode::Execute => {
                if *truth::EXECUTE {
                    Self::execute_tx(tx, self.confirmations, self.retry, self.reverts, "Burn")
                        .await?;
                }
            }
        }

        Ok(())
    }

    async fn handle_signed(&mut self, mode: Mode, tx: Transaction) -> anyhow::Result<()> {
        todo!()
    }

    async fn handle_exact_output(
        &mut self,
        mode: Mode,
        input: Address,
        output: Address,
        amount: U256,
        max_in: U256,
        fee: FeeTier,
    ) -> anyhow::Result<()> {
        let last_block = self.signer_middleware.get_block_number().await?;

        let last_block_timestmap = self
            .signer_middleware
            .get_block(last_block)
            .await?
            .expect("A block to be there")
            .timestamp;

        let tx = self.router.exact_output(ExactOutputParams {
            // encoded backwards because were using an exact output
            path: encode_path(&[output, input], fee).into(),
            recipient: self.signer_middleware.address(),
            // fixed 10 block deadline
            deadline: last_block_timestmap + 120,
            amount_out: amount,
            amount_in_maximum: max_in,
        });

        tracing::debug!(
            "calldata: {:?}",
            hex::encode(&tx.calldata().expect("calldata"))
        );
        tracing::debug!(
            "to {:?}",
            hex::encode(tx.tx.to().unwrap().as_address().unwrap())
        );

        match mode {
            Mode::EstimateGas => {
                let gas = tx.estimate_gas().await?;
                self.running_sum = Some(
                    self.running_sum
                        .expect("Running sum to be here on estimate gas")
                        + gas,
                );
            }
            Mode::Execute => {
                if *truth::EXECUTE {
                    Self::execute_tx(
                        tx,
                        self.confirmations,
                        self.retry,
                        self.reverts,
                        "Exact Output",
                    )
                    .await?;
                }
            }
        }

        Ok(())
    }

    async fn handle_exact_input(
        &mut self,
        mode: Mode,
        input: Address,
        output: Address,
        min_out: U256,
        amount: U256,
        fee: FeeTier,
    ) -> anyhow::Result<()> {
        let last_block = self.signer_middleware.get_block_number().await?;

        let last_block_timestmap = self
            .signer_middleware
            .get_block(last_block)
            .await?
            .expect("A block to be there")
            .timestamp;

        let tx = self.router.exact_input(ExactInputParams {
            path: encode_path(&[input, output], fee).into(),
            recipient: self.signer_middleware.address(),
            // fixed 10 block deadline
            deadline: last_block_timestmap + 120,
            amount_in: amount,
            amount_out_minimum: min_out,
        });

        match mode {
            Mode::EstimateGas => {
                let gas = tx.estimate_gas().await?;
                self.running_sum = Some(
                    self.running_sum
                        .expect("Running sum to be here on estimate gas")
                        + gas,
                );
            }
            Mode::Execute => {
                if *truth::EXECUTE {
                    Self::execute_tx(
                        tx,
                        self.confirmations,
                        self.retry,
                        self.reverts,
                        "Exact Input",
                    )
                    .await?;
                }
            }
        }

        Ok(())
    }

    async fn execute_tx<D>(
        tx: FunctionCall<D, M, S>,
        confirmations: usize,
        max_retry: usize,
        max_reverts: usize,
        label: &str,
    ) -> anyhow::Result<()>
    where
        D: ethers::abi::Detokenize,
    {
        let mut tries = 0_usize;
        let mut reverts = 0_usize;

        loop {
            if tries == max_retry {
                tracing::error!("{} Failed, Max Retries Reached", label);
                return Err(anyhow::anyhow!("To many retries for this bundle"));
            }

            match tx.send().await {
                Ok(tx) => match tx.confirmations(confirmations).await {
                    Ok(maybe_recipet) => {
                        if let Some(reciept) = maybe_recipet {
                            tracing::info!(
                                "{} Succeeded, TxHash: {:?}",
                                label,
                                reciept.transaction_hash
                            );
                            return Ok(());
                        } else {
                            tracing::error!("{} Failed, No Reciept", label);
                        }
                    }
                    Err(e) => {
                        tracing::error!("{} Failed: {}", label, e);
                    }
                },
                Err(ContractError::Revert(bytes)) => {
                    tracing::error!("{} Failed: Revert {:?}", label, bytes);

                    reverts += 1;
                    if reverts == max_reverts {
                        tracing::error!("{} Failed, Max Reverts Reached", label);
                        return Err(anyhow::anyhow!("Too many Reverts for this Bundle"));
                    }
                }
                Err(e) => {
                    tracing::error!("{} Failed: {}", label, e);
                }
            };

            tries += 1;
        }
    }
}

fn encode_path(addrs: &[Address], fee: FeeTier) -> Vec<u8> {
    let mut path = Vec::<u8>::new();
    let len = addrs.len();

    for (i, addr) in addrs.iter().enumerate() {
        path.extend_from_slice(&addr.as_bytes());

        if i != len - 1 {
            path.extend_from_slice(&fee.as_u24_bytes());
        }
    }

    path
}
