pub use staking::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod staking {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_signer"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_amkt"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_bonusBps"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DENOMINATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DENOMINATOR"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DOMAIN_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DOMAIN_TYPEHASH"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("STAKE_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("STAKE_TYPEHASH"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("amkt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("amkt"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("bonusNumerator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bonusNumerator"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositReward"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("depositReward"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("earlyExit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("earlyExit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("vaultId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("exit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("vaultId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("referred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("referred"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeFromRewardPot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeFromRewardPot",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rewardPot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rewardPot"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setBonusBps"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setBonusBps"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bps"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setSigner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setSigner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_signer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stake"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("referrer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("staked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("staked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("userVaultLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("userVaultLength"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("vaults"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("vaults"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("referrer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bonus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("exited"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("version"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("version"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("whitelistSigner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("whitelistSigner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AMKTStakingAlreadyExited"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AMKTStakingAlreadyExited",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AMKTStakingCantExit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AMKTStakingCantExit",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AMKTStakingDontEarlyExit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AMKTStakingDontEarlyExit",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AMKTStakingInvalidOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AMKTStakingInvalidOwner",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AMKTStakingInvalidSigner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AMKTStakingInvalidSigner",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static STAKING_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x13L8\x03\x80a\x13L\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\xDCV[a\083a\0pV[`\x04\x80T`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x05\x80T\x93\x90\x94\x16\x92\x16\x91\x90\x91\x17\x90\x91U`\x06Ua\x01\x18V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xD7W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\0\xF1W`\0\x80\xFD[a\0\xFA\x84a\0\xC0V[\x92Pa\x01\x08` \x85\x01a\0\xC0V[\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[a\x12%\x80a\x01'`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01MW`\x005`\xE0\x1C\x80c\x89\xEE\t\xF0\x11a\0\xC3W\x80c\x9B\xEC\xC2Z\x11a\0|W\x80c\x9B\xEC\xC2Z\x14a\x03$W\x80c\xA2\x02\xCAx\x14a\x037W\x80c\xB8\xAF=>\x14a\x03@W\x80c\xC6\x149\xDE\x14a\x03SW\x80c\xEF\x81\xB4\xD4\x14a\x03fW\x80c\xF2\xFD\xE3\x8B\x14a\x03yW`\0\x80\xFD[\x80c\x89\xEE\t\xF0\x14a\x02fW\x80c\x8Cd\xEAJ\x14a\x02\x8DW\x80c\x8D\xA5\xCB[\x14a\x02\xDAW\x80c\x8F\x16\x98\x16\x14a\x02\xFFW\x80c\x91\x8F\x86t\x14a\x03\x12W\x80c\x93\xADt@\x14a\x03\x1BW`\0\x80\xFD[\x80c6D\xE5\x15\x11a\x01\x15W\x80c6D\xE5\x15\x14a\x01\xF0W\x80c<\xA0.|\x14a\x01\xF8W\x80cT\xFDMP\x14a\x02\x0BW\x80cl\x19\xE7\x83\x14a\x028W\x80cqP\x18\xA6\x14a\x02KW\x80c\x7F\x86a\xA1\x14a\x02SW`\0\x80\xFD[\x80c\x15,d\xE8\x14a\x01RW\x80c\x1E' \xFF\x14a\x01xW\x80c `kp\x14a\x01\x8DW\x80c!\xDC\x83g\x14a\x01\xB4W\x80c2t($\x14a\x01\xC7W[`\0\x80\xFD[a\x01ea\x01`6`\x04a\x0F\xF0V[a\x03\x8CV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x8Ba\x01\x866`\x04a\x10wV[a\x07FV[\0[a\x01e\x7F\xD8|\xD6\xEFy\xD4\xE2\xB9^\x15\xCE\x8A\xBFs-\xB5\x1E\xC7q\xF1\xCA.\xDC\xCF\"\xA4lr\x9A\xC5dr\x81V[a\x01\x8Ba\x01\xC26`\x04a\x10wV[a\x07\xE5V[a\x01ea\x01\xD56`\x04a\x10\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[a\x01ea\x08;V[a\x01\x8Ba\x02\x066`\x04a\x10wV[a\t\x15V[a\x02+`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\x03`\xFC\x1B\x81RP\x81V[`@Qa\x01o\x91\x90a\x10\xB2V[a\x01\x8Ba\x02F6`\x04a\x10\x90V[a\t\"V[a\x01\x8Ba\tLV[a\x01\x8Ba\x02a6`\x04a\x10wV[a\t`V[a\x01e\x7F\x94\xE2\xAA\xEE91\x98\x1D\x99\x04\xD5K\x9D~w\x9E\x9C\xF3R\xE3\xF4tq\x94\xC4\xDFm4\x12\r.\xC8\x81V[a\x02\xA0a\x02\x9B6`\x04a\x10wV[a\n\xFCV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x97\x88\x16\x81R\x96\x90\x95\x16` \x87\x01R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R\x15\x15`\xA0\x82\x01R`\xC0\x01a\x01oV[`\0T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01oV[a\x01ea\x03\r6`\x04a\x11\x07V[a\x0BRV[a\x01ea'\x10\x81V[a\x01e`\x07T\x81V[a\x01ea\x0326`\x04a\x11\x07V[a\x0B\x83V[a\x01e`\x06T\x81V[a\x01\x8Ba\x03N6`\x04a\x10wV[a\x0B\x9FV[`\x05Ta\x02\xE7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x04Ta\x02\xE7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x8Ba\x03\x876`\x04a\x10\x90V[a\r\xA9V[3`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 T\x81Q\x7F\x94\xE2\xAA\xEE91\x98\x1D\x99\x04\xD5K\x9D~w\x9E\x9C\xF3R\xE3\xF4tq\x94\xC4\xDFm4\x12\r.\xC8\x93\x81\x01\x93\x90\x93R\x90\x82\x01\x93\x90\x93R``\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x86\x16`\x80\x82\x01R`\xA0\x81\x01\x92\x90\x92R\x90a\x04:\x90`\xC0\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` `\x1F\x87\x01\x81\x90\x04\x81\x02\x84\x01\x81\x01\x90\x92R\x85\x83R\x91\x90\x86\x90\x86\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0E'\x92PPPV[a\x04WW`@Qc\x11;\x1A\x01`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x04\x97W`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x02` \x90\x81R`@\x82 `\x03T\x81T`\x01\x81\x01\x83U\x91\x84R\x91\x90\x92 \x90\x91\x01U[`\0a'\x10`\x06T\x87a\x04\xAA\x91\x90a\x11GV[a\x04\xB4\x91\x90a\x11fV[\x90P\x80`\x07`\0\x82\x82Ta\x04\xC8\x91\x90a\x11\x88V[\x90\x91UPP`\x03\x80T3`\0\x81\x81R`\x01` \x81\x81R`@\x80\x84 \x80T\x80\x85\x01\x82U\x90\x85R\x82\x85 \x01\x86\x90U\x80Q`\xC0\x81\x01\x82R\x85\x81R`\x01`\x01`\xA0\x1B\x03\x8D\x81\x16\x93\x82\x01\x93\x84R\x81\x83\x01\x8F\x81R``\x83\x01\x8B\x81RB`\x80\x85\x01\x90\x81R`\xA0\x85\x01\x89\x81R\x8CT\x98\x89\x01\x8DU\x9B\x90\x98R\x92Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[`\x06\x90\x97\x02\x96\x87\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x92\x85\x16\x92\x90\x92\x17\x90U\x94Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8\\\x87\x01\x80T\x90\x96\x16\x90\x83\x16\x17\x90\x94U\x92Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8]\x85\x01UQ\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8^\x84\x01U\x92Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8_\x83\x01U\x94Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8`\x90\x91\x01\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90U`\x05T\x90Qc#\xB8r\xDD`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R0`$\x83\x01R`D\x82\x01\x8A\x90R\x91\x94P\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xA2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC6\x91\x90a\x11\x9FV[P`\x05T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07<\x91\x90a\x11\x9FV[PP\x94\x93PPPPV[a\x07Na\x0EYV[\x80`\x07`\0\x82\x82Ta\x07`\x91\x90a\x11\xC1V[\x90\x91UPP`\x05T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\xBDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xE1\x91\x90a\x11\x9FV[PPV[a\x07\xEDa\x0EYV[\x80`\x07`\0\x82\x82Ta\x07\xFF\x91\x90a\x11\x88V[\x90\x91UPP`\x05T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01a\x07\x9EV[`@\x80Q\x80\x82\x01\x82R`\x01\x81R`\x03`\xFC\x1B` \x91\x82\x01R\x81Q\x7F\xD8|\xD6\xEFy\xD4\xE2\xB9^\x15\xCE\x8A\xBFs-\xB5\x1E\xC7q\xF1\xCA.\xDC\xCF\"\xA4lr\x9A\xC5dr\x81\x83\x01R\x7F(\xDFe\xE3\xE7\x87\xDAu-x\xD9\x814J+\x93\x80\x9AJN\x14f\xABs\x03\xE4L\x1E\x89L\x98N\x81\x84\x01R\x7F\x04HR\xB2\xA6p\xAD\xE5@~x\xFB(c\xC5\x1D\xE9\xFC\xB9eB\xA0q\x86\xFE:\xED\xA6\xBB\x8A\x11m``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R\x7F\x14n\xB7\x97E\xAF\x93\x8D\xD3\\\0\x8F\x08\xE6\xA3x#\xA1'\x8B9-\xF4w\xF9\x84\x9EF\x19V\xC2z`\xC0\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xE0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[a\t\x1Da\x0EYV[`\x06UV[a\t*a\x0EYV[`\x04\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\tTa\x0EYV[a\t^`\0a\x0E\xB3V[V[3`\x01`\x01`\xA0\x1B\x03\x16`\x03\x82\x81T\x81\x10a\t}Wa\t}a\x11\xD9V[`\0\x91\x82R` \x90\x91 `\x06\x90\x91\x02\x01T`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xB5W`@Qc\xFD\x99\xC6\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x81\x81T\x81\x10a\t\xC8Wa\t\xC8a\x11\xD9V[`\0\x91\x82R` \x90\x91 `\x05`\x06\x90\x92\x02\x01\x01T`\xFF\x16\x15\x15`\x01\x03a\n\x01W`@Qc'\xFC\xA5K`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[B`\x03\x82\x81T\x81\x10a\n\x15Wa\n\x15a\x11\xD9V[\x90`\0R` `\0 \x90`\x06\x02\x01`\x04\x01Tc\x01\xE13\x80a\n6\x91\x90a\x11\xC1V[\x10a\nTW`@Qc\xC9\xE5\xA5\xEB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x03\x82\x81T\x81\x10a\niWa\nia\x11\xD9V[`\0\x91\x82R` \x90\x91 `\x06\x90\x91\x02\x01`\x05\x90\x81\x01\x80T`\xFF\x19\x16\x92\x15\x15\x92\x90\x92\x17\x90\x91UT`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xA9\x05\x9C\xBB\x913\x91\x85\x90\x81\x10a\n\xB7Wa\n\xB7a\x11\xD9V[`\0\x91\x82R` \x90\x91 `\x02`\x06\x90\x92\x02\x01\x01T`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01a\x07\x9EV[`\x03\x81\x81T\x81\x10a\x0B\x0CW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x06\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x90\x95\x01T`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x96P\x93\x90\x92\x16\x93\x90\x92`\xFF\x16\x86V[`\x01` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x0BnW`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x91P\x91PPT\x81V[`\x02` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x0BnW`\0\x80\xFD[3`\x01`\x01`\xA0\x1B\x03\x16`\x03\x82\x81T\x81\x10a\x0B\xBCWa\x0B\xBCa\x11\xD9V[`\0\x91\x82R` \x90\x91 `\x06\x90\x91\x02\x01T`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\xF4W`@Qc\xFD\x99\xC6\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x81\x81T\x81\x10a\x0C\x07Wa\x0C\x07a\x11\xD9V[`\0\x91\x82R` \x90\x91 `\x05`\x06\x90\x92\x02\x01\x01T`\xFF\x16\x15\x15`\x01\x03a\x0C@W`@Qc'\xFC\xA5K`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[B`\x03\x82\x81T\x81\x10a\x0CTWa\x0CTa\x11\xD9V[\x90`\0R` `\0 \x90`\x06\x02\x01`\x04\x01Tc\x01\xE13\x80a\x0Cu\x91\x90a\x11\xC1V[\x10\x15a\x0C\x94W`@Qc\xFAn.A`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x03\x82\x81T\x81\x10a\x0C\xA9Wa\x0C\xA9a\x11\xD9V[\x90`\0R` `\0 \x90`\x06\x02\x01`\x05\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x03\x81\x81T\x81\x10a\x0C\xE4Wa\x0C\xE4a\x11\xD9V[\x90`\0R` `\0 \x90`\x06\x02\x01`\x03\x01T`\x07`\0\x82\x82Ta\r\x07\x91\x90a\x11\xC1V[\x90\x91UPP`\x05T`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xA9\x05\x9C\xBB\x913\x91\x85\x90\x81\x10a\r7Wa\r7a\x11\xD9V[\x90`\0R` `\0 \x90`\x06\x02\x01`\x03\x01T`\x03\x85\x81T\x81\x10a\r\\Wa\r\\a\x11\xD9V[\x90`\0R` `\0 \x90`\x06\x02\x01`\x02\x01Ta\rx\x91\x90a\x11\x88V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01a\x07\x9EV[a\r\xB1a\x0EYV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0E\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x0E$\x81a\x0E\xB3V[PV[`\x04T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16a\x0EHa\x0EB\x85a\x0F\x03V[\x84a\x0F\\V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x93\x92PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x0E\x12V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\0\x80a\x0F\x0Ea\x08;V[\x83\x80Q\x90` \x01 `@Q` \x01a\x0F=\x92\x91\x90a\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[` \x81\x81\x01Q`@\x80\x84\x01Q``\x80\x86\x01Q\x83Q`\0\x80\x82R\x96\x81\x01\x80\x86R\x89\x90R\x90\x86\x1A\x93\x81\x01\x84\x90R\x90\x81\x01\x84\x90R`\x80\x81\x01\x82\x90R\x90\x91\x90`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0F\xBFW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x97\x96PPPPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F\xEBW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x10\x06W`\0\x80\xFD[\x845\x93Pa\x10\x16` \x86\x01a\x0F\xD4V[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x103W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x10GW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x10VW`\0\x80\xFD[\x88` \x82\x85\x01\x01\x11\x15a\x10hW`\0\x80\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[`\0` \x82\x84\x03\x12\x15a\x10\x89W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x10\xA2W`\0\x80\xFD[a\x10\xAB\x82a\x0F\xD4V[\x93\x92PPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x10\xDFW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x10\xC3V[\x81\x81\x11\x15a\x10\xF1W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x11\x1AW`\0\x80\xFD[a\x11#\x83a\x0F\xD4V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x11aWa\x11aa\x111V[P\x02\x90V[`\0\x82a\x11\x83WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x82\x82\x10\x15a\x11\x9AWa\x11\x9Aa\x111V[P\x03\x90V[`\0` \x82\x84\x03\x12\x15a\x11\xB1W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x10\xABW`\0\x80\xFD[`\0\x82\x19\x82\x11\x15a\x11\xD4Wa\x11\xD4a\x111V[P\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xD6\xC6!\x0C\xB0%\xEBQnd\xB9H3\xEC\xFC\x15\x04\x96jRX\xD7\x91\x84\xE2FI1\0\x11\x8A]dsolcC\0\x08\x0F\x003";
    /// The bytecode of the contract.
    pub static STAKING_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01MW`\x005`\xE0\x1C\x80c\x89\xEE\t\xF0\x11a\0\xC3W\x80c\x9B\xEC\xC2Z\x11a\0|W\x80c\x9B\xEC\xC2Z\x14a\x03$W\x80c\xA2\x02\xCAx\x14a\x037W\x80c\xB8\xAF=>\x14a\x03@W\x80c\xC6\x149\xDE\x14a\x03SW\x80c\xEF\x81\xB4\xD4\x14a\x03fW\x80c\xF2\xFD\xE3\x8B\x14a\x03yW`\0\x80\xFD[\x80c\x89\xEE\t\xF0\x14a\x02fW\x80c\x8Cd\xEAJ\x14a\x02\x8DW\x80c\x8D\xA5\xCB[\x14a\x02\xDAW\x80c\x8F\x16\x98\x16\x14a\x02\xFFW\x80c\x91\x8F\x86t\x14a\x03\x12W\x80c\x93\xADt@\x14a\x03\x1BW`\0\x80\xFD[\x80c6D\xE5\x15\x11a\x01\x15W\x80c6D\xE5\x15\x14a\x01\xF0W\x80c<\xA0.|\x14a\x01\xF8W\x80cT\xFDMP\x14a\x02\x0BW\x80cl\x19\xE7\x83\x14a\x028W\x80cqP\x18\xA6\x14a\x02KW\x80c\x7F\x86a\xA1\x14a\x02SW`\0\x80\xFD[\x80c\x15,d\xE8\x14a\x01RW\x80c\x1E' \xFF\x14a\x01xW\x80c `kp\x14a\x01\x8DW\x80c!\xDC\x83g\x14a\x01\xB4W\x80c2t($\x14a\x01\xC7W[`\0\x80\xFD[a\x01ea\x01`6`\x04a\x0F\xF0V[a\x03\x8CV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x8Ba\x01\x866`\x04a\x10wV[a\x07FV[\0[a\x01e\x7F\xD8|\xD6\xEFy\xD4\xE2\xB9^\x15\xCE\x8A\xBFs-\xB5\x1E\xC7q\xF1\xCA.\xDC\xCF\"\xA4lr\x9A\xC5dr\x81V[a\x01\x8Ba\x01\xC26`\x04a\x10wV[a\x07\xE5V[a\x01ea\x01\xD56`\x04a\x10\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[a\x01ea\x08;V[a\x01\x8Ba\x02\x066`\x04a\x10wV[a\t\x15V[a\x02+`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\x03`\xFC\x1B\x81RP\x81V[`@Qa\x01o\x91\x90a\x10\xB2V[a\x01\x8Ba\x02F6`\x04a\x10\x90V[a\t\"V[a\x01\x8Ba\tLV[a\x01\x8Ba\x02a6`\x04a\x10wV[a\t`V[a\x01e\x7F\x94\xE2\xAA\xEE91\x98\x1D\x99\x04\xD5K\x9D~w\x9E\x9C\xF3R\xE3\xF4tq\x94\xC4\xDFm4\x12\r.\xC8\x81V[a\x02\xA0a\x02\x9B6`\x04a\x10wV[a\n\xFCV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x97\x88\x16\x81R\x96\x90\x95\x16` \x87\x01R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R\x15\x15`\xA0\x82\x01R`\xC0\x01a\x01oV[`\0T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01oV[a\x01ea\x03\r6`\x04a\x11\x07V[a\x0BRV[a\x01ea'\x10\x81V[a\x01e`\x07T\x81V[a\x01ea\x0326`\x04a\x11\x07V[a\x0B\x83V[a\x01e`\x06T\x81V[a\x01\x8Ba\x03N6`\x04a\x10wV[a\x0B\x9FV[`\x05Ta\x02\xE7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x04Ta\x02\xE7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x8Ba\x03\x876`\x04a\x10\x90V[a\r\xA9V[3`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 T\x81Q\x7F\x94\xE2\xAA\xEE91\x98\x1D\x99\x04\xD5K\x9D~w\x9E\x9C\xF3R\xE3\xF4tq\x94\xC4\xDFm4\x12\r.\xC8\x93\x81\x01\x93\x90\x93R\x90\x82\x01\x93\x90\x93R``\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x86\x16`\x80\x82\x01R`\xA0\x81\x01\x92\x90\x92R\x90a\x04:\x90`\xC0\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R` `\x1F\x87\x01\x81\x90\x04\x81\x02\x84\x01\x81\x01\x90\x92R\x85\x83R\x91\x90\x86\x90\x86\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0E'\x92PPPV[a\x04WW`@Qc\x11;\x1A\x01`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x04\x97W`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x02` \x90\x81R`@\x82 `\x03T\x81T`\x01\x81\x01\x83U\x91\x84R\x91\x90\x92 \x90\x91\x01U[`\0a'\x10`\x06T\x87a\x04\xAA\x91\x90a\x11GV[a\x04\xB4\x91\x90a\x11fV[\x90P\x80`\x07`\0\x82\x82Ta\x04\xC8\x91\x90a\x11\x88V[\x90\x91UPP`\x03\x80T3`\0\x81\x81R`\x01` \x81\x81R`@\x80\x84 \x80T\x80\x85\x01\x82U\x90\x85R\x82\x85 \x01\x86\x90U\x80Q`\xC0\x81\x01\x82R\x85\x81R`\x01`\x01`\xA0\x1B\x03\x8D\x81\x16\x93\x82\x01\x93\x84R\x81\x83\x01\x8F\x81R``\x83\x01\x8B\x81RB`\x80\x85\x01\x90\x81R`\xA0\x85\x01\x89\x81R\x8CT\x98\x89\x01\x8DU\x9B\x90\x98R\x92Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[`\x06\x90\x97\x02\x96\x87\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x92\x85\x16\x92\x90\x92\x17\x90U\x94Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8\\\x87\x01\x80T\x90\x96\x16\x90\x83\x16\x17\x90\x94U\x92Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8]\x85\x01UQ\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8^\x84\x01U\x92Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8_\x83\x01U\x94Q\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8`\x90\x91\x01\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90U`\x05T\x90Qc#\xB8r\xDD`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R0`$\x83\x01R`D\x82\x01\x8A\x90R\x91\x94P\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xA2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC6\x91\x90a\x11\x9FV[P`\x05T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07<\x91\x90a\x11\x9FV[PP\x94\x93PPPPV[a\x07Na\x0EYV[\x80`\x07`\0\x82\x82Ta\x07`\x91\x90a\x11\xC1V[\x90\x91UPP`\x05T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\xBDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xE1\x91\x90a\x11\x9FV[PPV[a\x07\xEDa\x0EYV[\x80`\x07`\0\x82\x82Ta\x07\xFF\x91\x90a\x11\x88V[\x90\x91UPP`\x05T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01a\x07\x9EV[`@\x80Q\x80\x82\x01\x82R`\x01\x81R`\x03`\xFC\x1B` \x91\x82\x01R\x81Q\x7F\xD8|\xD6\xEFy\xD4\xE2\xB9^\x15\xCE\x8A\xBFs-\xB5\x1E\xC7q\xF1\xCA.\xDC\xCF\"\xA4lr\x9A\xC5dr\x81\x83\x01R\x7F(\xDFe\xE3\xE7\x87\xDAu-x\xD9\x814J+\x93\x80\x9AJN\x14f\xABs\x03\xE4L\x1E\x89L\x98N\x81\x84\x01R\x7F\x04HR\xB2\xA6p\xAD\xE5@~x\xFB(c\xC5\x1D\xE9\xFC\xB9eB\xA0q\x86\xFE:\xED\xA6\xBB\x8A\x11m``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R\x7F\x14n\xB7\x97E\xAF\x93\x8D\xD3\\\0\x8F\x08\xE6\xA3x#\xA1'\x8B9-\xF4w\xF9\x84\x9EF\x19V\xC2z`\xC0\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xE0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[a\t\x1Da\x0EYV[`\x06UV[a\t*a\x0EYV[`\x04\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\tTa\x0EYV[a\t^`\0a\x0E\xB3V[V[3`\x01`\x01`\xA0\x1B\x03\x16`\x03\x82\x81T\x81\x10a\t}Wa\t}a\x11\xD9V[`\0\x91\x82R` \x90\x91 `\x06\x90\x91\x02\x01T`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xB5W`@Qc\xFD\x99\xC6\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x81\x81T\x81\x10a\t\xC8Wa\t\xC8a\x11\xD9V[`\0\x91\x82R` \x90\x91 `\x05`\x06\x90\x92\x02\x01\x01T`\xFF\x16\x15\x15`\x01\x03a\n\x01W`@Qc'\xFC\xA5K`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[B`\x03\x82\x81T\x81\x10a\n\x15Wa\n\x15a\x11\xD9V[\x90`\0R` `\0 \x90`\x06\x02\x01`\x04\x01Tc\x01\xE13\x80a\n6\x91\x90a\x11\xC1V[\x10a\nTW`@Qc\xC9\xE5\xA5\xEB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x03\x82\x81T\x81\x10a\niWa\nia\x11\xD9V[`\0\x91\x82R` \x90\x91 `\x06\x90\x91\x02\x01`\x05\x90\x81\x01\x80T`\xFF\x19\x16\x92\x15\x15\x92\x90\x92\x17\x90\x91UT`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xA9\x05\x9C\xBB\x913\x91\x85\x90\x81\x10a\n\xB7Wa\n\xB7a\x11\xD9V[`\0\x91\x82R` \x90\x91 `\x02`\x06\x90\x92\x02\x01\x01T`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01a\x07\x9EV[`\x03\x81\x81T\x81\x10a\x0B\x0CW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x06\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x90\x95\x01T`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x96P\x93\x90\x92\x16\x93\x90\x92`\xFF\x16\x86V[`\x01` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x0BnW`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x91P\x91PPT\x81V[`\x02` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x0BnW`\0\x80\xFD[3`\x01`\x01`\xA0\x1B\x03\x16`\x03\x82\x81T\x81\x10a\x0B\xBCWa\x0B\xBCa\x11\xD9V[`\0\x91\x82R` \x90\x91 `\x06\x90\x91\x02\x01T`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\xF4W`@Qc\xFD\x99\xC6\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x81\x81T\x81\x10a\x0C\x07Wa\x0C\x07a\x11\xD9V[`\0\x91\x82R` \x90\x91 `\x05`\x06\x90\x92\x02\x01\x01T`\xFF\x16\x15\x15`\x01\x03a\x0C@W`@Qc'\xFC\xA5K`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[B`\x03\x82\x81T\x81\x10a\x0CTWa\x0CTa\x11\xD9V[\x90`\0R` `\0 \x90`\x06\x02\x01`\x04\x01Tc\x01\xE13\x80a\x0Cu\x91\x90a\x11\xC1V[\x10\x15a\x0C\x94W`@Qc\xFAn.A`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x03\x82\x81T\x81\x10a\x0C\xA9Wa\x0C\xA9a\x11\xD9V[\x90`\0R` `\0 \x90`\x06\x02\x01`\x05\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x03\x81\x81T\x81\x10a\x0C\xE4Wa\x0C\xE4a\x11\xD9V[\x90`\0R` `\0 \x90`\x06\x02\x01`\x03\x01T`\x07`\0\x82\x82Ta\r\x07\x91\x90a\x11\xC1V[\x90\x91UPP`\x05T`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xA9\x05\x9C\xBB\x913\x91\x85\x90\x81\x10a\r7Wa\r7a\x11\xD9V[\x90`\0R` `\0 \x90`\x06\x02\x01`\x03\x01T`\x03\x85\x81T\x81\x10a\r\\Wa\r\\a\x11\xD9V[\x90`\0R` `\0 \x90`\x06\x02\x01`\x02\x01Ta\rx\x91\x90a\x11\x88V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01a\x07\x9EV[a\r\xB1a\x0EYV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0E\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x0E$\x81a\x0E\xB3V[PV[`\x04T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16a\x0EHa\x0EB\x85a\x0F\x03V[\x84a\x0F\\V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x93\x92PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x0E\x12V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\0\x80a\x0F\x0Ea\x08;V[\x83\x80Q\x90` \x01 `@Q` \x01a\x0F=\x92\x91\x90a\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[` \x81\x81\x01Q`@\x80\x84\x01Q``\x80\x86\x01Q\x83Q`\0\x80\x82R\x96\x81\x01\x80\x86R\x89\x90R\x90\x86\x1A\x93\x81\x01\x84\x90R\x90\x81\x01\x84\x90R`\x80\x81\x01\x82\x90R\x90\x91\x90`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0F\xBFW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x97\x96PPPPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F\xEBW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x10\x06W`\0\x80\xFD[\x845\x93Pa\x10\x16` \x86\x01a\x0F\xD4V[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x103W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x10GW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x10VW`\0\x80\xFD[\x88` \x82\x85\x01\x01\x11\x15a\x10hW`\0\x80\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[`\0` \x82\x84\x03\x12\x15a\x10\x89W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x10\xA2W`\0\x80\xFD[a\x10\xAB\x82a\x0F\xD4V[\x93\x92PPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x10\xDFW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x10\xC3V[\x81\x81\x11\x15a\x10\xF1W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x11\x1AW`\0\x80\xFD[a\x11#\x83a\x0F\xD4V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x11aWa\x11aa\x111V[P\x02\x90V[`\0\x82a\x11\x83WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x82\x82\x10\x15a\x11\x9AWa\x11\x9Aa\x111V[P\x03\x90V[`\0` \x82\x84\x03\x12\x15a\x11\xB1W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x10\xABW`\0\x80\xFD[`\0\x82\x19\x82\x11\x15a\x11\xD4Wa\x11\xD4a\x111V[P\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xD6\xC6!\x0C\xB0%\xEBQnd\xB9H3\xEC\xFC\x15\x04\x96jRX\xD7\x91\x84\xE2FI1\0\x11\x8A]dsolcC\0\x08\x0F\x003";
    /// The deployed bytecode of the contract.
    pub static STAKING_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Staking<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Staking<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Staking<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Staking<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Staking<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Staking)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Staking<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    STAKING_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                STAKING_ABI.clone(),
                STAKING_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DENOMINATOR` (0x918f8674) function
        pub fn denominator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([145, 143, 134, 116], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DOMAIN_TYPEHASH` (0x20606b70) function
        pub fn domain_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([32, 96, 107, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `STAKE_TYPEHASH` (0x89ee09f0) function
        pub fn stake_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([137, 238, 9, 240], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `amkt` (0xc61439de) function
        pub fn amkt(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([198, 20, 57, 222], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bonusNumerator` (0xa202ca78) function
        pub fn bonus_numerator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([162, 2, 202, 120], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositReward` (0x1e2720ff) function
        pub fn deposit_reward(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 39, 32, 255], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `earlyExit` (0xb8af3d3e) function
        pub fn early_exit(
            &self,
            vault_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 175, 61, 62], vault_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exit` (0x7f8661a1) function
        pub fn exit(
            &self,
            vault_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 134, 97, 161], vault_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `referred` (0x9becc25a) function
        pub fn referred(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([155, 236, 194, 90], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeFromRewardPot` (0x21dc8367) function
        pub fn remove_from_reward_pot(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([33, 220, 131, 103], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardPot` (0x93ad7440) function
        pub fn reward_pot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([147, 173, 116, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBonusBps` (0x3ca02e7c) function
        pub fn set_bonus_bps(
            &self,
            bps: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 160, 46, 124], bps)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSigner` (0x6c19e783) function
        pub fn set_signer(
            &self,
            signer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 25, 231, 131], signer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stake` (0x152c64e8) function
        pub fn stake(
            &self,
            amount: ::ethers::core::types::U256,
            referrer: ::ethers::core::types::Address,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([21, 44, 100, 232], (amount, referrer, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `staked` (0x8f169816) function
        pub fn staked(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([143, 22, 152, 22], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `userVaultLength` (0x32742824) function
        pub fn user_vault_length(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([50, 116, 40, 36], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vaults` (0x8c64ea4a) function
        pub fn vaults(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                bool,
            ),
        > {
            self.0
                .method_hash([140, 100, 234, 74], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `version` (0x54fd4d50) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `whitelistSigner` (0xef81b4d4) function
        pub fn whitelist_signer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([239, 129, 180, 212], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Staking<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AMKTStakingAlreadyExited` with signature `AMKTStakingAlreadyExited()` and selector `0x9ff2952c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AMKTStakingAlreadyExited", abi = "AMKTStakingAlreadyExited()")]
    pub struct AMKTStakingAlreadyExited;
    ///Custom Error type `AMKTStakingCantExit` with signature `AMKTStakingCantExit()` and selector `0xc9e5a5eb`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AMKTStakingCantExit", abi = "AMKTStakingCantExit()")]
    pub struct AMKTStakingCantExit;
    ///Custom Error type `AMKTStakingDontEarlyExit` with signature `AMKTStakingDontEarlyExit()` and selector `0xfa6e2e41`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AMKTStakingDontEarlyExit", abi = "AMKTStakingDontEarlyExit()")]
    pub struct AMKTStakingDontEarlyExit;
    ///Custom Error type `AMKTStakingInvalidOwner` with signature `AMKTStakingInvalidOwner()` and selector `0xfd99c6fd`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AMKTStakingInvalidOwner", abi = "AMKTStakingInvalidOwner()")]
    pub struct AMKTStakingInvalidOwner;
    ///Custom Error type `AMKTStakingInvalidSigner` with signature `AMKTStakingInvalidSigner()` and selector `0x22763402`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AMKTStakingInvalidSigner", abi = "AMKTStakingInvalidSigner()")]
    pub struct AMKTStakingInvalidSigner;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum StakingErrors {
        AMKTStakingAlreadyExited(AMKTStakingAlreadyExited),
        AMKTStakingCantExit(AMKTStakingCantExit),
        AMKTStakingDontEarlyExit(AMKTStakingDontEarlyExit),
        AMKTStakingInvalidOwner(AMKTStakingInvalidOwner),
        AMKTStakingInvalidSigner(AMKTStakingInvalidSigner),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for StakingErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AMKTStakingAlreadyExited as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AMKTStakingAlreadyExited(decoded));
            }
            if let Ok(decoded) = <AMKTStakingCantExit as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AMKTStakingCantExit(decoded));
            }
            if let Ok(decoded) = <AMKTStakingDontEarlyExit as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AMKTStakingDontEarlyExit(decoded));
            }
            if let Ok(decoded) = <AMKTStakingInvalidOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AMKTStakingInvalidOwner(decoded));
            }
            if let Ok(decoded) = <AMKTStakingInvalidSigner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AMKTStakingInvalidSigner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StakingErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AMKTStakingAlreadyExited(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AMKTStakingCantExit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AMKTStakingDontEarlyExit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AMKTStakingInvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AMKTStakingInvalidSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for StakingErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AMKTStakingAlreadyExited as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AMKTStakingCantExit as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AMKTStakingDontEarlyExit as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AMKTStakingInvalidOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AMKTStakingInvalidSigner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for StakingErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AMKTStakingAlreadyExited(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AMKTStakingCantExit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AMKTStakingDontEarlyExit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AMKTStakingInvalidOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AMKTStakingInvalidSigner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for StakingErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AMKTStakingAlreadyExited> for StakingErrors {
        fn from(value: AMKTStakingAlreadyExited) -> Self {
            Self::AMKTStakingAlreadyExited(value)
        }
    }
    impl ::core::convert::From<AMKTStakingCantExit> for StakingErrors {
        fn from(value: AMKTStakingCantExit) -> Self {
            Self::AMKTStakingCantExit(value)
        }
    }
    impl ::core::convert::From<AMKTStakingDontEarlyExit> for StakingErrors {
        fn from(value: AMKTStakingDontEarlyExit) -> Self {
            Self::AMKTStakingDontEarlyExit(value)
        }
    }
    impl ::core::convert::From<AMKTStakingInvalidOwner> for StakingErrors {
        fn from(value: AMKTStakingInvalidOwner) -> Self {
            Self::AMKTStakingInvalidOwner(value)
        }
    }
    impl ::core::convert::From<AMKTStakingInvalidSigner> for StakingErrors {
        fn from(value: AMKTStakingInvalidSigner) -> Self {
            Self::AMKTStakingInvalidSigner(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `DENOMINATOR` function with signature `DENOMINATOR()` and selector `0x918f8674`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "DENOMINATOR", abi = "DENOMINATOR()")]
    pub struct DenominatorCall;
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `DOMAIN_TYPEHASH` function with signature `DOMAIN_TYPEHASH()` and selector `0x20606b70`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "DOMAIN_TYPEHASH", abi = "DOMAIN_TYPEHASH()")]
    pub struct DomainTypehashCall;
    ///Container type for all input parameters for the `STAKE_TYPEHASH` function with signature `STAKE_TYPEHASH()` and selector `0x89ee09f0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "STAKE_TYPEHASH", abi = "STAKE_TYPEHASH()")]
    pub struct StakeTypehashCall;
    ///Container type for all input parameters for the `amkt` function with signature `amkt()` and selector `0xc61439de`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "amkt", abi = "amkt()")]
    pub struct AmktCall;
    ///Container type for all input parameters for the `bonusNumerator` function with signature `bonusNumerator()` and selector `0xa202ca78`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "bonusNumerator", abi = "bonusNumerator()")]
    pub struct BonusNumeratorCall;
    ///Container type for all input parameters for the `depositReward` function with signature `depositReward(uint256)` and selector `0x1e2720ff`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "depositReward", abi = "depositReward(uint256)")]
    pub struct DepositRewardCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `earlyExit` function with signature `earlyExit(uint256)` and selector `0xb8af3d3e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "earlyExit", abi = "earlyExit(uint256)")]
    pub struct EarlyExitCall {
        pub vault_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `exit` function with signature `exit(uint256)` and selector `0x7f8661a1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "exit", abi = "exit(uint256)")]
    pub struct ExitCall {
        pub vault_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `referred` function with signature `referred(address,uint256)` and selector `0x9becc25a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "referred", abi = "referred(address,uint256)")]
    pub struct ReferredCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `removeFromRewardPot` function with signature `removeFromRewardPot(uint256)` and selector `0x21dc8367`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "removeFromRewardPot", abi = "removeFromRewardPot(uint256)")]
    pub struct RemoveFromRewardPotCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `rewardPot` function with signature `rewardPot()` and selector `0x93ad7440`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rewardPot", abi = "rewardPot()")]
    pub struct RewardPotCall;
    ///Container type for all input parameters for the `setBonusBps` function with signature `setBonusBps(uint256)` and selector `0x3ca02e7c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setBonusBps", abi = "setBonusBps(uint256)")]
    pub struct SetBonusBpsCall {
        pub bps: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setSigner` function with signature `setSigner(address)` and selector `0x6c19e783`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setSigner", abi = "setSigner(address)")]
    pub struct SetSignerCall {
        pub signer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `stake` function with signature `stake(uint256,address,bytes)` and selector `0x152c64e8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "stake", abi = "stake(uint256,address,bytes)")]
    pub struct StakeCall {
        pub amount: ::ethers::core::types::U256,
        pub referrer: ::ethers::core::types::Address,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `staked` function with signature `staked(address,uint256)` and selector `0x8f169816`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "staked", abi = "staked(address,uint256)")]
    pub struct StakedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `userVaultLength` function with signature `userVaultLength(address)` and selector `0x32742824`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "userVaultLength", abi = "userVaultLength(address)")]
    pub struct UserVaultLengthCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `vaults` function with signature `vaults(uint256)` and selector `0x8c64ea4a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "vaults", abi = "vaults(uint256)")]
    pub struct VaultsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `version` function with signature `version()` and selector `0x54fd4d50`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    ///Container type for all input parameters for the `whitelistSigner` function with signature `whitelistSigner()` and selector `0xef81b4d4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "whitelistSigner", abi = "whitelistSigner()")]
    pub struct WhitelistSignerCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum StakingCalls {
        Denominator(DenominatorCall),
        DomainSeparator(DomainSeparatorCall),
        DomainTypehash(DomainTypehashCall),
        StakeTypehash(StakeTypehashCall),
        Amkt(AmktCall),
        BonusNumerator(BonusNumeratorCall),
        DepositReward(DepositRewardCall),
        EarlyExit(EarlyExitCall),
        Exit(ExitCall),
        Owner(OwnerCall),
        Referred(ReferredCall),
        RemoveFromRewardPot(RemoveFromRewardPotCall),
        RenounceOwnership(RenounceOwnershipCall),
        RewardPot(RewardPotCall),
        SetBonusBps(SetBonusBpsCall),
        SetSigner(SetSignerCall),
        Stake(StakeCall),
        Staked(StakedCall),
        TransferOwnership(TransferOwnershipCall),
        UserVaultLength(UserVaultLengthCall),
        Vaults(VaultsCall),
        Version(VersionCall),
        WhitelistSigner(WhitelistSignerCall),
    }
    impl ::ethers::core::abi::AbiDecode for StakingCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DenominatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Denominator(decoded));
            }
            if let Ok(decoded) = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) = <DomainTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DomainTypehash(decoded));
            }
            if let Ok(decoded) = <StakeTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakeTypehash(decoded));
            }
            if let Ok(decoded) = <AmktCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Amkt(decoded));
            }
            if let Ok(decoded) = <BonusNumeratorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BonusNumerator(decoded));
            }
            if let Ok(decoded) = <DepositRewardCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DepositReward(decoded));
            }
            if let Ok(decoded) = <EarlyExitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EarlyExit(decoded));
            }
            if let Ok(decoded) = <ExitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Exit(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <ReferredCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Referred(decoded));
            }
            if let Ok(decoded) = <RemoveFromRewardPotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveFromRewardPot(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <RewardPotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RewardPot(decoded));
            }
            if let Ok(decoded) = <SetBonusBpsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetBonusBps(decoded));
            }
            if let Ok(decoded) = <SetSignerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetSigner(decoded));
            }
            if let Ok(decoded) = <StakeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Stake(decoded));
            }
            if let Ok(decoded) = <StakedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Staked(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UserVaultLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UserVaultLength(decoded));
            }
            if let Ok(decoded) = <VaultsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Vaults(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            if let Ok(decoded) = <WhitelistSignerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WhitelistSigner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StakingCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Denominator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Amkt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BonusNumerator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositReward(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EarlyExit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Exit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Referred(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveFromRewardPot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardPot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetBonusBps(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Stake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Staked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UserVaultLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Vaults(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WhitelistSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for StakingCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Denominator(element) => ::core::fmt::Display::fmt(element, f),
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::DomainTypehash(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeTypehash(element) => ::core::fmt::Display::fmt(element, f),
                Self::Amkt(element) => ::core::fmt::Display::fmt(element, f),
                Self::BonusNumerator(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::EarlyExit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Exit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Referred(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveFromRewardPot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardPot(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBonusBps(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Stake(element) => ::core::fmt::Display::fmt(element, f),
                Self::Staked(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserVaultLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::Vaults(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::WhitelistSigner(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DenominatorCall> for StakingCalls {
        fn from(value: DenominatorCall) -> Self {
            Self::Denominator(value)
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for StakingCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<DomainTypehashCall> for StakingCalls {
        fn from(value: DomainTypehashCall) -> Self {
            Self::DomainTypehash(value)
        }
    }
    impl ::core::convert::From<StakeTypehashCall> for StakingCalls {
        fn from(value: StakeTypehashCall) -> Self {
            Self::StakeTypehash(value)
        }
    }
    impl ::core::convert::From<AmktCall> for StakingCalls {
        fn from(value: AmktCall) -> Self {
            Self::Amkt(value)
        }
    }
    impl ::core::convert::From<BonusNumeratorCall> for StakingCalls {
        fn from(value: BonusNumeratorCall) -> Self {
            Self::BonusNumerator(value)
        }
    }
    impl ::core::convert::From<DepositRewardCall> for StakingCalls {
        fn from(value: DepositRewardCall) -> Self {
            Self::DepositReward(value)
        }
    }
    impl ::core::convert::From<EarlyExitCall> for StakingCalls {
        fn from(value: EarlyExitCall) -> Self {
            Self::EarlyExit(value)
        }
    }
    impl ::core::convert::From<ExitCall> for StakingCalls {
        fn from(value: ExitCall) -> Self {
            Self::Exit(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for StakingCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<ReferredCall> for StakingCalls {
        fn from(value: ReferredCall) -> Self {
            Self::Referred(value)
        }
    }
    impl ::core::convert::From<RemoveFromRewardPotCall> for StakingCalls {
        fn from(value: RemoveFromRewardPotCall) -> Self {
            Self::RemoveFromRewardPot(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for StakingCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RewardPotCall> for StakingCalls {
        fn from(value: RewardPotCall) -> Self {
            Self::RewardPot(value)
        }
    }
    impl ::core::convert::From<SetBonusBpsCall> for StakingCalls {
        fn from(value: SetBonusBpsCall) -> Self {
            Self::SetBonusBps(value)
        }
    }
    impl ::core::convert::From<SetSignerCall> for StakingCalls {
        fn from(value: SetSignerCall) -> Self {
            Self::SetSigner(value)
        }
    }
    impl ::core::convert::From<StakeCall> for StakingCalls {
        fn from(value: StakeCall) -> Self {
            Self::Stake(value)
        }
    }
    impl ::core::convert::From<StakedCall> for StakingCalls {
        fn from(value: StakedCall) -> Self {
            Self::Staked(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for StakingCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UserVaultLengthCall> for StakingCalls {
        fn from(value: UserVaultLengthCall) -> Self {
            Self::UserVaultLength(value)
        }
    }
    impl ::core::convert::From<VaultsCall> for StakingCalls {
        fn from(value: VaultsCall) -> Self {
            Self::Vaults(value)
        }
    }
    impl ::core::convert::From<VersionCall> for StakingCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<WhitelistSignerCall> for StakingCalls {
        fn from(value: WhitelistSignerCall) -> Self {
            Self::WhitelistSigner(value)
        }
    }
    ///Container type for all return fields from the `DENOMINATOR` function with signature `DENOMINATOR()` and selector `0x918f8674`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DenominatorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `DOMAIN_TYPEHASH` function with signature `DOMAIN_TYPEHASH()` and selector `0x20606b70`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DomainTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `STAKE_TYPEHASH` function with signature `STAKE_TYPEHASH()` and selector `0x89ee09f0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct StakeTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `amkt` function with signature `amkt()` and selector `0xc61439de`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AmktReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `bonusNumerator` function with signature `bonusNumerator()` and selector `0xa202ca78`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BonusNumeratorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `referred` function with signature `referred(address,uint256)` and selector `0x9becc25a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ReferredReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rewardPot` function with signature `rewardPot()` and selector `0x93ad7440`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RewardPotReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `stake` function with signature `stake(uint256,address,bytes)` and selector `0x152c64e8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct StakeReturn {
        pub nonce: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `staked` function with signature `staked(address,uint256)` and selector `0x8f169816`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct StakedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `userVaultLength` function with signature `userVaultLength(address)` and selector `0x32742824`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct UserVaultLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `vaults` function with signature `vaults(uint256)` and selector `0x8c64ea4a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct VaultsReturn {
        pub owner: ::ethers::core::types::Address,
        pub referrer: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub bonus: ::ethers::core::types::U256,
        pub timestamp: ::ethers::core::types::U256,
        pub exited: bool,
    }
    ///Container type for all return fields from the `version` function with signature `version()` and selector `0x54fd4d50`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct VersionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `whitelistSigner` function with signature `whitelistSigner()` and selector `0xef81b4d4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct WhitelistSignerReturn(pub ::ethers::core::types::Address);
}
