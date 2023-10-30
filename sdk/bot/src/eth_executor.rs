use crate::truth::{self, ISSUANCE_ADDRESS, UNISWAP_V3_ROUTER_ADDRESS};
use crate::utils::run_with_shutdown;
use bindings::{erc20::ERC20, issuance::Issuance};
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

type FunctionCall<D, M, S> =
    ethers::contract::FunctionCall<Arc<SignerMiddleware<M, S>>, SignerMiddleware<M, S>, D>;

pub enum Command {
    Swap {
        input: Address,
        output: Address,
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
    rx: mpsc::Receiver<Vec<Command>>,
    shutdown: Option<oneshot::Sender<()>>,
    issuance: Issuance<SignerMiddleware<M, S>>,
    router: ISwapRouter<SignerMiddleware<M, S>>,
    confirmations: usize,
    retry: usize,
    reverts: usize,
    should_shutdown: bool,
}

impl<S, M> EthExecutor<S, M>
where
    S: Signer + 'static,
    M: Middleware + 'static,
{
    /// creates a new thread and returns a cloneable sender to send commands to it
    pub fn spawn(
        middleware: M,
        signer: S,
        confirmations: usize,
        retry: usize,
        reverts: usize,
        should_shutdown: bool,
    ) -> (mpsc::Sender<Vec<Command>>, oneshot::Receiver<()>) {
        let (tx, rx) = mpsc::channel(20);
        let (shutdown, shutdown_recv) = oneshot::channel::<()>();

        let signer_middleware = Arc::new(ethers::middleware::SignerMiddleware::new(
            middleware, signer,
        ));

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
                shutdown: Some(shutdown),
                reverts,
                should_shutdown,
            };

            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Couldnt build EthExecutor runtime");

            rt.block_on(async move {
                run_with_shutdown(executor.handle_commands()).await;
            });
        });

        (tx, shutdown_recv)
    }

    /// blocks the runtime thread while it waits for message
    /// will return an Err if the channel is closed
    /// but this should never happen during runtime
    ///
    /// doesnt spawn a task becuase we want them to be serially executed
    async fn handle_commands(mut self) {
        while let Some(commands) = self.rx.recv().await {
            for command in commands {
                let res = match command {
                    Command::Swap {
                        input,
                        output,
                        amount,
                        fee,
                    } => {
                        tracing::info!("Swapping with {:?}", self.signer_middleware.address());

                        self.handle_swap(input, output, amount, fee).await
                    }
                    Command::Approve {
                        token,
                        spender,
                        amount,
                    } => {
                        tracing::info!("Approving with {:?}", self.signer_middleware.address());

                        self.handle_approve(token, spender, amount).await
                    }
                    Command::Mint { amount } => {
                        tracing::info!(
                            "Minting with {:?} amount {}",
                            self.signer_middleware.address(),
                            amount
                        );

                        self.handle_mint(amount).await
                    }
                    Command::Burn { amount } => {
                        tracing::info!(
                            "Burning with {:?} amount {}",
                            self.signer_middleware.address(),
                            amount
                        );

                        self.handle_burn(amount).await
                    }
                    Command::Signed { tx } => {
                        tracing::info!("Sending tx {:?}", tx);

                        self.handle_signed(tx).await
                    }
                };

                if let Err(_) = res {
                    // if weve reached this point it means that weve either exceeded the retry or revert limit
                    // we condintally shut down based on runtime command
                    if self.should_shutdown {
                        self.shutdown
                            .take()
                            .expect("Shutdown to be here")
                            .send(())
                            .expect("To send shutdown");
                    } else {
                        // if no shutdown then we should break and wait for next bundle
                        break;
                    }
                }
            }
        }
    }

    async fn handle_approve(
        &mut self,
        token: Address,
        spender: Address,
        amount: U256,
    ) -> anyhow::Result<()> {
        if *truth::EXECUTE {
            let erc20 = ERC20::new(token, self.signer_middleware.clone());
            let tx = erc20.approve(spender, amount);

            self.handle_tx(tx, "Approve").await?;
        }

        Ok(())
    }

    async fn handle_mint(&mut self, amount: U256) -> anyhow::Result<()> {
        if *truth::EXECUTE {
            let tx = self.issuance.issue(amount);

            self.handle_tx(tx, "Mint").await?;
        }

        Ok(())
    }

    async fn handle_burn(&mut self, amount: U256) -> anyhow::Result<()> {
        if *truth::EXECUTE {
            let tx = self.issuance.redeem(amount);

            self.handle_tx(tx, "Burn").await?;
        }

        Ok(())
    }

    async fn handle_signed(&mut self, tx: Transaction) -> anyhow::Result<()> {
        todo!()
    }

    async fn handle_swap(
        &mut self,
        input: Address,
        output: Address,
        amount: U256,
        fee: FeeTier,
    ) -> anyhow::Result<()> {
        if *truth::EXECUTE {
            let last_block = self.signer_middleware.get_block_number().await?;

            let last_block_timestmap = self
                .signer_middleware
                .get_block(last_block)
                .await?
                .expect("A block to be there")
                .timestamp;

            let tx = self.router.exact_output(ExactOutputParams {
                // encoded backwards because were using an exact output
                path: encode_path(&[output, input], fee),
                recipient: self.signer_middleware.address(),
                // fixed 10 block deadline
                deadline: last_block_timestmap + 120,
                amount_out: amount,
                amount_in_maximum: U256::MAX,
            });

            self.handle_tx(tx, "Swap").await?;
        }

        Ok(())
    }

    async fn handle_tx<D>(&mut self, tx: FunctionCall<D, M, S>, label: &str) -> anyhow::Result<()>
    where
        D: ethers::abi::Detokenize,
    {
        let mut tries = 0_usize;
        let mut reverts = 0_usize;

        loop {
            if tries == self.retry {
                tracing::error!("{} Failed, Max Retries Reached", label);
                return Err(anyhow::anyhow!(""));
            }

            match tx.send().await {
                Ok(tx) => match tx.confirmations(self.confirmations).await {
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
                    if reverts == self.reverts {
                        tracing::error!("{} Failed, Max Reverts Reached", label);
                        return Err(anyhow::anyhow!(""));
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

fn encode_path(addrs: &[Address], fee: FeeTier) -> Bytes {
    let mut path = Vec::<u8>::new();
    for addr in addrs {
        path.extend_from_slice(addr.as_bytes());
        path.extend_from_slice(&[fee as u8])
    }

    Bytes::from(path)
}
