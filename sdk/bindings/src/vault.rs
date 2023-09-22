pub use vault::*;
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
pub mod vault {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_indexToken"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IIndexToken"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_owner"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_feeRecipient"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_emergencyResponder"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_feeScaled"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("emergency"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emergency"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("emergencyResponder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emergencyResponder"),
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
                    ::std::borrow::ToOwned::to_owned("feeRecipient"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("feeRecipient"),
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
                    ::std::borrow::ToOwned::to_owned("feeScaled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("feeScaled"),
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
                    ::std::borrow::ToOwned::to_owned("indexToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("indexToken"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IIndexToken"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("invariantCheck"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("invariantCheck"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("invokeBurn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("invokeBurn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("invokeERC20"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("invokeERC20"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IVault.InvokeERC20Args",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("invokeERC20"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IVault.InvokeERC20Args[]",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("invokeMint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("invokeMint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("invokeSetMultiplier"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "invokeSetMultiplier",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_multiplier"),
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
                    ::std::borrow::ToOwned::to_owned("invokeSetNominal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("invokeSetNominal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IVault.SetNominalArgs[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("invokeSetNominal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IVault.SetNominalArgs",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("isUnderlying"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isUnderlying"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("issuance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("issuance"),
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
                    ::std::borrow::ToOwned::to_owned("multiplier"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("multiplier"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("trackedTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("trackedMultiplier"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newFeeAccrued"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("currentMultiplier"),
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
                    ::std::borrow::ToOwned::to_owned("proposeOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposeOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_proposedOwner"),
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
                    ::std::borrow::ToOwned::to_owned("proposedOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposedOwner"),
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
                    ::std::borrow::ToOwned::to_owned("realUnits"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("realUnits"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct TokenInfo[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("realUnits"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
                    ::std::borrow::ToOwned::to_owned("rebalancer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rebalancer"),
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
                    ::std::borrow::ToOwned::to_owned("setEmergency"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setEmergency"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_emergency"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("setEmergencyResponder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setEmergencyResponder",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_emergencyResponder",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("setFeeRecipient"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFeeRecipient"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_feeRecipient"),
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
                    ::std::borrow::ToOwned::to_owned("setFeeScaled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFeeScaled"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_feeScaled"),
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
                    ::std::borrow::ToOwned::to_owned("setIssuance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setIssuance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_issuance"),
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
                    ::std::borrow::ToOwned::to_owned("setRebalancer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setRebalancer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rebalancer"),
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
                    ::std::borrow::ToOwned::to_owned("tryInflation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tryInflation"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("underlying"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("underlying"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("underlyingLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("underlyingLength"),
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
                    ::std::borrow::ToOwned::to_owned("virtualUnits"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("virtualUnits"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("virtualUnits"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct TokenInfo[]"),
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
                    ::std::borrow::ToOwned::to_owned("AMKTVaultEmergency"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AMKTVaultEmergency"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AMKTVaultFeeTooLarge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AMKTVaultFeeTooLarge",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AMKTVaultOnly"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AMKTVaultOnly"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("who"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AMKTVaultOnlyInvokers"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AMKTVaultOnlyInvokers",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MultiplierFeeTooHigh"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MultiplierFeeTooHigh",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VaultInvariant"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("VaultInvariant"),
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
    pub static VAULT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x1F\x118\x03\x80b\0\x1F\x11\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x014V[b\0\0?3b\0\0\xCBV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11\x15b\0\0iW`@Qc\x1A\xA9\x97\xB5`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x85\x16`\x80Rb\0\0\x81\x84b\0\0\xCBV[`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x04\x80T\x94\x90\x93\x16\x93\x16\x92\x90\x92\x17\x90U`\x06UPPg\r\xE0\xB6\xB3\xA7d\0\0`\x08UB`\x07Ub\0\x01\xA8V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x011W`\0\x80\xFD[PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15b\0\x01MW`\0\x80\xFD[\x85Qb\0\x01Z\x81b\0\x01\x1BV[` \x87\x01Q\x90\x95Pb\0\x01m\x81b\0\x01\x1BV[`@\x87\x01Q\x90\x94Pb\0\x01\x80\x81b\0\x01\x1BV[``\x87\x01Q\x90\x93Pb\0\x01\x93\x81b\0\x01\x1BV[\x80\x92PP`\x80\x86\x01Q\x90P\x92\x95P\x92\x95\x90\x93PV[`\x80Qa\x1D*b\0\x01\xE7`\09`\0\x81\x81a\x04\x91\x01R\x81\x81a\x07*\x01R\x81\x81a\tO\x01R\x81\x81a\r$\x01R\x81\x81a\x0E)\x01Ra\x0F\x8C\x01Ra\x1D*`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x10W`\x005`\xE0\x1C\x80c}\xD0\x9F\xEC\x11a\x01%W\x80c\xB5\xED)\x8A\x11a\0\xADW\x80c\xE7K\x98\x1B\x11a\0|W\x80c\xE7K\x98\x1B\x14a\x04yW\x80c\xE7\xD0\x15\xF2\x14a\x04\x8CW\x80c\xF2\xFD\xE3\x8B\x14a\x04\xB3W\x80c\xFD\xC4\xCB\xC0\x14a\x04\xC6W\x80c\xFF\x02v(\x14a\x04\xD9W`\0\x80\xFD[\x80c\xB5\xED)\x8A\x14a\x04,W\x80c\xCA\xA6\xFE\xA4\x14a\x04?W\x80c\xD1S\xB6\x0C\x14a\x04SW\x80c\xDE\x1E.\xEA\x14a\x04fW`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11a\0\xF4W\x80c\x8D\xA5\xCB[\x14a\x03\xCAW\x80c\x9D5R\xC7\x14a\x03\xDBW\x80c\x9F\xF6\xA7\x8C\x14a\x03\xEEW\x80c\xA7\xD03w\x14a\x04\x01W\x80c\xB2\x13\xE3\xF7\x14a\x04\tW`\0\x80\xFD[\x80c}\xD0\x9F\xEC\x14a\x03qW\x80c\x81\xA0\xC8U\x14a\x03\x9AW\x80c\x866#\xBB\x14a\x03\xAFW\x80c\x8C0\xFB~\x14a\x03\xC2W`\0\x80\xFD[\x80cF\x90H@\x11a\x01\xA8W\x80co0}\xC3\x11a\x01wW\x80co0}\xC3\x14a\x03\x1BW\x80cqP\x18\xA6\x14a\x030W\x80csd\x19\xEF\x14a\x038W\x80cw\x05/\xC1\x14a\x03KW\x80cx)\xBA\x1F\x14a\x03^W`\0\x80\xFD[\x80cF\x90H@\x14a\x02\xD9W\x80cb\x97\xF7\x92\x14a\x02\xECW\x80cl\xFD\x15S\x14a\x02\xF5W\x80cm\xAB+\xC0\x14a\x03\x08W`\0\x80\xFD[\x80c\x0E76\xCF\x11a\x01\xE4W\x80c\x0E76\xCF\x14a\x02\x80W\x80c\x1B>\xD7\"\x14a\x02\x93W\x80c \x7F\xAA}\x14a\x02\xBBW\x80cE\t\x01~\x14a\x02\xD1W`\0\x80\xFD[\x80b\xD1O\x18\x14a\x02\x15W\x80c\x01\xD2,\xCD\x14a\x02*W\x80c\x05\x01\xD5V\x14a\x02ZW\x80c\x0B\x1B;\xCA\x14a\x02mW[`\0\x80\xFD[a\x02(a\x02#6`\x04a\x18\xDCV[a\x04\xECV[\0[`\x03Ta\x02=\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02(a\x02h6`\x04a\x19_V[a\x05\xA6V[a\x02(a\x02{6`\x04a\x19|V[a\x05\xFCV[a\x02(a\x02\x8E6`\x04a\x19\x94V[a\x06mV[a\x02\x9Ba\x06\xE5V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x02QV[a\x02\xC3a\x07\nV[`@Q\x90\x81R` \x01a\x02QV[a\x02(a\x07\x1AV[`\x04Ta\x02=\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xC3`\x06T\x81V[a\x02(a\x03\x036`\x04a\x19\xC2V[a\x08\x98V[a\x02(a\x03\x166`\x04a\x19\xDDV[a\x08\xF2V[a\x03#a\t\xB1V[`@Qa\x02Q\x91\x90a\x1A\x07V[a\x02(a\t\xBDV[a\x02(a\x03F6`\x04a\x19\xC2V[a\t\xD1V[`\x05Ta\x02=\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02(a\x03l6`\x04a\x1ATV[a\n+V[a\x02\xC3a\x03\x7F6`\x04a\x19\xC2V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x0C` R`@\x90 T\x90V[a\x03\xA2a\n\x9AV[`@Qa\x02Q\x91\x90a\x1AmV[`\x02Ta\x02=\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\xA2a\x0B\xA5V[`\0T`\x01`\x01`\xA0\x1B\x03\x16a\x02=V[a\x02\xC3a\x03\xE96`\x04a\x19\xC2V[a\x0C\xA4V[a\x02(a\x03\xFC6`\x04a\x19\xC2V[a\x0C\xC5V[a\x02\xC3a\r\x1FV[a\x04\x1Ca\x04\x176`\x04a\x19\xC2V[a\x0E\x91V[`@Q\x90\x15\x15\x81R` \x01a\x02QV[a\x02(a\x04:6`\x04a\x19\xC2V[a\x0E\xB4V[`\x01Ta\x04\x1C\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`\x01Ta\x02=\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02(a\x04t6`\x04a\x19\xDDV[a\x0F\x04V[a\x02(a\x04\x876`\x04a\x19\xC2V[a\x0F\xBBV[a\x02=\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02(a\x04\xC16`\x04a\x19\xC2V[a\x10\x15V[a\x02(a\x04\xD46`\x04a\x1ATV[a\x11JV[a\x02(a\x04\xE76`\x04a\x1A\xC5V[a\x11\xB2V[`\x01T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x05\x17W`@QcR\x01\xF2\xB7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\x05SW`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\0[\x82\x81\x10\x15a\x05\xA0Wa\x05\x8E\x84\x84\x83\x81\x81\x10a\x05sWa\x05sa\x1B(V[\x90P`@\x02\x01\x806\x03\x81\x01\x90a\x05\x89\x91\x90a\x1BTV[a\x12CV[\x80a\x05\x98\x81a\x1B\xCFV[\x91PPa\x05VV[PPPPV[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\x05\xDDW`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05JV[P`\x01\x80T\x91\x15\x15`\x01`\xA0\x1B\x02`\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80\x15\x90a\x06\"WP`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x06@W`@QcNe\xA3\x1B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06ja\x06P` \x83\x01\x83a\x19\xC2V[a\x06``@\x84\x01` \x85\x01a\x19\xC2V[\x83`@\x015a\x12\xACV[PV[`\x01T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x06\x98W`@QcR\x01\xF2\xB7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\x06\xCFW`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05JV[a\x06\xE1a\x05\x896\x84\x90\x03\x84\x01\x84a\x1BTV[PPV[`\0\x80`\0\x80a\x06\xFC`\x07T`\x08T`\x06Ta\x13\x1FV[\x92\x97\x91\x96P\x94P\x90\x92P\x90PV[`\0a\x07\x15`\tT\x90V[\x90P\x90V[`\0a\x07$a\n\x9AV[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x86W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xAA\x91\x90a\x1B\xE8V[\x90P`\0[\x82Q\x81\x10\x15a\x08\x93W`\0a\x07\xE1\x84\x83\x81Q\x81\x10a\x07\xCFWa\x07\xCFa\x1B(V[` \x02` \x01\x01Q` \x01Q\x84a\x147V[\x90P\x80\x84\x83\x81Q\x81\x10a\x07\xF6Wa\x07\xF6a\x1B(V[` \x90\x81\x02\x91\x90\x91\x01\x01QQ`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08GW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08k\x91\x90a\x1B\xE8V[\x10\x15a\x08\x8AW`@Qc\x1C\xC9*\xE1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x01a\x07\xAFV[PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\x08\xCFW`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05JV[P`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\t)W`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05JV[`@Qc'p\xA7\xEB`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x9D\xC2\x9F\xAC\x90`D\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\x94W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\xA8W=`\0\x80>=`\0\xFD[PPPPPPPV[``a\x07\x15`\ta\x14VV[a\t\xC5a\x15$V[a\t\xCF`\0a\x15~V[V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\n\x08W`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05JV[P`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\nbW`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05JV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x11\x15a\n\x8BW`@Qc\x1A\xA9\x97\xB5`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\x93a\r\x1FV[PP`\x06UV[`\t\x80T``\x91\x90`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\xBDWa\n\xBDa\x1B>V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\x02W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\n\xDBW\x90P[P\x90P`\0a\x0B\x0Fa\x06\xE5V[\x93PPPP`\0[\x83\x81\x10\x15a\x0B\x9BW`\0\x85\x82\x81T\x81\x10a\x0B3Wa\x0B3a\x1B(V[`\0\x91\x82R` \x91\x82\x90 \x01T`@\x80Q\x80\x82\x01\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x80\x82R\x92P\x90\x81\x01a\x0Bg\x83\x86a\x15\xCEV[\x81RP\x84\x83\x81Q\x81\x10a\x0B|Wa\x0B|a\x1B(V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x0B\x93\x90a\x1B\xCFV[\x91PPa\x0B\x17V[P\x90\x94\x93PPPPV[`\t\x80T``\x91\x90`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xC8Wa\x0B\xC8a\x1B>V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\rW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0B\xE6W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0C\x9CW`\0\x84\x82\x81T\x81\x10a\x0C/Wa\x0C/a\x1B(V[`\0\x91\x82R` \x80\x83 \x90\x91\x01T`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x80\x83R\x80\x85R`\x0C\x84R\x93 T\x91\x81\x01\x91\x90\x91R\x84Q\x91\x92P\x90\x84\x90\x84\x90\x81\x10a\x0C}Wa\x0C}a\x1B(V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x0C\x94\x90a\x1B\xCFV[\x91PPa\x0C\x13V[P\x93\x92PPPV[`\0\x80a\x0C\xAFa\x06\xE5V[\x93PPPPa\x0C\xBE\x83\x82a\x15\xCEV[\x93\x92PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\x0C\xFCW`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05JV[P`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xA4\x91\x90a\x1B\xE8V[\x90P`\0\x80`\0\x80a\r\xB4a\x06\xE5V[\x93P\x93P\x93P\x93Pg\r\xE0\xB6\xB3\xA7d\0\0\x82\x10\x15a\x0E\x88W`\0\x85a\r\xE1\x87a\r\xDC\x86a\x15\xF1V[a\x147V[a\r\xEB\x91\x90a\x1C\x01V[`\x08\x85\x90U`\x07\x86\x90U\x90P\x80\x15a\x0E\x86W`\x04\x80T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x81\x01\x92\x90\x92R`$\x82\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0EmW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x81W=`\0\x80>=`\0\xFD[PPPP[P[\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0B` R`@\x81 T`\xFF\x16[\x92\x91PPV[a\x0E\xBCa\x15$V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0E\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05J\x90a\x1C\x18V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x0F/W`@QcR\x01\xF2\xB7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\x0FfW`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05JV[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c@\xC1\x0F\x19\x90`D\x01a\tzV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\x0F\xF2W`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05JV[P`\x04\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x10;W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05J\x90a\x1C\x18V[`\x01T`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14a\x10\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FProposableOwnable: new owner is `D\x82\x01Rq77\xBA\x10897\xB87\xB9\xB2\xB2\x107\xBB\xB72\xB9`q\x1B`d\x82\x01R`\x84\x01a\x05JV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x111W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FProposableOwnable: this call mus`D\x82\x01R\x7Ft be made by the new owner\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05JV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90Ua\x06j\x81a\x15~V[`\x01T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x11uW`@QcR\x01\xF2\xB7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\x11\xACW`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05JV[P`\x08UV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80\x15\x90a\x11\xD8WP`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x11\xF6W`@QcNe\xA3\x1B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\0[\x81\x81\x10\x15a\x05\xA0W6\x84\x84\x83\x81\x81\x10a\x12\x15Wa\x12\x15a\x1B(V[``\x02\x91\x90\x91\x01\x91Pa\x120\x90Pa\x06P` \x83\x01\x83a\x19\xC2V[P\x80a\x12;\x81a\x1B\xCFV[\x91PPa\x11\xFAV[\x80Q` \x82\x01Q`\0\x81\x90\x03a\x12wW`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0C` R`@\x81 Ua\x08\x93`\t\x83a\x16\x05V[a\x12\x80\x82a\x0E\x91V[a\x12\x8FWa\x12\x8F`\t\x83a\x17\xF5V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\0\x90\x81R`\x0C` R`@\x90 UPV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12\xFBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xA0\x91\x90a\x1ChV[`\0\x80`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0\x85\x11\x15a\x13NW`@Qc4\xE7Z\xA1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x86\x93P`\0a\x13e\x86g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\x01V[\x90P`\0b\x01Q\x80a\x13w\x87Ba\x1C\x01V[a\x13\x81\x91\x90a\x1C\x85V[g\r\xE0\xB6\xB3\xA7d\0\0\x94P\x90P\x80\x15a\x13\xCEW`\0[\x81\x81\x10\x15a\x13\xB3Wa\x13\xA9\x85\x84a\x147V[\x94P`\x01\x01a\x13\x97V[Pa\x13\xC1\x81b\x01Q\x80a\x1C\xA7V[a\x13\xCB\x90\x87a\x1C\xC6V[\x95P[a\x13\xD8\x84\x89a\x147V[\x94P`\0a\x13\xE6\x87Ba\x1C\x01V[\x90P\x80\x15a\x14'W`\0a\x13\xFDb\x01Q\x80\x8Aa\x1C\x85V[\x90Pa\x14\x1F\x87a\x14\r\x84\x84a\x1C\xA7V[a\r\xDC\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\x01V[\x94PPa\x14+V[\x85\x93P[PPP\x93P\x93P\x93P\x93V[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x14L\x83\x85a\x1C\xA7V[a\x0C\xBE\x91\x90a\x1C\x85V[\x80T``\x90\x82\x90`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14xWa\x14xa\x1B>V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\xA1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x15\x1BW\x83\x81\x81T\x81\x10a\x14\xC1Wa\x14\xC1a\x1B(V[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x82\x82\x81Q\x81\x10a\x14\xF1Wa\x14\xF1a\x1B(V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x15\x13\x81a\x1B\xCFV[\x91PPa\x14\xA7V[P\x94\x93PPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05JV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0C` R`@\x81 Ta\x0C\xBE\x90\x83a\x147V[`\0a\x0E\xAEg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x18\xC7V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02\x83\x01` R`@\x90 T`\xFF\x16a\x16zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FVerifiableArray: element not fou`D\x82\x01Ra\x1B\x99`\xF2\x1B`d\x82\x01R`\x84\x01a\x05JV[`\0a\x16\x84\x83T\x90V[\x90P\x80`\x01\x03a\x16\xF5W`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x02\x84\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16\x90U`\x01\x86\x01\x90\x91R\x81 U\x82T\x83\x90\x80a\x16\xCEWa\x16\xCEa\x1C\xDEV[`\0\x82\x81R` \x90 \x81\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90UPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01\x80\x85\x01` R`@\x82 T\x91\x90\x85\x90a\x17\x1F\x90\x85a\x1C\x01V[\x81T\x81\x10a\x17/Wa\x17/a\x1B(V[`\0\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x80\x84R`\x01\x89\x01\x80\x84R`@\x80\x86 \x88\x90U\x92\x89\x16\x85R`\x02\x8A\x01\x84R\x82\x85 \x80T`\xFF\x19\x16\x90U\x90\x92R\x82 \x91\x90\x91U\x85T\x90\x91P\x81\x90\x86\x90\x84\x90\x81\x10a\x17\x92Wa\x17\x92a\x1B(V[`\0\x91\x82R` \x90\x91 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x84T\x85\x90\x80a\x17\xCCWa\x17\xCCa\x1C\xDEV[`\0\x82\x81R` \x90 \x81\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90UPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02\x83\x01` R`@\x90 T`\xFF\x16\x15a\x18pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FVerifiableArray: element already`D\x82\x01Rf exists`\xC8\x1B`d\x82\x01R`\x84\x01a\x05JV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x02\x84\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x86T\x87\x82\x01\x84R\x91\x84 \x82\x90U\x81\x01\x86U\x85\x83R\x91 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x91\x17\x90UPPV[`\0\x81a\x14Lg\r\xE0\xB6\xB3\xA7d\0\0\x85a\x1C\xA7V[`\0\x80` \x83\x85\x03\x12\x15a\x18\xEFW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x19\x07W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x19\x1BW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x19*W`\0\x80\xFD[\x86` \x82`\x06\x1B\x85\x01\x01\x11\x15a\x19?W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[\x80\x15\x15\x81\x14a\x06jW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x19qW`\0\x80\xFD[\x815a\x0C\xBE\x81a\x19QV[`\0``\x82\x84\x03\x12\x15a\x19\x8EW`\0\x80\xFD[P\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x19\x8EW`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x19\xBDW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x19\xD4W`\0\x80\xFD[a\x0C\xBE\x82a\x19\xA6V[`\0\x80`@\x83\x85\x03\x12\x15a\x19\xF0W`\0\x80\xFD[a\x19\xF9\x83a\x19\xA6V[\x94` \x93\x90\x93\x015\x93PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x1AHW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x1A#V[P\x90\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1AfW`\0\x80\xFD[P5\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x1A\xB8W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x86\x01Q\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x1A\x8AV[P\x91\x97\x96PPPPPPPV[`\0\x80` \x83\x85\x03\x12\x15a\x1A\xD8W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1A\xF0W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x1B\x04W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1B\x13W`\0\x80\xFD[\x86` ``\x83\x02\x85\x01\x01\x11\x15a\x19?W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`@\x82\x84\x03\x12\x15a\x1BfW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1B\x97WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Ra\x1B\xA3\x83a\x19\xA6V[\x81R` \x83\x015` \x82\x01R\x80\x91PP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x1B\xE1Wa\x1B\xE1a\x1B\xB9V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x1B\xFAW`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x82\x10\x15a\x1C\x13Wa\x1C\x13a\x1B\xB9V[P\x03\x90V[` \x80\x82R`0\x90\x82\x01R\x7FProposableOwnable: new owner is `@\x82\x01Rothe zero address`\x80\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x1CzW`\0\x80\xFD[\x81Qa\x0C\xBE\x81a\x19QV[`\0\x82a\x1C\xA2WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x1C\xC1Wa\x1C\xC1a\x1B\xB9V[P\x02\x90V[`\0\x82\x19\x82\x11\x15a\x1C\xD9Wa\x1C\xD9a\x1B\xB9V[P\x01\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \x8F\xCF\x94;\xB0\x86<y\x03\xF3\xF7\xF4\x14\xC4\xD8.eb,3\xB2\x18(u\xF2D\xCD\xCEclL\x03dsolcC\0\x08\x0F\x003";
    /// The bytecode of the contract.
    pub static VAULT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x10W`\x005`\xE0\x1C\x80c}\xD0\x9F\xEC\x11a\x01%W\x80c\xB5\xED)\x8A\x11a\0\xADW\x80c\xE7K\x98\x1B\x11a\0|W\x80c\xE7K\x98\x1B\x14a\x04yW\x80c\xE7\xD0\x15\xF2\x14a\x04\x8CW\x80c\xF2\xFD\xE3\x8B\x14a\x04\xB3W\x80c\xFD\xC4\xCB\xC0\x14a\x04\xC6W\x80c\xFF\x02v(\x14a\x04\xD9W`\0\x80\xFD[\x80c\xB5\xED)\x8A\x14a\x04,W\x80c\xCA\xA6\xFE\xA4\x14a\x04?W\x80c\xD1S\xB6\x0C\x14a\x04SW\x80c\xDE\x1E.\xEA\x14a\x04fW`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11a\0\xF4W\x80c\x8D\xA5\xCB[\x14a\x03\xCAW\x80c\x9D5R\xC7\x14a\x03\xDBW\x80c\x9F\xF6\xA7\x8C\x14a\x03\xEEW\x80c\xA7\xD03w\x14a\x04\x01W\x80c\xB2\x13\xE3\xF7\x14a\x04\tW`\0\x80\xFD[\x80c}\xD0\x9F\xEC\x14a\x03qW\x80c\x81\xA0\xC8U\x14a\x03\x9AW\x80c\x866#\xBB\x14a\x03\xAFW\x80c\x8C0\xFB~\x14a\x03\xC2W`\0\x80\xFD[\x80cF\x90H@\x11a\x01\xA8W\x80co0}\xC3\x11a\x01wW\x80co0}\xC3\x14a\x03\x1BW\x80cqP\x18\xA6\x14a\x030W\x80csd\x19\xEF\x14a\x038W\x80cw\x05/\xC1\x14a\x03KW\x80cx)\xBA\x1F\x14a\x03^W`\0\x80\xFD[\x80cF\x90H@\x14a\x02\xD9W\x80cb\x97\xF7\x92\x14a\x02\xECW\x80cl\xFD\x15S\x14a\x02\xF5W\x80cm\xAB+\xC0\x14a\x03\x08W`\0\x80\xFD[\x80c\x0E76\xCF\x11a\x01\xE4W\x80c\x0E76\xCF\x14a\x02\x80W\x80c\x1B>\xD7\"\x14a\x02\x93W\x80c \x7F\xAA}\x14a\x02\xBBW\x80cE\t\x01~\x14a\x02\xD1W`\0\x80\xFD[\x80b\xD1O\x18\x14a\x02\x15W\x80c\x01\xD2,\xCD\x14a\x02*W\x80c\x05\x01\xD5V\x14a\x02ZW\x80c\x0B\x1B;\xCA\x14a\x02mW[`\0\x80\xFD[a\x02(a\x02#6`\x04a\x18\xDCV[a\x04\xECV[\0[`\x03Ta\x02=\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02(a\x02h6`\x04a\x19_V[a\x05\xA6V[a\x02(a\x02{6`\x04a\x19|V[a\x05\xFCV[a\x02(a\x02\x8E6`\x04a\x19\x94V[a\x06mV[a\x02\x9Ba\x06\xE5V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x02QV[a\x02\xC3a\x07\nV[`@Q\x90\x81R` \x01a\x02QV[a\x02(a\x07\x1AV[`\x04Ta\x02=\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xC3`\x06T\x81V[a\x02(a\x03\x036`\x04a\x19\xC2V[a\x08\x98V[a\x02(a\x03\x166`\x04a\x19\xDDV[a\x08\xF2V[a\x03#a\t\xB1V[`@Qa\x02Q\x91\x90a\x1A\x07V[a\x02(a\t\xBDV[a\x02(a\x03F6`\x04a\x19\xC2V[a\t\xD1V[`\x05Ta\x02=\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02(a\x03l6`\x04a\x1ATV[a\n+V[a\x02\xC3a\x03\x7F6`\x04a\x19\xC2V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x0C` R`@\x90 T\x90V[a\x03\xA2a\n\x9AV[`@Qa\x02Q\x91\x90a\x1AmV[`\x02Ta\x02=\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\xA2a\x0B\xA5V[`\0T`\x01`\x01`\xA0\x1B\x03\x16a\x02=V[a\x02\xC3a\x03\xE96`\x04a\x19\xC2V[a\x0C\xA4V[a\x02(a\x03\xFC6`\x04a\x19\xC2V[a\x0C\xC5V[a\x02\xC3a\r\x1FV[a\x04\x1Ca\x04\x176`\x04a\x19\xC2V[a\x0E\x91V[`@Q\x90\x15\x15\x81R` \x01a\x02QV[a\x02(a\x04:6`\x04a\x19\xC2V[a\x0E\xB4V[`\x01Ta\x04\x1C\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`\x01Ta\x02=\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02(a\x04t6`\x04a\x19\xDDV[a\x0F\x04V[a\x02(a\x04\x876`\x04a\x19\xC2V[a\x0F\xBBV[a\x02=\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02(a\x04\xC16`\x04a\x19\xC2V[a\x10\x15V[a\x02(a\x04\xD46`\x04a\x1ATV[a\x11JV[a\x02(a\x04\xE76`\x04a\x1A\xC5V[a\x11\xB2V[`\x01T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x05\x17W`@QcR\x01\xF2\xB7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\x05SW`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\0[\x82\x81\x10\x15a\x05\xA0Wa\x05\x8E\x84\x84\x83\x81\x81\x10a\x05sWa\x05sa\x1B(V[\x90P`@\x02\x01\x806\x03\x81\x01\x90a\x05\x89\x91\x90a\x1BTV[a\x12CV[\x80a\x05\x98\x81a\x1B\xCFV[\x91PPa\x05VV[PPPPV[`\x05T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\x05\xDDW`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05JV[P`\x01\x80T\x91\x15\x15`\x01`\xA0\x1B\x02`\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80\x15\x90a\x06\"WP`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x06@W`@QcNe\xA3\x1B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06ja\x06P` \x83\x01\x83a\x19\xC2V[a\x06``@\x84\x01` \x85\x01a\x19\xC2V[\x83`@\x015a\x12\xACV[PV[`\x01T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x06\x98W`@QcR\x01\xF2\xB7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\x06\xCFW`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05JV[a\x06\xE1a\x05\x896\x84\x90\x03\x84\x01\x84a\x1BTV[PPV[`\0\x80`\0\x80a\x06\xFC`\x07T`\x08T`\x06Ta\x13\x1FV[\x92\x97\x91\x96P\x94P\x90\x92P\x90PV[`\0a\x07\x15`\tT\x90V[\x90P\x90V[`\0a\x07$a\n\x9AV[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x86W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xAA\x91\x90a\x1B\xE8V[\x90P`\0[\x82Q\x81\x10\x15a\x08\x93W`\0a\x07\xE1\x84\x83\x81Q\x81\x10a\x07\xCFWa\x07\xCFa\x1B(V[` \x02` \x01\x01Q` \x01Q\x84a\x147V[\x90P\x80\x84\x83\x81Q\x81\x10a\x07\xF6Wa\x07\xF6a\x1B(V[` \x90\x81\x02\x91\x90\x91\x01\x01QQ`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08GW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08k\x91\x90a\x1B\xE8V[\x10\x15a\x08\x8AW`@Qc\x1C\xC9*\xE1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x01a\x07\xAFV[PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\x08\xCFW`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05JV[P`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\t)W`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05JV[`@Qc'p\xA7\xEB`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x9D\xC2\x9F\xAC\x90`D\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\x94W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\xA8W=`\0\x80>=`\0\xFD[PPPPPPPV[``a\x07\x15`\ta\x14VV[a\t\xC5a\x15$V[a\t\xCF`\0a\x15~V[V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\n\x08W`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05JV[P`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\nbW`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05JV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x11\x15a\n\x8BW`@Qc\x1A\xA9\x97\xB5`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\x93a\r\x1FV[PP`\x06UV[`\t\x80T``\x91\x90`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\xBDWa\n\xBDa\x1B>V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\x02W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\n\xDBW\x90P[P\x90P`\0a\x0B\x0Fa\x06\xE5V[\x93PPPP`\0[\x83\x81\x10\x15a\x0B\x9BW`\0\x85\x82\x81T\x81\x10a\x0B3Wa\x0B3a\x1B(V[`\0\x91\x82R` \x91\x82\x90 \x01T`@\x80Q\x80\x82\x01\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x80\x82R\x92P\x90\x81\x01a\x0Bg\x83\x86a\x15\xCEV[\x81RP\x84\x83\x81Q\x81\x10a\x0B|Wa\x0B|a\x1B(V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x0B\x93\x90a\x1B\xCFV[\x91PPa\x0B\x17V[P\x90\x94\x93PPPPV[`\t\x80T``\x91\x90`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xC8Wa\x0B\xC8a\x1B>V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\rW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0B\xE6W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0C\x9CW`\0\x84\x82\x81T\x81\x10a\x0C/Wa\x0C/a\x1B(V[`\0\x91\x82R` \x80\x83 \x90\x91\x01T`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x80\x83R\x80\x85R`\x0C\x84R\x93 T\x91\x81\x01\x91\x90\x91R\x84Q\x91\x92P\x90\x84\x90\x84\x90\x81\x10a\x0C}Wa\x0C}a\x1B(V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x0C\x94\x90a\x1B\xCFV[\x91PPa\x0C\x13V[P\x93\x92PPPV[`\0\x80a\x0C\xAFa\x06\xE5V[\x93PPPPa\x0C\xBE\x83\x82a\x15\xCEV[\x93\x92PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\x0C\xFCW`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05JV[P`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xA4\x91\x90a\x1B\xE8V[\x90P`\0\x80`\0\x80a\r\xB4a\x06\xE5V[\x93P\x93P\x93P\x93Pg\r\xE0\xB6\xB3\xA7d\0\0\x82\x10\x15a\x0E\x88W`\0\x85a\r\xE1\x87a\r\xDC\x86a\x15\xF1V[a\x147V[a\r\xEB\x91\x90a\x1C\x01V[`\x08\x85\x90U`\x07\x86\x90U\x90P\x80\x15a\x0E\x86W`\x04\x80T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x81\x01\x92\x90\x92R`$\x82\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0EmW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x81W=`\0\x80>=`\0\xFD[PPPP[P[\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0B` R`@\x81 T`\xFF\x16[\x92\x91PPV[a\x0E\xBCa\x15$V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0E\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05J\x90a\x1C\x18V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x0F/W`@QcR\x01\xF2\xB7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\x0FfW`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05JV[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c@\xC1\x0F\x19\x90`D\x01a\tzV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\x0F\xF2W`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05JV[P`\x04\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x10;W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05J\x90a\x1C\x18V[`\x01T`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14a\x10\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FProposableOwnable: new owner is `D\x82\x01Rq77\xBA\x10897\xB87\xB9\xB2\xB2\x107\xBB\xB72\xB9`q\x1B`d\x82\x01R`\x84\x01a\x05JV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x111W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FProposableOwnable: this call mus`D\x82\x01R\x7Ft be made by the new owner\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05JV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90Ua\x06j\x81a\x15~V[`\x01T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x11uW`@QcR\x01\xF2\xB7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\x11\xACW`@Qcy\xFF\x07\x1D`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x05JV[P`\x08UV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80\x15\x90a\x11\xD8WP`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x11\xF6W`@QcNe\xA3\x1B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\0[\x81\x81\x10\x15a\x05\xA0W6\x84\x84\x83\x81\x81\x10a\x12\x15Wa\x12\x15a\x1B(V[``\x02\x91\x90\x91\x01\x91Pa\x120\x90Pa\x06P` \x83\x01\x83a\x19\xC2V[P\x80a\x12;\x81a\x1B\xCFV[\x91PPa\x11\xFAV[\x80Q` \x82\x01Q`\0\x81\x90\x03a\x12wW`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0C` R`@\x81 Ua\x08\x93`\t\x83a\x16\x05V[a\x12\x80\x82a\x0E\x91V[a\x12\x8FWa\x12\x8F`\t\x83a\x17\xF5V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\0\x90\x81R`\x0C` R`@\x90 UPV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12\xFBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xA0\x91\x90a\x1ChV[`\0\x80`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0\x85\x11\x15a\x13NW`@Qc4\xE7Z\xA1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x86\x93P`\0a\x13e\x86g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\x01V[\x90P`\0b\x01Q\x80a\x13w\x87Ba\x1C\x01V[a\x13\x81\x91\x90a\x1C\x85V[g\r\xE0\xB6\xB3\xA7d\0\0\x94P\x90P\x80\x15a\x13\xCEW`\0[\x81\x81\x10\x15a\x13\xB3Wa\x13\xA9\x85\x84a\x147V[\x94P`\x01\x01a\x13\x97V[Pa\x13\xC1\x81b\x01Q\x80a\x1C\xA7V[a\x13\xCB\x90\x87a\x1C\xC6V[\x95P[a\x13\xD8\x84\x89a\x147V[\x94P`\0a\x13\xE6\x87Ba\x1C\x01V[\x90P\x80\x15a\x14'W`\0a\x13\xFDb\x01Q\x80\x8Aa\x1C\x85V[\x90Pa\x14\x1F\x87a\x14\r\x84\x84a\x1C\xA7V[a\r\xDC\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\x01V[\x94PPa\x14+V[\x85\x93P[PPP\x93P\x93P\x93P\x93V[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x14L\x83\x85a\x1C\xA7V[a\x0C\xBE\x91\x90a\x1C\x85V[\x80T``\x90\x82\x90`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14xWa\x14xa\x1B>V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\xA1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x15\x1BW\x83\x81\x81T\x81\x10a\x14\xC1Wa\x14\xC1a\x1B(V[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x82\x82\x81Q\x81\x10a\x14\xF1Wa\x14\xF1a\x1B(V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x15\x13\x81a\x1B\xCFV[\x91PPa\x14\xA7V[P\x94\x93PPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05JV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0C` R`@\x81 Ta\x0C\xBE\x90\x83a\x147V[`\0a\x0E\xAEg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x18\xC7V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02\x83\x01` R`@\x90 T`\xFF\x16a\x16zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FVerifiableArray: element not fou`D\x82\x01Ra\x1B\x99`\xF2\x1B`d\x82\x01R`\x84\x01a\x05JV[`\0a\x16\x84\x83T\x90V[\x90P\x80`\x01\x03a\x16\xF5W`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x02\x84\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16\x90U`\x01\x86\x01\x90\x91R\x81 U\x82T\x83\x90\x80a\x16\xCEWa\x16\xCEa\x1C\xDEV[`\0\x82\x81R` \x90 \x81\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90UPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01\x80\x85\x01` R`@\x82 T\x91\x90\x85\x90a\x17\x1F\x90\x85a\x1C\x01V[\x81T\x81\x10a\x17/Wa\x17/a\x1B(V[`\0\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x80\x84R`\x01\x89\x01\x80\x84R`@\x80\x86 \x88\x90U\x92\x89\x16\x85R`\x02\x8A\x01\x84R\x82\x85 \x80T`\xFF\x19\x16\x90U\x90\x92R\x82 \x91\x90\x91U\x85T\x90\x91P\x81\x90\x86\x90\x84\x90\x81\x10a\x17\x92Wa\x17\x92a\x1B(V[`\0\x91\x82R` \x90\x91 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x84T\x85\x90\x80a\x17\xCCWa\x17\xCCa\x1C\xDEV[`\0\x82\x81R` \x90 \x81\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90UPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02\x83\x01` R`@\x90 T`\xFF\x16\x15a\x18pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FVerifiableArray: element already`D\x82\x01Rf exists`\xC8\x1B`d\x82\x01R`\x84\x01a\x05JV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x02\x84\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x86T\x87\x82\x01\x84R\x91\x84 \x82\x90U\x81\x01\x86U\x85\x83R\x91 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x91\x17\x90UPPV[`\0\x81a\x14Lg\r\xE0\xB6\xB3\xA7d\0\0\x85a\x1C\xA7V[`\0\x80` \x83\x85\x03\x12\x15a\x18\xEFW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x19\x07W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x19\x1BW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x19*W`\0\x80\xFD[\x86` \x82`\x06\x1B\x85\x01\x01\x11\x15a\x19?W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[\x80\x15\x15\x81\x14a\x06jW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x19qW`\0\x80\xFD[\x815a\x0C\xBE\x81a\x19QV[`\0``\x82\x84\x03\x12\x15a\x19\x8EW`\0\x80\xFD[P\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x19\x8EW`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x19\xBDW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x19\xD4W`\0\x80\xFD[a\x0C\xBE\x82a\x19\xA6V[`\0\x80`@\x83\x85\x03\x12\x15a\x19\xF0W`\0\x80\xFD[a\x19\xF9\x83a\x19\xA6V[\x94` \x93\x90\x93\x015\x93PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x1AHW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x1A#V[P\x90\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1AfW`\0\x80\xFD[P5\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x1A\xB8W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x86\x01Q\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x1A\x8AV[P\x91\x97\x96PPPPPPPV[`\0\x80` \x83\x85\x03\x12\x15a\x1A\xD8W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1A\xF0W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x1B\x04W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1B\x13W`\0\x80\xFD[\x86` ``\x83\x02\x85\x01\x01\x11\x15a\x19?W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`@\x82\x84\x03\x12\x15a\x1BfW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1B\x97WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Ra\x1B\xA3\x83a\x19\xA6V[\x81R` \x83\x015` \x82\x01R\x80\x91PP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x1B\xE1Wa\x1B\xE1a\x1B\xB9V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x1B\xFAW`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x82\x10\x15a\x1C\x13Wa\x1C\x13a\x1B\xB9V[P\x03\x90V[` \x80\x82R`0\x90\x82\x01R\x7FProposableOwnable: new owner is `@\x82\x01Rothe zero address`\x80\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x1CzW`\0\x80\xFD[\x81Qa\x0C\xBE\x81a\x19QV[`\0\x82a\x1C\xA2WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x1C\xC1Wa\x1C\xC1a\x1B\xB9V[P\x02\x90V[`\0\x82\x19\x82\x11\x15a\x1C\xD9Wa\x1C\xD9a\x1B\xB9V[P\x01\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \x8F\xCF\x94;\xB0\x86<y\x03\xF3\xF7\xF4\x14\xC4\xD8.eb,3\xB2\x18(u\xF2D\xCD\xCEclL\x03dsolcC\0\x08\x0F\x003";
    /// The deployed bytecode of the contract.
    pub static VAULT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Vault<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Vault<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Vault<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Vault<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Vault<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Vault)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Vault<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    VAULT_ABI.clone(),
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
                VAULT_ABI.clone(),
                VAULT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `emergency` (0xcaa6fea4) function
        pub fn emergency(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([202, 166, 254, 164], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emergencyResponder` (0x77052fc1) function
        pub fn emergency_responder(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([119, 5, 47, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeRecipient` (0x46904840) function
        pub fn fee_recipient(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([70, 144, 72, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeScaled` (0x6297f792) function
        pub fn fee_scaled(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([98, 151, 247, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `indexToken` (0xe7d015f2) function
        pub fn index_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([231, 208, 21, 242], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invariantCheck` (0x4509017e) function
        pub fn invariant_check(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([69, 9, 1, 126], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invokeBurn` (0x6dab2bc0) function
        pub fn invoke_burn(
            &self,
            from: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 171, 43, 192], (from, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invokeERC20` (0x0b1b3bca) function
        pub fn invoke_erc20(
            &self,
            args: InvokeERC20Args,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([11, 27, 59, 202], (args,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invokeERC20` (0xff027628) function
        pub fn invoke_erc_20_with_args(
            &self,
            args: ::std::vec::Vec<InvokeERC20Args>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([255, 2, 118, 40], args)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invokeMint` (0xde1e2eea) function
        pub fn invoke_mint(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 30, 46, 234], (to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invokeSetMultiplier` (0xfdc4cbc0) function
        pub fn invoke_set_multiplier(
            &self,
            multiplier: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 196, 203, 192], multiplier)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invokeSetNominal` (0x00d14f18) function
        pub fn invoke_set_nominal(
            &self,
            args: ::std::vec::Vec<SetNominalArgs>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([0, 209, 79, 24], args)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invokeSetNominal` (0x0e3736cf) function
        pub fn invoke_set_nominal_with_args(
            &self,
            args: SetNominalArgs,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 55, 54, 207], (args,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isUnderlying` (0xb213e3f7) function
        pub fn is_underlying(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([178, 19, 227, 247], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `issuance` (0x863623bb) function
        pub fn issuance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([134, 54, 35, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multiplier` (0x1b3ed722) function
        pub fn multiplier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([27, 62, 215, 34], ())
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
        ///Calls the contract's `proposeOwner` (0xb5ed298a) function
        pub fn propose_owner(
            &self,
            proposed_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 237, 41, 138], proposed_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposedOwner` (0xd153b60c) function
        pub fn proposed_owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([209, 83, 182, 12], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `realUnits` (0x81a0c855) function
        pub fn real_units(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<
                (::ethers::core::types::Address, ::ethers::core::types::U256),
            >,
        > {
            self.0
                .method_hash([129, 160, 200, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `realUnits` (0x9d3552c7) function
        pub fn real_units_with_token(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([157, 53, 82, 199], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rebalancer` (0x01d22ccd) function
        pub fn rebalancer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([1, 210, 44, 205], ())
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
        ///Calls the contract's `setEmergency` (0x0501d556) function
        pub fn set_emergency(
            &self,
            emergency: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 1, 213, 86], emergency)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEmergencyResponder` (0x736419ef) function
        pub fn set_emergency_responder(
            &self,
            emergency_responder: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([115, 100, 25, 239], emergency_responder)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeRecipient` (0xe74b981b) function
        pub fn set_fee_recipient(
            &self,
            fee_recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 75, 152, 27], fee_recipient)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeScaled` (0x7829ba1f) function
        pub fn set_fee_scaled(
            &self,
            fee_scaled: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 41, 186, 31], fee_scaled)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setIssuance` (0x9ff6a78c) function
        pub fn set_issuance(
            &self,
            issuance: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 246, 167, 140], issuance)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRebalancer` (0x6cfd1553) function
        pub fn set_rebalancer(
            &self,
            rebalancer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 253, 21, 83], rebalancer)
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
        ///Calls the contract's `tryInflation` (0xa7d03377) function
        pub fn try_inflation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([167, 208, 51, 119], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `underlying` (0x6f307dc3) function
        pub fn underlying(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([111, 48, 125, 195], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `underlyingLength` (0x207faa7d) function
        pub fn underlying_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([32, 127, 170, 125], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `virtualUnits` (0x7dd09fec) function
        pub fn virtual_units_with_token(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([125, 208, 159, 236], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `virtualUnits` (0x8c30fb7e) function
        pub fn virtual_units(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<TokenInfo>> {
            self.0
                .method_hash([140, 48, 251, 126], ())
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
    for Vault<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AMKTVaultEmergency` with signature `AMKTVaultEmergency()` and selector `0x5201f2b7`
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
    #[etherror(name = "AMKTVaultEmergency", abi = "AMKTVaultEmergency()")]
    pub struct AMKTVaultEmergency;
    ///Custom Error type `AMKTVaultFeeTooLarge` with signature `AMKTVaultFeeTooLarge()` and selector `0xd54cbda8`
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
    #[etherror(name = "AMKTVaultFeeTooLarge", abi = "AMKTVaultFeeTooLarge()")]
    pub struct AMKTVaultFeeTooLarge;
    ///Custom Error type `AMKTVaultOnly` with signature `AMKTVaultOnly(address)` and selector `0xf3fe0e3a`
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
    #[etherror(name = "AMKTVaultOnly", abi = "AMKTVaultOnly(address)")]
    pub struct AMKTVaultOnly {
        pub who: ::ethers::core::types::Address,
    }
    ///Custom Error type `AMKTVaultOnlyInvokers` with signature `AMKTVaultOnlyInvokers()` and selector `0x4e65a31b`
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
    #[etherror(name = "AMKTVaultOnlyInvokers", abi = "AMKTVaultOnlyInvokers()")]
    pub struct AMKTVaultOnlyInvokers;
    ///Custom Error type `MultiplierFeeTooHigh` with signature `MultiplierFeeTooHigh()` and selector `0xd39d6a84`
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
    #[etherror(name = "MultiplierFeeTooHigh", abi = "MultiplierFeeTooHigh()")]
    pub struct MultiplierFeeTooHigh;
    ///Custom Error type `VaultInvariant` with signature `VaultInvariant()` and selector `0x1cc92ae1`
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
    #[etherror(name = "VaultInvariant", abi = "VaultInvariant()")]
    pub struct VaultInvariant;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VaultErrors {
        AMKTVaultEmergency(AMKTVaultEmergency),
        AMKTVaultFeeTooLarge(AMKTVaultFeeTooLarge),
        AMKTVaultOnly(AMKTVaultOnly),
        AMKTVaultOnlyInvokers(AMKTVaultOnlyInvokers),
        MultiplierFeeTooHigh(MultiplierFeeTooHigh),
        VaultInvariant(VaultInvariant),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for VaultErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AMKTVaultEmergency as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AMKTVaultEmergency(decoded));
            }
            if let Ok(decoded) = <AMKTVaultFeeTooLarge as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AMKTVaultFeeTooLarge(decoded));
            }
            if let Ok(decoded) = <AMKTVaultOnly as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AMKTVaultOnly(decoded));
            }
            if let Ok(decoded) = <AMKTVaultOnlyInvokers as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AMKTVaultOnlyInvokers(decoded));
            }
            if let Ok(decoded) = <MultiplierFeeTooHigh as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MultiplierFeeTooHigh(decoded));
            }
            if let Ok(decoded) = <VaultInvariant as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VaultInvariant(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for VaultErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AMKTVaultEmergency(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AMKTVaultFeeTooLarge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AMKTVaultOnly(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AMKTVaultOnlyInvokers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MultiplierFeeTooHigh(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VaultInvariant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for VaultErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AMKTVaultEmergency as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AMKTVaultFeeTooLarge as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AMKTVaultOnly as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AMKTVaultOnlyInvokers as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MultiplierFeeTooHigh as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <VaultInvariant as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for VaultErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AMKTVaultEmergency(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AMKTVaultFeeTooLarge(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AMKTVaultOnly(element) => ::core::fmt::Display::fmt(element, f),
                Self::AMKTVaultOnlyInvokers(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MultiplierFeeTooHigh(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VaultInvariant(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for VaultErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AMKTVaultEmergency> for VaultErrors {
        fn from(value: AMKTVaultEmergency) -> Self {
            Self::AMKTVaultEmergency(value)
        }
    }
    impl ::core::convert::From<AMKTVaultFeeTooLarge> for VaultErrors {
        fn from(value: AMKTVaultFeeTooLarge) -> Self {
            Self::AMKTVaultFeeTooLarge(value)
        }
    }
    impl ::core::convert::From<AMKTVaultOnly> for VaultErrors {
        fn from(value: AMKTVaultOnly) -> Self {
            Self::AMKTVaultOnly(value)
        }
    }
    impl ::core::convert::From<AMKTVaultOnlyInvokers> for VaultErrors {
        fn from(value: AMKTVaultOnlyInvokers) -> Self {
            Self::AMKTVaultOnlyInvokers(value)
        }
    }
    impl ::core::convert::From<MultiplierFeeTooHigh> for VaultErrors {
        fn from(value: MultiplierFeeTooHigh) -> Self {
            Self::MultiplierFeeTooHigh(value)
        }
    }
    impl ::core::convert::From<VaultInvariant> for VaultErrors {
        fn from(value: VaultInvariant) -> Self {
            Self::VaultInvariant(value)
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
    ///Container type for all input parameters for the `emergency` function with signature `emergency()` and selector `0xcaa6fea4`
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
    #[ethcall(name = "emergency", abi = "emergency()")]
    pub struct EmergencyCall;
    ///Container type for all input parameters for the `emergencyResponder` function with signature `emergencyResponder()` and selector `0x77052fc1`
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
    #[ethcall(name = "emergencyResponder", abi = "emergencyResponder()")]
    pub struct EmergencyResponderCall;
    ///Container type for all input parameters for the `feeRecipient` function with signature `feeRecipient()` and selector `0x46904840`
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
    #[ethcall(name = "feeRecipient", abi = "feeRecipient()")]
    pub struct FeeRecipientCall;
    ///Container type for all input parameters for the `feeScaled` function with signature `feeScaled()` and selector `0x6297f792`
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
    #[ethcall(name = "feeScaled", abi = "feeScaled()")]
    pub struct FeeScaledCall;
    ///Container type for all input parameters for the `indexToken` function with signature `indexToken()` and selector `0xe7d015f2`
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
    #[ethcall(name = "indexToken", abi = "indexToken()")]
    pub struct IndexTokenCall;
    ///Container type for all input parameters for the `invariantCheck` function with signature `invariantCheck()` and selector `0x4509017e`
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
    #[ethcall(name = "invariantCheck", abi = "invariantCheck()")]
    pub struct InvariantCheckCall;
    ///Container type for all input parameters for the `invokeBurn` function with signature `invokeBurn(address,uint256)` and selector `0x6dab2bc0`
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
    #[ethcall(name = "invokeBurn", abi = "invokeBurn(address,uint256)")]
    pub struct InvokeBurnCall {
        pub from: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `invokeERC20` function with signature `invokeERC20((address,address,uint256))` and selector `0x0b1b3bca`
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
    #[ethcall(name = "invokeERC20", abi = "invokeERC20((address,address,uint256))")]
    pub struct InvokeERC20Call {
        pub args: InvokeERC20Args,
    }
    ///Container type for all input parameters for the `invokeERC20` function with signature `invokeERC20((address,address,uint256)[])` and selector `0xff027628`
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
    #[ethcall(name = "invokeERC20", abi = "invokeERC20((address,address,uint256)[])")]
    pub struct InvokeErc20WithArgsCall {
        pub args: ::std::vec::Vec<InvokeERC20Args>,
    }
    ///Container type for all input parameters for the `invokeMint` function with signature `invokeMint(address,uint256)` and selector `0xde1e2eea`
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
    #[ethcall(name = "invokeMint", abi = "invokeMint(address,uint256)")]
    pub struct InvokeMintCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `invokeSetMultiplier` function with signature `invokeSetMultiplier(uint256)` and selector `0xfdc4cbc0`
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
    #[ethcall(name = "invokeSetMultiplier", abi = "invokeSetMultiplier(uint256)")]
    pub struct InvokeSetMultiplierCall {
        pub multiplier: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `invokeSetNominal` function with signature `invokeSetNominal((address,uint256)[])` and selector `0x00d14f18`
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
    #[ethcall(name = "invokeSetNominal", abi = "invokeSetNominal((address,uint256)[])")]
    pub struct InvokeSetNominalCall {
        pub args: ::std::vec::Vec<SetNominalArgs>,
    }
    ///Container type for all input parameters for the `invokeSetNominal` function with signature `invokeSetNominal((address,uint256))` and selector `0x0e3736cf`
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
    #[ethcall(name = "invokeSetNominal", abi = "invokeSetNominal((address,uint256))")]
    pub struct InvokeSetNominalWithArgsCall {
        pub args: SetNominalArgs,
    }
    ///Container type for all input parameters for the `isUnderlying` function with signature `isUnderlying(address)` and selector `0xb213e3f7`
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
    #[ethcall(name = "isUnderlying", abi = "isUnderlying(address)")]
    pub struct IsUnderlyingCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `issuance` function with signature `issuance()` and selector `0x863623bb`
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
    #[ethcall(name = "issuance", abi = "issuance()")]
    pub struct IssuanceCall;
    ///Container type for all input parameters for the `multiplier` function with signature `multiplier()` and selector `0x1b3ed722`
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
    #[ethcall(name = "multiplier", abi = "multiplier()")]
    pub struct MultiplierCall;
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
    ///Container type for all input parameters for the `proposeOwner` function with signature `proposeOwner(address)` and selector `0xb5ed298a`
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
    #[ethcall(name = "proposeOwner", abi = "proposeOwner(address)")]
    pub struct ProposeOwnerCall {
        pub proposed_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `proposedOwner` function with signature `proposedOwner()` and selector `0xd153b60c`
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
    #[ethcall(name = "proposedOwner", abi = "proposedOwner()")]
    pub struct ProposedOwnerCall;
    ///Container type for all input parameters for the `realUnits` function with signature `realUnits()` and selector `0x81a0c855`
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
    #[ethcall(name = "realUnits", abi = "realUnits()")]
    pub struct RealUnitsCall;
    ///Container type for all input parameters for the `realUnits` function with signature `realUnits(address)` and selector `0x9d3552c7`
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
    #[ethcall(name = "realUnits", abi = "realUnits(address)")]
    pub struct RealUnitsWithTokenCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `rebalancer` function with signature `rebalancer()` and selector `0x01d22ccd`
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
    #[ethcall(name = "rebalancer", abi = "rebalancer()")]
    pub struct RebalancerCall;
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
    ///Container type for all input parameters for the `setEmergency` function with signature `setEmergency(bool)` and selector `0x0501d556`
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
    #[ethcall(name = "setEmergency", abi = "setEmergency(bool)")]
    pub struct SetEmergencyCall {
        pub emergency: bool,
    }
    ///Container type for all input parameters for the `setEmergencyResponder` function with signature `setEmergencyResponder(address)` and selector `0x736419ef`
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
    #[ethcall(name = "setEmergencyResponder", abi = "setEmergencyResponder(address)")]
    pub struct SetEmergencyResponderCall {
        pub emergency_responder: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setFeeRecipient` function with signature `setFeeRecipient(address)` and selector `0xe74b981b`
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
    #[ethcall(name = "setFeeRecipient", abi = "setFeeRecipient(address)")]
    pub struct SetFeeRecipientCall {
        pub fee_recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setFeeScaled` function with signature `setFeeScaled(uint256)` and selector `0x7829ba1f`
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
    #[ethcall(name = "setFeeScaled", abi = "setFeeScaled(uint256)")]
    pub struct SetFeeScaledCall {
        pub fee_scaled: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setIssuance` function with signature `setIssuance(address)` and selector `0x9ff6a78c`
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
    #[ethcall(name = "setIssuance", abi = "setIssuance(address)")]
    pub struct SetIssuanceCall {
        pub issuance: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setRebalancer` function with signature `setRebalancer(address)` and selector `0x6cfd1553`
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
    #[ethcall(name = "setRebalancer", abi = "setRebalancer(address)")]
    pub struct SetRebalancerCall {
        pub rebalancer: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `tryInflation` function with signature `tryInflation()` and selector `0xa7d03377`
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
    #[ethcall(name = "tryInflation", abi = "tryInflation()")]
    pub struct TryInflationCall;
    ///Container type for all input parameters for the `underlying` function with signature `underlying()` and selector `0x6f307dc3`
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
    #[ethcall(name = "underlying", abi = "underlying()")]
    pub struct UnderlyingCall;
    ///Container type for all input parameters for the `underlyingLength` function with signature `underlyingLength()` and selector `0x207faa7d`
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
    #[ethcall(name = "underlyingLength", abi = "underlyingLength()")]
    pub struct UnderlyingLengthCall;
    ///Container type for all input parameters for the `virtualUnits` function with signature `virtualUnits(address)` and selector `0x7dd09fec`
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
    #[ethcall(name = "virtualUnits", abi = "virtualUnits(address)")]
    pub struct VirtualUnitsWithTokenCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `virtualUnits` function with signature `virtualUnits()` and selector `0x8c30fb7e`
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
    #[ethcall(name = "virtualUnits", abi = "virtualUnits()")]
    pub struct VirtualUnitsCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VaultCalls {
        Emergency(EmergencyCall),
        EmergencyResponder(EmergencyResponderCall),
        FeeRecipient(FeeRecipientCall),
        FeeScaled(FeeScaledCall),
        IndexToken(IndexTokenCall),
        InvariantCheck(InvariantCheckCall),
        InvokeBurn(InvokeBurnCall),
        InvokeERC20(InvokeERC20Call),
        InvokeErc20WithArgs(InvokeErc20WithArgsCall),
        InvokeMint(InvokeMintCall),
        InvokeSetMultiplier(InvokeSetMultiplierCall),
        InvokeSetNominal(InvokeSetNominalCall),
        InvokeSetNominalWithArgs(InvokeSetNominalWithArgsCall),
        IsUnderlying(IsUnderlyingCall),
        Issuance(IssuanceCall),
        Multiplier(MultiplierCall),
        Owner(OwnerCall),
        ProposeOwner(ProposeOwnerCall),
        ProposedOwner(ProposedOwnerCall),
        RealUnits(RealUnitsCall),
        RealUnitsWithToken(RealUnitsWithTokenCall),
        Rebalancer(RebalancerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetEmergency(SetEmergencyCall),
        SetEmergencyResponder(SetEmergencyResponderCall),
        SetFeeRecipient(SetFeeRecipientCall),
        SetFeeScaled(SetFeeScaledCall),
        SetIssuance(SetIssuanceCall),
        SetRebalancer(SetRebalancerCall),
        TransferOwnership(TransferOwnershipCall),
        TryInflation(TryInflationCall),
        Underlying(UnderlyingCall),
        UnderlyingLength(UnderlyingLengthCall),
        VirtualUnitsWithToken(VirtualUnitsWithTokenCall),
        VirtualUnits(VirtualUnitsCall),
    }
    impl ::ethers::core::abi::AbiDecode for VaultCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <EmergencyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Emergency(decoded));
            }
            if let Ok(decoded) = <EmergencyResponderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmergencyResponder(decoded));
            }
            if let Ok(decoded) = <FeeRecipientCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeeRecipient(decoded));
            }
            if let Ok(decoded) = <FeeScaledCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeeScaled(decoded));
            }
            if let Ok(decoded) = <IndexTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IndexToken(decoded));
            }
            if let Ok(decoded) = <InvariantCheckCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvariantCheck(decoded));
            }
            if let Ok(decoded) = <InvokeBurnCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvokeBurn(decoded));
            }
            if let Ok(decoded) = <InvokeERC20Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvokeERC20(decoded));
            }
            if let Ok(decoded) = <InvokeErc20WithArgsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvokeErc20WithArgs(decoded));
            }
            if let Ok(decoded) = <InvokeMintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvokeMint(decoded));
            }
            if let Ok(decoded) = <InvokeSetMultiplierCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvokeSetMultiplier(decoded));
            }
            if let Ok(decoded) = <InvokeSetNominalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvokeSetNominal(decoded));
            }
            if let Ok(decoded) = <InvokeSetNominalWithArgsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvokeSetNominalWithArgs(decoded));
            }
            if let Ok(decoded) = <IsUnderlyingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsUnderlying(decoded));
            }
            if let Ok(decoded) = <IssuanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Issuance(decoded));
            }
            if let Ok(decoded) = <MultiplierCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Multiplier(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <ProposeOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProposeOwner(decoded));
            }
            if let Ok(decoded) = <ProposedOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProposedOwner(decoded));
            }
            if let Ok(decoded) = <RealUnitsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RealUnits(decoded));
            }
            if let Ok(decoded) = <RealUnitsWithTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RealUnitsWithToken(decoded));
            }
            if let Ok(decoded) = <RebalancerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Rebalancer(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetEmergencyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetEmergency(decoded));
            }
            if let Ok(decoded) = <SetEmergencyResponderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetEmergencyResponder(decoded));
            }
            if let Ok(decoded) = <SetFeeRecipientCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetFeeRecipient(decoded));
            }
            if let Ok(decoded) = <SetFeeScaledCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetFeeScaled(decoded));
            }
            if let Ok(decoded) = <SetIssuanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetIssuance(decoded));
            }
            if let Ok(decoded) = <SetRebalancerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetRebalancer(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <TryInflationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TryInflation(decoded));
            }
            if let Ok(decoded) = <UnderlyingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Underlying(decoded));
            }
            if let Ok(decoded) = <UnderlyingLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnderlyingLength(decoded));
            }
            if let Ok(decoded) = <VirtualUnitsWithTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VirtualUnitsWithToken(decoded));
            }
            if let Ok(decoded) = <VirtualUnitsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VirtualUnits(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for VaultCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Emergency(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmergencyResponder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeRecipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeScaled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IndexToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvariantCheck(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvokeBurn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvokeERC20(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvokeErc20WithArgs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvokeMint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvokeSetMultiplier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvokeSetNominal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvokeSetNominalWithArgs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsUnderlying(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Issuance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Multiplier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProposeOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposedOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RealUnits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RealUnitsWithToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Rebalancer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEmergency(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEmergencyResponder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFeeRecipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFeeScaled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetIssuance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetRebalancer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TryInflation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Underlying(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnderlyingLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VirtualUnitsWithToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VirtualUnits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for VaultCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Emergency(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmergencyResponder(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeeRecipient(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeScaled(element) => ::core::fmt::Display::fmt(element, f),
                Self::IndexToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvariantCheck(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvokeBurn(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvokeERC20(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvokeErc20WithArgs(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvokeMint(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvokeSetMultiplier(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvokeSetNominal(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvokeSetNominalWithArgs(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsUnderlying(element) => ::core::fmt::Display::fmt(element, f),
                Self::Issuance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Multiplier(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposeOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposedOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RealUnits(element) => ::core::fmt::Display::fmt(element, f),
                Self::RealUnitsWithToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Rebalancer(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEmergency(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEmergencyResponder(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetFeeRecipient(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeScaled(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetIssuance(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRebalancer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TryInflation(element) => ::core::fmt::Display::fmt(element, f),
                Self::Underlying(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnderlyingLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::VirtualUnitsWithToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VirtualUnits(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EmergencyCall> for VaultCalls {
        fn from(value: EmergencyCall) -> Self {
            Self::Emergency(value)
        }
    }
    impl ::core::convert::From<EmergencyResponderCall> for VaultCalls {
        fn from(value: EmergencyResponderCall) -> Self {
            Self::EmergencyResponder(value)
        }
    }
    impl ::core::convert::From<FeeRecipientCall> for VaultCalls {
        fn from(value: FeeRecipientCall) -> Self {
            Self::FeeRecipient(value)
        }
    }
    impl ::core::convert::From<FeeScaledCall> for VaultCalls {
        fn from(value: FeeScaledCall) -> Self {
            Self::FeeScaled(value)
        }
    }
    impl ::core::convert::From<IndexTokenCall> for VaultCalls {
        fn from(value: IndexTokenCall) -> Self {
            Self::IndexToken(value)
        }
    }
    impl ::core::convert::From<InvariantCheckCall> for VaultCalls {
        fn from(value: InvariantCheckCall) -> Self {
            Self::InvariantCheck(value)
        }
    }
    impl ::core::convert::From<InvokeBurnCall> for VaultCalls {
        fn from(value: InvokeBurnCall) -> Self {
            Self::InvokeBurn(value)
        }
    }
    impl ::core::convert::From<InvokeERC20Call> for VaultCalls {
        fn from(value: InvokeERC20Call) -> Self {
            Self::InvokeERC20(value)
        }
    }
    impl ::core::convert::From<InvokeErc20WithArgsCall> for VaultCalls {
        fn from(value: InvokeErc20WithArgsCall) -> Self {
            Self::InvokeErc20WithArgs(value)
        }
    }
    impl ::core::convert::From<InvokeMintCall> for VaultCalls {
        fn from(value: InvokeMintCall) -> Self {
            Self::InvokeMint(value)
        }
    }
    impl ::core::convert::From<InvokeSetMultiplierCall> for VaultCalls {
        fn from(value: InvokeSetMultiplierCall) -> Self {
            Self::InvokeSetMultiplier(value)
        }
    }
    impl ::core::convert::From<InvokeSetNominalCall> for VaultCalls {
        fn from(value: InvokeSetNominalCall) -> Self {
            Self::InvokeSetNominal(value)
        }
    }
    impl ::core::convert::From<InvokeSetNominalWithArgsCall> for VaultCalls {
        fn from(value: InvokeSetNominalWithArgsCall) -> Self {
            Self::InvokeSetNominalWithArgs(value)
        }
    }
    impl ::core::convert::From<IsUnderlyingCall> for VaultCalls {
        fn from(value: IsUnderlyingCall) -> Self {
            Self::IsUnderlying(value)
        }
    }
    impl ::core::convert::From<IssuanceCall> for VaultCalls {
        fn from(value: IssuanceCall) -> Self {
            Self::Issuance(value)
        }
    }
    impl ::core::convert::From<MultiplierCall> for VaultCalls {
        fn from(value: MultiplierCall) -> Self {
            Self::Multiplier(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for VaultCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<ProposeOwnerCall> for VaultCalls {
        fn from(value: ProposeOwnerCall) -> Self {
            Self::ProposeOwner(value)
        }
    }
    impl ::core::convert::From<ProposedOwnerCall> for VaultCalls {
        fn from(value: ProposedOwnerCall) -> Self {
            Self::ProposedOwner(value)
        }
    }
    impl ::core::convert::From<RealUnitsCall> for VaultCalls {
        fn from(value: RealUnitsCall) -> Self {
            Self::RealUnits(value)
        }
    }
    impl ::core::convert::From<RealUnitsWithTokenCall> for VaultCalls {
        fn from(value: RealUnitsWithTokenCall) -> Self {
            Self::RealUnitsWithToken(value)
        }
    }
    impl ::core::convert::From<RebalancerCall> for VaultCalls {
        fn from(value: RebalancerCall) -> Self {
            Self::Rebalancer(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for VaultCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetEmergencyCall> for VaultCalls {
        fn from(value: SetEmergencyCall) -> Self {
            Self::SetEmergency(value)
        }
    }
    impl ::core::convert::From<SetEmergencyResponderCall> for VaultCalls {
        fn from(value: SetEmergencyResponderCall) -> Self {
            Self::SetEmergencyResponder(value)
        }
    }
    impl ::core::convert::From<SetFeeRecipientCall> for VaultCalls {
        fn from(value: SetFeeRecipientCall) -> Self {
            Self::SetFeeRecipient(value)
        }
    }
    impl ::core::convert::From<SetFeeScaledCall> for VaultCalls {
        fn from(value: SetFeeScaledCall) -> Self {
            Self::SetFeeScaled(value)
        }
    }
    impl ::core::convert::From<SetIssuanceCall> for VaultCalls {
        fn from(value: SetIssuanceCall) -> Self {
            Self::SetIssuance(value)
        }
    }
    impl ::core::convert::From<SetRebalancerCall> for VaultCalls {
        fn from(value: SetRebalancerCall) -> Self {
            Self::SetRebalancer(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for VaultCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<TryInflationCall> for VaultCalls {
        fn from(value: TryInflationCall) -> Self {
            Self::TryInflation(value)
        }
    }
    impl ::core::convert::From<UnderlyingCall> for VaultCalls {
        fn from(value: UnderlyingCall) -> Self {
            Self::Underlying(value)
        }
    }
    impl ::core::convert::From<UnderlyingLengthCall> for VaultCalls {
        fn from(value: UnderlyingLengthCall) -> Self {
            Self::UnderlyingLength(value)
        }
    }
    impl ::core::convert::From<VirtualUnitsWithTokenCall> for VaultCalls {
        fn from(value: VirtualUnitsWithTokenCall) -> Self {
            Self::VirtualUnitsWithToken(value)
        }
    }
    impl ::core::convert::From<VirtualUnitsCall> for VaultCalls {
        fn from(value: VirtualUnitsCall) -> Self {
            Self::VirtualUnits(value)
        }
    }
    ///Container type for all return fields from the `emergency` function with signature `emergency()` and selector `0xcaa6fea4`
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
    pub struct EmergencyReturn(pub bool);
    ///Container type for all return fields from the `emergencyResponder` function with signature `emergencyResponder()` and selector `0x77052fc1`
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
    pub struct EmergencyResponderReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `feeRecipient` function with signature `feeRecipient()` and selector `0x46904840`
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
    pub struct FeeRecipientReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `feeScaled` function with signature `feeScaled()` and selector `0x6297f792`
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
    pub struct FeeScaledReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `indexToken` function with signature `indexToken()` and selector `0xe7d015f2`
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
    pub struct IndexTokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isUnderlying` function with signature `isUnderlying(address)` and selector `0xb213e3f7`
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
    pub struct IsUnderlyingReturn(pub bool);
    ///Container type for all return fields from the `issuance` function with signature `issuance()` and selector `0x863623bb`
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
    pub struct IssuanceReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `multiplier` function with signature `multiplier()` and selector `0x1b3ed722`
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
    pub struct MultiplierReturn {
        pub tracked_timestamp: ::ethers::core::types::U256,
        pub tracked_multiplier: ::ethers::core::types::U256,
        pub new_fee_accrued: ::ethers::core::types::U256,
        pub current_multiplier: ::ethers::core::types::U256,
    }
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
    ///Container type for all return fields from the `proposedOwner` function with signature `proposedOwner()` and selector `0xd153b60c`
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
    pub struct ProposedOwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `realUnits` function with signature `realUnits()` and selector `0x81a0c855`
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
    pub struct RealUnitsReturn(
        pub ::std::vec::Vec<
            (::ethers::core::types::Address, ::ethers::core::types::U256),
        >,
    );
    ///Container type for all return fields from the `realUnits` function with signature `realUnits(address)` and selector `0x9d3552c7`
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
    pub struct RealUnitsWithTokenReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rebalancer` function with signature `rebalancer()` and selector `0x01d22ccd`
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
    pub struct RebalancerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `tryInflation` function with signature `tryInflation()` and selector `0xa7d03377`
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
    pub struct TryInflationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `underlying` function with signature `underlying()` and selector `0x6f307dc3`
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
    pub struct UnderlyingReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `underlyingLength` function with signature `underlyingLength()` and selector `0x207faa7d`
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
    pub struct UnderlyingLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `virtualUnits` function with signature `virtualUnits(address)` and selector `0x7dd09fec`
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
    pub struct VirtualUnitsWithTokenReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `virtualUnits` function with signature `virtualUnits()` and selector `0x8c30fb7e`
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
    pub struct VirtualUnitsReturn(pub ::std::vec::Vec<TokenInfo>);
}
