use std::collections::HashMap;

use super::{DataProvider, RetryClient};
use crate::{data::TokenInfo, indicies::MarketCapDataProvider, truth::KnownTicker};
use reqwest::{header::HeaderValue, Client};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
    pub data: HashMap<u64, CMCQuote>,
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

    async fn ids_by_marketcap(&self, n: usize) -> anyhow::Result<CMCAssetResponse> {
        let req = self
            .client
            .get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/map")
            .query(&[("limit", n.to_string()), ("sort", "cmc_rank".to_string())]);

        RetryClient::retry(req, 5, 1).await
    }

    pub async fn quote(&self, ids: &[u64]) -> anyhow::Result<CMCQuoteResponse> {
        let req = self
            .client
            .get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest")
            .query(&[(
                "id",
                ids.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            )]);

        RetryClient::retry(req, 5, 1).await
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
    type ApiData = CMCQuote;

    async fn token_info(&self, ticker: KnownTicker) -> anyhow::Result<TokenInfo<Self::ApiData>> {
        let quote = self.quote_symbol(&[&ticker.to_str()]).await?;

        let (_, quote) = quote
            .data
            .into_iter()
            .next()
            .expect("We should have gotten a quote");

        Ok(TokenInfo::from((ticker, quote)))
    }
}

#[async_trait::async_trait]
impl MarketCapDataProvider for CMCDataProvider {
    async fn top_n_assets_by_market_cap(
        &self,
        n: usize,
    ) -> anyhow::Result<Vec<TokenInfo<CMCQuote>>> {
        // multiply by 10 to account for the fact that some assets may not be supported
        let ids = self.ids_by_marketcap(n * 10).await?;

        // do an initial filter to remove the assets that are not supported
        let filtered_ids = ids
            .data
            .iter()
            .filter_map(|a| {
                if is_supported_asset(a) {
                    tracing::info!("Found a supported asset: {:#?}", a);
                    Some(a.id)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if filtered_ids.len() < n {
            return Err(anyhow::anyhow!(
                "Failed to get {} assets, only got {}",
                n,
                filtered_ids.len()
            ));
        }

        // double check the returned quotes, removing the ones that are not supported
        let mut needs_trim = self
            .quote(&filtered_ids)
            .await?
            .data
            .into_iter()
            .filter_map(|(_, quote)| {
                tracing::info!("Found quote, {:#?}", quote);

                match KnownTicker::from_str(&quote.symbol) {
                    Ok(ticker) => Some((ticker, quote)),
                    Err(_) => {
                        tracing::warn!(
                            "Unsupported asset but we got a quote, this is a bug, {:#?}",
                            quote.symbol
                        );
                        None
                    }
                }
            })
            .collect::<Vec<_>>();

        // sort so we take the subset with the greatest market cap
        // lower rank means higher market cap
        needs_trim.sort_by(|(_, a), (_, b)| a.rank.cmp(&b.rank));

        let trimmed = needs_trim
            .into_iter()
            .map(|(ticker, quote)| TokenInfo::from((ticker, quote)))
            .take(n)
            .collect::<Vec<_>>();

        if trimmed.len() != n {
            return Err(anyhow::anyhow!(
                "Failed to get {} assets, only got {}",
                n,
                trimmed.len()
            ));
        } else {
            return Ok(trimmed);
        }
    }
}

pub fn is_supported_asset(asset: &CMCAsset) -> bool {
    match KnownTicker::from_str(&asset.symbol) {
        Ok(_) => true,
        Err(e) => {
            tracing::info!("intial filter: {} not supported", &asset.symbol);
            false
        }
    }
}

impl From<(KnownTicker, CMCQuote)> for TokenInfo<CMCQuote> {
    fn from((ticker, quote): (KnownTicker, CMCQuote)) -> Self {
        let data = quote.numeraire.get("USD").expect("USD to exist");

        TokenInfo {
            ticker,
            market_cap: data.price * quote.circulating_supply,
            price: data.price,
            api_data: Some(quote),
        }
    }
}
