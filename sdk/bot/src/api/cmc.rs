use super::{DataProvider, RetryClient};
use crate::truth::KnownTicker;
use reqwest::{header::HeaderValue, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct CMCDataProvider {
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMCAssetResponse {
    status: CMCResponseStatus,
    data: Vec<CMCAsset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMCQuoteResponse {
    status: CMCResponseStatus,
    /// key is the id of the asset
    pub data: HashMap<String, CMCQuote>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMCQuote {
    id: u64,
    #[serde(rename = "cmc_rank")]
    rank: u64,
    name: String,
    symbol: String,
    circulating_supply: f64,
    /// just need the usd quote for price
    #[serde(rename = "quote")]
    numeraire: HashMap<String, CMCQuoteData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CMCQuoteData {
    price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMCAsset {
    id: u64,
    rank: u64,
    name: String,
    symbol: String,
    slug: String,
    is_active: u64,
    first_historical_data: String,
    last_historical_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CMCResponseStatus {
    timestamp: String,
    error_code: u64,
    error_message: Option<String>,
    elapsed: u64,
    credit_count: u64,
    notice: Option<String>,
}

impl CMCDataProvider {
    pub fn new(api_key: &str) -> anyhow::Result<Self> {
        Ok(Self {
            client: Client::builder()
                .default_headers({
                    let mut headers = reqwest::header::HeaderMap::new();

                    headers.insert(
                        reqwest::header::CONTENT_TYPE,
                        reqwest::header::HeaderValue::from_static("application/json"),
                    );

                    headers.insert(
                        reqwest::header::ACCEPT,
                        reqwest::header::HeaderValue::from_static("application/json"),
                    );

                    headers.insert("X-CMC_PRO_API_KEY", HeaderValue::from_str(api_key)?);

                    headers
                })
                .build()?,
        })
    }

    pub async fn quote_symbol(&self, symbols: &[&str]) -> anyhow::Result<CMCQuoteResponse> {
        let req = self
            .client
            .get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest")
            .query(&[(
                "symbol",
                symbols
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            )]);

        RetryClient::retry(req, 5, 1).await
    }
}

#[async_trait::async_trait]
impl DataProvider for CMCDataProvider {
    async fn price(&self, ticker: KnownTicker) -> anyhow::Result<f64> {
        let quote = self.quote_symbol(&[&ticker.to_str()]).await?;

        let (_, quote) = quote
            .data
            .into_iter()
            .next()
            .expect(&format!("we should have gotten a quote for {}", ticker));

        Ok(quote
            .numeraire
            .get("USD")
            .expect("There to be a usd price")
            .price)
    }
}
