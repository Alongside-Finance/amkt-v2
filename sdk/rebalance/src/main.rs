use bindings::i_vault::InvokeSetNominalCall;
use bounty::{
    create_bounty, get_bounty_hash, get_fulfill_bounty_calldata, get_quote_for_bounty,
    get_set_hash_calldata,
};
use data::cmc::CMCDataProvider;
use ethers::abi::AbiEncode;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{Address, Bytes, U256};
use indicies::{IndexProvider, MarketCapDataProvider, NominalsProvider};
use std::fs;
use std::str::FromStr;

use std::time::SystemTime;

pub mod bounty;
pub mod data;
pub mod indicies;
pub mod truth;

#[derive(Debug)]
pub struct Config {
    pub num_assets: usize,
    pub index_value: f64,
    pub bounty_duration: u64,
    pub bounty_id: &'static str,
    pub invokeable_bounty_address: Address,
    pub active_bounty_address: Address,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let provider = CMCDataProvider::new(
        &std::env::var("CMC_API_KEY").expect("Failed to load CMC_API_KEY from env"),
    )?;

    let middleware = Provider::<Http>::try_from("https://eth.llamarpc.com")?;

    let config = Config {
        num_assets: 15,
        index_value: 0.08576,
        bounty_duration: 60 * 60 * 24 * 12, // 1 days
        bounty_id: "alongside::bounty::0",
        invokeable_bounty_address: Address::from_str("0x58947673e1CE2e374F39DcfAEAd210803EcbAD50")?,
        active_bounty_address: Address::from_str("0x7dD6663df8F5C3E47Eb7C25a6faCb9cE039E76aC")?,
    };
    let weights = provider.weights(config.num_assets).await?;

    let nominals = provider
        .nominals(config.num_assets, config.index_value)
        .await?;

    let one = nominals.get(1).expect("there to be an index here");

    let timestamp = middleware
        .get_block(middleware.get_block_number().await?)
        .await?
        .expect("to get a block")
        .timestamp;

    let deadline = timestamp + U256::from(config.bounty_duration);

    let salt = ethers::utils::keccak256(config.bounty_id);

    let bounty = create_bounty(middleware.clone().into(), nominals.clone(), deadline, salt).await?;

    let bounty_hash = get_bounty_hash(
        middleware.clone().into(),
        config.invokeable_bounty_address,
        bounty.clone(),
    )
    .await?;

    let current_bounty_quote = get_quote_for_bounty(
        middleware.clone().into(),
        config.invokeable_bounty_address,
        bounty.clone(),
    )
    .await?;

    let bounty_set_calldata = get_set_hash_calldata(
        middleware.clone().into(),
        config.active_bounty_address,
        bounty_hash,
    )
    .await?;

    let bounty_fulfill_calldata = get_fulfill_bounty_calldata(
        middleware.clone().into(),
        config.invokeable_bounty_address,
        bounty.clone(),
    )
    .await?;

    // Format the data as a string
    let data = format!(
        "config: {:#?}\nweights: {:#?}\nnominals: {:#?}\nbounty: {:#?}\nsalt: {:#?}\nbounty_hash: {:#?}\ncurrent_bounty_quote: {:#?}\nbounty_set_calldata: {:#?}\nbounty_fulfill_calldata: {:#?}",
        config,
        weights,
        nominals,
        bounty,
        salt.encode_hex(),
        bounty_hash.encode_hex(),
        current_bounty_quote,
        bounty_set_calldata,
        bounty_fulfill_calldata
    );

    // Get the current timestamp
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // Create the output directory if it doesn't exist
    fs::create_dir_all("out")?;

    // Save the data to a file
    let file_name = format!("out/rebalance-{}.txt", now);
    fs::write(file_name, data)?;
    Ok(())
}
