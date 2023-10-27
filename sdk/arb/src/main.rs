use clap::Parser;
use ethers::{providers::Http, types::U256};
use orchestrator::Orchestrator;
use utils::run_with_shutdown;

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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    crate::utils::start_tracing_sub();

    let _ = Args::parse();

    let middleware = std::sync::Arc::new(
        ethers::providers::Provider::<Http>::try_from("https://eth.llamarpc.com").unwrap(),
    );

    let data_provider = crate::api::cmc::CMCDataProvider::new(
        &std::env::var("CMC_API_KEY").expect("Failed to load CMC_API_KEY from env"),
    )
    .unwrap();

    let signer = ethers::signers::LocalWallet::new(&mut rand::thread_rng());

    let tx = eth_executor::EthExecutor::spawn(middleware.clone(), signer, 2);

    tx.send(eth_executor::Command::Mint {
        amount: U256::zero(),
    })
    .await?;

    let orc = Orchestrator::new(
        middleware,
        data_provider,
        tx,
        tokio::time::Duration::from_secs(10),
    )
    .add_strategy(Box::new(strategy::long_tail_float::LongTailFloat));

    let _ = orc.run().await;

    Ok(())
}
