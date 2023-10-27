pub mod cmc;
use crate::truth::KnownTicker;
use serde::de::DeserializeOwned;

#[async_trait::async_trait]
pub trait DataProvider {
    async fn price(&self, ticker: KnownTicker) -> anyhow::Result<f64>;
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
                        let body = resp.text().await?;
                        tracing::trace!("Got a successful response: {:#?}", body);
                        return Ok(serde_json::from_str(&body)?);
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
