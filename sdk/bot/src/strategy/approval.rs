use super::Strategy;
use crate::{
    api::DataProvider,
    eth_executor::{ExecutorHandle, TxType},
    orchestrator::Orchestrator,
    truth::{KnownTicker, AMKT_KNOWN_FEE, ISSUANCE_ADDRESS, UNISWAP_V3_ROUTER_ADDRESS},
};
use ethers::{
    middleware::Middleware,
    signers::Signer,
    types::{Address, H160, U256},
};
use std::sync::Arc;
use v3::{
    numeraire::{Numeraire, PoolPrice},
    pool::V3Pool,
};

#[derive(Clone)]
pub struct Approvals;

#[async_trait::async_trait]
impl<M, P> Strategy<M, P> for Approvals
where
    M: Middleware + Send + Sync + 'static,
    P: DataProvider + Send + Sync + 'static,
    Self: Send + Sync + 'static,
{
    async fn execute(
        &self,
        executor_handle: ExecutorHandle,
        orc: Arc<Orchestrator<M, P>>,
    ) -> anyhow::Result<()> {
        let mut underlying = orc.underlying().await?;

        underlying.push(KnownTicker::USDC.known_addresss());

        let signer_address = executor_handle.address.clone();

        let minimum_approval = U256::from_dec_str(&10e18.to_string())?;

        let max = U256::from_dec_str(&2_u128.pow(96).to_string())? - (U256::one() + U256::one());

        let current_approvals: Vec<_> = underlying
            .into_iter()
            .map(|addr| {
                let cloned = orc.middleware.clone();
                async move {
                    if addr != KnownTicker::USDC.known_addresss() {
                        let res = crate::utils::current_approval(
                            cloned,
                            addr,
                            signer_address,
                            *ISSUANCE_ADDRESS,
                        )
                        .await;

                        match res {
                            Ok(amount) => Ok((addr, amount)),
                            Err(e) => Err(e),
                        }
                    } else {
                        let res = crate::utils::current_approval(
                            cloned,
                            addr,
                            signer_address,
                            *UNISWAP_V3_ROUTER_ADDRESS,
                        )
                        .await;

                        match res {
                            Ok(amount) => Ok((addr, amount)),
                            Err(e) => Err(e),
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        let current_approvals = futures::future::try_join_all(current_approvals).await?;

        let bundle = current_approvals
            .into_iter()
            .filter_map(|(addr, amt)| {
                tracing::debug!("{} has {} approvals", addr, amt);
                if amt < minimum_approval {
                    tracing::debug!("{:?} needs approval", addr);

                    if addr != KnownTicker::USDC.known_addresss() {
                        Some(TxType::Approve {
                            token: addr,
                            spender: *ISSUANCE_ADDRESS,
                            amount: max,
                        })
                    } else {
                        Some(TxType::Approve {
                            token: addr,
                            spender: *UNISWAP_V3_ROUTER_ADDRESS,
                            amount: max,
                        })
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let status = executor_handle.execute(bundle).await?;

        let _ = status.await?;

        Ok(())
    }
}
