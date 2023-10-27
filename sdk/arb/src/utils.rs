use bindings::erc20::ERC20;
use ethers::{middleware::Middleware, types::Address};
use tokio::{
    select, signal,
    task::{spawn, JoinHandle},
};

/// useful for when you want to spawn a task that doesnt return usally
///
/// this task will return when a ctrl-c is received
pub fn spawn_with_shutdown<Fut>(future: Fut) -> JoinHandle<Option<()>>
where
    Fut: std::future::Future<Output = ()> + Send + 'static,
{
    spawn(run_with_shutdown(future))
}

/// useful for when you want to await a future that blocks the current context
///
/// this future will return when a ctrl-c is received
pub async fn run_with_shutdown<Fut, T>(future: Fut) -> Option<T>
where
    Fut: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    select! {
        ret = future => Some(ret),
        _ = signal::ctrl_c() => {
            None
        }
    }
}

pub async fn decimals<M: Middleware + 'static>(
    middleware: std::sync::Arc<M>,
    token: Address,
) -> anyhow::Result<u8> {
    let erc20 = ERC20::new(token, middleware);
    Ok(erc20.decimals().call().await?)
}

pub fn start_tracing_sub() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "bot=debug".to_string());

    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_max_level(tracing::Level::TRACE)
        .with_env_filter(filter)
        .init();
}
