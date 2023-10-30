use clap::Parser;
use ethers::{providers::Http, types::U256};
use orchestrator::Orchestrator;

pub mod api;
pub mod eth_executor;
pub mod orchestrator;
pub mod strategy;
pub mod truth;
pub mod utils;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Whether or not to actually submit the transactions
    #[clap(long, short)]
    execute: bool,

    /// The number of confirmations to wait for per tx
    #[clap(long, short, requires = "execute", default_value = "2")]
    confirmations: usize,

    /// Whether or not to shutdown on a tx error
    #[clap(long, short, requires = "retries", requires = "reverts")]
    shutdown_on_tx_error: bool,

    /// The number of retries (excluding reverts) per bundle before a shutdown is triggered
    #[clap(long, requires = "shutdown_on_tx_error")]
    retries: usize,

    /// The number of reverts we will retry on before a shutdown is triggered
    #[clap(long, requires = "shutdown_on_tx_error")]
    reverts: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    crate::utils::start_tracing_sub();

    let Args {
        shutdown_on_tx_error,
        retries,
        reverts,
        confirmations,
        ..
    } = Args::parse();

    let middleware = std::sync::Arc::new(
        ethers::providers::Provider::<Http>::try_from("https://eth.llamarpc.com").unwrap(),
    );

    let data_provider = crate::api::cmc::CMCDataProvider::new(
        &std::env::var("CMC_API_KEY").expect("Failed to load CMC_API_KEY from env"),
    )
    .unwrap();

    let signer = ethers::signers::LocalWallet::new(&mut rand::thread_rng());

    let (tx, shutdown) = eth_executor::EthExecutor::spawn(
        middleware.clone(),
        signer,
        confirmations,
        retries,
        reverts,
        shutdown_on_tx_error,
    );

    tx.send(vec![eth_executor::Command::Mint {
        amount: U256::zero(),
    }])
    .await?;

    let orc = Orchestrator::new(
        middleware,
        data_provider,
        tx,
        shutdown,
        tokio::time::Duration::from_secs(10),
    )
    .add_strategy(Box::new(strategy::long_tail_float::LongTailFloat));

    let _ = orc.run().await;

    Ok(())
}
