use clap::Parser;
use ethers::types::Address;
use serde::{Deserialize, Serialize};
use v3::pool::FeeTier;

lazy_static::lazy_static! {
    pub static ref ISSUANCE_ADDRESS: Address = "0x46ad895B3f627b22A4B14E010d0F2ddc5088492A".parse().unwrap();
    pub static ref VAULT_ADDRESS: Address = "0xE289fa751f8E4A05035B75B8033175ab4F38adDB".parse().unwrap();
    pub static ref UNISWAP_V3_ROUTER_ADDRESS: Address = "0xE592427A0AEce92De3Edee1F18E0157C05861564".parse().unwrap();
    pub static ref EXECUTE: bool = crate::Args::parse().execute;
}

pub const AMKT_KNOWN_FEE: FeeTier = FeeTier::Mid;

/// A known list of tickers expected to be actively maintained
/// Tickers and addresses are supplied like
/// `create_known_tickers!((ETH, "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"), (BTC, "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc3"))`
///
/// Ensure tickers are all uppercase, and addresses are checksummed
macro_rules! create_known_tickers {
    ($(($ticker:ident, $addr:tt),)+) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub enum KnownTicker {
            $($ticker,)+
        }

        impl KnownTicker {
            pub fn to_str(&self) -> &'static str {
                match self {
                    $(Self::$ticker => stringify!($ticker),)+
                }
            }

            pub fn known_addresss(&self) -> Address {
                match self {
                    $(Self::$ticker => $addr.parse().map_err(|e| anyhow::anyhow!("Expected known address {}: {} to parse", self.to_str(), e)).unwrap(),)+
                }
            }
        }

        impl std::fmt::Display for KnownTicker {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_str())
          }
        }

        impl std::str::FromStr for KnownTicker {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.to_uppercase().as_str() {
                    $(stringify!($ticker) => Ok(Self::$ticker),)+
                    _ => Err(anyhow::anyhow!("Tried to call from_str on an unknown ticker {}", s)),
                }
            }
        }

        impl std::convert::TryFrom<Address> for KnownTicker {
            type Error = anyhow::Error;

            fn try_from(address: Address) -> Result<Self, Self::Error> {
                let address = format!("{:#?}", address);

                $(
                    if address.eq_ignore_ascii_case($addr) {
                        return Ok(Self::$ticker);
                    }
                )+

                Err(anyhow::anyhow!("Tried to call KnownTicker::try_from on an unknown address {:?}", address))
            }
        }

    };
    ($(($ticker:ident, $addr:tt)),+) => {
        create_known_tickers!($(($ticker, $addr),)+);
    };
}

create_known_tickers! {
    (BTC,"0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599"),
    (ETH,"0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0"),
    (MATIC,"0x7D1AfA7B718fb893dB30A3aBc0Cfc608AaCfeBB0"),
    (FTM,"0x4E15361FD6b4BB609Fa63C81A2be19d873717870"),
    (SHIB,"0x95aD61b0a150d79219dCF64E1E6Cc01f0B64C4cE"),
    (LINK,"0x514910771AF9Ca656af840dff83E8264EcF986CA"),
    (UNI,"0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984"),
    (LDO,"0x5A98FcBEA516Cf06857215779Fd812CA3beF1B32"),
    (CRO,"0xA0b73E1Ff0B80914AB6fe0444E65848C4C34450b"),
    (QNT,"0x4a220E6096B25EADb88358cb44068A3248254675"),
    (ARB,"0xB50721BCf8d664c30412Cfbc6cf7a15145234ad1"),
    (MKR,"0x9f8F72aA9304c8B593d555F12eF6589cC3A579A2"),
    (AAVE,"0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9"),
    (GRT,"0xc944E90C64B2c07662A292be6244BDf05Cda44a7"),
    (BNB,"0x418D75f65a02b3D53B2418FB8E1fe493759c7605"),
    (SOL,"0xD31a59c85aE9D8edEFeC411D448f90841571b89c"),
    (AVAX,"0x85f138bfEE4ef8e540890CFb48F620571d67Eda3"),
    (OP,"0x1df721D242E0783F8fCab4A9FfE4F35bdf329909"),
    (MNT,"0x3c3a81e81dc49A522A592e7622A7E711c06bf354")
}
