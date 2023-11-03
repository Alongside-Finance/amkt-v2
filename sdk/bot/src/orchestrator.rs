use crate::{
    api::DataProvider,
    eth_executor::{Command, ExecutorHandle},
    strategy::Strategy,
    truth::{KnownTicker, QUOTER_ADDRESS, VAULT_ADDRESS},
    utils::run_with_shutdown,
};
use amkt_bindings::{
    quoter::Quoter,
    vault::{TokenInfo, Vault},
};
use ethers::{
    middleware::Middleware,
    types::{Address, U256},
};
use rug::{ops::Pow, Float};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use v3::{
    constants::FACTORY_ADDRESS,
    ethers_pool::{pool::Pool, Factory},
};

/// The Orchestrator can try to execute strategies and has convience methods for commonly used data
///
/// such as token / pool pricing or other things that could be useful
pub struct Orchestrator<M, P>
where
    M: Middleware + Send + Sync + 'static,
    P: DataProvider + Send + Sync + 'static,
{
    strats: Vec<Box<dyn Strategy<M, P>>>,
    sleep: tokio::time::Duration,
    executor_shutdown: Option<tokio::sync::oneshot::Receiver<()>>,
    /// A channel to send commands to the eth executor
    pub executor_handle: ExecutorHandle,
    pub middleware: Arc<M>,
    pub data_provider: P,
    /// The uniswap v3 factory wrapper
    pub factory: Factory<M>,
    /// Cached pools, useful so strategys dont need to create new ones on each iteratoion
    pub cached_pools: Mutex<HashMap<&'static str, Arc<Pool<M>>>>,
    /// The vault contract for amkt
    pub vault: Vault<M>,
    pub quoter: Quoter<M>,
}

impl<M, P> Orchestrator<M, P>
where
    M: Middleware + Send + Sync + 'static,
    P: DataProvider + Send + Sync + 'static,
    Self: Send + Sync + 'static,
{
    pub fn new(
        middleware: Arc<M>,
        executor_handle: ExecutorHandle,
        data_provider: P,
        executor_shutdown: Option<tokio::sync::oneshot::Receiver<()>>,
        sleep: tokio::time::Duration,
    ) -> Orchestrator<M, P> {
        let factory = Factory::new(*FACTORY_ADDRESS, middleware.clone());
        let vault = Vault::new(*VAULT_ADDRESS, middleware.clone());
        let quoter = Quoter::new(*QUOTER_ADDRESS, middleware.clone());

        Self {
            strats: Vec::new(),
            sleep,
            executor_shutdown,
            executor_handle,
            data_provider,
            middleware,
            factory,
            cached_pools: Mutex::new(HashMap::new()),
            vault,
            quoter,
        }
    }

    pub async fn run(mut self) {
        let executor_shutdown = self.executor_shutdown.take();

        let orc = Arc::new(self);

        if orc.strats.len() < 1 {
            panic!("No strategies added to orchestrator");
        }

        let fut = run_with_shutdown(async move {
            loop {
                orc.clone().execute_strategies().await;
                tokio::time::sleep(orc.sleep).await;
            }
        });

        match executor_shutdown {
            Some(executor_shutdown) => {
                tokio::select! {
                    _ = fut => {
                        tracing::error!("Shutdown flag enabled, Executor Reached max retries or reverts, shutting down");

                    },
                    _ = executor_shutdown => {
                        tracing::debug!("Shutdown recived from executor")
                    }
                }
            }
            None => {
                fut.await;
            }
        }
    }

    /// serially execute strategies
    pub async fn execute_strategies(self: Arc<Self>) {
        for strategy in &self.strats {
            match strategy
                .execute(self.executor_handle.clone(), self.clone())
                .await
            {
                Ok(_) => tracing::info!("Strategy executed"),
                Err(e) => tracing::warn!("Strategy failed to execute: {}", e),
            }
        }
    }

    pub fn add_strategy(mut self, strategy: Box<dyn Strategy<M, P>>) -> Self {
        self.strats.push(strategy);
        self
    }

    /// returns the NAV price of AMKT in USD
    pub async fn nav(&self) -> anyhow::Result<rug::Float> {
        let assets = self
            .vault
            .virtual_units()
            .call()
            .await?
            .into_iter()
            .map(
                |TokenInfo {
                     token: addr,
                     units: amt,
                 }| (addr, KnownTicker::try_from(addr), amt),
            )
            .collect::<Vec<_>>();

        let futs = assets
            .into_iter()
            .map(|(addr, maybe_ticker, nominal)| async move {
                match maybe_ticker {
                    Ok(ticker) => {
                        let price = self.data_provider.price(ticker).await?;

                        tracing::info!("{} price: {}", ticker, price);

                        let decimals =
                            crate::utils::decimals(self.middleware.clone(), addr).await?;

                        let scalar = Float::with_val(100, 10).pow(decimals);

                        let parsed = Float::parse(nominal.to_string())?;
                        let float_nominal = Float::with_val(100, parsed) / scalar;

                        tracing::debug!(
                            "{} float nominal: {}, u256 nominal: {}",
                            ticker,
                            float_nominal,
                            nominal
                        );

                        let value = float_nominal * price;

                        Ok(value)
                    }
                    Err(e) => Err(e),
                }
            })
            .collect::<Vec<_>>();

        let results = futures::future::try_join_all(futs).await?;

        Ok(results
            .into_iter()
            .fold(rug::Float::with_val(100, 0), |init, curr| init + curr))
    }

    pub async fn underlying(&self) -> anyhow::Result<Vec<Address>> {
        let assets = self
            .vault
            .virtual_units()
            .call()
            .await?
            .into_iter()
            .map(|TokenInfo { token: addr, .. }| addr)
            .collect::<Vec<_>>();

        Ok(assets)
    }

    pub async fn quote_mint(&self, amount: U256) -> anyhow::Result<Vec<TokenInfo>> {
        Ok(self.quoter.quote_issue(amount).call().await?)
    }

    pub async fn quote_burn(&self, amount: U256) -> anyhow::Result<Vec<TokenInfo>> {
        Ok(self.quoter.quote_redeem(amount).call().await?)
    }

    pub async fn data_provider_price(&self, ticker: KnownTicker) -> anyhow::Result<rug::Float> {
        Ok(rug::Float::with_val(
            100,
            self.data_provider.price(ticker).await?,
        ))
    }
}
