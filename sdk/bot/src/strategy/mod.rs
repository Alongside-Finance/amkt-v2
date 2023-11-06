pub mod approval;
pub mod long_tail_float;
use crate::{
    api::DataProvider,
    eth_executor::{Command, ExecutorHandle},
    orchestrator::Orchestrator,
};
use ethers::middleware::Middleware;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait Strategy<M, P>
where
    M: Middleware + Send + Sync + 'static,
    P: DataProvider + Send + Sync + 'static,
    Self: Send + Sync + 'static,
{
    async fn execute(
        &self,
        executor_handle: ExecutorHandle,
        data: Arc<Orchestrator<M, P>>,
    ) -> anyhow::Result<()>;
}
