pub mod cmc;
pub mod multi;
use crate::truth::KnownTicker;
use serde::de::DeserializeOwned;

/// could be expanded upon if we had a differnt metric
#[derive(Debug, Clone, Copy)]
pub struct TokenInfo<APIData> {
    pub ticker: KnownTicker,
    pub market_cap: f64,
    pub price: f64,
    pub api_data: Option<APIData>,
}

impl<APIData> TokenInfo<APIData> {
    pub fn supply(&self) -> f64 {
        (self.market_cap / self.price).floor()
    }
}

#[async_trait::async_trait]
pub trait DataProvider: Send + Sync {
    type ApiData;

    async fn token_info(&self, ticker: KnownTicker) -> anyhow::Result<TokenInfo<Self::ApiData>>;
}

pub struct RetryClient;

impl RetryClient {
    pub async fn retry<T: DeserializeOwned>(
        request: reqwest::RequestBuilder,
        mut retries: usize,
        delay: u64,
    ) -> anyhow::Result<T> {
        let mut tries = 0;

        loop {
            tries += 1;

            // this should only error out if the request is a stream
            let clone = request
                .try_clone()
                .expect("That you can clone the request builder");

            match clone.send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        return Ok(resp.json::<T>().await?);
                    } else {
                        tracing::warn!("Got a non 200 response: {:#?}", resp);
                    }
                }
                Err(e) => {
                    tracing::warn!("Reqwest error: {:#?}", e);
                }
            }

            if retries == 0 {
                return Err(anyhow::anyhow!(
                    "Failed to get a successful response after {} tries",
                    tries
                ));
            }

            retries -= 1;

            tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
        }
    }
}
