pub use i_vault::*;
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
pub mod i_vault {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
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
                                    name: ::std::borrow::ToOwned::to_owned("multiplier"),
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
                                    name: ::std::borrow::ToOwned::to_owned("target"),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "lastTrackedTimestamp",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "lastTrackedMultiplier",
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned("multiplier"),
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
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IVAULT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IVault<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IVault<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IVault<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IVault<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IVault<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IVault)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IVault<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IVAULT_ABI.clone(),
                    client,
                ),
            )
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
            target: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([178, 19, 227, 247], target)
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
        pub fn virtual_units(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([125, 208, 159, 236], token)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IVault<M> {
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
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IVaultErrors {
        AMKTVaultEmergency(AMKTVaultEmergency),
        AMKTVaultFeeTooLarge(AMKTVaultFeeTooLarge),
        AMKTVaultOnly(AMKTVaultOnly),
        AMKTVaultOnlyInvokers(AMKTVaultOnlyInvokers),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for IVaultErrors {
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IVaultErrors {
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
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for IVaultErrors {
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
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for IVaultErrors {
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
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for IVaultErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AMKTVaultEmergency> for IVaultErrors {
        fn from(value: AMKTVaultEmergency) -> Self {
            Self::AMKTVaultEmergency(value)
        }
    }
    impl ::core::convert::From<AMKTVaultFeeTooLarge> for IVaultErrors {
        fn from(value: AMKTVaultFeeTooLarge) -> Self {
            Self::AMKTVaultFeeTooLarge(value)
        }
    }
    impl ::core::convert::From<AMKTVaultOnly> for IVaultErrors {
        fn from(value: AMKTVaultOnly) -> Self {
            Self::AMKTVaultOnly(value)
        }
    }
    impl ::core::convert::From<AMKTVaultOnlyInvokers> for IVaultErrors {
        fn from(value: AMKTVaultOnlyInvokers) -> Self {
            Self::AMKTVaultOnlyInvokers(value)
        }
    }
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
        pub target: ::ethers::core::types::Address,
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
    pub struct VirtualUnitsCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IVaultCalls {
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
        RealUnits(RealUnitsCall),
        RealUnitsWithToken(RealUnitsWithTokenCall),
        Rebalancer(RebalancerCall),
        TryInflation(TryInflationCall),
        Underlying(UnderlyingCall),
        UnderlyingLength(UnderlyingLengthCall),
        VirtualUnits(VirtualUnitsCall),
    }
    impl ::ethers::core::abi::AbiDecode for IVaultCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
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
            if let Ok(decoded) = <VirtualUnitsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VirtualUnits(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IVaultCalls {
        fn encode(self) -> Vec<u8> {
            match self {
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
                Self::RealUnits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RealUnitsWithToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Rebalancer(element) => {
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
                Self::VirtualUnits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IVaultCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
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
                Self::RealUnits(element) => ::core::fmt::Display::fmt(element, f),
                Self::RealUnitsWithToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Rebalancer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TryInflation(element) => ::core::fmt::Display::fmt(element, f),
                Self::Underlying(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnderlyingLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::VirtualUnits(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FeeRecipientCall> for IVaultCalls {
        fn from(value: FeeRecipientCall) -> Self {
            Self::FeeRecipient(value)
        }
    }
    impl ::core::convert::From<FeeScaledCall> for IVaultCalls {
        fn from(value: FeeScaledCall) -> Self {
            Self::FeeScaled(value)
        }
    }
    impl ::core::convert::From<IndexTokenCall> for IVaultCalls {
        fn from(value: IndexTokenCall) -> Self {
            Self::IndexToken(value)
        }
    }
    impl ::core::convert::From<InvariantCheckCall> for IVaultCalls {
        fn from(value: InvariantCheckCall) -> Self {
            Self::InvariantCheck(value)
        }
    }
    impl ::core::convert::From<InvokeBurnCall> for IVaultCalls {
        fn from(value: InvokeBurnCall) -> Self {
            Self::InvokeBurn(value)
        }
    }
    impl ::core::convert::From<InvokeERC20Call> for IVaultCalls {
        fn from(value: InvokeERC20Call) -> Self {
            Self::InvokeERC20(value)
        }
    }
    impl ::core::convert::From<InvokeErc20WithArgsCall> for IVaultCalls {
        fn from(value: InvokeErc20WithArgsCall) -> Self {
            Self::InvokeErc20WithArgs(value)
        }
    }
    impl ::core::convert::From<InvokeMintCall> for IVaultCalls {
        fn from(value: InvokeMintCall) -> Self {
            Self::InvokeMint(value)
        }
    }
    impl ::core::convert::From<InvokeSetMultiplierCall> for IVaultCalls {
        fn from(value: InvokeSetMultiplierCall) -> Self {
            Self::InvokeSetMultiplier(value)
        }
    }
    impl ::core::convert::From<InvokeSetNominalCall> for IVaultCalls {
        fn from(value: InvokeSetNominalCall) -> Self {
            Self::InvokeSetNominal(value)
        }
    }
    impl ::core::convert::From<InvokeSetNominalWithArgsCall> for IVaultCalls {
        fn from(value: InvokeSetNominalWithArgsCall) -> Self {
            Self::InvokeSetNominalWithArgs(value)
        }
    }
    impl ::core::convert::From<IsUnderlyingCall> for IVaultCalls {
        fn from(value: IsUnderlyingCall) -> Self {
            Self::IsUnderlying(value)
        }
    }
    impl ::core::convert::From<IssuanceCall> for IVaultCalls {
        fn from(value: IssuanceCall) -> Self {
            Self::Issuance(value)
        }
    }
    impl ::core::convert::From<MultiplierCall> for IVaultCalls {
        fn from(value: MultiplierCall) -> Self {
            Self::Multiplier(value)
        }
    }
    impl ::core::convert::From<RealUnitsCall> for IVaultCalls {
        fn from(value: RealUnitsCall) -> Self {
            Self::RealUnits(value)
        }
    }
    impl ::core::convert::From<RealUnitsWithTokenCall> for IVaultCalls {
        fn from(value: RealUnitsWithTokenCall) -> Self {
            Self::RealUnitsWithToken(value)
        }
    }
    impl ::core::convert::From<RebalancerCall> for IVaultCalls {
        fn from(value: RebalancerCall) -> Self {
            Self::Rebalancer(value)
        }
    }
    impl ::core::convert::From<TryInflationCall> for IVaultCalls {
        fn from(value: TryInflationCall) -> Self {
            Self::TryInflation(value)
        }
    }
    impl ::core::convert::From<UnderlyingCall> for IVaultCalls {
        fn from(value: UnderlyingCall) -> Self {
            Self::Underlying(value)
        }
    }
    impl ::core::convert::From<UnderlyingLengthCall> for IVaultCalls {
        fn from(value: UnderlyingLengthCall) -> Self {
            Self::UnderlyingLength(value)
        }
    }
    impl ::core::convert::From<VirtualUnitsCall> for IVaultCalls {
        fn from(value: VirtualUnitsCall) -> Self {
            Self::VirtualUnits(value)
        }
    }
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
        pub last_tracked_timestamp: ::ethers::core::types::U256,
        pub last_tracked_multiplier: ::ethers::core::types::U256,
        pub new_fee_accrued: ::ethers::core::types::U256,
        pub multiplier: ::ethers::core::types::U256,
    }
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
    pub struct VirtualUnitsReturn(pub ::ethers::core::types::U256);
}
