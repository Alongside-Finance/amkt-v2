use super::Strategy;
use crate::{
    api::DataProvider,
    eth_executor::{ExecutorHandle, TxType},
    orchestrator::Orchestrator,
    truth::{KnownTicker, AMKT_KNOWN_FEE},
};
use amkt_bindings::shared_types::TokenInfo;
use ethers::{
    middleware::Middleware,
    types::{Address, U256},
};
use std::{collections::hash_map::Entry, sync::Arc};
use v3::{
    numeraire::{Numeraire, PoolPrice},
    pool::V3Pool,
};

lazy_static::lazy_static! {
    static ref AMKT_ADDRESS: Address = "0xF17A3fE536F8F7847F1385ec1bC967b2Ca9caE8D".parse().unwrap();
    static ref WETH_ADDRESS: Address = KnownTicker::WETH.known_addresss();
    static ref FLOAT_SCALARX18: rug::Float = rug::Float::with_val(100, 1e18);
    static ref FLOAT_SCALARX8: rug::Float = rug::Float::with_val(100, 1e8);
    static ref U256_SCALARX18: U256 = U256::from_dec_str(&1e18.to_string()).unwrap();
    static ref U256_SCALARX8: U256 = U256::from_dec_str(&1e18.to_string()).unwrap();
}

const AMKT_ETH_POOL_KEY: &'static str = "amkt_eth_pool";

/// An async strategy that expects a manually managed portfolio in an EOA of long tail float for the index.
/// The strategy will swap into btc/eth from USDC before (mint) execution
/// The strategy will swap into USDC from btc/eth after (burn) execution
///
/// Will use uni v3 crate to compute swap amounts for a given arb
/// assumes All the assets already have approvals
/// USDC should be approved to the swap router
/// All assets should be approved to issuance
#[derive(Clone)]
pub struct LongTailFloat;

#[async_trait::async_trait]
impl<M, P> Strategy<M, P> for LongTailFloat
where
    M: Middleware + Send + Sync + 'static,
    P: DataProvider + Send + Sync + 'static,
{
    async fn execute(
        &self,
        executor_handle: ExecutorHandle,
        orc: Arc<Orchestrator<M, P>>,
    ) -> anyhow::Result<()> {
        let mut bundle: Vec<TxType> = Vec::with_capacity(4);

        let mut lock = orc.cached_pools.lock().await;

        // check if weve already cached this pool instance
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

        let pool_price = pool.pool_price().await?.price_in(t_pos.clone());

        tracing::info!("amkt eth pool price: {}", pool_price);

        // get the nav price of amkt in eth
        let nav = orc.nav().await?;
        tracing::info!("nav usd: {}", nav);

        let eth_price = orc
            .data_provider_price(crate::truth::KnownTicker::WETH)
            .await?;

        // conver usd price to eth price
        let nav = nav / eth_price.clone();

        tracing::info!("nav eth: {}", nav);

        if pool_price > nav {
            tracing::info!("LongTailFloat: Pool price is greater than nav, minting");

            let mut amounts = pool
                .amounts_to_move_price(PoolPrice::new(&pool, nav.clone(), t_pos))
                .await?;

            tracing::info!("Amounts to peg pool: {:?}", amounts);

            let amkt_amount = amounts
                .remove(&*AMKT_ADDRESS)
                .expect("Amkt Address to be in the Deltas");

            // removing weth from pool so its negaitve
            let expected_amount_out = amounts
                .remove(&*WETH_ADDRESS)
                .expect("Weth Address to be in the Deltas")
                .abs();

            let cost_basis = float_to_u256(&amkt_amount * nav);

            let amkt_amount = float_to_u256(amkt_amount);

            let quote: Vec<TokenInfo> = orc.quote_mint(amkt_amount.clone()).await?;

            let (btc_amount, wsteth_amount) = extract_btc_wsteth_amounts(quote);

            populate_btc_wsteth_swap_commands(&mut bundle, orc.clone(), btc_amount, wsteth_amount)
                .await?;

            bundle.push(TxType::Mint {
                amount: amkt_amount.clone(),
            });

            bundle.push(TxType::ExactInput {
                input: *AMKT_ADDRESS,
                output: *WETH_ADDRESS,
                min_out: cost_basis.clone(),
                amount: amkt_amount,
                fee: v3::pool::FeeTier::Mid,
            });

            let gas_reciever = executor_handle.estimate_gas(bundle.clone()).await?;

            // the outer error means that the executor has dropped the sender
            // this can only mean that its been shutdown or panicked
            // the inner error tells us if gas estimation fails
            let gas = gas_reciever.await??;

            tracing::info!("Gas estimate: {}", gas);

            let cost_basis = cost_basis + gas;

            tracing::info!(
                "expect profit: {:?}",
                cost_basis - float_to_u256(expected_amount_out.clone())
            );

            if float_to_u256(expected_amount_out) > cost_basis {
                let last = bundle.last_mut().expect("Bundle to have at least one tx");

                match last {
                    TxType::ExactInput { min_out, .. } => *min_out = cost_basis,
                    _ => unreachable!(),
                }

                let _ = executor_handle.execute(bundle).await;
            }
        } else if pool_price < nav {
            tracing::info!("LongTailFloat: Pool price is less than nav, burning");

            let mut amounts = pool
                .amounts_to_move_price(PoolPrice::new(&pool, nav.clone(), t_pos))
                .await?;

            tracing::info!("Amounts to peg pool: {:?}", amounts);

            // were removing amkt from the pool so its negative
            let amkt_amount = amounts
                .remove(&*AMKT_ADDRESS)
                .expect("Amkt Address to be in the Deltas")
                .abs();

            let expected_amount_in = amounts
                .remove(&*WETH_ADDRESS)
                .expect("Weth Address to be in the Deltas");

            let burn_proceeds = float_to_u256(&amkt_amount * nav);

            let amkt_amount = float_to_u256(amkt_amount);

            let quote: Vec<TokenInfo> = orc.quote_burn(amkt_amount.clone()).await?;

            let (btc_amount, wsteth_amount) = extract_btc_wsteth_amounts(quote);

            populate_btc_wsteth_swap_commands(&mut bundle, orc.clone(), btc_amount, wsteth_amount)
                .await?;

            bundle.push(TxType::ExactOutput {
                input: *WETH_ADDRESS,
                output: *AMKT_ADDRESS,
                max_in: burn_proceeds.clone(),
                amount: amkt_amount,
                fee: v3::pool::FeeTier::Mid,
            });

            bundle.push(TxType::Burn {
                amount: amkt_amount,
            });

            let gas_reciever = executor_handle.estimate_gas(bundle.clone()).await?;

            // the outer error means that the executor has dropped the sender
            // this can only mean that its been shutdown or panicked
            // the inner error tells us if gas estimation fails
            let gas = gas_reciever.await??;

            tracing::info!("Gas estimate: {}", gas);

            let burn_proceeds = burn_proceeds - gas;

            tracing::info!(
                "expect profit: {:?}",
                float_to_u256(expected_amount_in.clone()) - burn_proceeds
            );

            if &float_to_u256(expected_amount_in) < &burn_proceeds {
                let len = bundle.len();

                let arb_swap = bundle
                    .get_mut(len - 2)
                    .expect("Bundle to have at least one tx");

                match arb_swap {
                    TxType::ExactOutput { max_in, .. } => *max_in = burn_proceeds,
                    _ => unreachable!(),
                }

                let _ = executor_handle.execute(bundle).await?;
            }
        } else {
            tracing::info!("Pool price is equal to nav, no action taken");
        }

        Ok(())
    }
}

