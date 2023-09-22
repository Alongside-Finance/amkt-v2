pub use dealer::*;
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
pub mod dealer {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("IS_TEST"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("IS_TEST"),
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
                    ::std::borrow::ToOwned::to_owned("dealToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dealToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("excludeArtifacts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("excludeArtifacts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "excludedArtifacts_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("excludeContracts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("excludeContracts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "excludedContracts_",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("excludeSenders"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("excludeSenders"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("excludedSenders_"),
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
                    ::std::borrow::ToOwned::to_owned("failed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("failed"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetArtifactSelectors"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "targetArtifactSelectors",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedArtifactSelectors_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct StdInvariant.FuzzSelector[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetArtifacts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetArtifacts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedArtifacts_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetContracts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetContracts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedContracts_",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("targetSelectors"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetSelectors"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedSelectors_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct StdInvariant.FuzzSelector[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetSenders"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetSenders"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetedSenders_"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("log"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_address"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_address"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_array"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_bytes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_bytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_bytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_bytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_int"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_int"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_address"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_address"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_array"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_bytes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_bytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_bytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_bytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_decimal_int"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "log_named_decimal_int",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("decimals"),
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
                    ::std::borrow::ToOwned::to_owned("log_named_decimal_uint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "log_named_decimal_uint",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("decimals"),
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
                    ::std::borrow::ToOwned::to_owned("log_named_int"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_int"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_string"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_string"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_uint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_uint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
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
                    ::std::borrow::ToOwned::to_owned("log_string"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_string"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_uint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_uint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("logs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("logs"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DEALER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\0\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x04\x80T\x90\x91\x16\x82\x17\x90U`\xE0`@RsZ\x98\xFC\xBE\xA5\x16\xCF\x06\x85r\x15w\x9F\xD8\x12\xCA;\xEF\x1B2`\xA0\x90\x81Rs\x82\x0F\xB2SR\xBB\x0C^\x03\xE0z\xFC\x1D\x86%/\xFD/\n\x18`\xC0R`\x80\x90\x81Rb\0\0f\x91`\x1B\x91\x90b\0\0{V[P4\x80\x15b\0\0tW`\0\x80\xFD[Pb\0\x01hV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90`\x02\x02\x81\x01\x92\x82\x15b\0\0\xCAW\x91` \x02\x82\x01[\x82\x81\x11\x15b\0\0\xCAW\x82Qb\0\0\xB9\x90\x83\x90`\x02b\0\0\xDCV[P\x91` \x01\x91\x90`\x02\x01\x90b\0\0\x9FV[Pb\0\0\xD8\x92\x91Pb\0\x015V[P\x90V[\x82`\x02\x81\x01\x92\x82\x15b\0\x01'W\x91` \x02\x82\x01[\x82\x81\x11\x15b\0\x01'W\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90b\0\0\xF0V[Pb\0\0\xD8\x92\x91Pb\0\x01QV[\x80\x82\x11\x15b\0\0\xD8W`\0\x80\x82U`\x01\x82\x01U`\x02\x01b\0\x015V[[\x80\x82\x11\x15b\0\0\xD8W`\0\x81U`\x01\x01b\0\x01RV[a$2\x80b\0\x01x`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xA9W`\x005`\xE0\x1C\x80c\x91j\x17\xC6\x11a\0qW\x80c\x91j\x17\xC6\x14a\x01\x06W\x80c\xB5P\x8A\xA9\x14a\x01\x0EW\x80c\xBAAO\xA6\x14a\x01\x16W\x80c\xE2\x0C\x9Fq\x14a\x01.W\x80c\xEC\x85S\r\x14a\x016W\x80c\xFAv&\xD4\x14a\x01KW`\0\x80\xFD[\x80c\x1E\xD7\x83\x1C\x14a\0\xAEW\x80c>^<#\x14a\0\xCCW\x80c?r\x86\xF4\x14a\0\xD4W\x80cf\xD9\xA9\xA0\x14a\0\xDCW\x80c\x85\"l\x81\x14a\0\xF1W[`\0\x80\xFD[a\0\xB6a\x01XV[`@Qa\0\xC3\x91\x90a\x1EOV[`@Q\x80\x91\x03\x90\xF3[a\0\xB6a\x01\xBAV[a\0\xB6a\x02\x1AV[a\0\xE4a\x02zV[`@Qa\0\xC3\x91\x90a\x1E\x9CV[a\0\xF9a\x03iV[`@Qa\0\xC3\x91\x90a\x1F\x7FV[a\0\xE4a\x049V[a\0\xF9a\x05\x1FV[a\x01\x1Ea\x05\xEFV[`@Q\x90\x15\x15\x81R` \x01a\0\xC3V[a\0\xB6a\x07\x1AV[a\x01Ia\x01D6`\x04a \x10V[a\x07zV[\0[`\0Ta\x01\x1E\x90`\xFF\x16\x81V[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xB0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\x92W[PPPPP\x90P\x90V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xB0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\x92WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xB0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\x92WPPPPP\x90P\x90V[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03`W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x03HW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x03\nW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x02\x9EV[PPPP\x90P\x90V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03`W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x03\xAC\x90a LV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xD8\x90a LV[\x80\x15a\x04%W\x80`\x1F\x10a\x03\xFAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04%V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\x08W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x03\x8DV[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03`W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x05\x07W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x04\xC9W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x04]V[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03`W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x05b\x90a LV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x8E\x90a LV[\x80\x15a\x05\xDBW\x80`\x1F\x10a\x05\xB0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xDBV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xBEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x05CV[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\x06\x0FWP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\x15W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x06\x9D\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a \x86V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x06\xB7\x91a \xB7V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x06\xF4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06\xF9V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x07\x11\x91\x90a \xD3V[\x91PP[\x91\x90PV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xB0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\x92WPPPPP\x90P\x90V[`\0\x80a\x07\x86\x85a\x07\xB3V[\x91P\x91P\x81\x15a\x07\xA0Wa\x07\x9B\x85\x85\x85a\x08jV[a\x07\xACV[a\x07\xAC\x85\x85\x85\x84a\x08|V[PPPPPV[`\0\x80`\0[`\x1BT\x81\x10\x15a\x08]W\x83`\x01`\x01`\xA0\x1B\x03\x16`\x1B\x82\x81T\x81\x10a\x07\xE0Wa\x07\xE0a \xF5V[`\0\x91\x82R` \x82 `\x02\x90\x91\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x03a\x08KW`\0`\x1B\x82\x81T\x81\x10a\x08\x14Wa\x08\x14a \xF5V[\x90`\0R` `\0 \x90`\x02\x02\x01`\x01`\x02\x81\x10a\x084Wa\x084a \xF5V[\x01T\x90\x95`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x94P\x92PPPV[\x80a\x08U\x81a!!V[\x91PPa\x07\xB9V[P`\x01\x93`\0\x93P\x91PPV[a\x08w\x83\x83\x83`\0a\t\\V[PPPV[`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\xD1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xE5W=`\0\x80>=`\0\xFD[PP`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R`$\x82\x01\x86\x90R\x87\x16\x92Pc\xA9\x05\x9C\xBB\x91P`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xAC\x91\x90a \xD3V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`D\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cp\xA0\x821`\xE0\x1B\x17\x90R\x91Q`\0\x92\x87\x16\x91a\t\xB0\x91a \xB7V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\t\xEDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t\xF2V[``\x91P[P\x91PP`\0\x81\x80` \x01\x90Q\x81\x01\x90a\n\x0C\x91\x90a!:V[\x90Pa\n>\x84a\n8\x87a\n2cp\xA0\x821`\xE0\x1Ba\n,`\x05\x8Da\x0BCV[\x90a\x0BkV[\x90a\x0B\x88V[\x90a\x0B\xB0V[\x82\x15a\x0B;W`@\x80Q`\x04\x81R`$\x81\x01\x82R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x18\x16\r\xDD`\xE0\x1B\x17\x90R\x90Q`\0\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x91a\n\x86\x91\x90a \xB7V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\n\xC3W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\xC8V[``\x91P[P\x91PP`\0\x81\x80` \x01\x90Q\x81\x01\x90a\n\xE2\x91\x90a!:V[\x90P\x82\x86\x10\x15a\x0B\x07Wa\n\xF6\x86\x84a!SV[a\x0B\0\x90\x82a!SV[\x90Pa\x0B\x1EV[a\x0B\x11\x83\x87a!SV[a\x0B\x1B\x90\x82a!jV[\x90P[a\x0B8\x81a\n8c\x18\x16\r\xDD`\xE0\x1Ba\n,`\x05\x8Da\x0BCV[PP[PPPPPPV[`\x05\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U`\0\x82[\x93\x92PPPV[`\x03\x82\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16`\xE0\x83\x90\x1C\x17\x90U`\0\x82a\x0BdV[`\x02\x82\x01\x80T`\x01\x81\x01\x82U`\0\x91\x82R` \x82 `\x01`\x01`\xA0\x1B\x03\x84\x16\x91\x01U\x82a\x0BdV[a\x0B\xBA\x82\x82a\x0B\xBEV[PPV[`\x05\x82\x01T`\x03\x83\x01T`\x04\x84\x01T`\x02\x85\x01\x80T`@\x80Q` \x80\x84\x02\x82\x01\x81\x01\x90\x92R\x82\x81R`\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x95`\xE0\x95\x90\x95\x1B\x94`\0\x93\x90\x92\x90\x91\x83\x01\x82\x82\x80\x15a\x0C/W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0C\x1BW[PPPPP\x90P`\0\x83a\x0CB\x83a\x0F\x1EV[`@Q` \x01a\x0CS\x92\x91\x90a \x86V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x01\x8B\x01` \x90\x81R\x83\x82 `\x01`\x01`\xE0\x1B\x03\x19\x8A\x16\x83R\x81R\x92\x81 \x91\x94P\x90\x92\x90\x91a\x0C\xA5\x91\x86\x91\x88\x91\x01a!\x82V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x83R\x90\x82\x01\x92\x90\x92R\x01`\0 T`\xFF\x16a\x0C\xDDWa\x0C\xDB\x87a\x0F\xC5V[P[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R` \x88\x81R`@\x80\x83 `\x01`\x01`\xE0\x1B\x03\x19\x88\x16\x84R\x82R\x80\x83 \x90Q\x90\x91\x83\x91a\r\x1C\x91\x87\x91\x89\x91\x01a!\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 T`\0\x1B\x90P`\0\x80\x87`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\ra\x91\x90a \xB7V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\r\x9CW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\r\xA1V[``\x91P[P\x91Pa\r\xBA\x90P\x81a\r\xB5\x88` a!\xBCV[a\x0F\xD6V[`@Qc\x06g\xF9\xD7`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x90\x92P`\0\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90cf\x7F\x9Dp\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0ED\x91\x90a!:V[\x90P\x80\x82\x14a\x0EnW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0Ee\x90a!\xDBV[`@Q\x80\x91\x03\x90\xFD[`@Qcp\xCA\x10\xBB`\xE0\x1B\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90cp\xCA\x10\xBB\x90a\x0E\xA9\x90\x8B\x90\x87\x90\x8E\x90`\x04\x01a\"vV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xC3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\xD7W=`\0\x80>=`\0\xFD[PPP`\x05\x8B\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UP`\x03\x8A\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x90Ua\x0F\n`\x02\x8B\x01`\0a\x1E\x15V[\x89`\x04\x01`\0\x90UPPPPPPPPPPV[```\0\x82Q` a\x0F0\x91\x90a!\xBCV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FHWa\x0FHa\"\x97V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0FrW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x0F\xBEW`\0\x84\x82\x81Q\x81\x10a\x0F\x95Wa\x0F\x95a \xF5V[` \x02` \x01\x01Q\x90P\x80\x82` \x02` \x01\x84\x01RP\x80\x80a\x0F\xB6\x90a!!V[\x91PPa\x0FxV[P\x92\x91PPV[`\0a\x0F\xD0\x82a\x10SV[\x92\x91PPV[`\0\x80`\0` \x85Q\x11a\x0F\xEBW\x84Qa\x0F\xEEV[` [\x90P`\0[\x81\x81\x10\x15a\x10IWa\x10\x06\x81`\x08a!\xBCV[\x86a\x10\x11\x83\x88a!jV[\x81Q\x81\x10a\x10!Wa\x10!a \xF5V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x1C\x92\x90\x92\x17\x91\x80a\x10A\x81a!!V[\x91PPa\x0F\xF3V[P\x90\x94\x93PPPPV[`\x05\x81\x01T`\x03\x82\x01T`\x04\x83\x01T`\x02\x84\x01\x80T`@\x80Q` \x80\x84\x02\x82\x01\x81\x01\x90\x92R\x82\x81R`\0\x96`\x01`\x01`\xA0\x1B\x03\x16\x95`\xE0\x1B\x94\x93\x87\x93\x91\x92\x90\x91\x90\x83\x01\x82\x82\x80\x15a\x10\xC3W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x10\xAFW[PPP`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x01\x8A\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xE0\x1B\x03\x19\x8A\x16\x84R\x82R\x80\x83 \x90Q\x95\x96P\x94\x91\x93Pa\x11\r\x92P\x85\x91\x87\x91\x01a!\x82V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x83R\x90\x82\x01\x92\x90\x92R\x01`\0 T`\xFF\x16\x15a\x11\xA9W`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R` \x87\x81R`@\x80\x83 `\x01`\x01`\xE0\x1B\x03\x19\x87\x16\x84R\x82R\x80\x83 \x90Q\x90\x92\x91a\x11y\x91\x85\x91\x87\x91\x01a!\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 T\x94PPPPP\x91\x90PV[`\0\x83a\x11\xB5\x83a\x1D\x02V[`@Q` \x01a\x11\xC6\x92\x91\x90a \x86V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80Q` a#\xDD\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16c&l\xF1\t`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12#W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x127W=`\0\x80>=`\0\xFD[PPPP`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x83`@Qa\x12V\x91\x90a \xB7V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x12\x91W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x12\x96V[``\x91P[P\x91Pa\x12\xAF\x90P\x81a\x12\xAA\x87` a!\xBCV[a\x1D\xA2V[`@Qce\xBC\x94\x81`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x16`\x04\x82\x01R\x90\x92P`\0\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90ce\xBC\x94\x81\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x13\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x138\x91\x90\x81\x01\x90a#HV[P\x90P\x80Q`\x01\x03a\x15\xFAW`\0`\0\x80Q` a#\xDD\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cf\x7F\x9Dp\x89\x84`\0\x81Q\x81\x10a\x13zWa\x13za \xF5V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\xB3\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xF4\x91\x90a!:V[\x90P\x80a\x14[W\x7F\x08\x0F\xC4\xA9f \xC4F.p[#\xF3FA?\xE3yk\xB6<o\x8D\x85\x91\xBA\xEC\x0E#\x15w\xA5\x88\x83`\0\x81Q\x81\x10a\x140Wa\x140a \xF5V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84R\x91\x83\x01R\x01`@Q\x80\x91\x03\x90\xA1[\x80\x83\x14a\x14zW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0Ee\x90a!\xDBV[\x7F\x9C\x95U\xB1\xE3\x10.<\xF4\x8FB}y\xCBg\x8F]\x9B\xD1\xED\n\xD5t8\x94a\xE2U\xF9Qp\xED\x88\x88\x87\x89`@Q` \x01a\x14\xB0\x92\x91\x90a!\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`\0\x81Q\x81\x10a\x14\xD9Wa\x14\xD9a \xF5V[` \x02` \x01\x01Q`\0\x1C`@Qa\x14\xF4\x94\x93\x92\x91\x90a#\xACV[`@Q\x80\x91\x03\x90\xA1\x81`\0\x81Q\x81\x10a\x15\x0FWa\x15\x0Fa \xF5V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R\x8C\x83R`@\x80\x82 `\x01`\x01`\xE0\x1B\x03\x19\x8C\x16\x83R\x84R\x80\x82 \x90Q\x92\x93\x90\x92a\x15X\x91\x8A\x91\x8C\x91\x01a!\x82V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x83R\x82\x82\x01\x93\x90\x93R\x90\x82\x01`\0\x90\x81 \x93\x90\x93U`\x01`\x01`\xA0\x1B\x03\x8B\x16\x83R`\x01\x8D\x81\x01\x82R\x82\x84 `\x01`\x01`\xE0\x1B\x03\x19\x8C\x16\x85R\x82R\x82\x84 \x92Q\x90\x93\x91a\x15\xC0\x91\x8A\x91\x8C\x91\x01a!\x82V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x83R\x90\x82\x01\x92\x90\x92R\x01`\0 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UPa\x1B\x8DV[`\x01\x81Q\x11\x15a\x1B\x1DW`\0[\x81Q\x81\x10\x15a\x1B\x17W`\0`\0\x80Q` a#\xDD\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cf\x7F\x9Dp\x8A\x85\x85\x81Q\x81\x10a\x16EWa\x16Ea \xF5V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16~\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xBF\x91\x90a!:V[\x90P\x80a\x17%W\x7F\x08\x0F\xC4\xA9f \xC4F.p[#\xF3FA?\xE3yk\xB6<o\x8D\x85\x91\xBA\xEC\x0E#\x15w\xA5\x89\x84\x84\x81Q\x81\x10a\x16\xFAWa\x16\xFAa \xF5V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84R\x91\x83\x01R\x01`@Q\x80\x91\x03\x90\xA1[`\0\x80Q` a#\xDD\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cp\xCA\x10\xBB\x8A\x85\x85\x81Q\x81\x10a\x17XWa\x17Xa \xF5V[` \x02` \x01\x01Qa\x137`\xF0\x1B`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\x84\x93\x92\x91\x90a\"vV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\x9EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\xB2W=`\0\x80>=`\0\xFD[PPPP`\0``\x8A`\x01`\x01`\xA0\x1B\x03\x16\x87`@Qa\x17\xD2\x91\x90a \xB7V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x18\rW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x18\x12V[``\x91P[P\x90\x92P\x90Pa\x18'\x81a\x12\xAA\x8B` a!\xBCV[\x95P\x81\x80\x15a\x18:WPa\x137`\xF0\x1B\x86\x14[\x15a\x1AuW\x7F\x9C\x95U\xB1\xE3\x10.<\xF4\x8FB}y\xCBg\x8F]\x9B\xD1\xED\n\xD5t8\x94a\xE2U\xF9Qp\xED\x8B\x8B\x8A\x8C`@Q` \x01a\x18u\x92\x91\x90a!\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x88\x88\x81Q\x81\x10a\x18\x9DWa\x18\x9Da \xF5V[` \x02` \x01\x01Q`\0\x1C`@Qa\x18\xB8\x94\x93\x92\x91\x90a#\xACV[`@Q\x80\x91\x03\x90\xA1\x84\x84\x81Q\x81\x10a\x18\xD2Wa\x18\xD2a \xF5V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x8D\x16`\0\x90\x81R\x8F\x83R`@\x80\x82 `\x01`\x01`\xE0\x1B\x03\x19\x8F\x16\x83R\x84R\x80\x82 \x90Q\x92\x93\x90\x92a\x19\x1B\x91\x8D\x91\x8F\x91\x01a!\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x8D`\x01\x01`\0\x8D`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8C`\x01`\x01`\xE0\x1B\x03\x19\x16`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8A\x8C`@Q` \x01a\x19\xA6\x92\x91\x90a!\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\0\x80Q` a#\xDD\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cp\xCA\x10\xBB\x8C\x87\x87\x81Q\x81\x10a\x1A\x14Wa\x1A\x14a \xF5V[` \x02` \x01\x01Q\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A;\x93\x92\x91\x90a\"vV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1AUW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1AiW=`\0\x80>=`\0\xFD[PPPPPPPa\x1B\x17V[`\0\x80Q` a#\xDD\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cp\xCA\x10\xBB\x8C\x87\x87\x81Q\x81\x10a\x1A\xA8Wa\x1A\xA8a \xF5V[` \x02` \x01\x01Q\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\xCF\x93\x92\x91\x90a\"vV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xE9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A\xFDW=`\0\x80>=`\0\xFD[PPPPPPP\x80\x80a\x1B\x0F\x90a!!V[\x91PPa\x16\x07V[Pa\x1B\x8DV[`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FstdStorage find(StdStorage): No `D\x82\x01R\x7Fstorage use detected for target.`d\x82\x01R`\x84\x01a\x0EeV[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x01\x8A\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xE0\x1B\x03\x19\x8A\x16\x84R\x82R\x80\x83 \x90Q\x90\x92\x91a\x1B\xCF\x91\x88\x91\x8A\x91\x01a!\x82V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x83R\x90\x82\x01\x92\x90\x92R\x01`\0 T`\xFF\x16a\x1C\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FstdStorage find(StdStorage): Slo`D\x82\x01Rn:\x149\x94\x9077\xBA\x1037\xBA\xB72\x17`\x89\x1B`d\x82\x01R`\x84\x01a\x0EeV[`\x05\x89\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U`\x03\x89\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x90Ua\x1C\x8B`\x02\x8A\x01`\0a\x1E\x15V[`\0`\x04\x8A\x01\x81\x90U`\x01`\x01`\xA0\x1B\x03\x88\x16\x81R` \x8A\x81R`@\x80\x83 `\x01`\x01`\xE0\x1B\x03\x19\x8A\x16\x84R\x82R\x80\x83 \x90Q\x90\x92\x91a\x1C\xCF\x91\x88\x91\x8A\x91\x01a!\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 T\x97PPPPPPPP\x91\x90PV[```\0\x82Q` a\x1D\x14\x91\x90a!\xBCV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D,Wa\x1D,a\"\x97V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1DVW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x0F\xBEW`\0\x84\x82\x81Q\x81\x10a\x1DyWa\x1Dya \xF5V[` \x02` \x01\x01Q\x90P\x80\x82` \x02` \x01\x84\x01RP\x80\x80a\x1D\x9A\x90a!!V[\x91PPa\x1D\\V[`\0\x80`\0` \x85Q\x11a\x1D\xB7W\x84Qa\x1D\xBAV[` [\x90P`\0[\x81\x81\x10\x15a\x10IWa\x1D\xD2\x81`\x08a!\xBCV[\x86a\x1D\xDD\x83\x88a!jV[\x81Q\x81\x10a\x1D\xEDWa\x1D\xEDa \xF5V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x1C\x92\x90\x92\x17\x91\x80a\x1E\r\x81a!!V[\x91PPa\x1D\xBFV[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90a\x1E3\x91\x90a\x1E6V[PV[[\x80\x82\x11\x15a\x1EKW`\0\x81U`\x01\x01a\x1E7V[P\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x1E\x90W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x1EkV[P\x90\x96\x95PPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a\x1F@W\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a\x1F+W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a\x1F\x01V[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a\x1E\xC4V[P\x91\x99\x98PPPPPPPPPV[`\0[\x83\x81\x10\x15a\x1FjW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1FRV[\x83\x81\x11\x15a\x1FyW`\0\x84\x84\x01R[PPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a\x1F\xECW\x87\x85\x03`?\x19\x01\x84R\x81Q\x80Q\x80\x87Ra\x1F\xCD\x81\x89\x89\x01\x8A\x85\x01a\x1FOV[`\x1F\x01`\x1F\x19\x16\x95\x90\x95\x01\x86\x01\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x1F\xA6V[P\x92\x97\x96PPPPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\x15W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a %W`\0\x80\xFD[a .\x84a\x1F\xF9V[\x92Pa <` \x85\x01a\x1F\xF9V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\x01\x81\x81\x1C\x90\x82\x16\x80a `W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a \x80WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a \xA9\x81`\x04\x85\x01` \x87\x01a\x1FOV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa \xC9\x81\x84` \x87\x01a\x1FOV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a \xE5W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0BdW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a!3Wa!3a!\x0BV[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a!LW`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x82\x10\x15a!eWa!ea!\x0BV[P\x03\x90V[`\0\x82\x19\x82\x11\x15a!}Wa!}a!\x0BV[P\x01\x90V[\x82Q`\0\x90\x82\x90` \x80\x87\x01\x84[\x83\x81\x10\x15a!\xACW\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a!\x90V[PP\x94\x82RP\x90\x92\x01\x93\x92PPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a!\xD6Wa!\xD6a!\x0BV[P\x02\x90V[` \x80\x82R`o\x90\x82\x01R\x7FstdStorage find(StdStorage): Pac`@\x82\x01R\x7Fked slot. This would cause dange``\x82\x01R\x7Frous overwriting and currently i`\x80\x82\x01Rn9\xB7\x13\xBA\x109\xBA\xB887\xB9:2\xB2\x17`\x89\x1B`\xA0\x82\x01R`\xC0\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\"\xBEW`\0\x80\xFD[\x81Q` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x11\x15a\"\xDBWa\"\xDBa\"\x97V[\x82`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x84\x82\x11\x17\x15a#\0Wa#\0a\"\x97V[`@R\x93\x84R\x85\x81\x01\x83\x01\x93\x83\x81\x01\x92P\x87\x85\x11\x15a#\x1EW`\0\x80\xFD[\x83\x87\x01\x91P[\x84\x82\x10\x15a#=W\x81Q\x83R\x91\x83\x01\x91\x90\x83\x01\x90a#$V[\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a#[W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a#sW`\0\x80\xFD[a#\x7F\x86\x83\x87\x01a\"\xADV[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a#\x95W`\0\x80\xFD[Pa#\xA2\x85\x82\x86\x01a\"\xADV[\x91PP\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x84R`\x01`\x01`\xE0\x1B\x03\x19\x92\x90\x92\x16` \x84\x01R`@\x83\x01R``\x82\x01R`\x80\x01\x90V\xFE\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\xA2dipfsX\"\x12 \xFC\xBD!\xFC\x18p\xA4\xE3\xD1\x9F\x8D{\x0Ck\x8C\xDCN\xD4\xBD\x11|hx\x1B\xE2\xA5?\xB1\x1Cs\x9ErdsolcC\0\x08\x0F\x003";
    /// The bytecode of the contract.
    pub static DEALER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xA9W`\x005`\xE0\x1C\x80c\x91j\x17\xC6\x11a\0qW\x80c\x91j\x17\xC6\x14a\x01\x06W\x80c\xB5P\x8A\xA9\x14a\x01\x0EW\x80c\xBAAO\xA6\x14a\x01\x16W\x80c\xE2\x0C\x9Fq\x14a\x01.W\x80c\xEC\x85S\r\x14a\x016W\x80c\xFAv&\xD4\x14a\x01KW`\0\x80\xFD[\x80c\x1E\xD7\x83\x1C\x14a\0\xAEW\x80c>^<#\x14a\0\xCCW\x80c?r\x86\xF4\x14a\0\xD4W\x80cf\xD9\xA9\xA0\x14a\0\xDCW\x80c\x85\"l\x81\x14a\0\xF1W[`\0\x80\xFD[a\0\xB6a\x01XV[`@Qa\0\xC3\x91\x90a\x1EOV[`@Q\x80\x91\x03\x90\xF3[a\0\xB6a\x01\xBAV[a\0\xB6a\x02\x1AV[a\0\xE4a\x02zV[`@Qa\0\xC3\x91\x90a\x1E\x9CV[a\0\xF9a\x03iV[`@Qa\0\xC3\x91\x90a\x1F\x7FV[a\0\xE4a\x049V[a\0\xF9a\x05\x1FV[a\x01\x1Ea\x05\xEFV[`@Q\x90\x15\x15\x81R` \x01a\0\xC3V[a\0\xB6a\x07\x1AV[a\x01Ia\x01D6`\x04a \x10V[a\x07zV[\0[`\0Ta\x01\x1E\x90`\xFF\x16\x81V[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xB0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\x92W[PPPPP\x90P\x90V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xB0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\x92WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xB0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\x92WPPPPP\x90P\x90V[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03`W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x03HW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x03\nW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x02\x9EV[PPPP\x90P\x90V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03`W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x03\xAC\x90a LV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xD8\x90a LV[\x80\x15a\x04%W\x80`\x1F\x10a\x03\xFAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04%V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\x08W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x03\x8DV[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03`W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x05\x07W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x04\xC9W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x04]V[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03`W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x05b\x90a LV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x8E\x90a LV[\x80\x15a\x05\xDBW\x80`\x1F\x10a\x05\xB0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xDBV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xBEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x05CV[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\x06\x0FWP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x07\x15W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x06\x9D\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a \x86V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x06\xB7\x91a \xB7V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x06\xF4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06\xF9V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x07\x11\x91\x90a \xD3V[\x91PP[\x91\x90PV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xB0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\x92WPPPPP\x90P\x90V[`\0\x80a\x07\x86\x85a\x07\xB3V[\x91P\x91P\x81\x15a\x07\xA0Wa\x07\x9B\x85\x85\x85a\x08jV[a\x07\xACV[a\x07\xAC\x85\x85\x85\x84a\x08|V[PPPPPV[`\0\x80`\0[`\x1BT\x81\x10\x15a\x08]W\x83`\x01`\x01`\xA0\x1B\x03\x16`\x1B\x82\x81T\x81\x10a\x07\xE0Wa\x07\xE0a \xF5V[`\0\x91\x82R` \x82 `\x02\x90\x91\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x03a\x08KW`\0`\x1B\x82\x81T\x81\x10a\x08\x14Wa\x08\x14a \xF5V[\x90`\0R` `\0 \x90`\x02\x02\x01`\x01`\x02\x81\x10a\x084Wa\x084a \xF5V[\x01T\x90\x95`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x94P\x92PPPV[\x80a\x08U\x81a!!V[\x91PPa\x07\xB9V[P`\x01\x93`\0\x93P\x91PPV[a\x08w\x83\x83\x83`\0a\t\\V[PPPV[`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\xD1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xE5W=`\0\x80>=`\0\xFD[PP`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R`$\x82\x01\x86\x90R\x87\x16\x92Pc\xA9\x05\x9C\xBB\x91P`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xAC\x91\x90a \xD3V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`D\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cp\xA0\x821`\xE0\x1B\x17\x90R\x91Q`\0\x92\x87\x16\x91a\t\xB0\x91a \xB7V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\t\xEDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t\xF2V[``\x91P[P\x91PP`\0\x81\x80` \x01\x90Q\x81\x01\x90a\n\x0C\x91\x90a!:V[\x90Pa\n>\x84a\n8\x87a\n2cp\xA0\x821`\xE0\x1Ba\n,`\x05\x8Da\x0BCV[\x90a\x0BkV[\x90a\x0B\x88V[\x90a\x0B\xB0V[\x82\x15a\x0B;W`@\x80Q`\x04\x81R`$\x81\x01\x82R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x18\x16\r\xDD`\xE0\x1B\x17\x90R\x90Q`\0\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x91a\n\x86\x91\x90a \xB7V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\n\xC3W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n\xC8V[``\x91P[P\x91PP`\0\x81\x80` \x01\x90Q\x81\x01\x90a\n\xE2\x91\x90a!:V[\x90P\x82\x86\x10\x15a\x0B\x07Wa\n\xF6\x86\x84a!SV[a\x0B\0\x90\x82a!SV[\x90Pa\x0B\x1EV[a\x0B\x11\x83\x87a!SV[a\x0B\x1B\x90\x82a!jV[\x90P[a\x0B8\x81a\n8c\x18\x16\r\xDD`\xE0\x1Ba\n,`\x05\x8Da\x0BCV[PP[PPPPPPV[`\x05\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U`\0\x82[\x93\x92PPPV[`\x03\x82\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16`\xE0\x83\x90\x1C\x17\x90U`\0\x82a\x0BdV[`\x02\x82\x01\x80T`\x01\x81\x01\x82U`\0\x91\x82R` \x82 `\x01`\x01`\xA0\x1B\x03\x84\x16\x91\x01U\x82a\x0BdV[a\x0B\xBA\x82\x82a\x0B\xBEV[PPV[`\x05\x82\x01T`\x03\x83\x01T`\x04\x84\x01T`\x02\x85\x01\x80T`@\x80Q` \x80\x84\x02\x82\x01\x81\x01\x90\x92R\x82\x81R`\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x95`\xE0\x95\x90\x95\x1B\x94`\0\x93\x90\x92\x90\x91\x83\x01\x82\x82\x80\x15a\x0C/W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0C\x1BW[PPPPP\x90P`\0\x83a\x0CB\x83a\x0F\x1EV[`@Q` \x01a\x0CS\x92\x91\x90a \x86V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x01\x8B\x01` \x90\x81R\x83\x82 `\x01`\x01`\xE0\x1B\x03\x19\x8A\x16\x83R\x81R\x92\x81 \x91\x94P\x90\x92\x90\x91a\x0C\xA5\x91\x86\x91\x88\x91\x01a!\x82V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x83R\x90\x82\x01\x92\x90\x92R\x01`\0 T`\xFF\x16a\x0C\xDDWa\x0C\xDB\x87a\x0F\xC5V[P[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R` \x88\x81R`@\x80\x83 `\x01`\x01`\xE0\x1B\x03\x19\x88\x16\x84R\x82R\x80\x83 \x90Q\x90\x91\x83\x91a\r\x1C\x91\x87\x91\x89\x91\x01a!\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 T`\0\x1B\x90P`\0\x80\x87`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\ra\x91\x90a \xB7V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\r\x9CW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\r\xA1V[``\x91P[P\x91Pa\r\xBA\x90P\x81a\r\xB5\x88` a!\xBCV[a\x0F\xD6V[`@Qc\x06g\xF9\xD7`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x90\x92P`\0\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90cf\x7F\x9Dp\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0ED\x91\x90a!:V[\x90P\x80\x82\x14a\x0EnW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0Ee\x90a!\xDBV[`@Q\x80\x91\x03\x90\xFD[`@Qcp\xCA\x10\xBB`\xE0\x1B\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90cp\xCA\x10\xBB\x90a\x0E\xA9\x90\x8B\x90\x87\x90\x8E\x90`\x04\x01a\"vV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xC3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\xD7W=`\0\x80>=`\0\xFD[PPP`\x05\x8B\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UP`\x03\x8A\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x90Ua\x0F\n`\x02\x8B\x01`\0a\x1E\x15V[\x89`\x04\x01`\0\x90UPPPPPPPPPPV[```\0\x82Q` a\x0F0\x91\x90a!\xBCV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FHWa\x0FHa\"\x97V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0FrW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x0F\xBEW`\0\x84\x82\x81Q\x81\x10a\x0F\x95Wa\x0F\x95a \xF5V[` \x02` \x01\x01Q\x90P\x80\x82` \x02` \x01\x84\x01RP\x80\x80a\x0F\xB6\x90a!!V[\x91PPa\x0FxV[P\x92\x91PPV[`\0a\x0F\xD0\x82a\x10SV[\x92\x91PPV[`\0\x80`\0` \x85Q\x11a\x0F\xEBW\x84Qa\x0F\xEEV[` [\x90P`\0[\x81\x81\x10\x15a\x10IWa\x10\x06\x81`\x08a!\xBCV[\x86a\x10\x11\x83\x88a!jV[\x81Q\x81\x10a\x10!Wa\x10!a \xF5V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x1C\x92\x90\x92\x17\x91\x80a\x10A\x81a!!V[\x91PPa\x0F\xF3V[P\x90\x94\x93PPPPV[`\x05\x81\x01T`\x03\x82\x01T`\x04\x83\x01T`\x02\x84\x01\x80T`@\x80Q` \x80\x84\x02\x82\x01\x81\x01\x90\x92R\x82\x81R`\0\x96`\x01`\x01`\xA0\x1B\x03\x16\x95`\xE0\x1B\x94\x93\x87\x93\x91\x92\x90\x91\x90\x83\x01\x82\x82\x80\x15a\x10\xC3W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x10\xAFW[PPP`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x01\x8A\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xE0\x1B\x03\x19\x8A\x16\x84R\x82R\x80\x83 \x90Q\x95\x96P\x94\x91\x93Pa\x11\r\x92P\x85\x91\x87\x91\x01a!\x82V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x83R\x90\x82\x01\x92\x90\x92R\x01`\0 T`\xFF\x16\x15a\x11\xA9W`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R` \x87\x81R`@\x80\x83 `\x01`\x01`\xE0\x1B\x03\x19\x87\x16\x84R\x82R\x80\x83 \x90Q\x90\x92\x91a\x11y\x91\x85\x91\x87\x91\x01a!\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 T\x94PPPPP\x91\x90PV[`\0\x83a\x11\xB5\x83a\x1D\x02V[`@Q` \x01a\x11\xC6\x92\x91\x90a \x86V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80Q` a#\xDD\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16c&l\xF1\t`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12#W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x127W=`\0\x80>=`\0\xFD[PPPP`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x83`@Qa\x12V\x91\x90a \xB7V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x12\x91W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x12\x96V[``\x91P[P\x91Pa\x12\xAF\x90P\x81a\x12\xAA\x87` a!\xBCV[a\x1D\xA2V[`@Qce\xBC\x94\x81`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x16`\x04\x82\x01R\x90\x92P`\0\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90ce\xBC\x94\x81\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x13\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x138\x91\x90\x81\x01\x90a#HV[P\x90P\x80Q`\x01\x03a\x15\xFAW`\0`\0\x80Q` a#\xDD\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cf\x7F\x9Dp\x89\x84`\0\x81Q\x81\x10a\x13zWa\x13za \xF5V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\xB3\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xF4\x91\x90a!:V[\x90P\x80a\x14[W\x7F\x08\x0F\xC4\xA9f \xC4F.p[#\xF3FA?\xE3yk\xB6<o\x8D\x85\x91\xBA\xEC\x0E#\x15w\xA5\x88\x83`\0\x81Q\x81\x10a\x140Wa\x140a \xF5V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84R\x91\x83\x01R\x01`@Q\x80\x91\x03\x90\xA1[\x80\x83\x14a\x14zW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0Ee\x90a!\xDBV[\x7F\x9C\x95U\xB1\xE3\x10.<\xF4\x8FB}y\xCBg\x8F]\x9B\xD1\xED\n\xD5t8\x94a\xE2U\xF9Qp\xED\x88\x88\x87\x89`@Q` \x01a\x14\xB0\x92\x91\x90a!\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`\0\x81Q\x81\x10a\x14\xD9Wa\x14\xD9a \xF5V[` \x02` \x01\x01Q`\0\x1C`@Qa\x14\xF4\x94\x93\x92\x91\x90a#\xACV[`@Q\x80\x91\x03\x90\xA1\x81`\0\x81Q\x81\x10a\x15\x0FWa\x15\x0Fa \xF5V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R\x8C\x83R`@\x80\x82 `\x01`\x01`\xE0\x1B\x03\x19\x8C\x16\x83R\x84R\x80\x82 \x90Q\x92\x93\x90\x92a\x15X\x91\x8A\x91\x8C\x91\x01a!\x82V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x83R\x82\x82\x01\x93\x90\x93R\x90\x82\x01`\0\x90\x81 \x93\x90\x93U`\x01`\x01`\xA0\x1B\x03\x8B\x16\x83R`\x01\x8D\x81\x01\x82R\x82\x84 `\x01`\x01`\xE0\x1B\x03\x19\x8C\x16\x85R\x82R\x82\x84 \x92Q\x90\x93\x91a\x15\xC0\x91\x8A\x91\x8C\x91\x01a!\x82V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x83R\x90\x82\x01\x92\x90\x92R\x01`\0 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UPa\x1B\x8DV[`\x01\x81Q\x11\x15a\x1B\x1DW`\0[\x81Q\x81\x10\x15a\x1B\x17W`\0`\0\x80Q` a#\xDD\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cf\x7F\x9Dp\x8A\x85\x85\x81Q\x81\x10a\x16EWa\x16Ea \xF5V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16~\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xBF\x91\x90a!:V[\x90P\x80a\x17%W\x7F\x08\x0F\xC4\xA9f \xC4F.p[#\xF3FA?\xE3yk\xB6<o\x8D\x85\x91\xBA\xEC\x0E#\x15w\xA5\x89\x84\x84\x81Q\x81\x10a\x16\xFAWa\x16\xFAa \xF5V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84R\x91\x83\x01R\x01`@Q\x80\x91\x03\x90\xA1[`\0\x80Q` a#\xDD\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cp\xCA\x10\xBB\x8A\x85\x85\x81Q\x81\x10a\x17XWa\x17Xa \xF5V[` \x02` \x01\x01Qa\x137`\xF0\x1B`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\x84\x93\x92\x91\x90a\"vV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\x9EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\xB2W=`\0\x80>=`\0\xFD[PPPP`\0``\x8A`\x01`\x01`\xA0\x1B\x03\x16\x87`@Qa\x17\xD2\x91\x90a \xB7V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x18\rW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x18\x12V[``\x91P[P\x90\x92P\x90Pa\x18'\x81a\x12\xAA\x8B` a!\xBCV[\x95P\x81\x80\x15a\x18:WPa\x137`\xF0\x1B\x86\x14[\x15a\x1AuW\x7F\x9C\x95U\xB1\xE3\x10.<\xF4\x8FB}y\xCBg\x8F]\x9B\xD1\xED\n\xD5t8\x94a\xE2U\xF9Qp\xED\x8B\x8B\x8A\x8C`@Q` \x01a\x18u\x92\x91\x90a!\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x88\x88\x81Q\x81\x10a\x18\x9DWa\x18\x9Da \xF5V[` \x02` \x01\x01Q`\0\x1C`@Qa\x18\xB8\x94\x93\x92\x91\x90a#\xACV[`@Q\x80\x91\x03\x90\xA1\x84\x84\x81Q\x81\x10a\x18\xD2Wa\x18\xD2a \xF5V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x8D\x16`\0\x90\x81R\x8F\x83R`@\x80\x82 `\x01`\x01`\xE0\x1B\x03\x19\x8F\x16\x83R\x84R\x80\x82 \x90Q\x92\x93\x90\x92a\x19\x1B\x91\x8D\x91\x8F\x91\x01a!\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x8D`\x01\x01`\0\x8D`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8C`\x01`\x01`\xE0\x1B\x03\x19\x16`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8A\x8C`@Q` \x01a\x19\xA6\x92\x91\x90a!\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\0\x80Q` a#\xDD\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cp\xCA\x10\xBB\x8C\x87\x87\x81Q\x81\x10a\x1A\x14Wa\x1A\x14a \xF5V[` \x02` \x01\x01Q\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A;\x93\x92\x91\x90a\"vV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1AUW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1AiW=`\0\x80>=`\0\xFD[PPPPPPPa\x1B\x17V[`\0\x80Q` a#\xDD\x839\x81Q\x91R`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cp\xCA\x10\xBB\x8C\x87\x87\x81Q\x81\x10a\x1A\xA8Wa\x1A\xA8a \xF5V[` \x02` \x01\x01Q\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\xCF\x93\x92\x91\x90a\"vV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xE9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A\xFDW=`\0\x80>=`\0\xFD[PPPPPPP\x80\x80a\x1B\x0F\x90a!!V[\x91PPa\x16\x07V[Pa\x1B\x8DV[`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FstdStorage find(StdStorage): No `D\x82\x01R\x7Fstorage use detected for target.`d\x82\x01R`\x84\x01a\x0EeV[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x01\x8A\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xE0\x1B\x03\x19\x8A\x16\x84R\x82R\x80\x83 \x90Q\x90\x92\x91a\x1B\xCF\x91\x88\x91\x8A\x91\x01a!\x82V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 \x83R\x90\x82\x01\x92\x90\x92R\x01`\0 T`\xFF\x16a\x1C\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FstdStorage find(StdStorage): Slo`D\x82\x01Rn:\x149\x94\x9077\xBA\x1037\xBA\xB72\x17`\x89\x1B`d\x82\x01R`\x84\x01a\x0EeV[`\x05\x89\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U`\x03\x89\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x90Ua\x1C\x8B`\x02\x8A\x01`\0a\x1E\x15V[`\0`\x04\x8A\x01\x81\x90U`\x01`\x01`\xA0\x1B\x03\x88\x16\x81R` \x8A\x81R`@\x80\x83 `\x01`\x01`\xE0\x1B\x03\x19\x8A\x16\x84R\x82R\x80\x83 \x90Q\x90\x92\x91a\x1C\xCF\x91\x88\x91\x8A\x91\x01a!\x82V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81R` \x01\x90\x81R` \x01`\0 T\x97PPPPPPPP\x91\x90PV[```\0\x82Q` a\x1D\x14\x91\x90a!\xBCV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D,Wa\x1D,a\"\x97V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1DVW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x0F\xBEW`\0\x84\x82\x81Q\x81\x10a\x1DyWa\x1Dya \xF5V[` \x02` \x01\x01Q\x90P\x80\x82` \x02` \x01\x84\x01RP\x80\x80a\x1D\x9A\x90a!!V[\x91PPa\x1D\\V[`\0\x80`\0` \x85Q\x11a\x1D\xB7W\x84Qa\x1D\xBAV[` [\x90P`\0[\x81\x81\x10\x15a\x10IWa\x1D\xD2\x81`\x08a!\xBCV[\x86a\x1D\xDD\x83\x88a!jV[\x81Q\x81\x10a\x1D\xEDWa\x1D\xEDa \xF5V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x1C\x92\x90\x92\x17\x91\x80a\x1E\r\x81a!!V[\x91PPa\x1D\xBFV[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90a\x1E3\x91\x90a\x1E6V[PV[[\x80\x82\x11\x15a\x1EKW`\0\x81U`\x01\x01a\x1E7V[P\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x1E\x90W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x1EkV[P\x90\x96\x95PPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a\x1F@W\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a\x1F+W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a\x1F\x01V[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a\x1E\xC4V[P\x91\x99\x98PPPPPPPPPV[`\0[\x83\x81\x10\x15a\x1FjW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1FRV[\x83\x81\x11\x15a\x1FyW`\0\x84\x84\x01R[PPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a\x1F\xECW\x87\x85\x03`?\x19\x01\x84R\x81Q\x80Q\x80\x87Ra\x1F\xCD\x81\x89\x89\x01\x8A\x85\x01a\x1FOV[`\x1F\x01`\x1F\x19\x16\x95\x90\x95\x01\x86\x01\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x1F\xA6V[P\x92\x97\x96PPPPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\x15W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a %W`\0\x80\xFD[a .\x84a\x1F\xF9V[\x92Pa <` \x85\x01a\x1F\xF9V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\x01\x81\x81\x1C\x90\x82\x16\x80a `W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a \x80WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a \xA9\x81`\x04\x85\x01` \x87\x01a\x1FOV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa \xC9\x81\x84` \x87\x01a\x1FOV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a \xE5W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0BdW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a!3Wa!3a!\x0BV[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a!LW`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x82\x10\x15a!eWa!ea!\x0BV[P\x03\x90V[`\0\x82\x19\x82\x11\x15a!}Wa!}a!\x0BV[P\x01\x90V[\x82Q`\0\x90\x82\x90` \x80\x87\x01\x84[\x83\x81\x10\x15a!\xACW\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a!\x90V[PP\x94\x82RP\x90\x92\x01\x93\x92PPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a!\xD6Wa!\xD6a!\x0BV[P\x02\x90V[` \x80\x82R`o\x90\x82\x01R\x7FstdStorage find(StdStorage): Pac`@\x82\x01R\x7Fked slot. This would cause dange``\x82\x01R\x7Frous overwriting and currently i`\x80\x82\x01Rn9\xB7\x13\xBA\x109\xBA\xB887\xB9:2\xB2\x17`\x89\x1B`\xA0\x82\x01R`\xC0\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\"\xBEW`\0\x80\xFD[\x81Q` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x11\x15a\"\xDBWa\"\xDBa\"\x97V[\x82`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x84\x82\x11\x17\x15a#\0Wa#\0a\"\x97V[`@R\x93\x84R\x85\x81\x01\x83\x01\x93\x83\x81\x01\x92P\x87\x85\x11\x15a#\x1EW`\0\x80\xFD[\x83\x87\x01\x91P[\x84\x82\x10\x15a#=W\x81Q\x83R\x91\x83\x01\x91\x90\x83\x01\x90a#$V[\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a#[W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a#sW`\0\x80\xFD[a#\x7F\x86\x83\x87\x01a\"\xADV[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a#\x95W`\0\x80\xFD[Pa#\xA2\x85\x82\x86\x01a\"\xADV[\x91PP\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x84R`\x01`\x01`\xE0\x1B\x03\x19\x92\x90\x92\x16` \x84\x01R`@\x83\x01R``\x82\x01R`\x80\x01\x90V\xFE\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\xA2dipfsX\"\x12 \xFC\xBD!\xFC\x18p\xA4\xE3\xD1\x9F\x8D{\x0Ck\x8C\xDCN\xD4\xBD\x11|hx\x1B\xE2\xA5?\xB1\x1Cs\x9ErdsolcC\0\x08\x0F\x003";
    /// The deployed bytecode of the contract.
    pub static DEALER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Dealer<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Dealer<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Dealer<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Dealer<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Dealer<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Dealer)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Dealer<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DEALER_ABI.clone(),
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
                DEALER_ABI.clone(),
                DEALER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `IS_TEST` (0xfa7626d4) function
        pub fn is_test(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 118, 38, 212], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dealToken` (0xec85530d) function
        pub fn deal_token(
            &self,
            token: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([236, 133, 83, 13], (token, to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `excludeArtifacts` (0xb5508aa9) function
        pub fn exclude_artifacts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([181, 80, 138, 169], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `excludeContracts` (0xe20c9f71) function
        pub fn exclude_contracts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([226, 12, 159, 113], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `excludeSenders` (0x1ed7831c) function
        pub fn exclude_senders(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([30, 215, 131, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failed` (0xba414fa6) function
        pub fn failed(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([186, 65, 79, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetArtifactSelectors` (0x66d9a9a0) function
        pub fn target_artifact_selectors(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<FuzzSelector>,
        > {
            self.0
                .method_hash([102, 217, 169, 160], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetArtifacts` (0x85226c81) function
        pub fn target_artifacts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([133, 34, 108, 129], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetContracts` (0x3f7286f4) function
        pub fn target_contracts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([63, 114, 134, 244], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetSelectors` (0x916a17c6) function
        pub fn target_selectors(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<FuzzSelector>,
        > {
            self.0
                .method_hash([145, 106, 23, 198], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetSenders` (0x3e5e3c23) function
        pub fn target_senders(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([62, 94, 60, 35], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `log` event
        pub fn log_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogFilter> {
            self.0.event()
        }
        ///Gets the contract's `log_address` event
        pub fn log_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogAddressFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogArray1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogArray2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_3_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogArray3Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_bytes` event
        pub fn log_bytes_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogBytesFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_bytes32` event
        pub fn log_bytes_32_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogBytes32Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_int` event
        pub fn log_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogIntFilter> {
            self.0.event()
        }
        ///Gets the contract's `log_named_address` event
        pub fn log_named_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedAddressFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedArray1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedArray2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_3_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedArray3Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_bytes` event
        pub fn log_named_bytes_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedBytesFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_bytes32` event
        pub fn log_named_bytes_32_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedBytes32Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_decimal_int` event
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedDecimalIntFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_decimal_uint` event
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedDecimalUintFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_int` event
        pub fn log_named_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedIntFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_string` event
        pub fn log_named_string_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedStringFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_uint` event
        pub fn log_named_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedUintFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_string` event
        pub fn log_string_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogStringFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_uint` event
        pub fn log_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogUintFilter> {
            self.0.event()
        }
        ///Gets the contract's `logs` event
        pub fn logs_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogsFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DealerEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Dealer<M> {
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
    #[ethevent(name = "log", abi = "log(string)")]
    pub struct LogFilter(pub ::std::string::String);
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
    #[ethevent(name = "log_address", abi = "log_address(address)")]
    pub struct LogAddressFilter(pub ::ethers::core::types::Address);
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
    #[ethevent(name = "log_array", abi = "log_array(uint256[])")]
    pub struct LogArray1Filter {
        pub val: ::std::vec::Vec<::ethers::core::types::U256>,
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
    #[ethevent(name = "log_array", abi = "log_array(int256[])")]
    pub struct LogArray2Filter {
        pub val: ::std::vec::Vec<::ethers::core::types::I256>,
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
    #[ethevent(name = "log_array", abi = "log_array(address[])")]
    pub struct LogArray3Filter {
        pub val: ::std::vec::Vec<::ethers::core::types::Address>,
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
    #[ethevent(name = "log_bytes", abi = "log_bytes(bytes)")]
    pub struct LogBytesFilter(pub ::ethers::core::types::Bytes);
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
    #[ethevent(name = "log_bytes32", abi = "log_bytes32(bytes32)")]
    pub struct LogBytes32Filter(pub [u8; 32]);
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
    #[ethevent(name = "log_int", abi = "log_int(int256)")]
    pub struct LogIntFilter(pub ::ethers::core::types::I256);
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
    #[ethevent(name = "log_named_address", abi = "log_named_address(string,address)")]
    pub struct LogNamedAddressFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::Address,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,uint256[])")]
    pub struct LogNamedArray1Filter {
        pub key: ::std::string::String,
        pub val: ::std::vec::Vec<::ethers::core::types::U256>,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,int256[])")]
    pub struct LogNamedArray2Filter {
        pub key: ::std::string::String,
        pub val: ::std::vec::Vec<::ethers::core::types::I256>,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,address[])")]
    pub struct LogNamedArray3Filter {
        pub key: ::std::string::String,
        pub val: ::std::vec::Vec<::ethers::core::types::Address>,
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
    #[ethevent(name = "log_named_bytes", abi = "log_named_bytes(string,bytes)")]
    pub struct LogNamedBytesFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "log_named_bytes32", abi = "log_named_bytes32(string,bytes32)")]
    pub struct LogNamedBytes32Filter {
        pub key: ::std::string::String,
        pub val: [u8; 32],
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
        name = "log_named_decimal_int",
        abi = "log_named_decimal_int(string,int256,uint256)"
    )]
    pub struct LogNamedDecimalIntFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::I256,
        pub decimals: ::ethers::core::types::U256,
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
        name = "log_named_decimal_uint",
        abi = "log_named_decimal_uint(string,uint256,uint256)"
    )]
    pub struct LogNamedDecimalUintFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::U256,
        pub decimals: ::ethers::core::types::U256,
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
    #[ethevent(name = "log_named_int", abi = "log_named_int(string,int256)")]
    pub struct LogNamedIntFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::I256,
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
    #[ethevent(name = "log_named_string", abi = "log_named_string(string,string)")]
    pub struct LogNamedStringFilter {
        pub key: ::std::string::String,
        pub val: ::std::string::String,
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
    #[ethevent(name = "log_named_uint", abi = "log_named_uint(string,uint256)")]
    pub struct LogNamedUintFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::U256,
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
    #[ethevent(name = "log_string", abi = "log_string(string)")]
    pub struct LogStringFilter(pub ::std::string::String);
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
    #[ethevent(name = "log_uint", abi = "log_uint(uint256)")]
    pub struct LogUintFilter(pub ::ethers::core::types::U256);
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
    #[ethevent(name = "logs", abi = "logs(bytes)")]
    pub struct LogsFilter(pub ::ethers::core::types::Bytes);
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DealerEvents {
        LogFilter(LogFilter),
        LogAddressFilter(LogAddressFilter),
        LogArray1Filter(LogArray1Filter),
        LogArray2Filter(LogArray2Filter),
        LogArray3Filter(LogArray3Filter),
        LogBytesFilter(LogBytesFilter),
        LogBytes32Filter(LogBytes32Filter),
        LogIntFilter(LogIntFilter),
        LogNamedAddressFilter(LogNamedAddressFilter),
        LogNamedArray1Filter(LogNamedArray1Filter),
        LogNamedArray2Filter(LogNamedArray2Filter),
        LogNamedArray3Filter(LogNamedArray3Filter),
        LogNamedBytesFilter(LogNamedBytesFilter),
        LogNamedBytes32Filter(LogNamedBytes32Filter),
        LogNamedDecimalIntFilter(LogNamedDecimalIntFilter),
        LogNamedDecimalUintFilter(LogNamedDecimalUintFilter),
        LogNamedIntFilter(LogNamedIntFilter),
        LogNamedStringFilter(LogNamedStringFilter),
        LogNamedUintFilter(LogNamedUintFilter),
        LogStringFilter(LogStringFilter),
        LogUintFilter(LogUintFilter),
        LogsFilter(LogsFilter),
    }
    impl ::ethers::contract::EthLogDecode for DealerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(DealerEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(DealerEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogArray1Filter::decode_log(log) {
                return Ok(DealerEvents::LogArray1Filter(decoded));
            }
            if let Ok(decoded) = LogArray2Filter::decode_log(log) {
                return Ok(DealerEvents::LogArray2Filter(decoded));
            }
            if let Ok(decoded) = LogArray3Filter::decode_log(log) {
                return Ok(DealerEvents::LogArray3Filter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(DealerEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(DealerEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(DealerEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(DealerEvents::LogNamedAddressFilter(decoded));
            }
            if let Ok(decoded) = LogNamedArray1Filter::decode_log(log) {
                return Ok(DealerEvents::LogNamedArray1Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray2Filter::decode_log(log) {
                return Ok(DealerEvents::LogNamedArray2Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray3Filter::decode_log(log) {
                return Ok(DealerEvents::LogNamedArray3Filter(decoded));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(DealerEvents::LogNamedBytesFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(DealerEvents::LogNamedBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(DealerEvents::LogNamedDecimalIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(DealerEvents::LogNamedDecimalUintFilter(decoded));
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(DealerEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(DealerEvents::LogNamedStringFilter(decoded));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(DealerEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(DealerEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(DealerEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(DealerEvents::LogsFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DealerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LogFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogAddressFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray1Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray2Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray3Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes32Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogIntFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedAddressFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedArray1Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedArray2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedArray3Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedBytesFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedBytes32Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedDecimalIntFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedDecimalUintFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedIntFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedStringFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedUintFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogStringFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogUintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogsFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LogFilter> for DealerEvents {
        fn from(value: LogFilter) -> Self {
            Self::LogFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter> for DealerEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogArray1Filter> for DealerEvents {
        fn from(value: LogArray1Filter) -> Self {
            Self::LogArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogArray2Filter> for DealerEvents {
        fn from(value: LogArray2Filter) -> Self {
            Self::LogArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogArray3Filter> for DealerEvents {
        fn from(value: LogArray3Filter) -> Self {
            Self::LogArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogBytesFilter> for DealerEvents {
        fn from(value: LogBytesFilter) -> Self {
            Self::LogBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogBytes32Filter> for DealerEvents {
        fn from(value: LogBytes32Filter) -> Self {
            Self::LogBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogIntFilter> for DealerEvents {
        fn from(value: LogIntFilter) -> Self {
            Self::LogIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedAddressFilter> for DealerEvents {
        fn from(value: LogNamedAddressFilter) -> Self {
            Self::LogNamedAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray1Filter> for DealerEvents {
        fn from(value: LogNamedArray1Filter) -> Self {
            Self::LogNamedArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray2Filter> for DealerEvents {
        fn from(value: LogNamedArray2Filter) -> Self {
            Self::LogNamedArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray3Filter> for DealerEvents {
        fn from(value: LogNamedArray3Filter) -> Self {
            Self::LogNamedArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytesFilter> for DealerEvents {
        fn from(value: LogNamedBytesFilter) -> Self {
            Self::LogNamedBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytes32Filter> for DealerEvents {
        fn from(value: LogNamedBytes32Filter) -> Self {
            Self::LogNamedBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalIntFilter> for DealerEvents {
        fn from(value: LogNamedDecimalIntFilter) -> Self {
            Self::LogNamedDecimalIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalUintFilter> for DealerEvents {
        fn from(value: LogNamedDecimalUintFilter) -> Self {
            Self::LogNamedDecimalUintFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedIntFilter> for DealerEvents {
        fn from(value: LogNamedIntFilter) -> Self {
            Self::LogNamedIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedStringFilter> for DealerEvents {
        fn from(value: LogNamedStringFilter) -> Self {
            Self::LogNamedStringFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedUintFilter> for DealerEvents {
        fn from(value: LogNamedUintFilter) -> Self {
            Self::LogNamedUintFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter> for DealerEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUintFilter> for DealerEvents {
        fn from(value: LogUintFilter) -> Self {
            Self::LogUintFilter(value)
        }
    }
    impl ::core::convert::From<LogsFilter> for DealerEvents {
        fn from(value: LogsFilter) -> Self {
            Self::LogsFilter(value)
        }
    }
    ///Container type for all input parameters for the `IS_TEST` function with signature `IS_TEST()` and selector `0xfa7626d4`
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
    #[ethcall(name = "IS_TEST", abi = "IS_TEST()")]
    pub struct IsTestCall;
    ///Container type for all input parameters for the `dealToken` function with signature `dealToken(address,address,uint256)` and selector `0xec85530d`
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
    #[ethcall(name = "dealToken", abi = "dealToken(address,address,uint256)")]
    pub struct DealTokenCall {
        pub token: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `excludeArtifacts` function with signature `excludeArtifacts()` and selector `0xb5508aa9`
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
    #[ethcall(name = "excludeArtifacts", abi = "excludeArtifacts()")]
    pub struct ExcludeArtifactsCall;
    ///Container type for all input parameters for the `excludeContracts` function with signature `excludeContracts()` and selector `0xe20c9f71`
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
    #[ethcall(name = "excludeContracts", abi = "excludeContracts()")]
    pub struct ExcludeContractsCall;
    ///Container type for all input parameters for the `excludeSenders` function with signature `excludeSenders()` and selector `0x1ed7831c`
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
    #[ethcall(name = "excludeSenders", abi = "excludeSenders()")]
    pub struct ExcludeSendersCall;
    ///Container type for all input parameters for the `failed` function with signature `failed()` and selector `0xba414fa6`
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
    #[ethcall(name = "failed", abi = "failed()")]
    pub struct FailedCall;
    ///Container type for all input parameters for the `targetArtifactSelectors` function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`
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
    #[ethcall(name = "targetArtifactSelectors", abi = "targetArtifactSelectors()")]
    pub struct TargetArtifactSelectorsCall;
    ///Container type for all input parameters for the `targetArtifacts` function with signature `targetArtifacts()` and selector `0x85226c81`
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
    #[ethcall(name = "targetArtifacts", abi = "targetArtifacts()")]
    pub struct TargetArtifactsCall;
    ///Container type for all input parameters for the `targetContracts` function with signature `targetContracts()` and selector `0x3f7286f4`
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
    #[ethcall(name = "targetContracts", abi = "targetContracts()")]
    pub struct TargetContractsCall;
    ///Container type for all input parameters for the `targetSelectors` function with signature `targetSelectors()` and selector `0x916a17c6`
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
    #[ethcall(name = "targetSelectors", abi = "targetSelectors()")]
    pub struct TargetSelectorsCall;
    ///Container type for all input parameters for the `targetSenders` function with signature `targetSenders()` and selector `0x3e5e3c23`
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
    #[ethcall(name = "targetSenders", abi = "targetSenders()")]
    pub struct TargetSendersCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DealerCalls {
        IsTest(IsTestCall),
        DealToken(DealTokenCall),
        ExcludeArtifacts(ExcludeArtifactsCall),
        ExcludeContracts(ExcludeContractsCall),
        ExcludeSenders(ExcludeSendersCall),
        Failed(FailedCall),
        TargetArtifactSelectors(TargetArtifactSelectorsCall),
        TargetArtifacts(TargetArtifactsCall),
        TargetContracts(TargetContractsCall),
        TargetSelectors(TargetSelectorsCall),
        TargetSenders(TargetSendersCall),
    }
    impl ::ethers::core::abi::AbiDecode for DealerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <IsTestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsTest(decoded));
            }
            if let Ok(decoded) = <DealTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DealToken(decoded));
            }
            if let Ok(decoded) = <ExcludeArtifactsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExcludeArtifacts(decoded));
            }
            if let Ok(decoded) = <ExcludeContractsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExcludeContracts(decoded));
            }
            if let Ok(decoded) = <ExcludeSendersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExcludeSenders(decoded));
            }
            if let Ok(decoded) = <FailedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Failed(decoded));
            }
            if let Ok(decoded) = <TargetArtifactSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetArtifactSelectors(decoded));
            }
            if let Ok(decoded) = <TargetArtifactsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetArtifacts(decoded));
            }
            if let Ok(decoded) = <TargetContractsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetContracts(decoded));
            }
            if let Ok(decoded) = <TargetSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetSelectors(decoded));
            }
            if let Ok(decoded) = <TargetSendersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetSenders(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DealerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsTest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DealToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExcludeArtifacts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExcludeContracts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExcludeSenders(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Failed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetArtifactSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetArtifacts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetContracts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetSenders(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DealerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsTest(element) => ::core::fmt::Display::fmt(element, f),
                Self::DealToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeArtifacts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeSenders(element) => ::core::fmt::Display::fmt(element, f),
                Self::Failed(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetArtifactSelectors(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TargetArtifacts(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSelectors(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSenders(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IsTestCall> for DealerCalls {
        fn from(value: IsTestCall) -> Self {
            Self::IsTest(value)
        }
    }
    impl ::core::convert::From<DealTokenCall> for DealerCalls {
        fn from(value: DealTokenCall) -> Self {
            Self::DealToken(value)
        }
    }
    impl ::core::convert::From<ExcludeArtifactsCall> for DealerCalls {
        fn from(value: ExcludeArtifactsCall) -> Self {
            Self::ExcludeArtifacts(value)
        }
    }
    impl ::core::convert::From<ExcludeContractsCall> for DealerCalls {
        fn from(value: ExcludeContractsCall) -> Self {
            Self::ExcludeContracts(value)
        }
    }
    impl ::core::convert::From<ExcludeSendersCall> for DealerCalls {
        fn from(value: ExcludeSendersCall) -> Self {
            Self::ExcludeSenders(value)
        }
    }
    impl ::core::convert::From<FailedCall> for DealerCalls {
        fn from(value: FailedCall) -> Self {
            Self::Failed(value)
        }
    }
    impl ::core::convert::From<TargetArtifactSelectorsCall> for DealerCalls {
        fn from(value: TargetArtifactSelectorsCall) -> Self {
            Self::TargetArtifactSelectors(value)
        }
    }
    impl ::core::convert::From<TargetArtifactsCall> for DealerCalls {
        fn from(value: TargetArtifactsCall) -> Self {
            Self::TargetArtifacts(value)
        }
    }
    impl ::core::convert::From<TargetContractsCall> for DealerCalls {
        fn from(value: TargetContractsCall) -> Self {
            Self::TargetContracts(value)
        }
    }
    impl ::core::convert::From<TargetSelectorsCall> for DealerCalls {
        fn from(value: TargetSelectorsCall) -> Self {
            Self::TargetSelectors(value)
        }
    }
    impl ::core::convert::From<TargetSendersCall> for DealerCalls {
        fn from(value: TargetSendersCall) -> Self {
            Self::TargetSenders(value)
        }
    }
    ///Container type for all return fields from the `IS_TEST` function with signature `IS_TEST()` and selector `0xfa7626d4`
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
    pub struct IsTestReturn(pub bool);
    ///Container type for all return fields from the `excludeArtifacts` function with signature `excludeArtifacts()` and selector `0xb5508aa9`
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
    pub struct ExcludeArtifactsReturn {
        pub excluded_artifacts: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all return fields from the `excludeContracts` function with signature `excludeContracts()` and selector `0xe20c9f71`
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
    pub struct ExcludeContractsReturn {
        pub excluded_contracts: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `excludeSenders` function with signature `excludeSenders()` and selector `0x1ed7831c`
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
    pub struct ExcludeSendersReturn {
        pub excluded_senders: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `failed` function with signature `failed()` and selector `0xba414fa6`
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
    pub struct FailedReturn(pub bool);
    ///Container type for all return fields from the `targetArtifactSelectors` function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`
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
    pub struct TargetArtifactSelectorsReturn {
        pub targeted_artifact_selectors: ::std::vec::Vec<FuzzSelector>,
    }
    ///Container type for all return fields from the `targetArtifacts` function with signature `targetArtifacts()` and selector `0x85226c81`
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
    pub struct TargetArtifactsReturn {
        pub targeted_artifacts: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all return fields from the `targetContracts` function with signature `targetContracts()` and selector `0x3f7286f4`
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
    pub struct TargetContractsReturn {
        pub targeted_contracts: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `targetSelectors` function with signature `targetSelectors()` and selector `0x916a17c6`
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
    pub struct TargetSelectorsReturn {
        pub targeted_selectors: ::std::vec::Vec<FuzzSelector>,
    }
    ///Container type for all return fields from the `targetSenders` function with signature `targetSenders()` and selector `0x3e5e3c23`
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
    pub struct TargetSendersReturn {
        pub targeted_senders: ::std::vec::Vec<::ethers::core::types::Address>,
    }
}
