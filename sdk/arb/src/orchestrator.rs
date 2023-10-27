use crate::{
    api::DataProvider,
    eth_executor::Command,
    strategy::Strategy,
    truth::{KnownTicker, VAULT_ADDRESS},
    utils::run_with_shutdown,
};
use bindings::vault::{TokenInfo, Vault};
use ethers::{middleware::Middleware, signers::Signer, types::Address};
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
    /// A channel to send commands to the eth executor
    pub tx: tokio::sync::mpsc::Sender<Command>,
    pub middleware: Arc<M>,
    pub data_provider: P,
    /// The uniswap v3 factory wrapper
    pub factory: Factory<M>,
    /// Cached pools, useful so strategys dont need to create new ones on each iteratoion
    pub cached_pools: Mutex<HashMap<&'static str, Arc<Pool<M>>>>,
    /// The vault contract for amkt
    pub vault: Vault<M>,
}

impl<M, P> Orchestrator<M, P>
where
    M: Middleware + Send + Sync + 'static,
    P: DataProvider + Send + Sync + 'static,
{
    pub fn new(
        middleware: Arc<M>,
        data_provider: P,
        tx: tokio::sync::mpsc::Sender<Command>,
        sleep: tokio::time::Duration,
    ) -> Orchestrator<M, P> {
        let factory = Factory::new(*FACTORY_ADDRESS, middleware.clone());
        let vault = Vault::new(*VAULT_ADDRESS, middleware.clone());

        Self {
            strats: Vec::new(),
            sleep,
            tx,
            data_provider,
            middleware,
            factory,
            cached_pools: Mutex::new(HashMap::new()),
            vault,
        }
    }

    pub async fn run(self) {
        let orc = Arc::new(self);

        if orc.strats.len() < 1 {
            panic!("No strategies added to orchestrator");
        }

        run_with_shutdown(async move {
            loop {
                orc.clone().execute_strategies().await;
                tokio::time::sleep(orc.sleep).await;
            }
        })
        .await;
    }

    /// serially execute strategies
    pub async fn execute_strategies(self: Arc<Self>) {
        for strategy in &self.strats {
            match strategy.execute(self.tx.clone(), self.clone()).await {
                Ok(_) => tracing::info!("Strategy executed"),
                Err(e) => tracing::warn!("Strategy failed to execute: {}", e),
            }
        }
    }

    pub fn add_strategy(mut self, strategy: Box<dyn Strategy<M, P>>) -> Self {
        self.strats.push(strategy);
        self
    }

    pub async fn nav(&self) -> anyhow::Result<f64> {
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

                        Ok(value.to_f64())
                    }
                    Err(e) => Err(e),
                }
            })
            .collect::<Vec<_>>();

        Ok(futures::future::try_join_all(futs).await?.iter().sum())
    }

    pub async fn data_provider_price(&self, ticker: KnownTicker) -> anyhow::Result<f64> {
        self.data_provider.price(ticker).await
    }
}
