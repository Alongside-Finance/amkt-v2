pub mod long_tail_float;
use crate::{api::DataProvider, eth_executor::Command, orchestrator::Orchestrator};
use ethers::middleware::Middleware;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait Strategy<M, P>
where
    M: Middleware + Send + Sync + 'static,
    P: DataProvider + Send + Sync + 'static,
    Self: Send + Sync,
{
    async fn execute(
        &self,
        tx: tokio::sync::mpsc::Sender<Command>,
        data: Arc<Orchestrator<M, P>>,
    ) -> anyhow::Result<()>;
}
