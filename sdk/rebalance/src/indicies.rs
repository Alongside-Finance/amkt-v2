use crate::data::{DataProvider, TokenInfo};
use crate::truth::KnownTicker;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct Weight<APIData> {
    pub info: TokenInfo<APIData>,
    pub weight: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Nominals {
    pub ticker: KnownTicker,
    pub amount: f64,
}

/// A top level trait for all providers that can create a weighting
///
/// WARNING! Implemnetors of this trait must ensure that the sum of the weights is equal to 1 or as close as a possible
#[async_trait::async_trait]
pub trait IndexProvider: DataProvider {
    async fn weights(&self, n: usize) -> anyhow::Result<Vec<Weight<Self::ApiData>>>;
}

/// A top level trait for all providers that provide weighting
#[async_trait::async_trait]
pub trait NominalsProvider: IndexProvider {
    async fn nominals(&self, n: usize, usd_value: f64) -> anyhow::Result<Vec<Nominals>> {
        let weights = self.weights(n).await?;

        Ok(Self::create_nominals(weights, usd_value))
    }

    fn create_nominals(weights: Vec<Weight<Self::ApiData>>, usd_value: f64) -> Vec<Nominals> {
        weights
            .into_iter()
            .map(|w| Nominals {
                ticker: w.info.ticker,
                amount: w.weight * usd_value / w.info.price,
            })
            .collect()
    }
}

impl<P: IndexProvider> NominalsProvider for P {}

#[async_trait::async_trait]
pub trait MarketCapDataProvider: DataProvider {
    async fn top_n_assets_by_market_cap(
        &self,
        n: usize,
    ) -> anyhow::Result<Vec<TokenInfo<Self::ApiData>>>;
}

#[async_trait::async_trait]
impl<MCIndex: MarketCapDataProvider> IndexProvider for MCIndex
where
    MCIndex: Send + Sync,
{
    async fn weights(&self, n: usize) -> anyhow::Result<Vec<Weight<Self::ApiData>>> {
        let top_n = self.top_n_assets_by_market_cap(n).await?;

        let total_market_cap = top_n.iter().map(|data| data.market_cap).sum::<f64>();

        let weights = top_n
            .into_iter()
            .map(|data| Weight {
                weight: data.market_cap / total_market_cap,
                info: data,
            })
            .collect::<Vec<_>>();

        Ok(weights)
    }
}
