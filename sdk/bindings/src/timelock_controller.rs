pub use timelock_controller::*;
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
pub mod timelock_controller {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("minDelay"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("proposers"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address[]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("executors"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address[]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("admin"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CANCELLER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("CANCELLER_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("EXECUTOR_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("EXECUTOR_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("PROPOSER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PROPOSER_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("TIMELOCK_ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TIMELOCK_ADMIN_ROLE",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("cancel"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("cancel"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("execute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("execute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payload"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("predecessor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeBatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeBatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payloads"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("predecessor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMinDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMinDelay"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("duration"),
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
                    ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("getTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTimestamp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
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
                    ::std::borrow::ToOwned::to_owned("grantRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("grantRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("hasRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("hashOperation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hashOperation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("predecessor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("hash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hashOperationBatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hashOperationBatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payloads"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("predecessor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("hash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isOperation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isOperation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("registered"),
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
                    ::std::borrow::ToOwned::to_owned("isOperationDone"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isOperationDone"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("done"),
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
                    ::std::borrow::ToOwned::to_owned("isOperationPending"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isOperationPending"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pending"),
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
                    ::std::borrow::ToOwned::to_owned("isOperationReady"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isOperationReady"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ready"),
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
                    ::std::borrow::ToOwned::to_owned("onERC1155BatchReceived"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "onERC1155BatchReceived",
                            ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onERC1155Received"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("onERC1155Received"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onERC721Received"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("onERC721Received"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("revokeRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("schedule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("schedule"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("predecessor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delay"),
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
                    ::std::borrow::ToOwned::to_owned("scheduleBatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("scheduleBatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payloads"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("predecessor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delay"),
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
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
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
                    ::std::borrow::ToOwned::to_owned("updateDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateDelay"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newDelay"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CallExecuted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("CallExecuted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CallScheduled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("CallScheduled"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("predecessor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("delay"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Cancelled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Cancelled"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MinDelayChange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MinDelayChange"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldDuration"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newDuration"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleGranted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleGranted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static TIMELOCKCONTROLLER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0##8\x03\x80b\0##\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x04\x08V[b\0\0O`\0\x80Q` b\0\"\xA3\x839\x81Q\x91R\x80b\0\x02-V[b\0\0y`\0\x80Q` b\0\"\xC3\x839\x81Q\x91R`\0\x80Q` b\0\"\xA3\x839\x81Q\x91Rb\0\x02-V[b\0\0\xA3`\0\x80Q` b\0\"\xE3\x839\x81Q\x91R`\0\x80Q` b\0\"\xA3\x839\x81Q\x91Rb\0\x02-V[b\0\0\xCD`\0\x80Q` b\0#\x03\x839\x81Q\x91R`\0\x80Q` b\0\"\xA3\x839\x81Q\x91Rb\0\x02-V[b\0\0\xE8`\0\x80Q` b\0\"\xA3\x839\x81Q\x91R0b\0\x02xV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15b\0\x01\x13Wb\0\x01\x13`\0\x80Q` b\0\"\xA3\x839\x81Q\x91R\x82b\0\x02xV[`\0[\x83Q\x81\x10\x15b\0\x01\x99Wb\0\x01]`\0\x80Q` b\0\"\xC3\x839\x81Q\x91R\x85\x83\x81Q\x81\x10b\0\x01IWb\0\x01Ib\0\x04\x8FV[` \x02` \x01\x01Qb\0\x02x` \x1B` \x1CV[b\0\x01\x86`\0\x80Q` b\0#\x03\x839\x81Q\x91R\x85\x83\x81Q\x81\x10b\0\x01IWb\0\x01Ib\0\x04\x8FV[b\0\x01\x91\x81b\0\x04\xA5V[\x90Pb\0\x01\x16V[P`\0[\x82Q\x81\x10\x15b\0\x01\xE3Wb\0\x01\xD0`\0\x80Q` b\0\"\xE3\x839\x81Q\x91R\x84\x83\x81Q\x81\x10b\0\x01IWb\0\x01Ib\0\x04\x8FV[b\0\x01\xDB\x81b\0\x04\xA5V[\x90Pb\0\x01\x9DV[P`\x02\x84\x90U`@\x80Q`\0\x81R` \x81\x01\x86\x90R\x7F\x11\xC2ON\xAD\x16P|i\xACF\x7F\xBD^N\xED_\xB5\xC6\x99bm,\xC6\xD6d!\xDF%8\x86\xD5\x91\x01`@Q\x80\x91\x03\x90\xA1PPPPb\0\x04\xCDV[`\0\x82\x81R` \x81\x90R`@\x80\x82 `\x01\x01\x80T\x90\x84\x90U\x90Q\x90\x91\x83\x91\x83\x91\x86\x91\x7F\xBDy\xB8o\xFE\n\xB8\xE8waQQB\x17\xCD|\xAC\xD5,\x90\x9FfG\\:\xF4N\x12\x9F\x0B\0\xFF\x91\x90\xA4PPPV[b\0\x02\x84\x82\x82b\0\x02\x88V[PPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16b\0\x02\x84W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\x02\xE43\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x03VW`\0\x80\xFD[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12b\0\x03mW`\0\x80\xFD[\x81Q` `\x01`\x01`@\x1B\x03\x80\x83\x11\x15b\0\x03\x8CWb\0\x03\x8Cb\0\x03(V[\x82`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x84\x82\x11\x17\x15b\0\x03\xB4Wb\0\x03\xB4b\0\x03(V[`@R\x93\x84R\x85\x81\x01\x83\x01\x93\x83\x81\x01\x92P\x87\x85\x11\x15b\0\x03\xD3W`\0\x80\xFD[\x83\x87\x01\x91P[\x84\x82\x10\x15b\0\x03\xFDWb\0\x03\xED\x82b\0\x03>V[\x83R\x91\x83\x01\x91\x90\x83\x01\x90b\0\x03\xD9V[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x04\x1FW`\0\x80\xFD[\x84Q` \x86\x01Q\x90\x94P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x04?W`\0\x80\xFD[b\0\x04M\x88\x83\x89\x01b\0\x03[V[\x94P`@\x87\x01Q\x91P\x80\x82\x11\x15b\0\x04dW`\0\x80\xFD[Pb\0\x04s\x87\x82\x88\x01b\0\x03[V[\x92PPb\0\x04\x84``\x86\x01b\0\x03>V[\x90P\x92\x95\x91\x94P\x92PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01b\0\x04\xC6WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[a\x1D\xC6\x80b\0\x04\xDD`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\xBBW`\x005`\xE0\x1C\x80c\x80ee\x7F\x11a\0\xECW\x80c\xBC\x19|\x81\x11a\0\x8AW\x80c\xD5Gt\x1F\x11a\0dW\x80c\xD5Gt\x1F\x14a\x05\x82W\x80c\xE3\x835\xE5\x14a\x05\xA2W\x80c\xF2:na\x14a\x05\xB5W\x80c\xF2z\x0C\x92\x14a\x05\xE1W`\0\x80\xFD[\x80c\xBC\x19|\x81\x14a\x05\tW\x80c\xC4\xD2R\xF5\x14a\x055W\x80c\xD4\\D5\x14a\x05UW`\0\x80\xFD[\x80c\x91\xD1HT\x11a\0\xC6W\x80c\x91\xD1HT\x14a\x04\x80W\x80c\xA2\x17\xFD\xDF\x14a\x04\xA0W\x80c\xB0\x8EQ\xC0\x14a\x04\xB5W\x80c\xB1\xC5\xF4'\x14a\x04\xE9W`\0\x80\xFD[\x80c\x80ee\x7F\x14a\x04\x0CW\x80c\x8F*\x0B\xB0\x14a\x04,W\x80c\x8Fa\xF4\xF5\x14a\x04LW`\0\x80\xFD[\x80c$\x8A\x9C\xA3\x11a\x01YW\x80c1\xD5\x07P\x11a\x013W\x80c1\xD5\x07P\x14a\x03\x8CW\x80c6V\x8A\xBE\x14a\x03\xACW\x80cXK\x15>\x14a\x03\xCCW\x80cd\xD6#S\x14a\x03\xECW`\0\x80\xFD[\x80c$\x8A\x9C\xA3\x14a\x03\x0BW\x80c*\xB0\xF5)\x14a\x03;W\x80c//\xF1]\x14a\x03lW`\0\x80\xFD[\x80c\r<\xF6\xFC\x11a\x01\x95W\x80c\r<\xF6\xFC\x14a\x02`W\x80c\x13@\x08\xD3\x14a\x02\x94W\x80c\x13\xBC\x9F \x14a\x02\xA7W\x80c\x15\x0Bz\x02\x14a\x02\xC7W`\0\x80\xFD[\x80c\x01\xD5\x06*\x14a\x01\xC7W\x80c\x01\xFF\xC9\xA7\x14a\x01\xE9W\x80c\x07\xBD\x02e\x14a\x02\x1EW`\0\x80\xFD[6a\x01\xC2W\0[`\0\x80\xFD[4\x80\x15a\x01\xD3W`\0\x80\xFD[Pa\x01\xE7a\x01\xE26`\x04a\x13\xC0V[a\x05\xF6V[\0[4\x80\x15a\x01\xF5W`\0\x80\xFD[Pa\x02\ta\x02\x046`\x04a\x144V[a\x06\x8BV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02*W`\0\x80\xFD[Pa\x02R\x7F\xD8\xAA\x0F1\x94\x97\x1A*\x11fy\xF7\xC2\t\x0Fi9\xC8\xD4\xE0\x1A*\x8D~A\xD5^SQF\x9Ec\x81V[`@Q\x90\x81R` \x01a\x02\x15V[4\x80\x15a\x02lW`\0\x80\xFD[Pa\x02R\x7F_X\xE3\xA21cI\x92<\xE3x\x0F\x8DX}\xB2\xD7#x\xAE\xD6j\x82a\xC9\x16TO\xA6\x84l\xA5\x81V[a\x01\xE7a\x02\xA26`\x04a\x14^V[a\x06\xB6V[4\x80\x15a\x02\xB3W`\0\x80\xFD[Pa\x02\ta\x02\xC26`\x04a\x14\xC9V[a\x07kV[4\x80\x15a\x02\xD3W`\0\x80\xFD[Pa\x02\xF2a\x02\xE26`\x04a\x15\x97V[c\n\x85\xBD\x01`\xE1\x1B\x94\x93PPPPV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x02\x15V[4\x80\x15a\x03\x17W`\0\x80\xFD[Pa\x02Ra\x03&6`\x04a\x14\xC9V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[4\x80\x15a\x03GW`\0\x80\xFD[Pa\x02\ta\x03V6`\x04a\x14\xC9V[`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 T\x14\x90V[4\x80\x15a\x03xW`\0\x80\xFD[Pa\x01\xE7a\x03\x876`\x04a\x15\xFEV[a\x07\x91V[4\x80\x15a\x03\x98W`\0\x80\xFD[Pa\x02\ta\x03\xA76`\x04a\x14\xC9V[a\x07\xBBV[4\x80\x15a\x03\xB8W`\0\x80\xFD[Pa\x01\xE7a\x03\xC76`\x04a\x15\xFEV[a\x07\xD4V[4\x80\x15a\x03\xD8W`\0\x80\xFD[Pa\x02\ta\x03\xE76`\x04a\x14\xC9V[a\x08WV[4\x80\x15a\x03\xF8W`\0\x80\xFD[Pa\x01\xE7a\x04\x076`\x04a\x14\xC9V[a\x08mV[4\x80\x15a\x04\x18W`\0\x80\xFD[Pa\x02Ra\x04'6`\x04a\x14^V[a\t\x11V[4\x80\x15a\x048W`\0\x80\xFD[Pa\x01\xE7a\x04G6`\x04a\x16nV[a\tPV[4\x80\x15a\x04XW`\0\x80\xFD[Pa\x02R\x7F\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1\x81V[4\x80\x15a\x04\x8CW`\0\x80\xFD[Pa\x02\ta\x04\x9B6`\x04a\x15\xFEV[a\n\xA2V[4\x80\x15a\x04\xACW`\0\x80\xFD[Pa\x02R`\0\x81V[4\x80\x15a\x04\xC1W`\0\x80\xFD[Pa\x02R\x7F\xFDd<rq\x0Cc\xC0\x18\x02Y\xAB\xA6\xB2\xD0TQ\xE3Y\x1A$\xE5\x8Bb#\x93x\x08W&\xF7\x83\x81V[4\x80\x15a\x04\xF5W`\0\x80\xFD[Pa\x02Ra\x05\x046`\x04a\x17\x1FV[a\n\xCBV[4\x80\x15a\x05\x15W`\0\x80\xFD[Pa\x02\xF2a\x05$6`\x04a\x18FV[c\xBC\x19|\x81`\xE0\x1B\x95\x94PPPPPV[4\x80\x15a\x05AW`\0\x80\xFD[Pa\x01\xE7a\x05P6`\x04a\x14\xC9V[a\x0B\x10V[4\x80\x15a\x05aW`\0\x80\xFD[Pa\x02Ra\x05p6`\x04a\x14\xC9V[`\0\x90\x81R`\x01` R`@\x90 T\x90V[4\x80\x15a\x05\x8EW`\0\x80\xFD[Pa\x01\xE7a\x05\x9D6`\x04a\x15\xFEV[a\x0B\xE5V[a\x01\xE7a\x05\xB06`\x04a\x17\x1FV[a\x0C\nV[4\x80\x15a\x05\xC1W`\0\x80\xFD[Pa\x02\xF2a\x05\xD06`\x04a\x18\xEFV[c\xF2:na`\xE0\x1B\x95\x94PPPPPV[4\x80\x15a\x05\xEDW`\0\x80\xFD[P`\x02Ta\x02RV[\x7F\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1a\x06 \x81a\r\x94V[`\0a\x060\x89\x89\x89\x89\x89\x89a\t\x11V[\x90Pa\x06<\x81\x84a\r\xA1V[`\0\x81\x7FL\xF4A\x0C\xC5p@\xE4Hb\xEF\x0FE\xF3\xDDZ^\x02\xDB\x8E\xB8\xAD\xD6H\xD4\xB0\xE26\xF1\xD0}\xCA\x8B\x8B\x8B\x8B\x8B\x8A`@Qa\x06x\x96\x95\x94\x93\x92\x91\x90a\x19|V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x02q\x18\x97`\xE5\x1B\x14\x80a\x06\xB0WPa\x06\xB0\x82a\x0E\x90V[\x92\x91PPV[\x7F\xD8\xAA\x0F1\x94\x97\x1A*\x11fy\xF7\xC2\t\x0Fi9\xC8\xD4\xE0\x1A*\x8D~A\xD5^SQF\x9Eca\x06\xE2\x81`\0a\n\xA2V[a\x06\xF0Wa\x06\xF0\x813a\x0E\xC5V[`\0a\x07\0\x88\x88\x88\x88\x88\x88a\t\x11V[\x90Pa\x07\x0C\x81\x85a\x0F\x1EV[a\x07\x18\x88\x88\x88\x88a\x0F\xBAV[`\0\x81\x7F\xC2a~\xFAi\xBA\xB6g\x82\xFA!\x95CqC8H\x9CN\x9E\x17\x82qV\n\x91\xB8,?a+X\x8A\x8A\x8A\x8A`@Qa\x07P\x94\x93\x92\x91\x90a\x19\xB9V[`@Q\x80\x91\x03\x90\xA3a\x07a\x81a\x10\x8DV[PPPPPPPPV[`\0\x81\x81R`\x01` R`@\x81 T`\x01\x81\x11\x80\x15a\x07\x8AWPB\x81\x11\x15[\x93\x92PPPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x07\xAC\x81a\r\x94V[a\x07\xB6\x83\x83a\x10\xC6V[PPPV[`\0\x81\x81R`\x01` R`@\x81 T\x81\x90[\x11\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x08IW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x08S\x82\x82a\x11JV[PPV[`\0\x81\x81R`\x01` \x81\x90R`@\x82 Ta\x07\xCDV[30\x14a\x08\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FTimelockController: caller must `D\x82\x01Rjbe timelock`\xA8\x1B`d\x82\x01R`\x84\x01a\x08@V[`\x02T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\x11\xC2ON\xAD\x16P|i\xACF\x7F\xBD^N\xED_\xB5\xC6\x99bm,\xC6\xD6d!\xDF%8\x86\xD5\x91\x01`@Q\x80\x91\x03\x90\xA1`\x02UV[`\0\x86\x86\x86\x86\x86\x86`@Q` \x01a\t.\x96\x95\x94\x93\x92\x91\x90a\x19|V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x96\x95PPPPPPV[\x7F\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1a\tz\x81a\r\x94V[\x88\x87\x14a\t\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08@\x90a\x19\xEBV[\x88\x85\x14a\t\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08@\x90a\x19\xEBV[`\0a\t\xCA\x8B\x8B\x8B\x8B\x8B\x8B\x8B\x8Ba\n\xCBV[\x90Pa\t\xD6\x81\x84a\r\xA1V[`\0[\x8A\x81\x10\x15a\n\x94W\x80\x82\x7FL\xF4A\x0C\xC5p@\xE4Hb\xEF\x0FE\xF3\xDDZ^\x02\xDB\x8E\xB8\xAD\xD6H\xD4\xB0\xE26\xF1\xD0}\xCA\x8E\x8E\x85\x81\x81\x10a\n\x16Wa\n\x16a\x1A.V[\x90P` \x02\x01` \x81\x01\x90a\n+\x91\x90a\x1ADV[\x8D\x8D\x86\x81\x81\x10a\n=Wa\n=a\x1A.V[\x90P` \x02\x015\x8C\x8C\x87\x81\x81\x10a\nVWa\nVa\x1A.V[\x90P` \x02\x81\x01\x90a\nh\x91\x90a\x1A_V[\x8C\x8B`@Qa\n|\x96\x95\x94\x93\x92\x91\x90a\x19|V[`@Q\x80\x91\x03\x90\xA3a\n\x8D\x81a\x1A\xBBV[\x90Pa\t\xD9V[PPPPPPPPPPPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0\x88\x88\x88\x88\x88\x88\x88\x88`@Q` \x01a\n\xEC\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x1BeV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x98\x97PPPPPPPPV[\x7F\xFDd<rq\x0Cc\xC0\x18\x02Y\xAB\xA6\xB2\xD0TQ\xE3Y\x1A$\xE5\x8Bb#\x93x\x08W&\xF7\x83a\x0B:\x81a\r\x94V[a\x0BC\x82a\x08WV[a\x0B\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FTimelockController: operation ca`D\x82\x01Rp\x1B\x9B\x9B\xDD\x08\x18\x99H\x18\xD8[\x98\xD9[\x1B\x19Y`z\x1B`d\x82\x01R`\x84\x01a\x08@V[`\0\x82\x81R`\x01` R`@\x80\x82 \x82\x90UQ\x83\x91\x7F\xBA\xA1\xEB\"\xF2\xA4\x92\xBA\x1A_\xEAa\xB8\xDFM'\xC6\xC8\xB5\xF3\x97\x1Ec\xBBX\xFA\x14\xFFr\xEE\xDBp\x91\xA2PPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x0C\0\x81a\r\x94V[a\x07\xB6\x83\x83a\x11JV[\x7F\xD8\xAA\x0F1\x94\x97\x1A*\x11fy\xF7\xC2\t\x0Fi9\xC8\xD4\xE0\x1A*\x8D~A\xD5^SQF\x9Eca\x0C6\x81`\0a\n\xA2V[a\x0CDWa\x0CD\x813a\x0E\xC5V[\x87\x86\x14a\x0CcW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08@\x90a\x19\xEBV[\x87\x84\x14a\x0C\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08@\x90a\x19\xEBV[`\0a\x0C\x94\x8A\x8A\x8A\x8A\x8A\x8A\x8A\x8Aa\n\xCBV[\x90Pa\x0C\xA0\x81\x85a\x0F\x1EV[`\0[\x89\x81\x10\x15a\r~W`\0\x8B\x8B\x83\x81\x81\x10a\x0C\xBFWa\x0C\xBFa\x1A.V[\x90P` \x02\x01` \x81\x01\x90a\x0C\xD4\x91\x90a\x1ADV[\x90P`\0\x8A\x8A\x84\x81\x81\x10a\x0C\xEAWa\x0C\xEAa\x1A.V[\x90P` \x02\x015\x90P6`\0\x8A\x8A\x86\x81\x81\x10a\r\x08Wa\r\x08a\x1A.V[\x90P` \x02\x81\x01\x90a\r\x1A\x91\x90a\x1A_V[\x91P\x91Pa\r*\x84\x84\x84\x84a\x0F\xBAV[\x84\x86\x7F\xC2a~\xFAi\xBA\xB6g\x82\xFA!\x95CqC8H\x9CN\x9E\x17\x82qV\n\x91\xB8,?a+X\x86\x86\x86\x86`@Qa\ra\x94\x93\x92\x91\x90a\x19\xB9V[`@Q\x80\x91\x03\x90\xA3PPPP\x80a\rw\x90a\x1A\xBBV[\x90Pa\x0C\xA3V[Pa\r\x88\x81a\x10\x8DV[PPPPPPPPPPV[a\r\x9E\x813a\x0E\xC5V[PV[a\r\xAA\x82a\x07\xBBV[\x15a\x0E\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FTimelockController: operation al`D\x82\x01Rn\x1C\x99XY\x1EH\x1C\xD8\xDA\x19Y\x1D[\x19Y`\x8A\x1B`d\x82\x01R`\x84\x01a\x08@V[`\x02T\x81\x10\x15a\x0EpW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FTimelockController: insufficient`D\x82\x01Re delay`\xD0\x1B`d\x82\x01R`\x84\x01a\x08@V[a\x0Ez\x81Ba\x1C\x10V[`\0\x92\x83R`\x01` R`@\x90\x92 \x91\x90\x91UPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x06\xB0WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x06\xB0V[a\x0E\xCF\x82\x82a\n\xA2V[a\x08SWa\x0E\xDC\x81a\x11\xAFV[a\x0E\xE7\x83` a\x11\xC1V[`@Q` \x01a\x0E\xF8\x92\x91\x90a\x1CXV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x08@\x91`\x04\x01a\x1C\xCDV[a\x0F'\x82a\x07kV[a\x0FCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08@\x90a\x1D\0V[\x80\x15\x80a\x0F_WP`\0\x81\x81R`\x01` \x81\x90R`@\x90\x91 T\x14[a\x08SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FTimelockController: missing depe`D\x82\x01Rendency`\xD0\x1B`d\x82\x01R`\x84\x01a\x08@V[`\0\x84`\x01`\x01`\xA0\x1B\x03\x16\x84\x84\x84`@Qa\x0F\xD7\x92\x91\x90a\x1DJV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x10\x14W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x10\x19V[``\x91P[PP\x90P\x80a\x10\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FTimelockController: underlying t`D\x82\x01Rr\x1C\x98[\x9C\xD8X\xDD\x1A[\xDB\x88\x1C\x99]\x99\\\x9D\x19Y`j\x1B`d\x82\x01R`\x84\x01a\x08@V[PPPPPV[a\x10\x96\x81a\x07kV[a\x10\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08@\x90a\x1D\0V[`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 UV[a\x10\xD0\x82\x82a\n\xA2V[a\x08SW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x11\x063\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\x11T\x82\x82a\n\xA2V[\x15a\x08SW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[``a\x06\xB0`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x11\xD0\x83`\x02a\x1DZV[a\x11\xDB\x90`\x02a\x1C\x10V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\xF2Wa\x11\xF2a\x14\xE2V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x12\x1CW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x127Wa\x127a\x1A.V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x12fWa\x12fa\x1A.V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x12\x8A\x84`\x02a\x1DZV[a\x12\x95\x90`\x01a\x1C\x10V[\x90P[`\x01\x81\x11\x15a\x13\rWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x12\xC9Wa\x12\xC9a\x1A.V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x12\xDFWa\x12\xDFa\x1A.V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x13\x06\x81a\x1DyV[\x90Pa\x12\x98V[P\x83\x15a\x07\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x08@V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13sW`\0\x80\xFD[\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x13\x8AW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\xA1W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x13\xB9W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a\x13\xDBW`\0\x80\xFD[a\x13\xE4\x88a\x13\\V[\x96P` \x88\x015\x95P`@\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\x06W`\0\x80\xFD[a\x14\x12\x8A\x82\x8B\x01a\x13xV[\x98\x9B\x97\x9AP\x98``\x81\x015\x97`\x80\x82\x015\x97P`\xA0\x90\x91\x015\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x14FW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x07\x8AW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x14wW`\0\x80\xFD[a\x14\x80\x87a\x13\\V[\x95P` \x87\x015\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xA2W`\0\x80\xFD[a\x14\xAE\x89\x82\x8A\x01a\x13xV[\x97\x9A\x96\x99P\x97``\x81\x015\x96`\x80\x90\x91\x015\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x14\xDBW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x15 Wa\x15 a\x14\xE2V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x159W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15RWa\x15Ra\x14\xE2V[a\x15e`\x1F\x82\x01`\x1F\x19\x16` \x01a\x14\xF8V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x15zW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x15\xADW`\0\x80\xFD[a\x15\xB6\x85a\x13\\V[\x93Pa\x15\xC4` \x86\x01a\x13\\V[\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\xE6W`\0\x80\xFD[a\x15\xF2\x87\x82\x88\x01a\x15(V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x16\x11W`\0\x80\xFD[\x825\x91Pa\x16!` \x84\x01a\x13\\V[\x90P\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x16<W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16SW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x13\xB9W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\0`\xC0\x8A\x8C\x03\x12\x15a\x16\x8CW`\0\x80\xFD[\x895`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x16\xA3W`\0\x80\xFD[a\x16\xAF\x8D\x83\x8E\x01a\x16*V[\x90\x9BP\x99P` \x8C\x015\x91P\x80\x82\x11\x15a\x16\xC8W`\0\x80\xFD[a\x16\xD4\x8D\x83\x8E\x01a\x16*V[\x90\x99P\x97P`@\x8C\x015\x91P\x80\x82\x11\x15a\x16\xEDW`\0\x80\xFD[Pa\x16\xFA\x8C\x82\x8D\x01a\x16*V[\x9A\x9D\x99\x9CP\x97\x9A\x96\x99\x97\x98``\x88\x015\x97`\x80\x81\x015\x97P`\xA0\x015\x95P\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\xA0\x89\x8B\x03\x12\x15a\x17;W`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x17RW`\0\x80\xFD[a\x17^\x8C\x83\x8D\x01a\x16*V[\x90\x9AP\x98P` \x8B\x015\x91P\x80\x82\x11\x15a\x17wW`\0\x80\xFD[a\x17\x83\x8C\x83\x8D\x01a\x16*V[\x90\x98P\x96P`@\x8B\x015\x91P\x80\x82\x11\x15a\x17\x9CW`\0\x80\xFD[Pa\x17\xA9\x8B\x82\x8C\x01a\x16*V[\x99\x9C\x98\x9BP\x96\x99\x95\x98\x96\x97``\x87\x015\x96`\x80\x015\x95P\x93PPPPV[`\0\x82`\x1F\x83\x01\x12a\x17\xD8W`\0\x80\xFD[\x815` `\x01`\x01`@\x1B\x03\x82\x11\x15a\x17\xF3Wa\x17\xF3a\x14\xE2V[\x81`\x05\x1Ba\x18\x02\x82\x82\x01a\x14\xF8V[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15a\x18\x1CW`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15a\x18;W\x825\x82R\x91\x83\x01\x91\x90\x83\x01\x90a\x18\"V[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x18^W`\0\x80\xFD[a\x18g\x86a\x13\\V[\x94Pa\x18u` \x87\x01a\x13\\V[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x18\x91W`\0\x80\xFD[a\x18\x9D\x89\x83\x8A\x01a\x17\xC7V[\x94P``\x88\x015\x91P\x80\x82\x11\x15a\x18\xB3W`\0\x80\xFD[a\x18\xBF\x89\x83\x8A\x01a\x17\xC7V[\x93P`\x80\x88\x015\x91P\x80\x82\x11\x15a\x18\xD5W`\0\x80\xFD[Pa\x18\xE2\x88\x82\x89\x01a\x15(V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x19\x07W`\0\x80\xFD[a\x19\x10\x86a\x13\\V[\x94Pa\x19\x1E` \x87\x01a\x13\\V[\x93P`@\x86\x015\x92P``\x86\x015\x91P`\x80\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19GW`\0\x80\xFD[a\x18\xE2\x88\x82\x89\x01a\x15(V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01\x80`\xA0\x1B\x03\x87\x16\x81R\x85` \x82\x01R`\xA0`@\x82\x01R`\0a\x19\xA4`\xA0\x83\x01\x86\x88a\x19SV[``\x83\x01\x94\x90\x94RP`\x80\x01R\x94\x93PPPPV[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R```@\x82\x01R`\0a\x19\xE1``\x83\x01\x84\x86a\x19SV[\x96\x95PPPPPPV[` \x80\x82R`#\x90\x82\x01R\x7FTimelockController: length misma`@\x82\x01Rb\x0E\x8Cm`\xEB\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x1AVW`\0\x80\xFD[a\x07\x8A\x82a\x13\\V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1AvW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1A\x90W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x13\xB9W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x1A\xCDWa\x1A\xCDa\x1A\xA5V[P`\x01\x01\x90V[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0[\x87\x81\x10\x15a\x1BXW\x82\x84\x03\x89R\x815`\x1E\x19\x886\x03\x01\x81\x12a\x1B\x0FW`\0\x80\xFD[\x87\x01\x85\x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B*W`\0\x80\xFD[\x806\x03\x82\x13\x15a\x1B9W`\0\x80\xFD[a\x1BD\x86\x82\x84a\x19SV[\x9A\x87\x01\x9A\x95PPP\x90\x84\x01\x90`\x01\x01a\x1A\xEEV[P\x91\x97\x96PPPPPPPV[`\xA0\x80\x82R\x81\x01\x88\x90R`\0\x89`\xC0\x83\x01\x82[\x8B\x81\x10\x15a\x1B\xA6W`\x01`\x01`\xA0\x1B\x03a\x1B\x91\x84a\x13\\V[\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x1BxV[P\x83\x81\x03` \x85\x01R\x88\x81R`\x01`\x01`\xFB\x1B\x03\x89\x11\x15a\x1B\xC6W`\0\x80\xFD[\x88`\x05\x1B\x91P\x81\x8A` \x83\x017\x81\x81\x01\x91PP` \x81\x01`\0\x81R` \x84\x83\x03\x01`@\x85\x01Ra\x1B\xF7\x81\x88\x8Aa\x1A\xD4V[``\x85\x01\x96\x90\x96RPPP`\x80\x01R\x96\x95PPPPPPV[`\0\x82\x19\x82\x11\x15a\x1C#Wa\x1C#a\x1A\xA5V[P\x01\x90V[`\0[\x83\x81\x10\x15a\x1CCW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1C+V[\x83\x81\x11\x15a\x1CRW`\0\x84\x84\x01R[PPPPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x1C\x90\x81`\x17\x85\x01` \x88\x01a\x1C(V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x1C\xC1\x81`(\x84\x01` \x88\x01a\x1C(V[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x1C\xEC\x81`@\x85\x01` \x87\x01a\x1C(V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[` \x80\x82R`*\x90\x82\x01R\x7FTimelockController: operation is`@\x82\x01Ri not ready`\xB0\x1B``\x82\x01R`\x80\x01\x90V[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x1DtWa\x1Dta\x1A\xA5V[P\x02\x90V[`\0\x81a\x1D\x88Wa\x1D\x88a\x1A\xA5V[P`\0\x19\x01\x90V\xFE\xA2dipfsX\"\x12 \xF2\xAB5\x18\xAB\0Yc\x0B\xAA\"\x07\xE7\x12yO\xCC\x1E\xEBA\xC5^y\xF1N*\xF1\xC4I,'SdsolcC\0\x08\x0F\x003_X\xE3\xA21cI\x92<\xE3x\x0F\x8DX}\xB2\xD7#x\xAE\xD6j\x82a\xC9\x16TO\xA6\x84l\xA5\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1\xD8\xAA\x0F1\x94\x97\x1A*\x11fy\xF7\xC2\t\x0Fi9\xC8\xD4\xE0\x1A*\x8D~A\xD5^SQF\x9Ec\xFDd<rq\x0Cc\xC0\x18\x02Y\xAB\xA6\xB2\xD0TQ\xE3Y\x1A$\xE5\x8Bb#\x93x\x08W&\xF7\x83";
    /// The bytecode of the contract.
    pub static TIMELOCKCONTROLLER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\xBBW`\x005`\xE0\x1C\x80c\x80ee\x7F\x11a\0\xECW\x80c\xBC\x19|\x81\x11a\0\x8AW\x80c\xD5Gt\x1F\x11a\0dW\x80c\xD5Gt\x1F\x14a\x05\x82W\x80c\xE3\x835\xE5\x14a\x05\xA2W\x80c\xF2:na\x14a\x05\xB5W\x80c\xF2z\x0C\x92\x14a\x05\xE1W`\0\x80\xFD[\x80c\xBC\x19|\x81\x14a\x05\tW\x80c\xC4\xD2R\xF5\x14a\x055W\x80c\xD4\\D5\x14a\x05UW`\0\x80\xFD[\x80c\x91\xD1HT\x11a\0\xC6W\x80c\x91\xD1HT\x14a\x04\x80W\x80c\xA2\x17\xFD\xDF\x14a\x04\xA0W\x80c\xB0\x8EQ\xC0\x14a\x04\xB5W\x80c\xB1\xC5\xF4'\x14a\x04\xE9W`\0\x80\xFD[\x80c\x80ee\x7F\x14a\x04\x0CW\x80c\x8F*\x0B\xB0\x14a\x04,W\x80c\x8Fa\xF4\xF5\x14a\x04LW`\0\x80\xFD[\x80c$\x8A\x9C\xA3\x11a\x01YW\x80c1\xD5\x07P\x11a\x013W\x80c1\xD5\x07P\x14a\x03\x8CW\x80c6V\x8A\xBE\x14a\x03\xACW\x80cXK\x15>\x14a\x03\xCCW\x80cd\xD6#S\x14a\x03\xECW`\0\x80\xFD[\x80c$\x8A\x9C\xA3\x14a\x03\x0BW\x80c*\xB0\xF5)\x14a\x03;W\x80c//\xF1]\x14a\x03lW`\0\x80\xFD[\x80c\r<\xF6\xFC\x11a\x01\x95W\x80c\r<\xF6\xFC\x14a\x02`W\x80c\x13@\x08\xD3\x14a\x02\x94W\x80c\x13\xBC\x9F \x14a\x02\xA7W\x80c\x15\x0Bz\x02\x14a\x02\xC7W`\0\x80\xFD[\x80c\x01\xD5\x06*\x14a\x01\xC7W\x80c\x01\xFF\xC9\xA7\x14a\x01\xE9W\x80c\x07\xBD\x02e\x14a\x02\x1EW`\0\x80\xFD[6a\x01\xC2W\0[`\0\x80\xFD[4\x80\x15a\x01\xD3W`\0\x80\xFD[Pa\x01\xE7a\x01\xE26`\x04a\x13\xC0V[a\x05\xF6V[\0[4\x80\x15a\x01\xF5W`\0\x80\xFD[Pa\x02\ta\x02\x046`\x04a\x144V[a\x06\x8BV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02*W`\0\x80\xFD[Pa\x02R\x7F\xD8\xAA\x0F1\x94\x97\x1A*\x11fy\xF7\xC2\t\x0Fi9\xC8\xD4\xE0\x1A*\x8D~A\xD5^SQF\x9Ec\x81V[`@Q\x90\x81R` \x01a\x02\x15V[4\x80\x15a\x02lW`\0\x80\xFD[Pa\x02R\x7F_X\xE3\xA21cI\x92<\xE3x\x0F\x8DX}\xB2\xD7#x\xAE\xD6j\x82a\xC9\x16TO\xA6\x84l\xA5\x81V[a\x01\xE7a\x02\xA26`\x04a\x14^V[a\x06\xB6V[4\x80\x15a\x02\xB3W`\0\x80\xFD[Pa\x02\ta\x02\xC26`\x04a\x14\xC9V[a\x07kV[4\x80\x15a\x02\xD3W`\0\x80\xFD[Pa\x02\xF2a\x02\xE26`\x04a\x15\x97V[c\n\x85\xBD\x01`\xE1\x1B\x94\x93PPPPV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x02\x15V[4\x80\x15a\x03\x17W`\0\x80\xFD[Pa\x02Ra\x03&6`\x04a\x14\xC9V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[4\x80\x15a\x03GW`\0\x80\xFD[Pa\x02\ta\x03V6`\x04a\x14\xC9V[`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 T\x14\x90V[4\x80\x15a\x03xW`\0\x80\xFD[Pa\x01\xE7a\x03\x876`\x04a\x15\xFEV[a\x07\x91V[4\x80\x15a\x03\x98W`\0\x80\xFD[Pa\x02\ta\x03\xA76`\x04a\x14\xC9V[a\x07\xBBV[4\x80\x15a\x03\xB8W`\0\x80\xFD[Pa\x01\xE7a\x03\xC76`\x04a\x15\xFEV[a\x07\xD4V[4\x80\x15a\x03\xD8W`\0\x80\xFD[Pa\x02\ta\x03\xE76`\x04a\x14\xC9V[a\x08WV[4\x80\x15a\x03\xF8W`\0\x80\xFD[Pa\x01\xE7a\x04\x076`\x04a\x14\xC9V[a\x08mV[4\x80\x15a\x04\x18W`\0\x80\xFD[Pa\x02Ra\x04'6`\x04a\x14^V[a\t\x11V[4\x80\x15a\x048W`\0\x80\xFD[Pa\x01\xE7a\x04G6`\x04a\x16nV[a\tPV[4\x80\x15a\x04XW`\0\x80\xFD[Pa\x02R\x7F\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1\x81V[4\x80\x15a\x04\x8CW`\0\x80\xFD[Pa\x02\ta\x04\x9B6`\x04a\x15\xFEV[a\n\xA2V[4\x80\x15a\x04\xACW`\0\x80\xFD[Pa\x02R`\0\x81V[4\x80\x15a\x04\xC1W`\0\x80\xFD[Pa\x02R\x7F\xFDd<rq\x0Cc\xC0\x18\x02Y\xAB\xA6\xB2\xD0TQ\xE3Y\x1A$\xE5\x8Bb#\x93x\x08W&\xF7\x83\x81V[4\x80\x15a\x04\xF5W`\0\x80\xFD[Pa\x02Ra\x05\x046`\x04a\x17\x1FV[a\n\xCBV[4\x80\x15a\x05\x15W`\0\x80\xFD[Pa\x02\xF2a\x05$6`\x04a\x18FV[c\xBC\x19|\x81`\xE0\x1B\x95\x94PPPPPV[4\x80\x15a\x05AW`\0\x80\xFD[Pa\x01\xE7a\x05P6`\x04a\x14\xC9V[a\x0B\x10V[4\x80\x15a\x05aW`\0\x80\xFD[Pa\x02Ra\x05p6`\x04a\x14\xC9V[`\0\x90\x81R`\x01` R`@\x90 T\x90V[4\x80\x15a\x05\x8EW`\0\x80\xFD[Pa\x01\xE7a\x05\x9D6`\x04a\x15\xFEV[a\x0B\xE5V[a\x01\xE7a\x05\xB06`\x04a\x17\x1FV[a\x0C\nV[4\x80\x15a\x05\xC1W`\0\x80\xFD[Pa\x02\xF2a\x05\xD06`\x04a\x18\xEFV[c\xF2:na`\xE0\x1B\x95\x94PPPPPV[4\x80\x15a\x05\xEDW`\0\x80\xFD[P`\x02Ta\x02RV[\x7F\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1a\x06 \x81a\r\x94V[`\0a\x060\x89\x89\x89\x89\x89\x89a\t\x11V[\x90Pa\x06<\x81\x84a\r\xA1V[`\0\x81\x7FL\xF4A\x0C\xC5p@\xE4Hb\xEF\x0FE\xF3\xDDZ^\x02\xDB\x8E\xB8\xAD\xD6H\xD4\xB0\xE26\xF1\xD0}\xCA\x8B\x8B\x8B\x8B\x8B\x8A`@Qa\x06x\x96\x95\x94\x93\x92\x91\x90a\x19|V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x02q\x18\x97`\xE5\x1B\x14\x80a\x06\xB0WPa\x06\xB0\x82a\x0E\x90V[\x92\x91PPV[\x7F\xD8\xAA\x0F1\x94\x97\x1A*\x11fy\xF7\xC2\t\x0Fi9\xC8\xD4\xE0\x1A*\x8D~A\xD5^SQF\x9Eca\x06\xE2\x81`\0a\n\xA2V[a\x06\xF0Wa\x06\xF0\x813a\x0E\xC5V[`\0a\x07\0\x88\x88\x88\x88\x88\x88a\t\x11V[\x90Pa\x07\x0C\x81\x85a\x0F\x1EV[a\x07\x18\x88\x88\x88\x88a\x0F\xBAV[`\0\x81\x7F\xC2a~\xFAi\xBA\xB6g\x82\xFA!\x95CqC8H\x9CN\x9E\x17\x82qV\n\x91\xB8,?a+X\x8A\x8A\x8A\x8A`@Qa\x07P\x94\x93\x92\x91\x90a\x19\xB9V[`@Q\x80\x91\x03\x90\xA3a\x07a\x81a\x10\x8DV[PPPPPPPPV[`\0\x81\x81R`\x01` R`@\x81 T`\x01\x81\x11\x80\x15a\x07\x8AWPB\x81\x11\x15[\x93\x92PPPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x07\xAC\x81a\r\x94V[a\x07\xB6\x83\x83a\x10\xC6V[PPPV[`\0\x81\x81R`\x01` R`@\x81 T\x81\x90[\x11\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x08IW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x08S\x82\x82a\x11JV[PPV[`\0\x81\x81R`\x01` \x81\x90R`@\x82 Ta\x07\xCDV[30\x14a\x08\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FTimelockController: caller must `D\x82\x01Rjbe timelock`\xA8\x1B`d\x82\x01R`\x84\x01a\x08@V[`\x02T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\x11\xC2ON\xAD\x16P|i\xACF\x7F\xBD^N\xED_\xB5\xC6\x99bm,\xC6\xD6d!\xDF%8\x86\xD5\x91\x01`@Q\x80\x91\x03\x90\xA1`\x02UV[`\0\x86\x86\x86\x86\x86\x86`@Q` \x01a\t.\x96\x95\x94\x93\x92\x91\x90a\x19|V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x96\x95PPPPPPV[\x7F\xB0\x9A\xA5\xAE\xB3p,\xFDP\xB6\xB6+\xC4S&\x04\x93\x8F!$\x8A'\xA1\xD5\xCAs`\x82\xB6\x81\x9C\xC1a\tz\x81a\r\x94V[\x88\x87\x14a\t\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08@\x90a\x19\xEBV[\x88\x85\x14a\t\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08@\x90a\x19\xEBV[`\0a\t\xCA\x8B\x8B\x8B\x8B\x8B\x8B\x8B\x8Ba\n\xCBV[\x90Pa\t\xD6\x81\x84a\r\xA1V[`\0[\x8A\x81\x10\x15a\n\x94W\x80\x82\x7FL\xF4A\x0C\xC5p@\xE4Hb\xEF\x0FE\xF3\xDDZ^\x02\xDB\x8E\xB8\xAD\xD6H\xD4\xB0\xE26\xF1\xD0}\xCA\x8E\x8E\x85\x81\x81\x10a\n\x16Wa\n\x16a\x1A.V[\x90P` \x02\x01` \x81\x01\x90a\n+\x91\x90a\x1ADV[\x8D\x8D\x86\x81\x81\x10a\n=Wa\n=a\x1A.V[\x90P` \x02\x015\x8C\x8C\x87\x81\x81\x10a\nVWa\nVa\x1A.V[\x90P` \x02\x81\x01\x90a\nh\x91\x90a\x1A_V[\x8C\x8B`@Qa\n|\x96\x95\x94\x93\x92\x91\x90a\x19|V[`@Q\x80\x91\x03\x90\xA3a\n\x8D\x81a\x1A\xBBV[\x90Pa\t\xD9V[PPPPPPPPPPPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0\x88\x88\x88\x88\x88\x88\x88\x88`@Q` \x01a\n\xEC\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x1BeV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x98\x97PPPPPPPPV[\x7F\xFDd<rq\x0Cc\xC0\x18\x02Y\xAB\xA6\xB2\xD0TQ\xE3Y\x1A$\xE5\x8Bb#\x93x\x08W&\xF7\x83a\x0B:\x81a\r\x94V[a\x0BC\x82a\x08WV[a\x0B\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FTimelockController: operation ca`D\x82\x01Rp\x1B\x9B\x9B\xDD\x08\x18\x99H\x18\xD8[\x98\xD9[\x1B\x19Y`z\x1B`d\x82\x01R`\x84\x01a\x08@V[`\0\x82\x81R`\x01` R`@\x80\x82 \x82\x90UQ\x83\x91\x7F\xBA\xA1\xEB\"\xF2\xA4\x92\xBA\x1A_\xEAa\xB8\xDFM'\xC6\xC8\xB5\xF3\x97\x1Ec\xBBX\xFA\x14\xFFr\xEE\xDBp\x91\xA2PPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x0C\0\x81a\r\x94V[a\x07\xB6\x83\x83a\x11JV[\x7F\xD8\xAA\x0F1\x94\x97\x1A*\x11fy\xF7\xC2\t\x0Fi9\xC8\xD4\xE0\x1A*\x8D~A\xD5^SQF\x9Eca\x0C6\x81`\0a\n\xA2V[a\x0CDWa\x0CD\x813a\x0E\xC5V[\x87\x86\x14a\x0CcW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08@\x90a\x19\xEBV[\x87\x84\x14a\x0C\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08@\x90a\x19\xEBV[`\0a\x0C\x94\x8A\x8A\x8A\x8A\x8A\x8A\x8A\x8Aa\n\xCBV[\x90Pa\x0C\xA0\x81\x85a\x0F\x1EV[`\0[\x89\x81\x10\x15a\r~W`\0\x8B\x8B\x83\x81\x81\x10a\x0C\xBFWa\x0C\xBFa\x1A.V[\x90P` \x02\x01` \x81\x01\x90a\x0C\xD4\x91\x90a\x1ADV[\x90P`\0\x8A\x8A\x84\x81\x81\x10a\x0C\xEAWa\x0C\xEAa\x1A.V[\x90P` \x02\x015\x90P6`\0\x8A\x8A\x86\x81\x81\x10a\r\x08Wa\r\x08a\x1A.V[\x90P` \x02\x81\x01\x90a\r\x1A\x91\x90a\x1A_V[\x91P\x91Pa\r*\x84\x84\x84\x84a\x0F\xBAV[\x84\x86\x7F\xC2a~\xFAi\xBA\xB6g\x82\xFA!\x95CqC8H\x9CN\x9E\x17\x82qV\n\x91\xB8,?a+X\x86\x86\x86\x86`@Qa\ra\x94\x93\x92\x91\x90a\x19\xB9V[`@Q\x80\x91\x03\x90\xA3PPPP\x80a\rw\x90a\x1A\xBBV[\x90Pa\x0C\xA3V[Pa\r\x88\x81a\x10\x8DV[PPPPPPPPPPV[a\r\x9E\x813a\x0E\xC5V[PV[a\r\xAA\x82a\x07\xBBV[\x15a\x0E\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FTimelockController: operation al`D\x82\x01Rn\x1C\x99XY\x1EH\x1C\xD8\xDA\x19Y\x1D[\x19Y`\x8A\x1B`d\x82\x01R`\x84\x01a\x08@V[`\x02T\x81\x10\x15a\x0EpW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FTimelockController: insufficient`D\x82\x01Re delay`\xD0\x1B`d\x82\x01R`\x84\x01a\x08@V[a\x0Ez\x81Ba\x1C\x10V[`\0\x92\x83R`\x01` R`@\x90\x92 \x91\x90\x91UPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x06\xB0WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x06\xB0V[a\x0E\xCF\x82\x82a\n\xA2V[a\x08SWa\x0E\xDC\x81a\x11\xAFV[a\x0E\xE7\x83` a\x11\xC1V[`@Q` \x01a\x0E\xF8\x92\x91\x90a\x1CXV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x08@\x91`\x04\x01a\x1C\xCDV[a\x0F'\x82a\x07kV[a\x0FCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08@\x90a\x1D\0V[\x80\x15\x80a\x0F_WP`\0\x81\x81R`\x01` \x81\x90R`@\x90\x91 T\x14[a\x08SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FTimelockController: missing depe`D\x82\x01Rendency`\xD0\x1B`d\x82\x01R`\x84\x01a\x08@V[`\0\x84`\x01`\x01`\xA0\x1B\x03\x16\x84\x84\x84`@Qa\x0F\xD7\x92\x91\x90a\x1DJV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x10\x14W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x10\x19V[``\x91P[PP\x90P\x80a\x10\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FTimelockController: underlying t`D\x82\x01Rr\x1C\x98[\x9C\xD8X\xDD\x1A[\xDB\x88\x1C\x99]\x99\\\x9D\x19Y`j\x1B`d\x82\x01R`\x84\x01a\x08@V[PPPPPV[a\x10\x96\x81a\x07kV[a\x10\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08@\x90a\x1D\0V[`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 UV[a\x10\xD0\x82\x82a\n\xA2V[a\x08SW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x11\x063\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\x11T\x82\x82a\n\xA2V[\x15a\x08SW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[``a\x06\xB0`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x11\xD0\x83`\x02a\x1DZV[a\x11\xDB\x90`\x02a\x1C\x10V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\xF2Wa\x11\xF2a\x14\xE2V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x12\x1CW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x127Wa\x127a\x1A.V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x12fWa\x12fa\x1A.V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x12\x8A\x84`\x02a\x1DZV[a\x12\x95\x90`\x01a\x1C\x10V[\x90P[`\x01\x81\x11\x15a\x13\rWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x12\xC9Wa\x12\xC9a\x1A.V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x12\xDFWa\x12\xDFa\x1A.V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x13\x06\x81a\x1DyV[\x90Pa\x12\x98V[P\x83\x15a\x07\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x08@V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13sW`\0\x80\xFD[\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x13\x8AW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\xA1W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x13\xB9W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a\x13\xDBW`\0\x80\xFD[a\x13\xE4\x88a\x13\\V[\x96P` \x88\x015\x95P`@\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\x06W`\0\x80\xFD[a\x14\x12\x8A\x82\x8B\x01a\x13xV[\x98\x9B\x97\x9AP\x98``\x81\x015\x97`\x80\x82\x015\x97P`\xA0\x90\x91\x015\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x14FW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x07\x8AW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x14wW`\0\x80\xFD[a\x14\x80\x87a\x13\\V[\x95P` \x87\x015\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xA2W`\0\x80\xFD[a\x14\xAE\x89\x82\x8A\x01a\x13xV[\x97\x9A\x96\x99P\x97``\x81\x015\x96`\x80\x90\x91\x015\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x14\xDBW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x15 Wa\x15 a\x14\xE2V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x159W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15RWa\x15Ra\x14\xE2V[a\x15e`\x1F\x82\x01`\x1F\x19\x16` \x01a\x14\xF8V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x15zW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x15\xADW`\0\x80\xFD[a\x15\xB6\x85a\x13\\V[\x93Pa\x15\xC4` \x86\x01a\x13\\V[\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\xE6W`\0\x80\xFD[a\x15\xF2\x87\x82\x88\x01a\x15(V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x16\x11W`\0\x80\xFD[\x825\x91Pa\x16!` \x84\x01a\x13\\V[\x90P\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x16<W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16SW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x13\xB9W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\0`\xC0\x8A\x8C\x03\x12\x15a\x16\x8CW`\0\x80\xFD[\x895`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x16\xA3W`\0\x80\xFD[a\x16\xAF\x8D\x83\x8E\x01a\x16*V[\x90\x9BP\x99P` \x8C\x015\x91P\x80\x82\x11\x15a\x16\xC8W`\0\x80\xFD[a\x16\xD4\x8D\x83\x8E\x01a\x16*V[\x90\x99P\x97P`@\x8C\x015\x91P\x80\x82\x11\x15a\x16\xEDW`\0\x80\xFD[Pa\x16\xFA\x8C\x82\x8D\x01a\x16*V[\x9A\x9D\x99\x9CP\x97\x9A\x96\x99\x97\x98``\x88\x015\x97`\x80\x81\x015\x97P`\xA0\x015\x95P\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\xA0\x89\x8B\x03\x12\x15a\x17;W`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x17RW`\0\x80\xFD[a\x17^\x8C\x83\x8D\x01a\x16*V[\x90\x9AP\x98P` \x8B\x015\x91P\x80\x82\x11\x15a\x17wW`\0\x80\xFD[a\x17\x83\x8C\x83\x8D\x01a\x16*V[\x90\x98P\x96P`@\x8B\x015\x91P\x80\x82\x11\x15a\x17\x9CW`\0\x80\xFD[Pa\x17\xA9\x8B\x82\x8C\x01a\x16*V[\x99\x9C\x98\x9BP\x96\x99\x95\x98\x96\x97``\x87\x015\x96`\x80\x015\x95P\x93PPPPV[`\0\x82`\x1F\x83\x01\x12a\x17\xD8W`\0\x80\xFD[\x815` `\x01`\x01`@\x1B\x03\x82\x11\x15a\x17\xF3Wa\x17\xF3a\x14\xE2V[\x81`\x05\x1Ba\x18\x02\x82\x82\x01a\x14\xF8V[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15a\x18\x1CW`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15a\x18;W\x825\x82R\x91\x83\x01\x91\x90\x83\x01\x90a\x18\"V[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x18^W`\0\x80\xFD[a\x18g\x86a\x13\\V[\x94Pa\x18u` \x87\x01a\x13\\V[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x18\x91W`\0\x80\xFD[a\x18\x9D\x89\x83\x8A\x01a\x17\xC7V[\x94P``\x88\x015\x91P\x80\x82\x11\x15a\x18\xB3W`\0\x80\xFD[a\x18\xBF\x89\x83\x8A\x01a\x17\xC7V[\x93P`\x80\x88\x015\x91P\x80\x82\x11\x15a\x18\xD5W`\0\x80\xFD[Pa\x18\xE2\x88\x82\x89\x01a\x15(V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x19\x07W`\0\x80\xFD[a\x19\x10\x86a\x13\\V[\x94Pa\x19\x1E` \x87\x01a\x13\\V[\x93P`@\x86\x015\x92P``\x86\x015\x91P`\x80\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19GW`\0\x80\xFD[a\x18\xE2\x88\x82\x89\x01a\x15(V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01\x80`\xA0\x1B\x03\x87\x16\x81R\x85` \x82\x01R`\xA0`@\x82\x01R`\0a\x19\xA4`\xA0\x83\x01\x86\x88a\x19SV[``\x83\x01\x94\x90\x94RP`\x80\x01R\x94\x93PPPPV[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R```@\x82\x01R`\0a\x19\xE1``\x83\x01\x84\x86a\x19SV[\x96\x95PPPPPPV[` \x80\x82R`#\x90\x82\x01R\x7FTimelockController: length misma`@\x82\x01Rb\x0E\x8Cm`\xEB\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x1AVW`\0\x80\xFD[a\x07\x8A\x82a\x13\\V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1AvW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1A\x90W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x13\xB9W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x1A\xCDWa\x1A\xCDa\x1A\xA5V[P`\x01\x01\x90V[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0[\x87\x81\x10\x15a\x1BXW\x82\x84\x03\x89R\x815`\x1E\x19\x886\x03\x01\x81\x12a\x1B\x0FW`\0\x80\xFD[\x87\x01\x85\x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B*W`\0\x80\xFD[\x806\x03\x82\x13\x15a\x1B9W`\0\x80\xFD[a\x1BD\x86\x82\x84a\x19SV[\x9A\x87\x01\x9A\x95PPP\x90\x84\x01\x90`\x01\x01a\x1A\xEEV[P\x91\x97\x96PPPPPPPV[`\xA0\x80\x82R\x81\x01\x88\x90R`\0\x89`\xC0\x83\x01\x82[\x8B\x81\x10\x15a\x1B\xA6W`\x01`\x01`\xA0\x1B\x03a\x1B\x91\x84a\x13\\V[\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x1BxV[P\x83\x81\x03` \x85\x01R\x88\x81R`\x01`\x01`\xFB\x1B\x03\x89\x11\x15a\x1B\xC6W`\0\x80\xFD[\x88`\x05\x1B\x91P\x81\x8A` \x83\x017\x81\x81\x01\x91PP` \x81\x01`\0\x81R` \x84\x83\x03\x01`@\x85\x01Ra\x1B\xF7\x81\x88\x8Aa\x1A\xD4V[``\x85\x01\x96\x90\x96RPPP`\x80\x01R\x96\x95PPPPPPV[`\0\x82\x19\x82\x11\x15a\x1C#Wa\x1C#a\x1A\xA5V[P\x01\x90V[`\0[\x83\x81\x10\x15a\x1CCW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1C+V[\x83\x81\x11\x15a\x1CRW`\0\x84\x84\x01R[PPPPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x1C\x90\x81`\x17\x85\x01` \x88\x01a\x1C(V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x1C\xC1\x81`(\x84\x01` \x88\x01a\x1C(V[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x1C\xEC\x81`@\x85\x01` \x87\x01a\x1C(V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[` \x80\x82R`*\x90\x82\x01R\x7FTimelockController: operation is`@\x82\x01Ri not ready`\xB0\x1B``\x82\x01R`\x80\x01\x90V[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x1DtWa\x1Dta\x1A\xA5V[P\x02\x90V[`\0\x81a\x1D\x88Wa\x1D\x88a\x1A\xA5V[P`\0\x19\x01\x90V\xFE\xA2dipfsX\"\x12 \xF2\xAB5\x18\xAB\0Yc\x0B\xAA\"\x07\xE7\x12yO\xCC\x1E\xEBA\xC5^y\xF1N*\xF1\xC4I,'SdsolcC\0\x08\x0F\x003";
    /// The deployed bytecode of the contract.
    pub static TIMELOCKCONTROLLER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct TimelockController<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TimelockController<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TimelockController<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TimelockController<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TimelockController<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(TimelockController))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TimelockController<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    TIMELOCKCONTROLLER_ABI.clone(),
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
                TIMELOCKCONTROLLER_ABI.clone(),
                TIMELOCKCONTROLLER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `CANCELLER_ROLE` (0xb08e51c0) function
        pub fn canceller_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([176, 142, 81, 192], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `EXECUTOR_ROLE` (0x07bd0265) function
        pub fn executor_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([7, 189, 2, 101], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PROPOSER_ROLE` (0x8f61f4f5) function
        pub fn proposer_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([143, 97, 244, 245], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `TIMELOCK_ADMIN_ROLE` (0x0d3cf6fc) function
        pub fn timelock_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([13, 60, 246, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cancel` (0xc4d252f5) function
        pub fn cancel(
            &self,
            id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 210, 82, 245], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execute` (0x134008d3) function
        pub fn execute(
            &self,
            target: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            payload: ::ethers::core::types::Bytes,
            predecessor: [u8; 32],
            salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [19, 64, 8, 211],
                    (target, value, payload, predecessor, salt),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeBatch` (0xe38335e5) function
        pub fn execute_batch(
            &self,
            targets: ::std::vec::Vec<::ethers::core::types::Address>,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
            payloads: ::std::vec::Vec<::ethers::core::types::Bytes>,
            predecessor: [u8; 32],
            salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [227, 131, 53, 229],
                    (targets, values, payloads, predecessor, salt),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMinDelay` (0xf27a0c92) function
        pub fn get_min_delay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([242, 122, 12, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTimestamp` (0xd45c4435) function
        pub fn get_timestamp(
            &self,
            id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([212, 92, 68, 53], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `grantRole` (0x2f2ff15d) function
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasRole` (0x91d14854) function
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hashOperation` (0x8065657f) function
        pub fn hash_operation(
            &self,
            target: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            predecessor: [u8; 32],
            salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [128, 101, 101, 127],
                    (target, value, data, predecessor, salt),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hashOperationBatch` (0xb1c5f427) function
        pub fn hash_operation_batch(
            &self,
            targets: ::std::vec::Vec<::ethers::core::types::Address>,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
            payloads: ::std::vec::Vec<::ethers::core::types::Bytes>,
            predecessor: [u8; 32],
            salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [177, 197, 244, 39],
                    (targets, values, payloads, predecessor, salt),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isOperation` (0x31d50750) function
        pub fn is_operation(
            &self,
            id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([49, 213, 7, 80], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isOperationDone` (0x2ab0f529) function
        pub fn is_operation_done(
            &self,
            id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([42, 176, 245, 41], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isOperationPending` (0x584b153e) function
        pub fn is_operation_pending(
            &self,
            id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([88, 75, 21, 62], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isOperationReady` (0x13bc9f20) function
        pub fn is_operation_ready(
            &self,
            id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([19, 188, 159, 32], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC1155BatchReceived` (0xbc197c81) function
        pub fn on_erc1155_batch_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::std::vec::Vec<::ethers::core::types::U256>,
            p3: ::std::vec::Vec<::ethers::core::types::U256>,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([188, 25, 124, 129], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC1155Received` (0xf23a6e61) function
        pub fn on_erc1155_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::U256,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([242, 58, 110, 97], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC721Received` (0x150b7a02) function
        pub fn on_erc721_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([21, 11, 122, 2], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceRole` (0x36568abe) function
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRole` (0xd547741f) function
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `schedule` (0x01d5062a) function
        pub fn schedule(
            &self,
            target: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            predecessor: [u8; 32],
            salt: [u8; 32],
            delay: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [1, 213, 6, 42],
                    (target, value, data, predecessor, salt, delay),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `scheduleBatch` (0x8f2a0bb0) function
        pub fn schedule_batch(
            &self,
            targets: ::std::vec::Vec<::ethers::core::types::Address>,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
            payloads: ::std::vec::Vec<::ethers::core::types::Bytes>,
            predecessor: [u8; 32],
            salt: [u8; 32],
            delay: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [143, 42, 11, 176],
                    (targets, values, payloads, predecessor, salt, delay),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateDelay` (0x64d62353) function
        pub fn update_delay(
            &self,
            new_delay: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 214, 35, 83], new_delay)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `CallExecuted` event
        pub fn call_executed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CallExecutedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `CallScheduled` event
        pub fn call_scheduled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CallScheduledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Cancelled` event
        pub fn cancelled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CancelledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MinDelayChange` event
        pub fn min_delay_change_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MinDelayChangeFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleAdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleGrantedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleRevokedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TimelockControllerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for TimelockController<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
        name = "CallExecuted",
        abi = "CallExecuted(bytes32,uint256,address,uint256,bytes)"
    )]
    pub struct CallExecutedFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub index: ::ethers::core::types::U256,
        pub target: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
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
        name = "CallScheduled",
        abi = "CallScheduled(bytes32,uint256,address,uint256,bytes,bytes32,uint256)"
    )]
    pub struct CallScheduledFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub index: ::ethers::core::types::U256,
        pub target: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub predecessor: [u8; 32],
        pub delay: ::ethers::core::types::U256,
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
    #[ethevent(name = "Cancelled", abi = "Cancelled(bytes32)")]
    pub struct CancelledFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
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
    #[ethevent(name = "MinDelayChange", abi = "MinDelayChange(uint256,uint256)")]
    pub struct MinDelayChangeFilter {
        pub old_duration: ::ethers::core::types::U256,
        pub new_duration: ::ethers::core::types::U256,
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
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
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
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TimelockControllerEvents {
        CallExecutedFilter(CallExecutedFilter),
        CallScheduledFilter(CallScheduledFilter),
        CancelledFilter(CancelledFilter),
        MinDelayChangeFilter(MinDelayChangeFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ::ethers::contract::EthLogDecode for TimelockControllerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = CallExecutedFilter::decode_log(log) {
                return Ok(TimelockControllerEvents::CallExecutedFilter(decoded));
            }
            if let Ok(decoded) = CallScheduledFilter::decode_log(log) {
                return Ok(TimelockControllerEvents::CallScheduledFilter(decoded));
            }
            if let Ok(decoded) = CancelledFilter::decode_log(log) {
                return Ok(TimelockControllerEvents::CancelledFilter(decoded));
            }
            if let Ok(decoded) = MinDelayChangeFilter::decode_log(log) {
                return Ok(TimelockControllerEvents::MinDelayChangeFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(TimelockControllerEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(TimelockControllerEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(TimelockControllerEvents::RoleRevokedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for TimelockControllerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CallExecutedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CallScheduledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CancelledFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinDelayChangeFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CallExecutedFilter> for TimelockControllerEvents {
        fn from(value: CallExecutedFilter) -> Self {
            Self::CallExecutedFilter(value)
        }
    }
    impl ::core::convert::From<CallScheduledFilter> for TimelockControllerEvents {
        fn from(value: CallScheduledFilter) -> Self {
            Self::CallScheduledFilter(value)
        }
    }
    impl ::core::convert::From<CancelledFilter> for TimelockControllerEvents {
        fn from(value: CancelledFilter) -> Self {
            Self::CancelledFilter(value)
        }
    }
    impl ::core::convert::From<MinDelayChangeFilter> for TimelockControllerEvents {
        fn from(value: MinDelayChangeFilter) -> Self {
            Self::MinDelayChangeFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for TimelockControllerEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for TimelockControllerEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for TimelockControllerEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    ///Container type for all input parameters for the `CANCELLER_ROLE` function with signature `CANCELLER_ROLE()` and selector `0xb08e51c0`
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
    #[ethcall(name = "CANCELLER_ROLE", abi = "CANCELLER_ROLE()")]
    pub struct CancellerRoleCall;
    ///Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    ///Container type for all input parameters for the `EXECUTOR_ROLE` function with signature `EXECUTOR_ROLE()` and selector `0x07bd0265`
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
    #[ethcall(name = "EXECUTOR_ROLE", abi = "EXECUTOR_ROLE()")]
    pub struct ExecutorRoleCall;
    ///Container type for all input parameters for the `PROPOSER_ROLE` function with signature `PROPOSER_ROLE()` and selector `0x8f61f4f5`
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
    #[ethcall(name = "PROPOSER_ROLE", abi = "PROPOSER_ROLE()")]
    pub struct ProposerRoleCall;
    ///Container type for all input parameters for the `TIMELOCK_ADMIN_ROLE` function with signature `TIMELOCK_ADMIN_ROLE()` and selector `0x0d3cf6fc`
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
    #[ethcall(name = "TIMELOCK_ADMIN_ROLE", abi = "TIMELOCK_ADMIN_ROLE()")]
    pub struct TimelockAdminRoleCall;
    ///Container type for all input parameters for the `cancel` function with signature `cancel(bytes32)` and selector `0xc4d252f5`
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
    #[ethcall(name = "cancel", abi = "cancel(bytes32)")]
    pub struct CancelCall {
        pub id: [u8; 32],
    }
    ///Container type for all input parameters for the `execute` function with signature `execute(address,uint256,bytes,bytes32,bytes32)` and selector `0x134008d3`
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
    #[ethcall(name = "execute", abi = "execute(address,uint256,bytes,bytes32,bytes32)")]
    pub struct ExecuteCall {
        pub target: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub payload: ::ethers::core::types::Bytes,
        pub predecessor: [u8; 32],
        pub salt: [u8; 32],
    }
    ///Container type for all input parameters for the `executeBatch` function with signature `executeBatch(address[],uint256[],bytes[],bytes32,bytes32)` and selector `0xe38335e5`
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
    #[ethcall(
        name = "executeBatch",
        abi = "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)"
    )]
    pub struct ExecuteBatchCall {
        pub targets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
        pub payloads: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub predecessor: [u8; 32],
        pub salt: [u8; 32],
    }
    ///Container type for all input parameters for the `getMinDelay` function with signature `getMinDelay()` and selector `0xf27a0c92`
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
    #[ethcall(name = "getMinDelay", abi = "getMinDelay()")]
    pub struct GetMinDelayCall;
    ///Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `getTimestamp` function with signature `getTimestamp(bytes32)` and selector `0xd45c4435`
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
    #[ethcall(name = "getTimestamp", abi = "getTimestamp(bytes32)")]
    pub struct GetTimestampCall {
        pub id: [u8; 32],
    }
    ///Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`
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
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hashOperation` function with signature `hashOperation(address,uint256,bytes,bytes32,bytes32)` and selector `0x8065657f`
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
    #[ethcall(
        name = "hashOperation",
        abi = "hashOperation(address,uint256,bytes,bytes32,bytes32)"
    )]
    pub struct HashOperationCall {
        pub target: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub predecessor: [u8; 32],
        pub salt: [u8; 32],
    }
    ///Container type for all input parameters for the `hashOperationBatch` function with signature `hashOperationBatch(address[],uint256[],bytes[],bytes32,bytes32)` and selector `0xb1c5f427`
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
    #[ethcall(
        name = "hashOperationBatch",
        abi = "hashOperationBatch(address[],uint256[],bytes[],bytes32,bytes32)"
    )]
    pub struct HashOperationBatchCall {
        pub targets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
        pub payloads: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub predecessor: [u8; 32],
        pub salt: [u8; 32],
    }
    ///Container type for all input parameters for the `isOperation` function with signature `isOperation(bytes32)` and selector `0x31d50750`
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
    #[ethcall(name = "isOperation", abi = "isOperation(bytes32)")]
    pub struct IsOperationCall {
        pub id: [u8; 32],
    }
    ///Container type for all input parameters for the `isOperationDone` function with signature `isOperationDone(bytes32)` and selector `0x2ab0f529`
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
    #[ethcall(name = "isOperationDone", abi = "isOperationDone(bytes32)")]
    pub struct IsOperationDoneCall {
        pub id: [u8; 32],
    }
    ///Container type for all input parameters for the `isOperationPending` function with signature `isOperationPending(bytes32)` and selector `0x584b153e`
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
    #[ethcall(name = "isOperationPending", abi = "isOperationPending(bytes32)")]
    pub struct IsOperationPendingCall {
        pub id: [u8; 32],
    }
    ///Container type for all input parameters for the `isOperationReady` function with signature `isOperationReady(bytes32)` and selector `0x13bc9f20`
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
    #[ethcall(name = "isOperationReady", abi = "isOperationReady(bytes32)")]
    pub struct IsOperationReadyCall {
        pub id: [u8; 32],
    }
    ///Container type for all input parameters for the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `0xbc197c81`
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
    #[ethcall(
        name = "onERC1155BatchReceived",
        abi = "onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)"
    )]
    pub struct OnERC1155BatchReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `0xf23a6e61`
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
    #[ethcall(
        name = "onERC1155Received",
        abi = "onERC1155Received(address,address,uint256,uint256,bytes)"
    )]
    pub struct OnERC1155ReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
    #[ethcall(
        name = "onERC721Received",
        abi = "onERC721Received(address,address,uint256,bytes)"
    )]
    pub struct OnERC721ReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`
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
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`
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
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `schedule` function with signature `schedule(address,uint256,bytes,bytes32,bytes32,uint256)` and selector `0x01d5062a`
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
    #[ethcall(
        name = "schedule",
        abi = "schedule(address,uint256,bytes,bytes32,bytes32,uint256)"
    )]
    pub struct ScheduleCall {
        pub target: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub predecessor: [u8; 32],
        pub salt: [u8; 32],
        pub delay: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `scheduleBatch` function with signature `scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)` and selector `0x8f2a0bb0`
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
    #[ethcall(
        name = "scheduleBatch",
        abi = "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)"
    )]
    pub struct ScheduleBatchCall {
        pub targets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
        pub payloads: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub predecessor: [u8; 32],
        pub salt: [u8; 32],
        pub delay: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `updateDelay` function with signature `updateDelay(uint256)` and selector `0x64d62353`
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
    #[ethcall(name = "updateDelay", abi = "updateDelay(uint256)")]
    pub struct UpdateDelayCall {
        pub new_delay: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TimelockControllerCalls {
        CancellerRole(CancellerRoleCall),
        DefaultAdminRole(DefaultAdminRoleCall),
        ExecutorRole(ExecutorRoleCall),
        ProposerRole(ProposerRoleCall),
        TimelockAdminRole(TimelockAdminRoleCall),
        Cancel(CancelCall),
        Execute(ExecuteCall),
        ExecuteBatch(ExecuteBatchCall),
        GetMinDelay(GetMinDelayCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetTimestamp(GetTimestampCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        HashOperation(HashOperationCall),
        HashOperationBatch(HashOperationBatchCall),
        IsOperation(IsOperationCall),
        IsOperationDone(IsOperationDoneCall),
        IsOperationPending(IsOperationPendingCall),
        IsOperationReady(IsOperationReadyCall),
        OnERC1155BatchReceived(OnERC1155BatchReceivedCall),
        OnERC1155Received(OnERC1155ReceivedCall),
        OnERC721Received(OnERC721ReceivedCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        Schedule(ScheduleCall),
        ScheduleBatch(ScheduleBatchCall),
        SupportsInterface(SupportsInterfaceCall),
        UpdateDelay(UpdateDelayCall),
    }
    impl ::ethers::core::abi::AbiDecode for TimelockControllerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CancellerRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CancellerRole(decoded));
            }
            if let Ok(decoded) = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <ExecutorRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecutorRole(decoded));
            }
            if let Ok(decoded) = <ProposerRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProposerRole(decoded));
            }
            if let Ok(decoded) = <TimelockAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TimelockAdminRole(decoded));
            }
            if let Ok(decoded) = <CancelCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cancel(decoded));
            }
            if let Ok(decoded) = <ExecuteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded) = <ExecuteBatchCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteBatch(decoded));
            }
            if let Ok(decoded) = <GetMinDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMinDelay(decoded));
            }
            if let Ok(decoded) = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) = <GetTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTimestamp(decoded));
            }
            if let Ok(decoded) = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded) = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded) = <HashOperationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HashOperation(decoded));
            }
            if let Ok(decoded) = <HashOperationBatchCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HashOperationBatch(decoded));
            }
            if let Ok(decoded) = <IsOperationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsOperation(decoded));
            }
            if let Ok(decoded) = <IsOperationDoneCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsOperationDone(decoded));
            }
            if let Ok(decoded) = <IsOperationPendingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsOperationPending(decoded));
            }
            if let Ok(decoded) = <IsOperationReadyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsOperationReady(decoded));
            }
            if let Ok(decoded) = <OnERC1155BatchReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnERC1155BatchReceived(decoded));
            }
            if let Ok(decoded) = <OnERC1155ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnERC1155Received(decoded));
            }
            if let Ok(decoded) = <OnERC721ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnERC721Received(decoded));
            }
            if let Ok(decoded) = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded) = <ScheduleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Schedule(decoded));
            }
            if let Ok(decoded) = <ScheduleBatchCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ScheduleBatch(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <UpdateDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateDelay(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TimelockControllerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CancellerRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecutorRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposerRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TimelockAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cancel(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteBatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMinDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HashOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HashOperationBatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsOperationDone(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsOperationPending(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsOperationReady(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC1155BatchReceived(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC1155Received(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC721Received(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Schedule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ScheduleBatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for TimelockControllerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CancellerRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecutorRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposerRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::TimelockAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Cancel(element) => ::core::fmt::Display::fmt(element, f),
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteBatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashOperation(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashOperationBatch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsOperation(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsOperationDone(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsOperationPending(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsOperationReady(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC1155BatchReceived(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnERC1155Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC721Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Schedule(element) => ::core::fmt::Display::fmt(element, f),
                Self::ScheduleBatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateDelay(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CancellerRoleCall> for TimelockControllerCalls {
        fn from(value: CancellerRoleCall) -> Self {
            Self::CancellerRole(value)
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for TimelockControllerCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<ExecutorRoleCall> for TimelockControllerCalls {
        fn from(value: ExecutorRoleCall) -> Self {
            Self::ExecutorRole(value)
        }
    }
    impl ::core::convert::From<ProposerRoleCall> for TimelockControllerCalls {
        fn from(value: ProposerRoleCall) -> Self {
            Self::ProposerRole(value)
        }
    }
    impl ::core::convert::From<TimelockAdminRoleCall> for TimelockControllerCalls {
        fn from(value: TimelockAdminRoleCall) -> Self {
            Self::TimelockAdminRole(value)
        }
    }
    impl ::core::convert::From<CancelCall> for TimelockControllerCalls {
        fn from(value: CancelCall) -> Self {
            Self::Cancel(value)
        }
    }
    impl ::core::convert::From<ExecuteCall> for TimelockControllerCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<ExecuteBatchCall> for TimelockControllerCalls {
        fn from(value: ExecuteBatchCall) -> Self {
            Self::ExecuteBatch(value)
        }
    }
    impl ::core::convert::From<GetMinDelayCall> for TimelockControllerCalls {
        fn from(value: GetMinDelayCall) -> Self {
            Self::GetMinDelay(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for TimelockControllerCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GetTimestampCall> for TimelockControllerCalls {
        fn from(value: GetTimestampCall) -> Self {
            Self::GetTimestamp(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for TimelockControllerCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for TimelockControllerCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<HashOperationCall> for TimelockControllerCalls {
        fn from(value: HashOperationCall) -> Self {
            Self::HashOperation(value)
        }
    }
    impl ::core::convert::From<HashOperationBatchCall> for TimelockControllerCalls {
        fn from(value: HashOperationBatchCall) -> Self {
            Self::HashOperationBatch(value)
        }
    }
    impl ::core::convert::From<IsOperationCall> for TimelockControllerCalls {
        fn from(value: IsOperationCall) -> Self {
            Self::IsOperation(value)
        }
    }
    impl ::core::convert::From<IsOperationDoneCall> for TimelockControllerCalls {
        fn from(value: IsOperationDoneCall) -> Self {
            Self::IsOperationDone(value)
        }
    }
    impl ::core::convert::From<IsOperationPendingCall> for TimelockControllerCalls {
        fn from(value: IsOperationPendingCall) -> Self {
            Self::IsOperationPending(value)
        }
    }
    impl ::core::convert::From<IsOperationReadyCall> for TimelockControllerCalls {
        fn from(value: IsOperationReadyCall) -> Self {
            Self::IsOperationReady(value)
        }
    }
    impl ::core::convert::From<OnERC1155BatchReceivedCall> for TimelockControllerCalls {
        fn from(value: OnERC1155BatchReceivedCall) -> Self {
            Self::OnERC1155BatchReceived(value)
        }
    }
    impl ::core::convert::From<OnERC1155ReceivedCall> for TimelockControllerCalls {
        fn from(value: OnERC1155ReceivedCall) -> Self {
            Self::OnERC1155Received(value)
        }
    }
    impl ::core::convert::From<OnERC721ReceivedCall> for TimelockControllerCalls {
        fn from(value: OnERC721ReceivedCall) -> Self {
            Self::OnERC721Received(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for TimelockControllerCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for TimelockControllerCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<ScheduleCall> for TimelockControllerCalls {
        fn from(value: ScheduleCall) -> Self {
            Self::Schedule(value)
        }
    }
    impl ::core::convert::From<ScheduleBatchCall> for TimelockControllerCalls {
        fn from(value: ScheduleBatchCall) -> Self {
            Self::ScheduleBatch(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for TimelockControllerCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<UpdateDelayCall> for TimelockControllerCalls {
        fn from(value: UpdateDelayCall) -> Self {
            Self::UpdateDelay(value)
        }
    }
    ///Container type for all return fields from the `CANCELLER_ROLE` function with signature `CANCELLER_ROLE()` and selector `0xb08e51c0`
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
    pub struct CancellerRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `EXECUTOR_ROLE` function with signature `EXECUTOR_ROLE()` and selector `0x07bd0265`
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
    pub struct ExecutorRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PROPOSER_ROLE` function with signature `PROPOSER_ROLE()` and selector `0x8f61f4f5`
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
    pub struct ProposerRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `TIMELOCK_ADMIN_ROLE` function with signature `TIMELOCK_ADMIN_ROLE()` and selector `0x0d3cf6fc`
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
    pub struct TimelockAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getMinDelay` function with signature `getMinDelay()` and selector `0xf27a0c92`
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
    pub struct GetMinDelayReturn {
        pub duration: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getTimestamp` function with signature `getTimestamp(bytes32)` and selector `0xd45c4435`
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
    pub struct GetTimestampReturn {
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    pub struct HasRoleReturn(pub bool);
    ///Container type for all return fields from the `hashOperation` function with signature `hashOperation(address,uint256,bytes,bytes32,bytes32)` and selector `0x8065657f`
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
    pub struct HashOperationReturn {
        pub hash: [u8; 32],
    }
    ///Container type for all return fields from the `hashOperationBatch` function with signature `hashOperationBatch(address[],uint256[],bytes[],bytes32,bytes32)` and selector `0xb1c5f427`
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
    pub struct HashOperationBatchReturn {
        pub hash: [u8; 32],
    }
    ///Container type for all return fields from the `isOperation` function with signature `isOperation(bytes32)` and selector `0x31d50750`
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
    pub struct IsOperationReturn {
        pub registered: bool,
    }
    ///Container type for all return fields from the `isOperationDone` function with signature `isOperationDone(bytes32)` and selector `0x2ab0f529`
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
    pub struct IsOperationDoneReturn {
        pub done: bool,
    }
    ///Container type for all return fields from the `isOperationPending` function with signature `isOperationPending(bytes32)` and selector `0x584b153e`
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
    pub struct IsOperationPendingReturn {
        pub pending: bool,
    }
    ///Container type for all return fields from the `isOperationReady` function with signature `isOperationReady(bytes32)` and selector `0x13bc9f20`
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
    pub struct IsOperationReadyReturn {
        pub ready: bool,
    }
    ///Container type for all return fields from the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `0xbc197c81`
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
    pub struct OnERC1155BatchReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `0xf23a6e61`
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
    pub struct OnERC1155ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
    pub struct OnERC721ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
}
