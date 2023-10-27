use crate::truth::{self, ISSUANCE_ADDRESS};
use crate::utils::run_with_shutdown;
use bindings::{erc20::ERC20, issuance::Issuance};
use ethers::middleware::SignerMiddleware;
use ethers::{
    providers::Middleware,
    signers::Signer,
    types::{Address, Transaction, U256},
};
use std::sync::Arc;
use tokio::sync::mpsc;

pub enum Command {
    Swap {
        input: Address,
        output: Address,
        amount: U256,
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
    issuance: Issuance<SignerMiddleware<M, S>>,
    confirmations: u64,
}

impl<S, M> EthExecutor<S, M>
where
    S: Signer + 'static,
    M: Middleware + 'static,
{
    /// creates a new thread and returns a cloneable sender to send commands to it
    pub fn spawn(middleware: M, signer: S, confirmations: u64) -> mpsc::Sender<Command> {
        let (tx, rx) = mpsc::channel(20);

        let signer_middleware = Arc::new(ethers::middleware::SignerMiddleware::new(
            middleware, signer,
        ));

        let _ = std::thread::spawn(move || {
            let issuance = Issuance::new(*ISSUANCE_ADDRESS, signer_middleware.clone());

            let executor = Self {
                signer_middleware,
                issuance,
                rx,
                confirmations,
            };

            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Couldnt build EthExecutor runtime");

            rt.block_on(async move {
                run_with_shutdown(executor.handle_commands()).await;
            });
        });

        tx
    }

    /// blocks the runtime thread while it waits for message
    /// will return an Err if the channel is closed
    /// but this should never happen during runtime
    ///
    /// doesnt spawn a task becuase we want them to be serially executed
    async fn handle_commands(mut self) {
        while let Some(command) = self.rx.recv().await {
            match command {
                Command::Swap {
                    input,
                    output,
                    amount,
                } => {
                    tracing::info!("Swapping with {:?}", self.signer_middleware.address());
                    self.handle_swap(input, output, amount).await;
                }
                Command::Approve {
                    token,
                    spender,
                    amount,
                } => {
                    tracing::info!("Approving with {:?}", self.signer_middleware.address());
                    self.handle_approve(token, spender, amount).await;
                }
                Command::Mint { amount } => {
                    tracing::info!(
                        "Minting with {:?} amount {}",
                        self.signer_middleware.address(),
                        amount
                    );
                    self.handle_mint(amount).await;
                }
                Command::Burn { amount } => {
                    tracing::info!(
                        "Burning with {:?} amount {}",
                        self.signer_middleware.address(),
                        amount
                    );
                    self.handle_burn(amount).await;
                }
                Command::Signed { tx } => {
                    tracing::info!("Sending tx {:?}", tx);
                    self.handle_signed(tx).await;
                }
            }
        }
    }

    async fn handle_approve(&self, token: Address, spender: Address, amount: U256) {
        if *truth::EXECUTE {
            let erc20 = ERC20::new(token, self.signer_middleware.clone());
            let tx = erc20.approve(spender, amount);

            match tx.send().await {
                Ok(tx) => match tx.await {
                    Ok(maybe_recipet) => {
                        if let Some(reciept) = maybe_recipet {
                            tracing::info!(
                                "Approval Succeeded, TxHash: {:?}",
                                reciept.transaction_hash
                            );
                        } else {
                            tracing::error!("Approval Failed, No Reciept");
                        }
                    }
                    Err(e) => {
                        tracing::error!("Approval Failed: {}", e);
                    }
                },
                Err(e) => {
                    tracing::error!("Approval Failed: {}", e);
                }
            };
        }
    }

    async fn handle_mint(&self, amount: U256) {
        if *truth::EXECUTE {
            let tx = self.issuance.issue(amount);

            match tx.send().await {
                Ok(tx) => match tx.await {
                    Ok(maybe_recipet) => {
                        if let Some(reciept) = maybe_recipet {
                            tracing::info!(
                                "Mint Succeeded, TxHash: {:?}",
                                reciept.transaction_hash
                            );
                        } else {
                            tracing::error!("Mint Failed, No Reciept");
                        }
                    }
                    Err(e) => {
                        tracing::error!("Mint Failed: {}", e);
                    }
                },
                Err(e) => {
                    tracing::error!("Mint Failed: {}", e);
                }
            };
        }
    }

    async fn handle_burn(&self, amount: U256) {
        if *truth::EXECUTE {
            let tx = self.issuance.redeem(amount);

            match tx.send().await {
                Ok(tx) => match tx.await {
                    Ok(maybe_recipet) => {
                        if let Some(reciept) = maybe_recipet {
                            tracing::info!(
                                "Burn Succeeded, TxHash: {:?}",
                                reciept.transaction_hash
                            );
                        } else {
                            tracing::error!("Burn Failed, No Reciept");
                        }
                    }
                    Err(e) => {
                        tracing::error!("Burn Failed: {}", e);
                    }
                },
                Err(e) => {
                    tracing::error!("Burn Failed: {}", e);
                }
            };
        }
    }

    async fn handle_signed(&self, tx: Transaction) {
        todo!()
    }

    async fn handle_swap(&self, input: Address, output: Address, amount: U256) {
        todo!()
    }
}
