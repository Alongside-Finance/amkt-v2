///`Bounty((address,uint256)[],uint256,bytes32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Bounty {
    pub infos: ::std::vec::Vec<TokenInfo>,
    pub deadline: ::ethers::core::types::U256,
    pub salt: [u8; 32],
}
///`Checkpoint(uint32,uint224)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Checkpoint {
    pub from_block: u32,
    pub votes: ::ethers::core::types::U256,
}
///`InvokeERC20Args(address,address,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct InvokeERC20Args {
    pub token: ::ethers::core::types::Address,
    pub to: ::ethers::core::types::Address,
    pub amount: ::ethers::core::types::U256,
}
///`SetNominalArgs(address,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct SetNominalArgs {
    pub token: ::ethers::core::types::Address,
    pub virtual_units: ::ethers::core::types::U256,
}
///`FuzzSelector(address,bytes4[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct FuzzSelector {
    pub addr: ::ethers::core::types::Address,
    pub selectors: ::std::vec::Vec<[u8; 4]>,
}
///`TokenInfo(address,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct TokenInfo {
    pub token: ::ethers::core::types::Address,
    pub units: ::ethers::core::types::U256,
}
