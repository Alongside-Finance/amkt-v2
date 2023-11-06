use clap::Parser;
use ethers::{providers::Http, signers::LocalWallet, types::U256};
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

    /// Whether or not to shutdown on when we breach the retry/revert limit
    #[clap(long, short)]
    shutdown_on_tx_error: bool,

    /// The number of retries per bundle before a shutdown may be triggered
    /// or bundle is skipped
    ///
    /// generally a retry is for errors that dont cost money
    #[clap(long, default_value = "3")]
    retries: usize,

    /// The number of reverts we will retry on before a shutdown may be triggered
    /// or bundle is skipped
    #[clap(long, default_value = "2")]
    reverts: usize,

    /// The time to sleep in between trying all the strategies
    #[clap(long, short = 't', default_value = "20")]
    sleep: u64,

    /// The private key that will be used to sign all transactions
    /// You can optionally set the env var $ARB_BOT_PK
    #[clap(long)]
    private_key: Option<String>,

    /// You can optionally set the env var $CMC_API_KEY
    #[clap(long)]
    cmc_api_key: Option<String>,

    /// You can optionally set the env var $PROVIDER_URL
    #[clap(long)]
    provider_url: Option<String>,
}

pub struct Secrets {
    pub cmc_api_key: String,
    pub pk: String,
    pub provider_url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    crate::utils::start_tracing_sub();

    let Args {
        shutdown_on_tx_error,
        retries,
        reverts,
        confirmations,
        sleep,
        ..
    } = Args::parse();

    let Secrets {
        pk,
        cmc_api_key,
        provider_url,
    } = try_get_secrets(Args::parse())?;

    let middleware =
        std::sync::Arc::new(ethers::providers::Provider::<Http>::try_from(provider_url).unwrap());

    let data_provider = crate::api::cmc::CMCDataProvider::new(&cmc_api_key)?;

    let signer = pk.parse::<LocalWallet>()?;

    let (handle, shutdown) = eth_executor::EthExecutor::spawn(
        middleware.clone(),
        signer,
        confirmations,
        retries,
        reverts,
        shutdown_on_tx_error,
    );

    let orc = Orchestrator::new(
        middleware,
        handle,
        data_provider,
        shutdown,
        tokio::time::Duration::from_secs(sleep),
    )
    .add_strategy(Box::new(strategy::approval::Approvals))
    .add_strategy(Box::new(strategy::long_tail_float::LongTailFloat));

    let _ = orc.run().await;

    Ok(())
}

fn try_get_secrets(args: Args) -> anyhow::Result<Secrets> {
    let arb_bot_pk: String;
    let cmc_api_key: String;
    let provider_url: String;

    if let Some(pk) = args.private_key {
        arb_bot_pk = pk;
    } else {
        arb_bot_pk = std::env::var("ARB_BOT_PK")?;
    }

    if let Some(key) = args.cmc_api_key {
        cmc_api_key = key;
    } else {
        cmc_api_key = std::env::var("CMC_API_KEY")?;
    }

    if let Some(url) = args.provider_url {
        provider_url = url;
    } else {
        match std::env::var("PROVIDER_URL") {
            Ok(url) => provider_url = url,
            Err(_) => provider_url = "https://eth.llamarpc.com".to_string(),
        }
    }

    Ok(Secrets {
        pk: arb_bot_pk,
        cmc_api_key,
        provider_url,
    })
}
