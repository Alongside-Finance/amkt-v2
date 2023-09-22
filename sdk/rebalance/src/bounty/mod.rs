use crate::indicies::Nominals;
use bindings::{
    active_bounty::ActiveBounty,
    erc20::ERC20,
    invokeable_bounty::{Bounty, InvokeableBounty},
    shared_types::TokenInfo,
};
use ethers::providers::Middleware;
use ethers::types::{Address, Bytes, U256};

pub async fn create_bounty<M: Middleware + 'static>(
    middleware: std::sync::Arc<M>,
    nominals: Vec<Nominals>,
    deadline: U256,
    salt: [u8; 32],
) -> anyhow::Result<Bounty> {
    let mut bounty = Bounty::default();
    bounty.infos.reserve(nominals.len());

    for Nominals { ticker, amount } in nominals {
        let contract = ERC20::new(ticker.known_addresss(), middleware.clone());

        let decimals = contract.decimals().await?;
        bounty.infos.push(TokenInfo {
            token: ticker.known_addresss(),
            units: ethers::utils::parse_units(amount, decimals as usize)?.into(),
        });
    }

    bounty.deadline = deadline;
    bounty.salt = salt;

    Ok(bounty)
}

pub async fn get_bounty_hash<M: Middleware + 'static>(
    middleware: std::sync::Arc<M>,
    target: Address,
    bounty: Bounty,
) -> anyhow::Result<[u8; 32]> {
    let invokeable_bounty = InvokeableBounty::new(target, middleware);
    let bounty_hash = invokeable_bounty.hash_bounty(bounty).await?;
    Ok(bounty_hash)
}

pub async fn get_set_hash_calldata<M: Middleware + 'static>(
    middleware: std::sync::Arc<M>,
    target: Address,
    bounty_hash: [u8; 32],
) -> anyhow::Result<Bytes> {
    let invokeable_bounty = ActiveBounty::new(target, middleware);
    let calldata = invokeable_bounty
        .set_hash(bounty_hash)
        .calldata()
        .expect("to get calldata");
    Ok(calldata)
}

pub async fn get_fulfill_bounty_calldata<M: Middleware + 'static>(
    middleware: std::sync::Arc<M>,
    target: Address,
    bounty: Bounty,
) -> anyhow::Result<Bytes> {
    let invokeable_bounty = InvokeableBounty::new(target, middleware);
    let calldata = invokeable_bounty
        .fulfill_bounty(bounty, false)
        .calldata()
        .expect("to get calldata");
    Ok(calldata)
}

pub async fn get_quote_for_bounty<M: Middleware + 'static>(
    middleware: std::sync::Arc<M>,
    target: Address,
    bounty: Bounty,
) -> anyhow::Result<(Vec<TokenInfo>, Vec<TokenInfo>)> {
    let invokeable_bounty = InvokeableBounty::new(target, middleware);
    let quote: (Vec<TokenInfo>, Vec<TokenInfo>) = invokeable_bounty.quote_bounty(bounty).await?;
    Ok(quote)
}
