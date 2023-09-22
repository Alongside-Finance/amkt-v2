use super::DataProvider;
use crate::{
    data::TokenInfo,
    indicies::{IndexProvider, Weight},
    truth::KnownTicker,
};
use futures::future::try_join_all;

/// Provider Wrapper deletes the extra data from its API before finally passing it up the multi provider
struct ProviderWrapper<API, P: IndexProvider<ApiData = API>> {
    inner: P,
}

impl<API, P> ProviderWrapper<API, P>
where
    P: IndexProvider<ApiData = API>,
{
    pub fn wrap(provider: P) -> Box<ProviderWrapper<API, P>> {
        Box::new(ProviderWrapper { inner: provider })
    }
}

#[async_trait::async_trait]
impl<API, P> DataProvider for ProviderWrapper<API, P>
where
    P: IndexProvider<ApiData = API>,
{
    type ApiData = ();

    async fn token_info(&self, ticker: KnownTicker) -> anyhow::Result<TokenInfo<Self::ApiData>> {
        Ok(cast_token_info(self.inner.token_info(ticker).await?))
    }
}

#[async_trait::async_trait]
impl<API, P> IndexProvider for ProviderWrapper<API, P>
where
    P: IndexProvider<ApiData = API>,
{
    async fn weights(&self, n: usize) -> anyhow::Result<Vec<Weight<Self::ApiData>>> {
        let weights = self.inner.weights(n).await?;

        let weights = weights
            .into_iter()
            .map(|weight| Weight {
                weight: weight.weight,
                info: cast_token_info(weight.info),
            })
            .collect();

        Ok(weights)
    }
}

/// A struct intended to be used with `ProviderWrapper` to create a weighted average of multiple providers
struct MultiProvider {
    inner: Option<Vec<(Box<dyn IndexProvider<ApiData = ()>>, f64)>>,
}

impl MultiProvider {
    pub fn builder() -> Self {
        Self { inner: None }
    }

    pub fn with(mut self, provider: Box<dyn IndexProvider<ApiData = ()>>, weighting: f64) -> Self {
        match self.inner {
            Some(ref mut inner) => inner.push((provider, weighting)),
            None => self.inner = Some(vec![(provider, weighting)]),
        }

        self
    }

    pub fn build(self) -> anyhow::Result<Self> {
        match self.inner {
            Some(inner) => Ok(Self { inner: Some(inner) }),
            None => Err(anyhow::anyhow!("Tried to build an empty MultiProvider")),
        }
    }
}
// todo this whole process can be done cleaner by making these generic over a type that implements average or something, since we have reuse across the two follwing traits
#[async_trait::async_trait]
impl DataProvider for MultiProvider {
    type ApiData = ();

    async fn token_info(&self, ticker: KnownTicker) -> anyhow::Result<TokenInfo<Self::ApiData>> {
        let inner = self
            .inner
            .as_ref()
            .expect("We should have built the MultiProvider");

        let all_token_infos = inner
            .iter()
            .map(|(provider, _)| provider.token_info(ticker))
            .collect::<Vec<_>>();

        let all_token_infos = try_join_all(all_token_infos).await?;

        let normalization_denominator = inner.iter().map(|(_, weighting)| weighting).sum::<f64>();

        let mut normalized_info: TokenInfo<()> = TokenInfo {
            ticker: ticker,
            market_cap: 0.0,
            price: 0.0,
            api_data: None,
        };

        for (i, info) in all_token_infos.into_iter().enumerate() {
            // the ith provider has a weighting of inner[i].1
            // so divide out by the sum of all weights
            // all operations so far has preserved order with providers
            let normalization_factor = inner[i].1 / normalization_denominator;

            normalized_info.market_cap += info.market_cap * normalization_factor;

            normalized_info.price += info.price * normalization_factor;
        }

        Ok(normalized_info)
    }
}

#[async_trait::async_trait]
impl IndexProvider for MultiProvider {
    async fn weights(&self, n: usize) -> anyhow::Result<Vec<Weight<()>>> {
        let inner = self
            .inner
            .as_ref()
            .expect("We should have built the MultiProvider");

        let all_weights = inner
            .iter()
            .map(|(provider, _)| provider.weights(n))
            .collect::<Vec<_>>();

        let all_weights = try_join_all(all_weights).await?;

        // convert into hashmaps from ticker to weight
        let mut all_weights = all_weights
            .into_iter()
            .map(|weights| {
                weights
                    .into_iter()
                    .map(|weight| (weight.info.ticker, weight))
                    .collect::<std::collections::HashMap<_, _>>()
            })
            .collect::<Vec<_>>();

        let normalization_denominator = inner.iter().map(|(_, weighting)| weighting).sum::<f64>();

        let mut ret: Vec<Weight<()>> = Vec::with_capacity(n);

        // for each ticker we get all the weigths from all the providers
        for i in 0..n {
            // just use the 0th provider as source of truth for the tickers
            // since we expect that all providers will have the same tickers
            //
            // todo this will casue error in next loop if the tickers are not the same
            let ticker = all_weights[0]
                .keys()
                .nth(i)
                .expect("We should have a ticker")
                .clone();

            let mut normalized_weight: Weight<()> = Weight {
                weight: 0.0,
                info: TokenInfo {
                    ticker: ticker,
                    market_cap: 0.0,
                    price: 0.0,
                    api_data: None,
                },
            };

            for (i, w) in all_weights.iter_mut().enumerate() {
                let current_weight = w.remove(&ticker).ok_or_else(|| {
                    anyhow::anyhow!("We should have a weight for the ticker {ticker}")
                })?;

                // the ith provider has a weighting of inner[i].1
                // so divide out by the sum of all weights
                // all operations so far has preserved order with providers
                let normalization_factor = inner[i].1 / normalization_denominator;

                normalized_weight.weight += current_weight.weight * normalization_factor;

                normalized_weight.info.market_cap +=
                    current_weight.info.market_cap * normalization_factor;

                normalized_weight.info.price += current_weight.info.price * normalization_factor;
            }

            ret.push(normalized_weight);
        }

        Ok(ret)
    }
}

/// Sets the api data to None
///
/// Casts the type to `()`
fn cast_token_info<T>(info: TokenInfo<T>) -> TokenInfo<()> {
    TokenInfo {
        ticker: info.ticker,
        market_cap: info.market_cap,
        price: info.price,
        api_data: None,
    }
}
