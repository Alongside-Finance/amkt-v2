use super::Strategy;
use crate::{
    api::DataProvider, eth_executor::Command, orchestrator::Orchestrator, truth::AMKT_KNOWN_FEE,
};
use ethers::{middleware::Middleware, types::Address};
use std::{collections::hash_map::Entry, sync::Arc};
use v3::{numeraire::Numeraire, pool::V3Pool};

lazy_static::lazy_static! {
    static ref AMKT_ADDRESS: Address = "0xF17A3fE536F8F7847F1385ec1bC967b2Ca9caE8D".parse().unwrap();
    static ref WETH_ADDRESS: Address = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".parse().unwrap();
}

const AMKT_ETH_POOL_KEY: &'static str = "amkt_eth_pool";

/// An async strategy that expects a manually managed portfolio in an EOA of long tail float for the index.
/// The strategy will swap into btc/eth from USDC before (mint) execution
/// The strategy will swap into USDC from btc/eth after (burn) execution
///
/// Will use uni v3 crate to compute swap amounts for a given arb
pub struct LongTailFloat;

#[async_trait::async_trait]
impl<M, P> Strategy<M, P> for LongTailFloat
where
    M: Middleware + Send + Sync + 'static,
    P: DataProvider + Send + Sync + 'static,
{
    async fn execute(
        &self,
        tx: tokio::sync::mpsc::Sender<Vec<Command>>,
        orc: Arc<Orchestrator<M, P>>,
    ) -> anyhow::Result<()> {
        let mut lock = orc.cached_pools.lock().await;

        let pool = match lock.entry(AMKT_ETH_POOL_KEY) {
            Entry::Occupied(pool) => pool.get().clone(),
            Entry::Vacant(ve) => {
                let pool = orc
                    .factory
                    .pool(*AMKT_ADDRESS, *WETH_ADDRESS, AMKT_KNOWN_FEE)
                    .await?;

                ve.insert(Arc::new(pool)).clone()
            }
        };

        drop(lock);

        let t_pos = if pool.token0() == *WETH_ADDRESS {
            v3::numeraire::Token::Zero
        } else {
            v3::numeraire::Token::One
        };

        let pool_price = pool.pool_price().await?.price_in(t_pos);

        tracing::info!("pool price: {}", pool_price);

        let nav = rug::Float::with_val(100, orc.nav().await?);

        tracing::info!("nav: {}", nav);

        if pool_price > nav {
            tracing::info!("LongTailFloat: Pool price is greater than nav, minting");
        } else if pool_price < nav {
            tracing::info!("LongTailFloat: Pool price is less than nav, burning");
        } else {
            tracing::info!("Pool price is equal to nav, no action taken");
        }

        Ok(())
    }
}