fn float_to_u256(float: rug::Float) -> ethers::types::U256 {
    U256::from_dec_str(&trim_decimal_string(float.to_string())).expect("Float to be a valid U256")
}

#[allow(dead_code)]
fn u256_to_float(u256: ethers::types::U256) -> rug::Float {
    let parsed = rug::Float::parse(u256.to_string()).expect("U256 to be a valid Float");

    rug::Float::with_val(100, parsed)
}

fn trim_decimal_string(mut dec: String) -> String {
    match dec.find('.') {
        Some(idx) => {
            dec.truncate(idx);
        }
        None => {}
    }

    dec
}

fn extract_btc_wsteth_amounts(quote: Vec<TokenInfo>) -> (U256, U256) {
    let wsteth_amount = quote
        .iter()
        .find(|TokenInfo { token, .. }| token == &KnownTicker::WSTETH.known_addresss())
        .expect("Weth address to be in the quote")
        .units;

    let btc_amount = quote
        .iter()
        .find(|TokenInfo { token, .. }| token == &KnownTicker::BTC.known_addresss())
        .expect("BTC address to be in the quote")
        .units;

    (btc_amount, wsteth_amount)
}

async fn populate_btc_wsteth_swap_commands<M, P>(
    bundle: &mut Vec<TxType>,
    orc: Arc<Orchestrator<M, P>>,
    btc_amount: U256,
    wsteth_amount: U256,
) -> anyhow::Result<()>
where
    M: Middleware + Send + Sync + 'static,
    P: DataProvider + Send + Sync + 'static,
{
    let btc_price = float_to_u256(orc.data_provider_price(KnownTicker::BTC).await?);
    let wsteth_price = float_to_u256(orc.data_provider_price(KnownTicker::WSTETH).await?);

    let btc_max_in = btc_price * btc_amount / *U256_SCALARX8;
    let wsteth_max_in = wsteth_price * wsteth_amount / *U256_SCALARX18;

    let btc_command = TxType::ExactOutput {
        input: KnownTicker::USDC.known_addresss(),
        output: KnownTicker::BTC.known_addresss(),
        max_in: btc_max_in,
        amount: btc_amount,
        fee: v3::pool::FeeTier::Mid,
    };

    let wsteth_command = TxType::ExactOutput {
        input: KnownTicker::USDC.known_addresss(),
        output: KnownTicker::WSTETH.known_addresss(),
        max_in: wsteth_max_in,
        amount: wsteth_amount,
        fee: v3::pool::FeeTier::Mid,
    };

    bundle.push(btc_command);
    bundle.push(wsteth_command);

    Ok(())
}
