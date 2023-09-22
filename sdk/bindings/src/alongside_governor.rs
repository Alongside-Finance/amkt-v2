pub use alongside_governor::*;
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
pub mod alongside_governor {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_token"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IVotes"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_timelock"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract TimelockController",
                            ),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BALLOT_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BALLOT_TYPEHASH"),
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
                    ::std::borrow::ToOwned::to_owned("COUNTING_MODE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("COUNTING_MODE"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EXTENDED_BALLOT_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EXTENDED_BALLOT_TYPEHASH",
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
                    ::std::borrow::ToOwned::to_owned("castVote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("castVote"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("support"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("castVoteBySig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("castVoteBySig"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("support"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
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
                    ::std::borrow::ToOwned::to_owned("castVoteWithReason"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("castVoteWithReason"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("support"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("castVoteWithReasonAndParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "castVoteWithReasonAndParams",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("support"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("castVoteWithReasonAndParamsBySig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "castVoteWithReasonAndParamsBySig",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("support"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
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
                    ::std::borrow::ToOwned::to_owned("execute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("execute"),
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
                                    name: ::std::borrow::ToOwned::to_owned("calldatas"),
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
                                    name: ::std::borrow::ToOwned::to_owned("descriptionHash"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getVotes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getVotes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
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
                    ::std::borrow::ToOwned::to_owned("getVotesWithParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getVotesWithParams"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("hasVoted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasVoted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("hashProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hashProposal"),
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
                                    name: ::std::borrow::ToOwned::to_owned("calldatas"),
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
                                    name: ::std::borrow::ToOwned::to_owned("descriptionHash"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
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
                    ::std::borrow::ToOwned::to_owned("proposalDeadline"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposalDeadline"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
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
                    ::std::borrow::ToOwned::to_owned("proposalEta"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposalEta"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
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
                    ::std::borrow::ToOwned::to_owned("proposalSnapshot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposalSnapshot"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
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
                    ::std::borrow::ToOwned::to_owned("proposalThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposalThreshold"),
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
                    ::std::borrow::ToOwned::to_owned("proposalVotes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposalVotes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
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
                                    name: ::std::borrow::ToOwned::to_owned("againstVotes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("forVotes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("abstainVotes"),
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
                    ::std::borrow::ToOwned::to_owned("propose"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("propose"),
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
                                    name: ::std::borrow::ToOwned::to_owned("calldatas"),
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
                                    name: ::std::borrow::ToOwned::to_owned("description"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("queue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("queue"),
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
                                    name: ::std::borrow::ToOwned::to_owned("calldatas"),
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
                                    name: ::std::borrow::ToOwned::to_owned("descriptionHash"),
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
                    ::std::borrow::ToOwned::to_owned("quorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quorum"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
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
                    ::std::borrow::ToOwned::to_owned("quorumDenominator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quorumDenominator"),
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
                    ::std::borrow::ToOwned::to_owned("quorumNumerator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quorumNumerator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quorumNumerator"),
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
                    ::std::borrow::ToOwned::to_owned("relay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("relay"),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setProposalThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setProposalThreshold",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newProposalThreshold",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("setVotingDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setVotingDelay"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newVotingDelay"),
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
                    ::std::borrow::ToOwned::to_owned("setVotingPeriod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setVotingPeriod"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newVotingPeriod"),
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
                    ::std::borrow::ToOwned::to_owned("state"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("state"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IGovernor.ProposalState",
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
                    ::std::borrow::ToOwned::to_owned("timelock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("timelock"),
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
                    ::std::borrow::ToOwned::to_owned("token"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("token"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IVotes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateQuorumNumerator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateQuorumNumerator",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newQuorumNumerator",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("updateTimelock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateTimelock"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newTimelock"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract TimelockController",
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
                    ::std::borrow::ToOwned::to_owned("votingDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("votingDelay"),
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
                    ::std::borrow::ToOwned::to_owned("votingPeriod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("votingPeriod"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ProposalCanceled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ProposalCanceled"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
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
                    ::std::borrow::ToOwned::to_owned("ProposalCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ProposalCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("targets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("signatures"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("calldatas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("startBlock"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("endBlock"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("description"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProposalExecuted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ProposalExecuted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
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
                    ::std::borrow::ToOwned::to_owned("ProposalQueued"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ProposalQueued"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eta"),
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
                    ::std::borrow::ToOwned::to_owned("ProposalThresholdSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ProposalThresholdSet",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "oldProposalThreshold",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newProposalThreshold",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("QuorumNumeratorUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "QuorumNumeratorUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "oldQuorumNumerator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newQuorumNumerator",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("TimelockChange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TimelockChange"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldTimelock"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newTimelock"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VoteCast"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("VoteCast"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("voter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("support"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("weight"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VoteCastWithParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("VoteCastWithParams"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("voter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("support"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("weight"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VotingDelaySet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("VotingDelaySet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldVotingDelay"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newVotingDelay"),
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
                    ::std::borrow::ToOwned::to_owned("VotingPeriodSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("VotingPeriodSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldVotingPeriod"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newVotingPeriod"),
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
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Empty"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Empty"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ALONGSIDEGOVERNOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01``@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0E]8\x03\x80b\0E]\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x08\x1AV[\x80`\n\x83b\0\0H`\x0Ca\x0E\x10b\0\x08YV[b\0\0X`\x0Cb\x05F\0b\0\x08YV[`@\x80Q\x80\x82\x01\x90\x91R`\x12\x81Rq \xB67\xB73\xB9\xB4\xB22\x90#\xB7\xBB2\xB972\xB9`q\x1B` \x82\x01R`d\x90\x80b\0\0\xA4`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`1`\xF8\x1B` \x82\x01R\x90V[\x81Q` \x92\x83\x01 \x81Q\x91\x83\x01\x91\x90\x91 `\xE0\x82\x90Ra\x01\0\x81\x90RF`\xA0\x81\x81R`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81\x88\x01\x81\x90R\x81\x83\x01\x96\x90\x96R``\x81\x01\x94\x90\x94R`\x80\x80\x85\x01\x93\x90\x93R0\x84\x83\x01\x81\x90R\x81Q\x80\x86\x03\x90\x93\x01\x83R`\xC0\x94\x85\x01\x90\x91R\x81Q\x91\x90\x95\x01 \x90R\x91\x90\x91Ra\x01 R`\0b\0\x01A\x82\x82b\0\t!V[Pb\0\x01O\x90P\x83b\0\x01\x95V[b\0\x01Z\x82b\0\x01\xD6V[b\0\x01e\x81b\0\x02}V[PPP`\x01`\x01`\xA0\x1B\x03\x16a\x01@Rb\0\x01\x80\x81b\0\x02\xBEV[Pb\0\x01\x8C\x81b\0\x046V[PPPb\0\n\x13V[`\x04T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xC5e\xB0E@=\xC0<.\xEA\x82\xB8\x1A\x04e\xED\xAD\x9E.\x7F\xC4\xD9~\x11B\x1C \x9D\xA9=z\x93\x91\x01`@Q\x80\x91\x03\x90\xA1`\x04UV[`\0\x81\x11b\0\x02<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FGovernorSettings: voting period `D\x82\x01Rftoo low`\xC8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\x05T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F~?\x7F\x07\x08\xA8M\xE9 06\xAB\xAAE\r\xCC\xC8Z\xD5\xFFR\xF7\x8C\x17\x0F>\xDBU\xCF^\x88(\x91\x01`@Q\x80\x91\x03\x90\xA1`\x05UV[`\x06T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xCC\xB4]\xA8\xD5q~lEDiB\x97\xC4\xBA\\\xF1Q\xD4U\xC9\xBB\x0E\xD4\xFCz8A\x1B\xC0Ta\x91\x01`@Q\x80\x91\x03\x90\xA1`\x06UV[`d\x81\x11\x15b\0\x03CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FGovernorVotesQuorumFraction: quo`D\x82\x01R\x7FrumNumerator over quorumDenomina`d\x82\x01Rb:7\xB9`\xE9\x1B`\x84\x82\x01R`\xA4\x01b\0\x023V[`\0b\0\x03Ob\0\x04\x9FV[\x90P\x80\x15\x80\x15\x90b\0\x03aWP`\tT\x15[\x15b\0\x03\xDCW`\t`\0\x01`@Q\x80`@\x01`@R\x80`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01b\0\x03\x9A\x84b\0\x04\xD7` \x1Bb\0\x14\xA6\x17` \x1CV[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U`\0\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[b\0\x03\xF7\x82`\tb\0\x05F` \x1Bb\0\x15\x13\x17\x90\x91\x90` \x1CV[PP`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\x05SGk\xF0.\xF2rn\x8C\xE5\xCE\xD7\x8Dc\xE2n`.J\"W\xB1\xF5YA\x8E$\xB4c9\x97\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`\nT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\x08\xF7N\xA4n\xF7\x89Oe\xEA\xBF\xB5\xE6\xE6\x95\xDEw:\0\x0BG\xC5)\xABU\x91x\x06\x9B\"d\x01\x91\x01`@Q\x80\x91\x03\x90\xA1`\n\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\tT`\0\x90\x15b\0\x04\xD0Wb\0\x04\xC2`\tb\0\x05\x99` \x1Bb\0\x15I\x17` \x1CV[`\x01`\x01`\xE0\x1B\x03\x16\x90P\x90V[P`\x08T\x90V[`\0`\x01`\x01`\xE0\x1B\x03\x82\x11\x15b\0\x05BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01b\0\x023V[P\x90V[`\0\x80b\0\x05\x83\x84`\0\x01b\0\x05gCb\0\x05\xE7` \x1Bb\0\x15\x87\x17` \x1CV[b\0\x05}\x86b\0\x04\xD7` \x1Bb\0\x14\xA6\x17` \x1CV[b\0\x06NV[`\x01`\x01`\xE0\x1B\x03\x91\x82\x16\x96\x91\x16\x94P\x92PPPV[\x80T`\0\x90\x80\x15b\0\x05\xDDWb\0\x05\xC5\x83b\0\x05\xB7`\x01\x84b\0\t\xEDV[`\0\x91\x82R` \x90\x91 \x01\x90V[Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16b\0\x05\xE0V[`\0[\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x05BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01b\0\x023V[\x82T`\0\x90\x81\x90\x80\x15b\0\x07\xA3W`\0b\0\x06p\x87b\0\x05\xB7`\x01\x85b\0\t\xEDV[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84Rd\x01\0\0\0\0\x90\x92\x04`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15b\0\x06\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCheckpoint: invalid key\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x023V[\x80Qc\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03b\0\x07BW\x84b\0\x07\x19\x88b\0\x05\xB7`\x01\x86b\0\t\xEDV[\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\x07\x92V[`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU`\0\x8D\x81R\x91\x90\x91 \x94Q\x91Q\x90\x92\x16d\x01\0\0\0\0\x02\x92\x16\x91\x90\x91\x17\x91\x01U[` \x01Q\x92P\x83\x91Pb\0\x07\xF9\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU`\0\x8A\x81R\x91\x82 \x95Q\x92Q\x90\x93\x16d\x01\0\0\0\0\x02\x91\x90\x93\x16\x17\x92\x01\x91\x90\x91U\x90P\x81[\x93P\x93\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x08\x17W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x08.W`\0\x80\xFD[\x82Qb\0\x08;\x81b\0\x08\x01V[` \x84\x01Q\x90\x92Pb\0\x08N\x81b\0\x08\x01V[\x80\x91PP\x92P\x92\x90PV[`\0\x82b\0\x08wWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x08\xA7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x08\xC8WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\t\x1CW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x08\xF7WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\t\x18W\x82\x81U`\x01\x01b\0\t\x03V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\t=Wb\0\t=b\0\x08|V[b\0\tU\x81b\0\tN\x84Tb\0\x08\x92V[\x84b\0\x08\xCEV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\t\x8DW`\0\x84\x15b\0\ttWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\t\x18V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\t\xBEW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\t\x9DV[P\x85\x82\x10\x15b\0\t\xDDW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x82\x82\x10\x15b\0\n\x0EWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x03\x90V[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa:\xE1b\0\n|`\09`\0\x81\x81a\x08i\x01R\x81\x81a\x1F\xE1\x01Ra!\xD5\x01R`\0a$\xF4\x01R`\0a%C\x01R`\0a%\x1E\x01R`\0a$w\x01R`\0a$\xA1\x01R`\0a$\xCB\x01Ra:\xE1`\0\xF3\xFE`\x80`@R`\x046\x10a\x02JW`\x005`\xE0\x1C\x80c{<q\xD3\x11a\x019W\x80c\xC2\x8B\xC2\xFA\x11a\0\xB6W\x80c\xEA\x02\x17\xCF\x11a\0zW\x80c\xEA\x02\x17\xCF\x14a\x07\xABW\x80c\xEB\x90\x19\xD4\x14a\x07\xCBW\x80c\xEC\xE4\x0C\xC1\x14a\x07\xEBW\x80c\xF2:na\x14a\x08\x0BW\x80c\xF8\xCEV\n\x14a\x087W\x80c\xFC\x0CTj\x14a\x08WW`\0\x80\xFD[\x80c\xC2\x8B\xC2\xFA\x14a\x06\xCCW\x80c\xC5\x90W\xE4\x14a\x06\xDFW\x80c\xD32\x19\xB4\x14a\x06\xFFW\x80c\xDDN+\xA5\x14a\x071W\x80c\xDE\xAA\xA7\xCC\x14a\x07wW`\0\x80\xFD[\x80c\xA8\x90\xC9\x10\x11a\0\xFDW\x80c\xA8\x90\xC9\x10\x14a\x06+W\x80c\xABX\xFB\x8E\x14a\x06KW\x80c\xB5\x811\xB0\x14a\x06kW\x80c\xBC\x19|\x81\x14a\x06\x80W\x80c\xC0\x1F\x9E7\x14a\x06\xACW`\0\x80\xFD[\x80c{<q\xD3\x14a\x05\xA2W\x80c}^\x81\xE2\x14a\x05\xC2W\x80c\x97\xC3\xD34\x14a\x05\xE2W\x80c\x9A\x80*m\x14a\x05\xF6W\x80c\xA7q:p\x14a\x06\x16W`\0\x80\xFD[\x80c92\xAB\xB1\x11a\x01\xC7W\x80cT\xFDMP\x11a\x01\x8BW\x80cT\xFDMP\x14a\x04\xF8W\x80cVx\x13\x88\x14a\x05\"W\x80c_9\x8A\x14\x14a\x05BW\x80c`\xC4$\x7F\x14a\x05bW\x80cp\xB0\xF6`\x14a\x05\x82W`\0\x80\xFD[\x80c92\xAB\xB1\x14a\x03\xF7W\x80c;\xCC\xF4\xFD\x14a\x04\x0CW\x80c>OI\xE6\x14a\x04,W\x80cC\x85\x962\x14a\x04YW\x80cTO\xFC\x9C\x14a\x04\xA3W`\0\x80\xFD[\x80c\x15\x0Bz\x02\x11a\x02\x0EW\x80c\x15\x0Bz\x02\x14a\x03,W\x80c\x16\x0C\xBE\xD7\x14a\x03pW\x80c&V\"}\x14a\x03\x90W\x80c-c\xF6\x93\x14a\x03\xA3W\x80c/\xE3\xE2a\x14a\x03\xC3W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x02rW\x80c\x02\xA2Q\xA3\x14a\x02\xA7W\x80c\x03B\x01\x81\x14a\x02\xCAW\x80c\x06\xF3\xF9\xE6\x14a\x02\xEAW\x80c\x06\xFD\xDE\x03\x14a\x03\nW`\0\x80\xFD[6a\x02mW0a\x02Xa\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x02kW`\0\x80\xFD[\0[`\0\x80\xFD[4\x80\x15a\x02~W`\0\x80\xFD[Pa\x02\x92a\x02\x8D6`\x04a,\x1AV[a\x08\xA4V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xB3W`\0\x80\xFD[Pa\x02\xBCa\x08\xB5V[`@Q\x90\x81R` \x01a\x02\x9EV[4\x80\x15a\x02\xD6W`\0\x80\xFD[Pa\x02\xBCa\x02\xE56`\x04a-XV[a\x08\xC0V[4\x80\x15a\x02\xF6W`\0\x80\xFD[Pa\x02ka\x03\x056`\x04a-\xFEV[a\t\xB8V[4\x80\x15a\x03\x16W`\0\x80\xFD[Pa\x03\x1Fa\nKV[`@Qa\x02\x9E\x91\x90a.dV[4\x80\x15a\x038W`\0\x80\xFD[Pa\x03Wa\x03G6`\x04a.\x8CV[c\n\x85\xBD\x01`\xE1\x1B\x94\x93PPPPV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x02\x9EV[4\x80\x15a\x03|W`\0\x80\xFD[Pa\x02\xBCa\x03\x8B6`\x04a0hV[a\n\xDDV[a\x02\xBCa\x03\x9E6`\x04a0hV[a\x0C\xDDV[4\x80\x15a\x03\xAFW`\0\x80\xFD[Pa\x02\xBCa\x03\xBE6`\x04a-\xFEV[a\r\xCAV[4\x80\x15a\x03\xCFW`\0\x80\xFD[Pa\x02\xBC\x7F\xB3\xB3\xF3\xB7\x03\xCD\x84\xCE5!\x97\xDC\xFF#+\x1B]<\xFB %\xCEG\xCF\x04t-\x06Q\xF1\xAF\x88\x81V[4\x80\x15a\x04\x03W`\0\x80\xFD[Pa\x02\xBCa\x0E\x01V[4\x80\x15a\x04\x18W`\0\x80\xFD[Pa\x02\xBCa\x04'6`\x04a0\xF7V[a\x0E\x0CV[4\x80\x15a\x048W`\0\x80\xFD[Pa\x04La\x04G6`\x04a-\xFEV[a\x0E\x82V[`@Qa\x02\x9E\x91\x90a1[V[4\x80\x15a\x04eW`\0\x80\xFD[Pa\x02\x92a\x04t6`\x04a1\x83V[`\0\x82\x81R`\x07` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R`\x03\x01\x90\x91R\x90 T`\xFF\x16\x92\x91PPV[4\x80\x15a\x04\xAFW`\0\x80\xFD[Pa\x04\xDDa\x04\xBE6`\x04a-\xFEV[`\0\x90\x81R`\x07` R`@\x90 \x80T`\x01\x82\x01T`\x02\x90\x92\x01T\x90\x92V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\x9EV[4\x80\x15a\x05\x04W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`1`\xF8\x1B` \x82\x01Ra\x03\x1FV[4\x80\x15a\x05.W`\0\x80\xFD[Pa\x02\xBCa\x05=6`\x04a1\xB3V[a\x0E\x8DV[4\x80\x15a\x05NW`\0\x80\xFD[Pa\x02\xBCa\x05]6`\x04a1\xDFV[a\x0E\xB6V[4\x80\x15a\x05nW`\0\x80\xFD[Pa\x02\xBCa\x05}6`\x04a-\xFEV[a\x0F\0V[4\x80\x15a\x05\x8EW`\0\x80\xFD[Pa\x02ka\x05\x9D6`\x04a-\xFEV[a\x0F\x97V[4\x80\x15a\x05\xAEW`\0\x80\xFD[Pa\x02\xBCa\x05\xBD6`\x04a2bV[a\x10\x1EV[4\x80\x15a\x05\xCEW`\0\x80\xFD[Pa\x02\xBCa\x05\xDD6`\x04a2\xBBV[a\x10pV[4\x80\x15a\x05\xEEW`\0\x80\xFD[P`da\x02\xBCV[4\x80\x15a\x06\x02W`\0\x80\xFD[Pa\x02\xBCa\x06\x116`\x04a3oV[a\x10\x87V[4\x80\x15a\x06\"W`\0\x80\xFD[Pa\x02\xBCa\x10\x9EV[4\x80\x15a\x067W`\0\x80\xFD[Pa\x02ka\x06F6`\x04a3\xC7V[a\x10\xC8V[4\x80\x15a\x06WW`\0\x80\xFD[Pa\x02\xBCa\x06f6`\x04a-\xFEV[a\x11OV[4\x80\x15a\x06wW`\0\x80\xFD[Pa\x02\xBCa\x11\xE9V[4\x80\x15a\x06\x8CW`\0\x80\xFD[Pa\x03Wa\x06\x9B6`\x04a3\xE4V[c\xBC\x19|\x81`\xE0\x1B\x95\x94PPPPPV[4\x80\x15a\x06\xB8W`\0\x80\xFD[Pa\x02\xBCa\x06\xC76`\x04a-\xFEV[a\x11\xF4V[a\x02ka\x06\xDA6`\x04a4wV[a\x12#V[4\x80\x15a\x06\xEBW`\0\x80\xFD[Pa\x02\xBCa\x06\xFA6`\x04a0hV[a\x132V[4\x80\x15a\x07\x0BW`\0\x80\xFD[P`\nT`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x9EV[4\x80\x15a\x07=W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R` \x80\x82R\x7Fsupport=bravo&quorum=for,abstain\x90\x82\x01Ra\x03\x1FV[4\x80\x15a\x07\x83W`\0\x80\xFD[Pa\x02\xBC\x7F\x15\x02\x14\xD7MY\xB7\xD1\xE9\x0Cs\xFC\"\xEF=\x99\x1D\xD0\xA7k\x04eC\xD4\xD8\n\xB9-*P2\x8F\x81V[4\x80\x15a\x07\xB7W`\0\x80\xFD[Pa\x02ka\x07\xC66`\x04a-\xFEV[a\x13lV[4\x80\x15a\x07\xD7W`\0\x80\xFD[Pa\x02\xBCa\x07\xE66`\x04a4\xBAV[a\x13\xF3V[4\x80\x15a\x07\xF7W`\0\x80\xFD[Pa\x02ka\x08\x066`\x04a-\xFEV[a\x14\x14V[4\x80\x15a\x08\x17W`\0\x80\xFD[Pa\x03Wa\x08&6`\x04a4\xE6V[c\xF2:na`\xE0\x1B\x95\x94PPPPPV[4\x80\x15a\x08CW`\0\x80\xFD[Pa\x02\xBCa\x08R6`\x04a-\xFEV[a\x14\x9BV[4\x80\x15a\x08cW`\0\x80\xFD[Pa\x07\x19\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0a\x08\x9F`\nT`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90P\x90V[`\0a\x08\xAF\x82a\x15\xECV[\x92\x91PPV[`\0a\x08\x9F`\x05T\x90V[`\0\x80a\tda\t\\\x7F\xB3\xB3\xF3\xB7\x03\xCD\x84\xCE5!\x97\xDC\xFF#+\x1B]<\xFB %\xCEG\xCF\x04t-\x06Q\xF1\xAF\x88\x8C\x8C\x8C\x8C`@Qa\x08\xFC\x92\x91\x90a5NV[`@Q\x80\x91\x03\x90 \x8B\x80Q\x90` \x01 `@Q` \x01a\tA\x95\x94\x93\x92\x91\x90\x94\x85R` \x85\x01\x93\x90\x93R`\xFF\x91\x90\x91\x16`@\x84\x01R``\x83\x01R`\x80\x82\x01R`\xA0\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x16\x11V[\x86\x86\x86a\x16_V[\x90Pa\t\xAA\x8A\x82\x8B\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8D\x92Pa\x16}\x91PPV[\x9A\x99PPPPPPPPPPV[a\t\xC0a\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x90a5^V[`@Q\x80\x91\x03\x90\xFD[0a\n\x02a\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\n?W`\0\x806`@Qa\n\"\x92\x91\x90a5NV[`@Q\x80\x91\x03\x90 \x90P[\x80a\n8`\x02a\x17\xE2V[\x03a\n-WP[a\nH\x81a\x18aV[PV[```\0\x80Ta\nZ\x90a5\x95V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\x86\x90a5\x95V[\x80\x15a\n\xD3W\x80`\x1F\x10a\n\xA8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\xD3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\xB6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0\x80a\n\xEC\x86\x86\x86\x86a\x132V[\x90P`\x04a\n\xF9\x82a\x0E\x82V[`\x07\x81\x11\x15a\x0B\nWa\x0B\na1EV[\x14a\x0B'W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x90a5\xCFV[`\nT`@\x80Qcy=\x06I`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xF2z\x0C\x92\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0BqW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x95\x91\x90a6\x10V[`\nT`@Qc\xB1\xC5\xF4'`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB1\xC5\xF4'\x90a\x0B\xCF\x90\x8A\x90\x8A\x90\x8A\x90`\0\x90\x8B\x90`\x04\x01a6\xF2V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xECW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x10\x91\x90a6\x10V[`\0\x83\x81R`\x0B` R`@\x80\x82 \x92\x90\x92U`\nT\x91Qc\x08\xF2\xA0\xBB`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x8F*\x0B\xB0\x91a\x0C[\x91\x8B\x91\x8B\x91\x8B\x91\x90\x8B\x90\x89\x90`\x04\x01a7@V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0CuW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0C\x89W=`\0\x80>=`\0\xFD[PPPP\x7F\x9A.B\xFDg\"\x81=i\x11>}\0y\xD3\xD9@\x17\x14(\xDFss\xDF\x9C\x7Fv\x17\xCF\xDA(\x92\x82\x82Ba\x0C\xBB\x91\x90a7\xAEV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01`@Q\x80\x91\x03\x90\xA1P\x95\x94PPPPPV[`\0\x80a\x0C\xEC\x86\x86\x86\x86a\x132V[\x90P`\0a\x0C\xF9\x82a\x0E\x82V[\x90P`\x04\x81`\x07\x81\x11\x15a\r\x0FWa\r\x0Fa1EV[\x14\x80a\r,WP`\x05\x81`\x07\x81\x11\x15a\r*Wa\r*a1EV[\x14[a\rHW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x90a5\xCFV[`\0\x82\x81R`\x01` \x81\x81R`@\x92\x83\x90 `\x02\x01\x80T`\xFF\x19\x16\x90\x92\x17\x90\x91U\x90Q\x83\x81R\x7Fq*\xE18?y\xAC\x85?\x8D\x88!Sw\x8E\x02`\xEF\x8F\x03\xB5\x04\xE2\x86n\x05\x93\xE0M+)\x1F\x91\x01`@Q\x80\x91\x03\x90\xA1a\r\xA6\x82\x88\x88\x88\x88a\x19\xADV[a\r\xB3\x82\x88\x88\x88\x88a\x1AOV[a\r\xC0\x82\x88\x88\x88\x88a\x1A\\V[P\x95\x94PPPPPV[`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x81Q\x92\x83\x01\x90\x91RT`\x01`\x01`@\x1B\x03\x16\x90\x81\x90R[`\x01`\x01`@\x1B\x03\x16\x92\x91PPV[`\0a\x08\x9F`\x04T\x90V[`@\x80Q\x7F\x15\x02\x14\xD7MY\xB7\xD1\xE9\x0Cs\xFC\"\xEF=\x99\x1D\xD0\xA7k\x04eC\xD4\xD8\n\xB9-*P2\x8F` \x82\x01R\x90\x81\x01\x86\x90R`\xFF\x85\x16``\x82\x01R`\0\x90\x81\x90a\x0EZ\x90a\t\\\x90`\x80\x01a\tAV[\x90Pa\x0Ew\x87\x82\x88`@Q\x80` \x01`@R\x80`\0\x81RPa\x1A\x95V[\x97\x96PPPPPPPV[`\0a\x08\xAF\x82a\x1A\xB8V[`\0\x803\x90Pa\x0E\xAE\x84\x82\x85`@Q\x80` \x01`@R\x80`\0\x81RPa\x1A\x95V[\x94\x93PPPPV[`\0\x803\x90Pa\x0Ew\x87\x82\x88\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8A\x92Pa\x16}\x91PPV[`\tT`\0\x90\x80\x82\x03a\x0F\x17WPP`\x08T\x91\x90PV[`\0`\ta\x0F&`\x01\x84a7\xC6V[\x81T\x81\x10a\x0F6Wa\x0F6a7\xDDV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83R`\x01` \x1B\x90\x91\x04`\x01`\x01`\xE0\x1B\x03\x16\x92\x82\x01\x92\x90\x92R\x91P\x84\x10a\x0F\x8CW` \x01Q`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[a\x0E\xAE`\t\x85a\x1C\x02V[a\x0F\x9Fa\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x90a5^V[0a\x0F\xD8a\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10\x15W`\0\x806`@Qa\x0F\xF8\x92\x91\x90a5NV[`@Q\x80\x91\x03\x90 \x90P[\x80a\x10\x0E`\x02a\x17\xE2V[\x03a\x10\x03WP[a\nH\x81a\x1C\xB4V[`\0\x803\x90Pa\x10f\x86\x82\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1A\x95\x92PPPV[\x96\x95PPPPPPV[`\0a\x10~\x85\x85\x85\x85a\x1C\xF5V[\x95\x94PPPPPV[`\0a\x10\x94\x84\x84\x84a\x1F\xB8V[\x90P[\x93\x92PPPV[`\tT`\0\x90\x15a\x10\xC1Wa\x10\xB3`\ta\x15IV[`\x01`\x01`\xE0\x1B\x03\x16\x90P\x90V[P`\x08T\x90V[a\x10\xD0a\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11\0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x90a5^V[0a\x11\ta\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11FW`\0\x806`@Qa\x11)\x92\x91\x90a5NV[`@Q\x80\x91\x03\x90 \x90P[\x80a\x11?`\x02a\x17\xE2V[\x03a\x114WP[a\nH\x81a NV[`\nT`\0\x82\x81R`\x0B` R`@\x80\x82 T\x90Qc\xD4\\D5`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R\x90\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xD4\\D5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xD0\x91\x90a6\x10V[\x90P\x80`\x01\x14a\x11\xE0W\x80a\x10\x97V[`\0\x93\x92PPPV[`\0a\x08\x9F`\x06T\x90V[`\0\x81\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q\x92\x83\x01\x90\x91R\x90\x91\x01T`\x01`\x01`@\x1B\x03\x16\x90\x81\x90Ra\r\xF2V[a\x12+a\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x12[W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x90a5^V[0a\x12da\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x12\xA1W`\0\x806`@Qa\x12\x84\x92\x91\x90a5NV[`@Q\x80\x91\x03\x90 \x90P[\x80a\x12\x9A`\x02a\x17\xE2V[\x03a\x12\x8FWP[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85\x85\x85`@Qa\x12\xBF\x92\x91\x90a5NV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x12\xFCW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x13\x01V[``\x91P[P\x91P\x91Pa\x13)\x82\x82`@Q\x80``\x01`@R\x80`(\x81R` \x01a:\x84`(\x919a \xB7V[PPPPPPPV[`\0\x84\x84\x84\x84`@Q` \x01a\x13K\x94\x93\x92\x91\x90a7\xF3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[a\x13ta\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x13\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x90a5^V[0a\x13\xADa\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x13\xEAW`\0\x806`@Qa\x13\xCD\x92\x91\x90a5NV[`@Q\x80\x91\x03\x90 \x90P[\x80a\x13\xE3`\x02a\x17\xE2V[\x03a\x13\xD8WP[a\nH\x81a \xD0V[`\0a\x10\x97\x83\x83a\x14\x0F`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[a\x1F\xB8V[a\x14\x1Ca\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x14LW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x90a5^V[0a\x14Ua\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x14\x92W`\0\x806`@Qa\x14u\x92\x91\x90a5NV[`@Q\x80\x91\x03\x90 \x90P[\x80a\x14\x8B`\x02a\x17\xE2V[\x03a\x14\x80WP[a\nH\x81a!qV[`\0a\x08\xAF\x82a!\xB2V[`\0`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\x15\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\t\xF0V[P\x90V[`\0\x80a\x151\x84a\x15#Ca\x15\x87V[a\x15,\x86a\x14\xA6V[a\"\\V[`\x01`\x01`\xE0\x1B\x03\x91\x82\x16\x93P\x16\x90P[\x92P\x92\x90PV[\x80T`\0\x90\x80\x15a\x11\xE0Wa\x15q\x83a\x15c`\x01\x84a7\xC6V[`\0\x91\x82R` \x90\x91 \x01\x90V[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x10\x97V[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\x15\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\t\xF0V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cnf\\\xED`\xE0\x1B\x14\x80a\x08\xAFWPa\x08\xAF\x82a#\xFFV[`\0a\x08\xAFa\x16\x1Ea$jV[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\0\x80`\0a\x16p\x87\x87\x87\x87a%\x91V[\x91P\x91Pa\r\xC0\x81a&UV[`\0\x85\x81R`\x01` \x81\x90R`@\x82 \x90a\x16\x97\x88a\x0E\x82V[`\x07\x81\x11\x15a\x16\xA8Wa\x16\xA8a1EV[\x14a\x17\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FGovernor: vote not currently act`D\x82\x01Rbive`\xE8\x1B`d\x82\x01R`\x84\x01a\t\xF0V[`@\x80Q` \x81\x01\x90\x91R\x81T`\x01`\x01`@\x1B\x03\x16\x90\x81\x90R`\0\x90a\x17*\x90\x88\x90\x86a\x1F\xB8V[\x90Pa\x179\x88\x88\x88\x84\x88a'\x9FV[\x83Q`\0\x03a\x17\x8EW\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xB8\xE18\x88}\n\xA1;\xABD~\x82\xDE\x9D\\\x17w\x04\x1E\xCD!\xCA6\xBA\x82O\xF1\xE6\xC0}\xDD\xA4\x89\x88\x84\x89`@Qa\x17\x81\x94\x93\x92\x91\x90a8>V[`@Q\x80\x91\x03\x90\xA2a\x0EwV[\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xE2\xBA\xBF\xBA\xC5\x88\x9Ap\x9Bc\xBB\x7FY\x8B2N\x08\xBCZO\xB9\xECd\x7F\xB3\xCB\xC9\xEC\x07\xEB\x87\x12\x89\x88\x84\x89\x89`@Qa\x17\xCF\x95\x94\x93\x92\x91\x90a8fV[`@Q\x80\x91\x03\x90\xA2\x97\x96PPPPPPPV[`\0a\x17\xFD\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15a\x18\x1BW`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B`\0\x81\x81R`\x01\x80\x84\x01` R`@\x82 \x80T\x92\x90U\x83To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x92\x01`\x01`\x01`\x80\x1B\x03\x16\x91\x90\x91\x17\x90\x91U\x90V[`d\x81\x11\x15a\x18\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FGovernorVotesQuorumFraction: quo`D\x82\x01R\x7FrumNumerator over quorumDenomina`d\x82\x01Rb:7\xB9`\xE9\x1B`\x84\x82\x01R`\xA4\x01a\t\xF0V[`\0a\x18\xEEa\x10\x9EV[\x90P\x80\x15\x80\x15\x90a\x18\xFFWP`\tT\x15[\x15a\x19cW`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R`\t\x90` \x81\x01a\x19\"\x84a\x14\xA6V[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U`\0\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16`\x01` \x1B\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[a\x19n`\t\x83a\x15\x13V[PP`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\x05SGk\xF0.\xF2rn\x8C\xE5\xCE\xD7\x8Dc\xE2n`.J\"W\xB1\xF5YA\x8E$\xB4c9\x97\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[0a\x19\xB6a\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1AHW`\0[\x84Q\x81\x10\x15a\x1AFW0`\x01`\x01`\xA0\x1B\x03\x16\x85\x82\x81Q\x81\x10a\x19\xECWa\x19\xECa7\xDDV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1A6Wa\x1A6\x83\x82\x81Q\x81\x10a\x1A\x17Wa\x1A\x17a7\xDDV[` \x02` \x01\x01Q\x80Q\x90` \x01 `\x02a)\x19\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x1A?\x81a8\xACV[\x90Pa\x19\xC7V[P[PPPPPV[a\x1AH\x85\x85\x85\x85\x85a)UV[0a\x1Aea\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1AHW`\x02T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15a\x1AHW`\0`\x02Ua\x1AHV[`\0a\x10~\x85\x85\x85\x85a\x1A\xB3`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[a\x16}V[`\0\x80a\x1A\xC4\x83a)\xC9V[\x90P`\x04\x81`\x07\x81\x11\x15a\x1A\xDAWa\x1A\xDAa1EV[\x14a\x1A\xE5W\x92\x91PPV[`\0\x83\x81R`\x0B` R`@\x90 T\x80a\x1B\0WP\x92\x91PPV[`\nT`@Qc*\xB0\xF5)`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c*\xB0\xF5)\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BIW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Bm\x91\x90a8\xC5V[\x15a\x1B|WP`\x07\x93\x92PPPV[`\nT`@Qc,%\x8A\x9F`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cXK\x15>\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xE9\x91\x90a8\xC5V[\x15a\x1B\xF8WP`\x05\x93\x92PPPV[P`\x02\x93\x92PPPV[`\0C\x82\x10a\x1CSW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FCheckpoints: block not yet mined`D\x82\x01R`d\x01a\t\xF0V[`\0a\x1C^\x83a\x15\x87V[\x84T\x90\x91P`\0a\x1Cq\x86\x84\x83\x85a*\xD8V[\x90P\x80\x15a\x1C\x9EWa\x1C\x88\x86a\x15c`\x01\x84a7\xC6V[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x1C\xA1V[`\0[`\x01`\x01`\xE0\x1B\x03\x16\x96\x95PPPPPPV[`\x04T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xC5e\xB0E@=\xC0<.\xEA\x82\xB8\x1A\x04e\xED\xAD\x9E.\x7F\xC4\xD9~\x11B\x1C \x9D\xA9=z\x93\x91\x01`@Q\x80\x91\x03\x90\xA1`\x04UV[`\0a\x1C\xFFa\x11\xE9V[a\x1D\x0E3a\x07\xE6`\x01Ca7\xC6V[\x10\x15a\x1DvW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FGovernor: proposer votes below p`D\x82\x01Rp\x1C\x9B\xDC\x1B\xDC\xD8[\x08\x1D\x1A\x1C\x99\\\xDA\x1B\xDB\x19`z\x1B`d\x82\x01R`\x84\x01a\t\xF0V[`\0a\x1D\x8B\x86\x86\x86\x86\x80Q\x90` \x01 a\x132V[\x90P\x84Q\x86Q\x14a\x1D\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x90a8\xE7V[\x83Q\x86Q\x14a\x1D\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x90a8\xE7V[`\0\x86Q\x11a\x1E W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FGovernor: empty proposal\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xF0V[`\0\x81\x81R`\x01` \x90\x81R`@\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R\x81T`\x01`\x01`@\x1B\x03\x16\x90\x81\x90R\x15a\x1E\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FGovernor: proposal already exist`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\t\xF0V[`\0a\x1E\xB2a\x1E\xADa\x0E\x01V[a+6V[a\x1E\xBBCa+6V[a\x1E\xC5\x91\x90a9(V[\x90P`\0a\x1E\xD4a\x1E\xADa\x08\xB5V[a\x1E\xDE\x90\x83a9(V[\x83Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x84\x16\x17\x84U\x90P`\x01\x83\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17\x90U\x7F}\x84\xA6&:\xE0\xD9\x8D3)\xBD{F\xBBN\x8Do\x98\xCD5\xA7\xAD\xB4\\'L\x8B\x7F\xD5\xEB\xD5\xE0\x843\x8B\x8B\x8DQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1FVWa\x1FVa,\x9BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1F\x89W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1FtW\x90P[P\x8C\x88\x88\x8E`@Qa\x1F\xA3\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a9SV[`@Q\x80\x91\x03\x90\xA1P\x91\x97\x96PPPPPPPV[`@Qc\x07H\xD65`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c:F\xB1\xA8\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a *W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x94\x91\x90a6\x10V[`\nT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\x08\xF7N\xA4n\xF7\x89Oe\xEA\xBF\xB5\xE6\xE6\x95\xDEw:\0\x0BG\xC5)\xABU\x91x\x06\x9B\"d\x01\x91\x01`@Q\x80\x91\x03\x90\xA1`\n\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``\x83\x15a \xC6WP\x81a\x10\x97V[a\x10\x97\x83\x83a+\x9EV[`\0\x81\x11a!0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FGovernorSettings: voting period `D\x82\x01Rftoo low`\xC8\x1B`d\x82\x01R`\x84\x01a\t\xF0V[`\x05T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F~?\x7F\x07\x08\xA8M\xE9 06\xAB\xAAE\r\xCC\xC8Z\xD5\xFFR\xF7\x8C\x17\x0F>\xDBU\xCF^\x88(\x91\x01`@Q\x80\x91\x03\x90\xA1`\x05UV[`\x06T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xCC\xB4]\xA8\xD5q~lEDiB\x97\xC4\xBA\\\xF1Q\xD4U\xC9\xBB\x0E\xD4\xFCz8A\x1B\xC0Ta\x91\x01`@Q\x80\x91\x03\x90\xA1`\x06UV[`\0`da!\xBF\x83a\x0F\0V[`@Qc#\x94\xE7\xA3`\xE2\x1B\x81R`\x04\x81\x01\x85\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x8ES\x9E\x8C\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"H\x91\x90a6\x10V[a\"R\x91\x90a:BV[a\x08\xAF\x91\x90a:aV[\x82T`\0\x90\x81\x90\x80\x15a#\xA2W`\0a\"z\x87a\x15c`\x01\x85a7\xC6V[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a\"\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCheckpoint: invalid key\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xF0V[\x80Qc\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03a#CW\x84a#\x1C\x88a\x15c`\x01\x86a7\xC6V[\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16`\x01` \x1B\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua#\x92V[`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU`\0\x8D\x81R\x91\x90\x91 \x94Q\x91Q\x90\x92\x16`\x01` \x1B\x02\x92\x16\x91\x90\x91\x17\x91\x01U[` \x01Q\x92P\x83\x91Pa#\xF7\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU`\0\x8A\x81R\x91\x82 \x95Q\x92Q\x90\x93\x16`\x01` \x1B\x02\x91\x90\x93\x16\x17\x92\x01\x91\x90\x91U\x90P\x81[\x93P\x93\x91PPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\xBF&\xD8\x97`\xE0\x1B\x14\x80a$0WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cy\xDDyo`\xE0\x1B\x14[\x80a$KWP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x02q\x18\x97`\xE5\x1B\x14[\x80a\x08\xAFWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x08\xAFV[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a$\xC3WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a$\xEDWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[P`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01RF`\x80\x83\x01R0`\xA0\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a%\xC8WP`\0\x90P`\x03a&LV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a&\x1CW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a&EW`\0`\x01\x92P\x92PPa&LV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81`\x04\x81\x11\x15a&iWa&ia1EV[\x03a&qWPV[`\x01\x81`\x04\x81\x11\x15a&\x85Wa&\x85a1EV[\x03a&\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xF0V[`\x02\x81`\x04\x81\x11\x15a&\xE6Wa&\xE6a1EV[\x03a'3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\t\xF0V[`\x03\x81`\x04\x81\x11\x15a'GWa'Ga1EV[\x03a\nHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\t\xF0V[`\0\x85\x81R`\x07` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x88\x16\x84R`\x03\x81\x01\x90\x92R\x90\x91 T`\xFF\x16\x15a('W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FGovernorVotingSimple: vote alrea`D\x82\x01Rf\x19\x1EH\x18\xD8\\\xDD`\xCA\x1B`d\x82\x01R`\x84\x01a\t\xF0V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x03\x82\x01` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\xFF\x84\x16a(sW\x82\x81`\0\x01`\0\x82\x82Ta(h\x91\x90a7\xAEV[\x90\x91UPa\x1AF\x90PV[`\0\x19`\xFF\x85\x16\x01a(\x93W\x82\x81`\x01\x01`\0\x82\x82Ta(h\x91\x90a7\xAEV[`\x01\x19`\xFF\x85\x16\x01a(\xB3W\x82\x81`\x02\x01`\0\x82\x82Ta(h\x91\x90a7\xAEV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FGovernorVotingSimple: invalid va`D\x82\x01Rtlue for enum VoteType`X\x1B`d\x82\x01R`\x84\x01a\t\xF0V[\x81T`\x01`\x80\x1B\x90\x81\x90\x04`\x0F\x0B`\0\x81\x81R`\x01\x80\x86\x01` R`@\x90\x91 \x93\x90\x93U\x83T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x90\x91\x01\x16\x02\x17\x90UV[`\nT`@Qc\xE3\x835\xE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE3\x835\xE5\x904\x90a)\x90\x90\x88\x90\x88\x90\x88\x90`\0\x90\x89\x90`\x04\x01a6\xF2V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a)\xA9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a)\xBDW=`\0\x80>=`\0\xFD[PPPPPPPPPPV[`\0\x81\x81R`\x01` R`@\x81 `\x02\x81\x01T`\xFF\x16\x15a)\xEDWP`\x07\x92\x91PPV[`\x02\x81\x01Ta\x01\0\x90\x04`\xFF\x16\x15a*\x08WP`\x02\x92\x91PPV[`\0a*\x13\x84a\r\xCAV[\x90P\x80`\0\x03a*eW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FGovernor: unknown proposal id\0\0\0`D\x82\x01R`d\x01a\t\xF0V[C\x81\x10a*vWP`\0\x93\x92PPPV[`\0a*\x81\x85a\x11\xF4V[\x90PC\x81\x10a*\x95WP`\x01\x94\x93PPPPV[a*\x9E\x85a+\xC8V[\x80\x15a*\xBDWP`\0\x85\x81R`\x07` R`@\x90 \x80T`\x01\x90\x91\x01T\x11[\x15a*\xCDWP`\x04\x94\x93PPPPV[P`\x03\x94\x93PPPPV[`\0[\x81\x83\x10\x15a+.W`\0a*\xEF\x84\x84a+\xFFV[`\0\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a+\x1AW\x80\x92Pa+(V[a+%\x81`\x01a7\xAEV[\x93P[Pa*\xDBV[P\x93\x92PPPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x15\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 6`D\x82\x01Re4 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\t\xF0V[\x81Q\x15a+\xAEW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x91\x90a.dV[`\0\x81\x81R`\x07` R`@\x81 `\x02\x81\x01T`\x01\x82\x01Ta+\xEA\x91\x90a7\xAEV[a+\xF6a\x08R\x85a\r\xCAV[\x11\x15\x93\x92PPPV[`\0a,\x0E`\x02\x84\x84\x18a:aV[a\x10\x97\x90\x84\x84\x16a7\xAEV[`\0` \x82\x84\x03\x12\x15a,,W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x10\x97W`\0\x80\xFD[\x805`\xFF\x81\x16\x81\x14a,UW`\0\x80\xFD[\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a,lW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a,\x83W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x15BW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a,\xD9Wa,\xD9a,\x9BV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x83\x11\x15a,\xFAWa,\xFAa,\x9BV[a-\r`\x1F\x84\x01`\x1F\x19\x16` \x01a,\xB1V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a-!W`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a-IW`\0\x80\xFD[a\x10\x97\x83\x835` \x85\x01a,\xE1V[`\0\x80`\0\x80`\0\x80`\0\x80`\xE0\x89\x8B\x03\x12\x15a-tW`\0\x80\xFD[\x885\x97Pa-\x84` \x8A\x01a,DV[\x96P`@\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a-\xA0W`\0\x80\xFD[a-\xAC\x8C\x83\x8D\x01a,ZV[\x90\x98P\x96P``\x8B\x015\x91P\x80\x82\x11\x15a-\xC5W`\0\x80\xFD[Pa-\xD2\x8B\x82\x8C\x01a-8V[\x94PPa-\xE1`\x80\x8A\x01a,DV[\x92P`\xA0\x89\x015\x91P`\xC0\x89\x015\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0` \x82\x84\x03\x12\x15a.\x10W`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a.=W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a.!V[\x81\x81\x11\x15a.OW`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x10\x97` \x83\x01\x84a.\x17V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\nHW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a.\xA2W`\0\x80\xFD[\x845a.\xAD\x81a.wV[\x93P` \x85\x015a.\xBD\x81a.wV[\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.\xDFW`\0\x80\xFD[a.\xEB\x87\x82\x88\x01a-8V[\x91PP\x92\x95\x91\x94P\x92PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a/\x10Wa/\x10a,\x9BV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a/+W`\0\x80\xFD[\x815` a/@a/;\x83a.\xF7V[a,\xB1V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a/_W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a/\x83W\x805a/v\x81a.wV[\x83R\x91\x83\x01\x91\x83\x01a/cV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a/\x9FW`\0\x80\xFD[\x815` a/\xAFa/;\x83a.\xF7V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a/\xCEW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a/\x83W\x805\x83R\x91\x83\x01\x91\x83\x01a/\xD2V[`\0\x82`\x1F\x83\x01\x12a/\xFAW`\0\x80\xFD[\x815` a0\na/;\x83a.\xF7V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a0)W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a/\x83W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a0LW`\0\x80\x81\xFD[a0Z\x89\x86\x83\x8B\x01\x01a-8V[\x84RP\x91\x83\x01\x91\x83\x01a0-V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a0~W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a0\x95W`\0\x80\xFD[a0\xA1\x88\x83\x89\x01a/\x1AV[\x95P` \x87\x015\x91P\x80\x82\x11\x15a0\xB7W`\0\x80\xFD[a0\xC3\x88\x83\x89\x01a/\x8EV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a0\xD9W`\0\x80\xFD[Pa0\xE6\x87\x82\x88\x01a/\xE9V[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a1\x0FW`\0\x80\xFD[\x855\x94Pa1\x1F` \x87\x01a,DV[\x93Pa1-`@\x87\x01a,DV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x08\x83\x10a1}WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0\x80`@\x83\x85\x03\x12\x15a1\x96W`\0\x80\xFD[\x825\x91P` \x83\x015a1\xA8\x81a.wV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a1\xC6W`\0\x80\xFD[\x825\x91Pa1\xD6` \x84\x01a,DV[\x90P\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a1\xF7W`\0\x80\xFD[\x855\x94Pa2\x07` \x87\x01a,DV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a2#W`\0\x80\xFD[a2/\x89\x83\x8A\x01a,ZV[\x90\x95P\x93P``\x88\x015\x91P\x80\x82\x11\x15a2HW`\0\x80\xFD[Pa2U\x88\x82\x89\x01a-8V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a2xW`\0\x80\xFD[\x845\x93Pa2\x88` \x86\x01a,DV[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xA3W`\0\x80\xFD[a2\xAF\x87\x82\x88\x01a,ZV[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a2\xD1W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a2\xE8W`\0\x80\xFD[a2\xF4\x88\x83\x89\x01a/\x1AV[\x95P` \x87\x015\x91P\x80\x82\x11\x15a3\nW`\0\x80\xFD[a3\x16\x88\x83\x89\x01a/\x8EV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a3,W`\0\x80\xFD[a38\x88\x83\x89\x01a/\xE9V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a3NW`\0\x80\xFD[P\x85\x01`\x1F\x81\x01\x87\x13a3`W`\0\x80\xFD[a.\xEB\x87\x825` \x84\x01a,\xE1V[`\0\x80`\0``\x84\x86\x03\x12\x15a3\x84W`\0\x80\xFD[\x835a3\x8F\x81a.wV[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\xB1W`\0\x80\xFD[a3\xBD\x86\x82\x87\x01a-8V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a3\xD9W`\0\x80\xFD[\x815a\x10\x97\x81a.wV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a3\xFCW`\0\x80\xFD[\x855a4\x07\x81a.wV[\x94P` \x86\x015a4\x17\x81a.wV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a43W`\0\x80\xFD[a4?\x89\x83\x8A\x01a/\x8EV[\x94P``\x88\x015\x91P\x80\x82\x11\x15a4UW`\0\x80\xFD[a4a\x89\x83\x8A\x01a/\x8EV[\x93P`\x80\x88\x015\x91P\x80\x82\x11\x15a2HW`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a4\x8DW`\0\x80\xFD[\x845a4\x98\x81a.wV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xA3W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a4\xCDW`\0\x80\xFD[\x825a4\xD8\x81a.wV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a4\xFEW`\0\x80\xFD[\x855a5\t\x81a.wV[\x94P` \x86\x015a5\x19\x81a.wV[\x93P`@\x86\x015\x92P``\x86\x015\x91P`\x80\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a5BW`\0\x80\xFD[a2U\x88\x82\x89\x01a-8V[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[` \x80\x82R`\x18\x90\x82\x01R\x7FGovernor: onlyGovernance\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a5\xA9W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a5\xC9WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x80\x82R`!\x90\x82\x01R\x7FGovernor: proposal not successfu`@\x82\x01R`\x1B`\xFA\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a6\"W`\0\x80\xFD[PQ\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a6bW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a6=V[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a6bW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a6\x81V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0[\x85\x81\x10\x15a6\xE5W\x82\x84\x03\x89Ra6\xD3\x84\x83Qa.\x17V[\x98\x85\x01\x98\x93P\x90\x84\x01\x90`\x01\x01a6\xBBV[P\x91\x97\x96PPPPPPPV[`\xA0\x81R`\0a7\x05`\xA0\x83\x01\x88a6)V[\x82\x81\x03` \x84\x01Ra7\x17\x81\x88a6mV[\x90P\x82\x81\x03`@\x84\x01Ra7+\x81\x87a6\x9DV[``\x84\x01\x95\x90\x95RPP`\x80\x01R\x93\x92PPPV[`\xC0\x81R`\0a7S`\xC0\x83\x01\x89a6)V[\x82\x81\x03` \x84\x01Ra7e\x81\x89a6mV[\x90P\x82\x81\x03`@\x84\x01Ra7y\x81\x88a6\x9DV[``\x84\x01\x96\x90\x96RPP`\x80\x81\x01\x92\x90\x92R`\xA0\x90\x91\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a7\xC1Wa7\xC1a7\x98V[P\x01\x90V[`\0\x82\x82\x10\x15a7\xD8Wa7\xD8a7\x98V[P\x03\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x80\x81R`\0a8\x06`\x80\x83\x01\x87a6)V[\x82\x81\x03` \x84\x01Ra8\x18\x81\x87a6mV[\x90P\x82\x81\x03`@\x84\x01Ra8,\x81\x86a6\x9DV[\x91PP\x82``\x83\x01R\x95\x94PPPPPV[\x84\x81R`\xFF\x84\x16` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x10f`\x80\x83\x01\x84a.\x17V[\x85\x81R`\xFF\x85\x16` \x82\x01R\x83`@\x82\x01R`\xA0``\x82\x01R`\0a8\x8E`\xA0\x83\x01\x85a.\x17V[\x82\x81\x03`\x80\x84\x01Ra8\xA0\x81\x85a.\x17V[\x98\x97PPPPPPPPV[`\0`\x01\x82\x01a8\xBEWa8\xBEa7\x98V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a8\xD7W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x10\x97W`\0\x80\xFD[` \x80\x82R`!\x90\x82\x01R\x7FGovernor: invalid proposal lengt`@\x82\x01R`\r`\xFB\x1B``\x82\x01R`\x80\x01\x90V[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a9JWa9Ja7\x98V[\x01\x94\x93PPPPV[`\0a\x01 \x8B\x83R` `\x01\x80`\xA0\x1B\x03\x8C\x16\x81\x85\x01R\x81`@\x85\x01Ra9|\x82\x85\x01\x8Ca6)V[\x91P\x83\x82\x03``\x85\x01Ra9\x90\x82\x8Ba6mV[\x91P\x83\x82\x03`\x80\x85\x01R\x81\x89Q\x80\x84R\x82\x84\x01\x91P\x82\x81`\x05\x1B\x85\x01\x01\x83\x8C\x01`\0[\x83\x81\x10\x15a9\xE1W`\x1F\x19\x87\x84\x03\x01\x85Ra9\xCF\x83\x83Qa.\x17V[\x94\x86\x01\x94\x92P\x90\x85\x01\x90`\x01\x01a9\xB3V[PP\x86\x81\x03`\xA0\x88\x01Ra9\xF5\x81\x8Ca6\x9DV[\x94PPPPPa:\x10`\xC0\x84\x01\x87`\x01`\x01`@\x1B\x03\x16\x90RV[`\x01`\x01`@\x1B\x03\x85\x16`\xE0\x84\x01R\x82\x81\x03a\x01\0\x84\x01Ra:2\x81\x85a.\x17V[\x9C\x9BPPPPPPPPPPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a:\\Wa:\\a7\x98V[P\x02\x90V[`\0\x82a:~WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFEGovernor: relay reverted without message\xA2dipfsX\"\x12 \xF0M\x89qk\xEE\x85\xE3bg\x0BM\x84\x9F\xBA[\xD8\x16\xBF\xFE\x1A\x98\xCD13\xD5\xF8A\"J\xEE\x9CdsolcC\0\x08\x0F\x003";
    /// The bytecode of the contract.
    pub static ALONGSIDEGOVERNOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x02JW`\x005`\xE0\x1C\x80c{<q\xD3\x11a\x019W\x80c\xC2\x8B\xC2\xFA\x11a\0\xB6W\x80c\xEA\x02\x17\xCF\x11a\0zW\x80c\xEA\x02\x17\xCF\x14a\x07\xABW\x80c\xEB\x90\x19\xD4\x14a\x07\xCBW\x80c\xEC\xE4\x0C\xC1\x14a\x07\xEBW\x80c\xF2:na\x14a\x08\x0BW\x80c\xF8\xCEV\n\x14a\x087W\x80c\xFC\x0CTj\x14a\x08WW`\0\x80\xFD[\x80c\xC2\x8B\xC2\xFA\x14a\x06\xCCW\x80c\xC5\x90W\xE4\x14a\x06\xDFW\x80c\xD32\x19\xB4\x14a\x06\xFFW\x80c\xDDN+\xA5\x14a\x071W\x80c\xDE\xAA\xA7\xCC\x14a\x07wW`\0\x80\xFD[\x80c\xA8\x90\xC9\x10\x11a\0\xFDW\x80c\xA8\x90\xC9\x10\x14a\x06+W\x80c\xABX\xFB\x8E\x14a\x06KW\x80c\xB5\x811\xB0\x14a\x06kW\x80c\xBC\x19|\x81\x14a\x06\x80W\x80c\xC0\x1F\x9E7\x14a\x06\xACW`\0\x80\xFD[\x80c{<q\xD3\x14a\x05\xA2W\x80c}^\x81\xE2\x14a\x05\xC2W\x80c\x97\xC3\xD34\x14a\x05\xE2W\x80c\x9A\x80*m\x14a\x05\xF6W\x80c\xA7q:p\x14a\x06\x16W`\0\x80\xFD[\x80c92\xAB\xB1\x11a\x01\xC7W\x80cT\xFDMP\x11a\x01\x8BW\x80cT\xFDMP\x14a\x04\xF8W\x80cVx\x13\x88\x14a\x05\"W\x80c_9\x8A\x14\x14a\x05BW\x80c`\xC4$\x7F\x14a\x05bW\x80cp\xB0\xF6`\x14a\x05\x82W`\0\x80\xFD[\x80c92\xAB\xB1\x14a\x03\xF7W\x80c;\xCC\xF4\xFD\x14a\x04\x0CW\x80c>OI\xE6\x14a\x04,W\x80cC\x85\x962\x14a\x04YW\x80cTO\xFC\x9C\x14a\x04\xA3W`\0\x80\xFD[\x80c\x15\x0Bz\x02\x11a\x02\x0EW\x80c\x15\x0Bz\x02\x14a\x03,W\x80c\x16\x0C\xBE\xD7\x14a\x03pW\x80c&V\"}\x14a\x03\x90W\x80c-c\xF6\x93\x14a\x03\xA3W\x80c/\xE3\xE2a\x14a\x03\xC3W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x02rW\x80c\x02\xA2Q\xA3\x14a\x02\xA7W\x80c\x03B\x01\x81\x14a\x02\xCAW\x80c\x06\xF3\xF9\xE6\x14a\x02\xEAW\x80c\x06\xFD\xDE\x03\x14a\x03\nW`\0\x80\xFD[6a\x02mW0a\x02Xa\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x02kW`\0\x80\xFD[\0[`\0\x80\xFD[4\x80\x15a\x02~W`\0\x80\xFD[Pa\x02\x92a\x02\x8D6`\x04a,\x1AV[a\x08\xA4V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xB3W`\0\x80\xFD[Pa\x02\xBCa\x08\xB5V[`@Q\x90\x81R` \x01a\x02\x9EV[4\x80\x15a\x02\xD6W`\0\x80\xFD[Pa\x02\xBCa\x02\xE56`\x04a-XV[a\x08\xC0V[4\x80\x15a\x02\xF6W`\0\x80\xFD[Pa\x02ka\x03\x056`\x04a-\xFEV[a\t\xB8V[4\x80\x15a\x03\x16W`\0\x80\xFD[Pa\x03\x1Fa\nKV[`@Qa\x02\x9E\x91\x90a.dV[4\x80\x15a\x038W`\0\x80\xFD[Pa\x03Wa\x03G6`\x04a.\x8CV[c\n\x85\xBD\x01`\xE1\x1B\x94\x93PPPPV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x02\x9EV[4\x80\x15a\x03|W`\0\x80\xFD[Pa\x02\xBCa\x03\x8B6`\x04a0hV[a\n\xDDV[a\x02\xBCa\x03\x9E6`\x04a0hV[a\x0C\xDDV[4\x80\x15a\x03\xAFW`\0\x80\xFD[Pa\x02\xBCa\x03\xBE6`\x04a-\xFEV[a\r\xCAV[4\x80\x15a\x03\xCFW`\0\x80\xFD[Pa\x02\xBC\x7F\xB3\xB3\xF3\xB7\x03\xCD\x84\xCE5!\x97\xDC\xFF#+\x1B]<\xFB %\xCEG\xCF\x04t-\x06Q\xF1\xAF\x88\x81V[4\x80\x15a\x04\x03W`\0\x80\xFD[Pa\x02\xBCa\x0E\x01V[4\x80\x15a\x04\x18W`\0\x80\xFD[Pa\x02\xBCa\x04'6`\x04a0\xF7V[a\x0E\x0CV[4\x80\x15a\x048W`\0\x80\xFD[Pa\x04La\x04G6`\x04a-\xFEV[a\x0E\x82V[`@Qa\x02\x9E\x91\x90a1[V[4\x80\x15a\x04eW`\0\x80\xFD[Pa\x02\x92a\x04t6`\x04a1\x83V[`\0\x82\x81R`\x07` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R`\x03\x01\x90\x91R\x90 T`\xFF\x16\x92\x91PPV[4\x80\x15a\x04\xAFW`\0\x80\xFD[Pa\x04\xDDa\x04\xBE6`\x04a-\xFEV[`\0\x90\x81R`\x07` R`@\x90 \x80T`\x01\x82\x01T`\x02\x90\x92\x01T\x90\x92V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\x9EV[4\x80\x15a\x05\x04W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`1`\xF8\x1B` \x82\x01Ra\x03\x1FV[4\x80\x15a\x05.W`\0\x80\xFD[Pa\x02\xBCa\x05=6`\x04a1\xB3V[a\x0E\x8DV[4\x80\x15a\x05NW`\0\x80\xFD[Pa\x02\xBCa\x05]6`\x04a1\xDFV[a\x0E\xB6V[4\x80\x15a\x05nW`\0\x80\xFD[Pa\x02\xBCa\x05}6`\x04a-\xFEV[a\x0F\0V[4\x80\x15a\x05\x8EW`\0\x80\xFD[Pa\x02ka\x05\x9D6`\x04a-\xFEV[a\x0F\x97V[4\x80\x15a\x05\xAEW`\0\x80\xFD[Pa\x02\xBCa\x05\xBD6`\x04a2bV[a\x10\x1EV[4\x80\x15a\x05\xCEW`\0\x80\xFD[Pa\x02\xBCa\x05\xDD6`\x04a2\xBBV[a\x10pV[4\x80\x15a\x05\xEEW`\0\x80\xFD[P`da\x02\xBCV[4\x80\x15a\x06\x02W`\0\x80\xFD[Pa\x02\xBCa\x06\x116`\x04a3oV[a\x10\x87V[4\x80\x15a\x06\"W`\0\x80\xFD[Pa\x02\xBCa\x10\x9EV[4\x80\x15a\x067W`\0\x80\xFD[Pa\x02ka\x06F6`\x04a3\xC7V[a\x10\xC8V[4\x80\x15a\x06WW`\0\x80\xFD[Pa\x02\xBCa\x06f6`\x04a-\xFEV[a\x11OV[4\x80\x15a\x06wW`\0\x80\xFD[Pa\x02\xBCa\x11\xE9V[4\x80\x15a\x06\x8CW`\0\x80\xFD[Pa\x03Wa\x06\x9B6`\x04a3\xE4V[c\xBC\x19|\x81`\xE0\x1B\x95\x94PPPPPV[4\x80\x15a\x06\xB8W`\0\x80\xFD[Pa\x02\xBCa\x06\xC76`\x04a-\xFEV[a\x11\xF4V[a\x02ka\x06\xDA6`\x04a4wV[a\x12#V[4\x80\x15a\x06\xEBW`\0\x80\xFD[Pa\x02\xBCa\x06\xFA6`\x04a0hV[a\x132V[4\x80\x15a\x07\x0BW`\0\x80\xFD[P`\nT`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x9EV[4\x80\x15a\x07=W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R` \x80\x82R\x7Fsupport=bravo&quorum=for,abstain\x90\x82\x01Ra\x03\x1FV[4\x80\x15a\x07\x83W`\0\x80\xFD[Pa\x02\xBC\x7F\x15\x02\x14\xD7MY\xB7\xD1\xE9\x0Cs\xFC\"\xEF=\x99\x1D\xD0\xA7k\x04eC\xD4\xD8\n\xB9-*P2\x8F\x81V[4\x80\x15a\x07\xB7W`\0\x80\xFD[Pa\x02ka\x07\xC66`\x04a-\xFEV[a\x13lV[4\x80\x15a\x07\xD7W`\0\x80\xFD[Pa\x02\xBCa\x07\xE66`\x04a4\xBAV[a\x13\xF3V[4\x80\x15a\x07\xF7W`\0\x80\xFD[Pa\x02ka\x08\x066`\x04a-\xFEV[a\x14\x14V[4\x80\x15a\x08\x17W`\0\x80\xFD[Pa\x03Wa\x08&6`\x04a4\xE6V[c\xF2:na`\xE0\x1B\x95\x94PPPPPV[4\x80\x15a\x08CW`\0\x80\xFD[Pa\x02\xBCa\x08R6`\x04a-\xFEV[a\x14\x9BV[4\x80\x15a\x08cW`\0\x80\xFD[Pa\x07\x19\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0a\x08\x9F`\nT`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90P\x90V[`\0a\x08\xAF\x82a\x15\xECV[\x92\x91PPV[`\0a\x08\x9F`\x05T\x90V[`\0\x80a\tda\t\\\x7F\xB3\xB3\xF3\xB7\x03\xCD\x84\xCE5!\x97\xDC\xFF#+\x1B]<\xFB %\xCEG\xCF\x04t-\x06Q\xF1\xAF\x88\x8C\x8C\x8C\x8C`@Qa\x08\xFC\x92\x91\x90a5NV[`@Q\x80\x91\x03\x90 \x8B\x80Q\x90` \x01 `@Q` \x01a\tA\x95\x94\x93\x92\x91\x90\x94\x85R` \x85\x01\x93\x90\x93R`\xFF\x91\x90\x91\x16`@\x84\x01R``\x83\x01R`\x80\x82\x01R`\xA0\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x16\x11V[\x86\x86\x86a\x16_V[\x90Pa\t\xAA\x8A\x82\x8B\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8D\x92Pa\x16}\x91PPV[\x9A\x99PPPPPPPPPPV[a\t\xC0a\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x90a5^V[`@Q\x80\x91\x03\x90\xFD[0a\n\x02a\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\n?W`\0\x806`@Qa\n\"\x92\x91\x90a5NV[`@Q\x80\x91\x03\x90 \x90P[\x80a\n8`\x02a\x17\xE2V[\x03a\n-WP[a\nH\x81a\x18aV[PV[```\0\x80Ta\nZ\x90a5\x95V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\x86\x90a5\x95V[\x80\x15a\n\xD3W\x80`\x1F\x10a\n\xA8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\xD3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\xB6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0\x80a\n\xEC\x86\x86\x86\x86a\x132V[\x90P`\x04a\n\xF9\x82a\x0E\x82V[`\x07\x81\x11\x15a\x0B\nWa\x0B\na1EV[\x14a\x0B'W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x90a5\xCFV[`\nT`@\x80Qcy=\x06I`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xF2z\x0C\x92\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0BqW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x95\x91\x90a6\x10V[`\nT`@Qc\xB1\xC5\xF4'`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB1\xC5\xF4'\x90a\x0B\xCF\x90\x8A\x90\x8A\x90\x8A\x90`\0\x90\x8B\x90`\x04\x01a6\xF2V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xECW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x10\x91\x90a6\x10V[`\0\x83\x81R`\x0B` R`@\x80\x82 \x92\x90\x92U`\nT\x91Qc\x08\xF2\xA0\xBB`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x8F*\x0B\xB0\x91a\x0C[\x91\x8B\x91\x8B\x91\x8B\x91\x90\x8B\x90\x89\x90`\x04\x01a7@V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0CuW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0C\x89W=`\0\x80>=`\0\xFD[PPPP\x7F\x9A.B\xFDg\"\x81=i\x11>}\0y\xD3\xD9@\x17\x14(\xDFss\xDF\x9C\x7Fv\x17\xCF\xDA(\x92\x82\x82Ba\x0C\xBB\x91\x90a7\xAEV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01`@Q\x80\x91\x03\x90\xA1P\x95\x94PPPPPV[`\0\x80a\x0C\xEC\x86\x86\x86\x86a\x132V[\x90P`\0a\x0C\xF9\x82a\x0E\x82V[\x90P`\x04\x81`\x07\x81\x11\x15a\r\x0FWa\r\x0Fa1EV[\x14\x80a\r,WP`\x05\x81`\x07\x81\x11\x15a\r*Wa\r*a1EV[\x14[a\rHW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x90a5\xCFV[`\0\x82\x81R`\x01` \x81\x81R`@\x92\x83\x90 `\x02\x01\x80T`\xFF\x19\x16\x90\x92\x17\x90\x91U\x90Q\x83\x81R\x7Fq*\xE18?y\xAC\x85?\x8D\x88!Sw\x8E\x02`\xEF\x8F\x03\xB5\x04\xE2\x86n\x05\x93\xE0M+)\x1F\x91\x01`@Q\x80\x91\x03\x90\xA1a\r\xA6\x82\x88\x88\x88\x88a\x19\xADV[a\r\xB3\x82\x88\x88\x88\x88a\x1AOV[a\r\xC0\x82\x88\x88\x88\x88a\x1A\\V[P\x95\x94PPPPPV[`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x81Q\x92\x83\x01\x90\x91RT`\x01`\x01`@\x1B\x03\x16\x90\x81\x90R[`\x01`\x01`@\x1B\x03\x16\x92\x91PPV[`\0a\x08\x9F`\x04T\x90V[`@\x80Q\x7F\x15\x02\x14\xD7MY\xB7\xD1\xE9\x0Cs\xFC\"\xEF=\x99\x1D\xD0\xA7k\x04eC\xD4\xD8\n\xB9-*P2\x8F` \x82\x01R\x90\x81\x01\x86\x90R`\xFF\x85\x16``\x82\x01R`\0\x90\x81\x90a\x0EZ\x90a\t\\\x90`\x80\x01a\tAV[\x90Pa\x0Ew\x87\x82\x88`@Q\x80` \x01`@R\x80`\0\x81RPa\x1A\x95V[\x97\x96PPPPPPPV[`\0a\x08\xAF\x82a\x1A\xB8V[`\0\x803\x90Pa\x0E\xAE\x84\x82\x85`@Q\x80` \x01`@R\x80`\0\x81RPa\x1A\x95V[\x94\x93PPPPV[`\0\x803\x90Pa\x0Ew\x87\x82\x88\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8A\x92Pa\x16}\x91PPV[`\tT`\0\x90\x80\x82\x03a\x0F\x17WPP`\x08T\x91\x90PV[`\0`\ta\x0F&`\x01\x84a7\xC6V[\x81T\x81\x10a\x0F6Wa\x0F6a7\xDDV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83R`\x01` \x1B\x90\x91\x04`\x01`\x01`\xE0\x1B\x03\x16\x92\x82\x01\x92\x90\x92R\x91P\x84\x10a\x0F\x8CW` \x01Q`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[a\x0E\xAE`\t\x85a\x1C\x02V[a\x0F\x9Fa\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x90a5^V[0a\x0F\xD8a\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10\x15W`\0\x806`@Qa\x0F\xF8\x92\x91\x90a5NV[`@Q\x80\x91\x03\x90 \x90P[\x80a\x10\x0E`\x02a\x17\xE2V[\x03a\x10\x03WP[a\nH\x81a\x1C\xB4V[`\0\x803\x90Pa\x10f\x86\x82\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1A\x95\x92PPPV[\x96\x95PPPPPPV[`\0a\x10~\x85\x85\x85\x85a\x1C\xF5V[\x95\x94PPPPPV[`\0a\x10\x94\x84\x84\x84a\x1F\xB8V[\x90P[\x93\x92PPPV[`\tT`\0\x90\x15a\x10\xC1Wa\x10\xB3`\ta\x15IV[`\x01`\x01`\xE0\x1B\x03\x16\x90P\x90V[P`\x08T\x90V[a\x10\xD0a\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11\0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x90a5^V[0a\x11\ta\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11FW`\0\x806`@Qa\x11)\x92\x91\x90a5NV[`@Q\x80\x91\x03\x90 \x90P[\x80a\x11?`\x02a\x17\xE2V[\x03a\x114WP[a\nH\x81a NV[`\nT`\0\x82\x81R`\x0B` R`@\x80\x82 T\x90Qc\xD4\\D5`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R\x90\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xD4\\D5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xD0\x91\x90a6\x10V[\x90P\x80`\x01\x14a\x11\xE0W\x80a\x10\x97V[`\0\x93\x92PPPV[`\0a\x08\x9F`\x06T\x90V[`\0\x81\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q\x92\x83\x01\x90\x91R\x90\x91\x01T`\x01`\x01`@\x1B\x03\x16\x90\x81\x90Ra\r\xF2V[a\x12+a\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x12[W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x90a5^V[0a\x12da\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x12\xA1W`\0\x806`@Qa\x12\x84\x92\x91\x90a5NV[`@Q\x80\x91\x03\x90 \x90P[\x80a\x12\x9A`\x02a\x17\xE2V[\x03a\x12\x8FWP[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85\x85\x85`@Qa\x12\xBF\x92\x91\x90a5NV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x12\xFCW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x13\x01V[``\x91P[P\x91P\x91Pa\x13)\x82\x82`@Q\x80``\x01`@R\x80`(\x81R` \x01a:\x84`(\x919a \xB7V[PPPPPPPV[`\0\x84\x84\x84\x84`@Q` \x01a\x13K\x94\x93\x92\x91\x90a7\xF3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[a\x13ta\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x13\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x90a5^V[0a\x13\xADa\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x13\xEAW`\0\x806`@Qa\x13\xCD\x92\x91\x90a5NV[`@Q\x80\x91\x03\x90 \x90P[\x80a\x13\xE3`\x02a\x17\xE2V[\x03a\x13\xD8WP[a\nH\x81a \xD0V[`\0a\x10\x97\x83\x83a\x14\x0F`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[a\x1F\xB8V[a\x14\x1Ca\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x14LW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x90a5^V[0a\x14Ua\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x14\x92W`\0\x806`@Qa\x14u\x92\x91\x90a5NV[`@Q\x80\x91\x03\x90 \x90P[\x80a\x14\x8B`\x02a\x17\xE2V[\x03a\x14\x80WP[a\nH\x81a!qV[`\0a\x08\xAF\x82a!\xB2V[`\0`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\x15\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\t\xF0V[P\x90V[`\0\x80a\x151\x84a\x15#Ca\x15\x87V[a\x15,\x86a\x14\xA6V[a\"\\V[`\x01`\x01`\xE0\x1B\x03\x91\x82\x16\x93P\x16\x90P[\x92P\x92\x90PV[\x80T`\0\x90\x80\x15a\x11\xE0Wa\x15q\x83a\x15c`\x01\x84a7\xC6V[`\0\x91\x82R` \x90\x91 \x01\x90V[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x10\x97V[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\x15\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\t\xF0V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cnf\\\xED`\xE0\x1B\x14\x80a\x08\xAFWPa\x08\xAF\x82a#\xFFV[`\0a\x08\xAFa\x16\x1Ea$jV[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\0\x80`\0a\x16p\x87\x87\x87\x87a%\x91V[\x91P\x91Pa\r\xC0\x81a&UV[`\0\x85\x81R`\x01` \x81\x90R`@\x82 \x90a\x16\x97\x88a\x0E\x82V[`\x07\x81\x11\x15a\x16\xA8Wa\x16\xA8a1EV[\x14a\x17\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FGovernor: vote not currently act`D\x82\x01Rbive`\xE8\x1B`d\x82\x01R`\x84\x01a\t\xF0V[`@\x80Q` \x81\x01\x90\x91R\x81T`\x01`\x01`@\x1B\x03\x16\x90\x81\x90R`\0\x90a\x17*\x90\x88\x90\x86a\x1F\xB8V[\x90Pa\x179\x88\x88\x88\x84\x88a'\x9FV[\x83Q`\0\x03a\x17\x8EW\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xB8\xE18\x88}\n\xA1;\xABD~\x82\xDE\x9D\\\x17w\x04\x1E\xCD!\xCA6\xBA\x82O\xF1\xE6\xC0}\xDD\xA4\x89\x88\x84\x89`@Qa\x17\x81\x94\x93\x92\x91\x90a8>V[`@Q\x80\x91\x03\x90\xA2a\x0EwV[\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xE2\xBA\xBF\xBA\xC5\x88\x9Ap\x9Bc\xBB\x7FY\x8B2N\x08\xBCZO\xB9\xECd\x7F\xB3\xCB\xC9\xEC\x07\xEB\x87\x12\x89\x88\x84\x89\x89`@Qa\x17\xCF\x95\x94\x93\x92\x91\x90a8fV[`@Q\x80\x91\x03\x90\xA2\x97\x96PPPPPPPV[`\0a\x17\xFD\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15a\x18\x1BW`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B`\0\x81\x81R`\x01\x80\x84\x01` R`@\x82 \x80T\x92\x90U\x83To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x92\x01`\x01`\x01`\x80\x1B\x03\x16\x91\x90\x91\x17\x90\x91U\x90V[`d\x81\x11\x15a\x18\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FGovernorVotesQuorumFraction: quo`D\x82\x01R\x7FrumNumerator over quorumDenomina`d\x82\x01Rb:7\xB9`\xE9\x1B`\x84\x82\x01R`\xA4\x01a\t\xF0V[`\0a\x18\xEEa\x10\x9EV[\x90P\x80\x15\x80\x15\x90a\x18\xFFWP`\tT\x15[\x15a\x19cW`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R`\t\x90` \x81\x01a\x19\"\x84a\x14\xA6V[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U`\0\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16`\x01` \x1B\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[a\x19n`\t\x83a\x15\x13V[PP`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\x05SGk\xF0.\xF2rn\x8C\xE5\xCE\xD7\x8Dc\xE2n`.J\"W\xB1\xF5YA\x8E$\xB4c9\x97\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[0a\x19\xB6a\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1AHW`\0[\x84Q\x81\x10\x15a\x1AFW0`\x01`\x01`\xA0\x1B\x03\x16\x85\x82\x81Q\x81\x10a\x19\xECWa\x19\xECa7\xDDV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1A6Wa\x1A6\x83\x82\x81Q\x81\x10a\x1A\x17Wa\x1A\x17a7\xDDV[` \x02` \x01\x01Q\x80Q\x90` \x01 `\x02a)\x19\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x1A?\x81a8\xACV[\x90Pa\x19\xC7V[P[PPPPPV[a\x1AH\x85\x85\x85\x85\x85a)UV[0a\x1Aea\x08\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1AHW`\x02T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15a\x1AHW`\0`\x02Ua\x1AHV[`\0a\x10~\x85\x85\x85\x85a\x1A\xB3`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[a\x16}V[`\0\x80a\x1A\xC4\x83a)\xC9V[\x90P`\x04\x81`\x07\x81\x11\x15a\x1A\xDAWa\x1A\xDAa1EV[\x14a\x1A\xE5W\x92\x91PPV[`\0\x83\x81R`\x0B` R`@\x90 T\x80a\x1B\0WP\x92\x91PPV[`\nT`@Qc*\xB0\xF5)`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c*\xB0\xF5)\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BIW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Bm\x91\x90a8\xC5V[\x15a\x1B|WP`\x07\x93\x92PPPV[`\nT`@Qc,%\x8A\x9F`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cXK\x15>\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xE9\x91\x90a8\xC5V[\x15a\x1B\xF8WP`\x05\x93\x92PPPV[P`\x02\x93\x92PPPV[`\0C\x82\x10a\x1CSW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FCheckpoints: block not yet mined`D\x82\x01R`d\x01a\t\xF0V[`\0a\x1C^\x83a\x15\x87V[\x84T\x90\x91P`\0a\x1Cq\x86\x84\x83\x85a*\xD8V[\x90P\x80\x15a\x1C\x9EWa\x1C\x88\x86a\x15c`\x01\x84a7\xC6V[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x1C\xA1V[`\0[`\x01`\x01`\xE0\x1B\x03\x16\x96\x95PPPPPPV[`\x04T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xC5e\xB0E@=\xC0<.\xEA\x82\xB8\x1A\x04e\xED\xAD\x9E.\x7F\xC4\xD9~\x11B\x1C \x9D\xA9=z\x93\x91\x01`@Q\x80\x91\x03\x90\xA1`\x04UV[`\0a\x1C\xFFa\x11\xE9V[a\x1D\x0E3a\x07\xE6`\x01Ca7\xC6V[\x10\x15a\x1DvW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FGovernor: proposer votes below p`D\x82\x01Rp\x1C\x9B\xDC\x1B\xDC\xD8[\x08\x1D\x1A\x1C\x99\\\xDA\x1B\xDB\x19`z\x1B`d\x82\x01R`\x84\x01a\t\xF0V[`\0a\x1D\x8B\x86\x86\x86\x86\x80Q\x90` \x01 a\x132V[\x90P\x84Q\x86Q\x14a\x1D\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x90a8\xE7V[\x83Q\x86Q\x14a\x1D\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x90a8\xE7V[`\0\x86Q\x11a\x1E W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FGovernor: empty proposal\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xF0V[`\0\x81\x81R`\x01` \x90\x81R`@\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R\x81T`\x01`\x01`@\x1B\x03\x16\x90\x81\x90R\x15a\x1E\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FGovernor: proposal already exist`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\t\xF0V[`\0a\x1E\xB2a\x1E\xADa\x0E\x01V[a+6V[a\x1E\xBBCa+6V[a\x1E\xC5\x91\x90a9(V[\x90P`\0a\x1E\xD4a\x1E\xADa\x08\xB5V[a\x1E\xDE\x90\x83a9(V[\x83Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x84\x16\x17\x84U\x90P`\x01\x83\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17\x90U\x7F}\x84\xA6&:\xE0\xD9\x8D3)\xBD{F\xBBN\x8Do\x98\xCD5\xA7\xAD\xB4\\'L\x8B\x7F\xD5\xEB\xD5\xE0\x843\x8B\x8B\x8DQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1FVWa\x1FVa,\x9BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1F\x89W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1FtW\x90P[P\x8C\x88\x88\x8E`@Qa\x1F\xA3\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a9SV[`@Q\x80\x91\x03\x90\xA1P\x91\x97\x96PPPPPPPV[`@Qc\x07H\xD65`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c:F\xB1\xA8\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a *W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x94\x91\x90a6\x10V[`\nT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\x08\xF7N\xA4n\xF7\x89Oe\xEA\xBF\xB5\xE6\xE6\x95\xDEw:\0\x0BG\xC5)\xABU\x91x\x06\x9B\"d\x01\x91\x01`@Q\x80\x91\x03\x90\xA1`\n\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``\x83\x15a \xC6WP\x81a\x10\x97V[a\x10\x97\x83\x83a+\x9EV[`\0\x81\x11a!0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FGovernorSettings: voting period `D\x82\x01Rftoo low`\xC8\x1B`d\x82\x01R`\x84\x01a\t\xF0V[`\x05T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F~?\x7F\x07\x08\xA8M\xE9 06\xAB\xAAE\r\xCC\xC8Z\xD5\xFFR\xF7\x8C\x17\x0F>\xDBU\xCF^\x88(\x91\x01`@Q\x80\x91\x03\x90\xA1`\x05UV[`\x06T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xCC\xB4]\xA8\xD5q~lEDiB\x97\xC4\xBA\\\xF1Q\xD4U\xC9\xBB\x0E\xD4\xFCz8A\x1B\xC0Ta\x91\x01`@Q\x80\x91\x03\x90\xA1`\x06UV[`\0`da!\xBF\x83a\x0F\0V[`@Qc#\x94\xE7\xA3`\xE2\x1B\x81R`\x04\x81\x01\x85\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x8ES\x9E\x8C\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"H\x91\x90a6\x10V[a\"R\x91\x90a:BV[a\x08\xAF\x91\x90a:aV[\x82T`\0\x90\x81\x90\x80\x15a#\xA2W`\0a\"z\x87a\x15c`\x01\x85a7\xC6V[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a\"\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCheckpoint: invalid key\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xF0V[\x80Qc\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03a#CW\x84a#\x1C\x88a\x15c`\x01\x86a7\xC6V[\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16`\x01` \x1B\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua#\x92V[`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU`\0\x8D\x81R\x91\x90\x91 \x94Q\x91Q\x90\x92\x16`\x01` \x1B\x02\x92\x16\x91\x90\x91\x17\x91\x01U[` \x01Q\x92P\x83\x91Pa#\xF7\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU`\0\x8A\x81R\x91\x82 \x95Q\x92Q\x90\x93\x16`\x01` \x1B\x02\x91\x90\x93\x16\x17\x92\x01\x91\x90\x91U\x90P\x81[\x93P\x93\x91PPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\xBF&\xD8\x97`\xE0\x1B\x14\x80a$0WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cy\xDDyo`\xE0\x1B\x14[\x80a$KWP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x02q\x18\x97`\xE5\x1B\x14[\x80a\x08\xAFWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x08\xAFV[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a$\xC3WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a$\xEDWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[P`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01RF`\x80\x83\x01R0`\xA0\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a%\xC8WP`\0\x90P`\x03a&LV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a&\x1CW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a&EW`\0`\x01\x92P\x92PPa&LV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81`\x04\x81\x11\x15a&iWa&ia1EV[\x03a&qWPV[`\x01\x81`\x04\x81\x11\x15a&\x85Wa&\x85a1EV[\x03a&\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xF0V[`\x02\x81`\x04\x81\x11\x15a&\xE6Wa&\xE6a1EV[\x03a'3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\t\xF0V[`\x03\x81`\x04\x81\x11\x15a'GWa'Ga1EV[\x03a\nHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\t\xF0V[`\0\x85\x81R`\x07` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x88\x16\x84R`\x03\x81\x01\x90\x92R\x90\x91 T`\xFF\x16\x15a('W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FGovernorVotingSimple: vote alrea`D\x82\x01Rf\x19\x1EH\x18\xD8\\\xDD`\xCA\x1B`d\x82\x01R`\x84\x01a\t\xF0V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x03\x82\x01` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\xFF\x84\x16a(sW\x82\x81`\0\x01`\0\x82\x82Ta(h\x91\x90a7\xAEV[\x90\x91UPa\x1AF\x90PV[`\0\x19`\xFF\x85\x16\x01a(\x93W\x82\x81`\x01\x01`\0\x82\x82Ta(h\x91\x90a7\xAEV[`\x01\x19`\xFF\x85\x16\x01a(\xB3W\x82\x81`\x02\x01`\0\x82\x82Ta(h\x91\x90a7\xAEV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FGovernorVotingSimple: invalid va`D\x82\x01Rtlue for enum VoteType`X\x1B`d\x82\x01R`\x84\x01a\t\xF0V[\x81T`\x01`\x80\x1B\x90\x81\x90\x04`\x0F\x0B`\0\x81\x81R`\x01\x80\x86\x01` R`@\x90\x91 \x93\x90\x93U\x83T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x90\x91\x01\x16\x02\x17\x90UV[`\nT`@Qc\xE3\x835\xE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE3\x835\xE5\x904\x90a)\x90\x90\x88\x90\x88\x90\x88\x90`\0\x90\x89\x90`\x04\x01a6\xF2V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a)\xA9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a)\xBDW=`\0\x80>=`\0\xFD[PPPPPPPPPPV[`\0\x81\x81R`\x01` R`@\x81 `\x02\x81\x01T`\xFF\x16\x15a)\xEDWP`\x07\x92\x91PPV[`\x02\x81\x01Ta\x01\0\x90\x04`\xFF\x16\x15a*\x08WP`\x02\x92\x91PPV[`\0a*\x13\x84a\r\xCAV[\x90P\x80`\0\x03a*eW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FGovernor: unknown proposal id\0\0\0`D\x82\x01R`d\x01a\t\xF0V[C\x81\x10a*vWP`\0\x93\x92PPPV[`\0a*\x81\x85a\x11\xF4V[\x90PC\x81\x10a*\x95WP`\x01\x94\x93PPPPV[a*\x9E\x85a+\xC8V[\x80\x15a*\xBDWP`\0\x85\x81R`\x07` R`@\x90 \x80T`\x01\x90\x91\x01T\x11[\x15a*\xCDWP`\x04\x94\x93PPPPV[P`\x03\x94\x93PPPPV[`\0[\x81\x83\x10\x15a+.W`\0a*\xEF\x84\x84a+\xFFV[`\0\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a+\x1AW\x80\x92Pa+(V[a+%\x81`\x01a7\xAEV[\x93P[Pa*\xDBV[P\x93\x92PPPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x15\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 6`D\x82\x01Re4 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\t\xF0V[\x81Q\x15a+\xAEW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xF0\x91\x90a.dV[`\0\x81\x81R`\x07` R`@\x81 `\x02\x81\x01T`\x01\x82\x01Ta+\xEA\x91\x90a7\xAEV[a+\xF6a\x08R\x85a\r\xCAV[\x11\x15\x93\x92PPPV[`\0a,\x0E`\x02\x84\x84\x18a:aV[a\x10\x97\x90\x84\x84\x16a7\xAEV[`\0` \x82\x84\x03\x12\x15a,,W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x10\x97W`\0\x80\xFD[\x805`\xFF\x81\x16\x81\x14a,UW`\0\x80\xFD[\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a,lW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a,\x83W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x15BW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a,\xD9Wa,\xD9a,\x9BV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x83\x11\x15a,\xFAWa,\xFAa,\x9BV[a-\r`\x1F\x84\x01`\x1F\x19\x16` \x01a,\xB1V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a-!W`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a-IW`\0\x80\xFD[a\x10\x97\x83\x835` \x85\x01a,\xE1V[`\0\x80`\0\x80`\0\x80`\0\x80`\xE0\x89\x8B\x03\x12\x15a-tW`\0\x80\xFD[\x885\x97Pa-\x84` \x8A\x01a,DV[\x96P`@\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a-\xA0W`\0\x80\xFD[a-\xAC\x8C\x83\x8D\x01a,ZV[\x90\x98P\x96P``\x8B\x015\x91P\x80\x82\x11\x15a-\xC5W`\0\x80\xFD[Pa-\xD2\x8B\x82\x8C\x01a-8V[\x94PPa-\xE1`\x80\x8A\x01a,DV[\x92P`\xA0\x89\x015\x91P`\xC0\x89\x015\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0` \x82\x84\x03\x12\x15a.\x10W`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a.=W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a.!V[\x81\x81\x11\x15a.OW`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x10\x97` \x83\x01\x84a.\x17V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\nHW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a.\xA2W`\0\x80\xFD[\x845a.\xAD\x81a.wV[\x93P` \x85\x015a.\xBD\x81a.wV[\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.\xDFW`\0\x80\xFD[a.\xEB\x87\x82\x88\x01a-8V[\x91PP\x92\x95\x91\x94P\x92PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a/\x10Wa/\x10a,\x9BV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a/+W`\0\x80\xFD[\x815` a/@a/;\x83a.\xF7V[a,\xB1V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a/_W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a/\x83W\x805a/v\x81a.wV[\x83R\x91\x83\x01\x91\x83\x01a/cV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a/\x9FW`\0\x80\xFD[\x815` a/\xAFa/;\x83a.\xF7V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a/\xCEW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a/\x83W\x805\x83R\x91\x83\x01\x91\x83\x01a/\xD2V[`\0\x82`\x1F\x83\x01\x12a/\xFAW`\0\x80\xFD[\x815` a0\na/;\x83a.\xF7V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a0)W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a/\x83W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a0LW`\0\x80\x81\xFD[a0Z\x89\x86\x83\x8B\x01\x01a-8V[\x84RP\x91\x83\x01\x91\x83\x01a0-V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a0~W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a0\x95W`\0\x80\xFD[a0\xA1\x88\x83\x89\x01a/\x1AV[\x95P` \x87\x015\x91P\x80\x82\x11\x15a0\xB7W`\0\x80\xFD[a0\xC3\x88\x83\x89\x01a/\x8EV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a0\xD9W`\0\x80\xFD[Pa0\xE6\x87\x82\x88\x01a/\xE9V[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a1\x0FW`\0\x80\xFD[\x855\x94Pa1\x1F` \x87\x01a,DV[\x93Pa1-`@\x87\x01a,DV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x08\x83\x10a1}WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0\x80`@\x83\x85\x03\x12\x15a1\x96W`\0\x80\xFD[\x825\x91P` \x83\x015a1\xA8\x81a.wV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a1\xC6W`\0\x80\xFD[\x825\x91Pa1\xD6` \x84\x01a,DV[\x90P\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a1\xF7W`\0\x80\xFD[\x855\x94Pa2\x07` \x87\x01a,DV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a2#W`\0\x80\xFD[a2/\x89\x83\x8A\x01a,ZV[\x90\x95P\x93P``\x88\x015\x91P\x80\x82\x11\x15a2HW`\0\x80\xFD[Pa2U\x88\x82\x89\x01a-8V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a2xW`\0\x80\xFD[\x845\x93Pa2\x88` \x86\x01a,DV[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xA3W`\0\x80\xFD[a2\xAF\x87\x82\x88\x01a,ZV[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a2\xD1W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a2\xE8W`\0\x80\xFD[a2\xF4\x88\x83\x89\x01a/\x1AV[\x95P` \x87\x015\x91P\x80\x82\x11\x15a3\nW`\0\x80\xFD[a3\x16\x88\x83\x89\x01a/\x8EV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a3,W`\0\x80\xFD[a38\x88\x83\x89\x01a/\xE9V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a3NW`\0\x80\xFD[P\x85\x01`\x1F\x81\x01\x87\x13a3`W`\0\x80\xFD[a.\xEB\x87\x825` \x84\x01a,\xE1V[`\0\x80`\0``\x84\x86\x03\x12\x15a3\x84W`\0\x80\xFD[\x835a3\x8F\x81a.wV[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\xB1W`\0\x80\xFD[a3\xBD\x86\x82\x87\x01a-8V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a3\xD9W`\0\x80\xFD[\x815a\x10\x97\x81a.wV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a3\xFCW`\0\x80\xFD[\x855a4\x07\x81a.wV[\x94P` \x86\x015a4\x17\x81a.wV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a43W`\0\x80\xFD[a4?\x89\x83\x8A\x01a/\x8EV[\x94P``\x88\x015\x91P\x80\x82\x11\x15a4UW`\0\x80\xFD[a4a\x89\x83\x8A\x01a/\x8EV[\x93P`\x80\x88\x015\x91P\x80\x82\x11\x15a2HW`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a4\x8DW`\0\x80\xFD[\x845a4\x98\x81a.wV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xA3W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a4\xCDW`\0\x80\xFD[\x825a4\xD8\x81a.wV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a4\xFEW`\0\x80\xFD[\x855a5\t\x81a.wV[\x94P` \x86\x015a5\x19\x81a.wV[\x93P`@\x86\x015\x92P``\x86\x015\x91P`\x80\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a5BW`\0\x80\xFD[a2U\x88\x82\x89\x01a-8V[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[` \x80\x82R`\x18\x90\x82\x01R\x7FGovernor: onlyGovernance\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a5\xA9W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a5\xC9WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x80\x82R`!\x90\x82\x01R\x7FGovernor: proposal not successfu`@\x82\x01R`\x1B`\xFA\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a6\"W`\0\x80\xFD[PQ\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a6bW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a6=V[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a6bW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a6\x81V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0[\x85\x81\x10\x15a6\xE5W\x82\x84\x03\x89Ra6\xD3\x84\x83Qa.\x17V[\x98\x85\x01\x98\x93P\x90\x84\x01\x90`\x01\x01a6\xBBV[P\x91\x97\x96PPPPPPPV[`\xA0\x81R`\0a7\x05`\xA0\x83\x01\x88a6)V[\x82\x81\x03` \x84\x01Ra7\x17\x81\x88a6mV[\x90P\x82\x81\x03`@\x84\x01Ra7+\x81\x87a6\x9DV[``\x84\x01\x95\x90\x95RPP`\x80\x01R\x93\x92PPPV[`\xC0\x81R`\0a7S`\xC0\x83\x01\x89a6)V[\x82\x81\x03` \x84\x01Ra7e\x81\x89a6mV[\x90P\x82\x81\x03`@\x84\x01Ra7y\x81\x88a6\x9DV[``\x84\x01\x96\x90\x96RPP`\x80\x81\x01\x92\x90\x92R`\xA0\x90\x91\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a7\xC1Wa7\xC1a7\x98V[P\x01\x90V[`\0\x82\x82\x10\x15a7\xD8Wa7\xD8a7\x98V[P\x03\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x80\x81R`\0a8\x06`\x80\x83\x01\x87a6)V[\x82\x81\x03` \x84\x01Ra8\x18\x81\x87a6mV[\x90P\x82\x81\x03`@\x84\x01Ra8,\x81\x86a6\x9DV[\x91PP\x82``\x83\x01R\x95\x94PPPPPV[\x84\x81R`\xFF\x84\x16` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x10f`\x80\x83\x01\x84a.\x17V[\x85\x81R`\xFF\x85\x16` \x82\x01R\x83`@\x82\x01R`\xA0``\x82\x01R`\0a8\x8E`\xA0\x83\x01\x85a.\x17V[\x82\x81\x03`\x80\x84\x01Ra8\xA0\x81\x85a.\x17V[\x98\x97PPPPPPPPV[`\0`\x01\x82\x01a8\xBEWa8\xBEa7\x98V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a8\xD7W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x10\x97W`\0\x80\xFD[` \x80\x82R`!\x90\x82\x01R\x7FGovernor: invalid proposal lengt`@\x82\x01R`\r`\xFB\x1B``\x82\x01R`\x80\x01\x90V[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a9JWa9Ja7\x98V[\x01\x94\x93PPPPV[`\0a\x01 \x8B\x83R` `\x01\x80`\xA0\x1B\x03\x8C\x16\x81\x85\x01R\x81`@\x85\x01Ra9|\x82\x85\x01\x8Ca6)V[\x91P\x83\x82\x03``\x85\x01Ra9\x90\x82\x8Ba6mV[\x91P\x83\x82\x03`\x80\x85\x01R\x81\x89Q\x80\x84R\x82\x84\x01\x91P\x82\x81`\x05\x1B\x85\x01\x01\x83\x8C\x01`\0[\x83\x81\x10\x15a9\xE1W`\x1F\x19\x87\x84\x03\x01\x85Ra9\xCF\x83\x83Qa.\x17V[\x94\x86\x01\x94\x92P\x90\x85\x01\x90`\x01\x01a9\xB3V[PP\x86\x81\x03`\xA0\x88\x01Ra9\xF5\x81\x8Ca6\x9DV[\x94PPPPPa:\x10`\xC0\x84\x01\x87`\x01`\x01`@\x1B\x03\x16\x90RV[`\x01`\x01`@\x1B\x03\x85\x16`\xE0\x84\x01R\x82\x81\x03a\x01\0\x84\x01Ra:2\x81\x85a.\x17V[\x9C\x9BPPPPPPPPPPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a:\\Wa:\\a7\x98V[P\x02\x90V[`\0\x82a:~WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFEGovernor: relay reverted without message\xA2dipfsX\"\x12 \xF0M\x89qk\xEE\x85\xE3bg\x0BM\x84\x9F\xBA[\xD8\x16\xBF\xFE\x1A\x98\xCD13\xD5\xF8A\"J\xEE\x9CdsolcC\0\x08\x0F\x003";
    /// The deployed bytecode of the contract.
    pub static ALONGSIDEGOVERNOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AlongsideGovernor<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AlongsideGovernor<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AlongsideGovernor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AlongsideGovernor<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AlongsideGovernor<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AlongsideGovernor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AlongsideGovernor<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ALONGSIDEGOVERNOR_ABI.clone(),
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
                ALONGSIDEGOVERNOR_ABI.clone(),
                ALONGSIDEGOVERNOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `BALLOT_TYPEHASH` (0xdeaaa7cc) function
        pub fn ballot_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([222, 170, 167, 204], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `COUNTING_MODE` (0xdd4e2ba5) function
        pub fn counting_mode(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([221, 78, 43, 165], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `EXTENDED_BALLOT_TYPEHASH` (0x2fe3e261) function
        pub fn extended_ballot_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([47, 227, 226, 97], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `castVote` (0x56781388) function
        pub fn cast_vote(
            &self,
            proposal_id: ::ethers::core::types::U256,
            support: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([86, 120, 19, 136], (proposal_id, support))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `castVoteBySig` (0x3bccf4fd) function
        pub fn cast_vote_by_sig(
            &self,
            proposal_id: ::ethers::core::types::U256,
            support: u8,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([59, 204, 244, 253], (proposal_id, support, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `castVoteWithReason` (0x7b3c71d3) function
        pub fn cast_vote_with_reason(
            &self,
            proposal_id: ::ethers::core::types::U256,
            support: u8,
            reason: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([123, 60, 113, 211], (proposal_id, support, reason))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `castVoteWithReasonAndParams` (0x5f398a14) function
        pub fn cast_vote_with_reason_and_params(
            &self,
            proposal_id: ::ethers::core::types::U256,
            support: u8,
            reason: ::std::string::String,
            params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([95, 57, 138, 20], (proposal_id, support, reason, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `castVoteWithReasonAndParamsBySig` (0x03420181) function
        pub fn cast_vote_with_reason_and_params_by_sig(
            &self,
            proposal_id: ::ethers::core::types::U256,
            support: u8,
            reason: ::std::string::String,
            params: ::ethers::core::types::Bytes,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [3, 66, 1, 129],
                    (proposal_id, support, reason, params, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execute` (0x2656227d) function
        pub fn execute(
            &self,
            targets: ::std::vec::Vec<::ethers::core::types::Address>,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
            calldatas: ::std::vec::Vec<::ethers::core::types::Bytes>,
            description_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [38, 86, 34, 125],
                    (targets, values, calldatas, description_hash),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVotes` (0xeb9019d4) function
        pub fn get_votes(
            &self,
            account: ::ethers::core::types::Address,
            block_number: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([235, 144, 25, 212], (account, block_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVotesWithParams` (0x9a802a6d) function
        pub fn get_votes_with_params(
            &self,
            account: ::ethers::core::types::Address,
            block_number: ::ethers::core::types::U256,
            params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([154, 128, 42, 109], (account, block_number, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasVoted` (0x43859632) function
        pub fn has_voted(
            &self,
            proposal_id: ::ethers::core::types::U256,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([67, 133, 150, 50], (proposal_id, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hashProposal` (0xc59057e4) function
        pub fn hash_proposal(
            &self,
            targets: ::std::vec::Vec<::ethers::core::types::Address>,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
            calldatas: ::std::vec::Vec<::ethers::core::types::Bytes>,
            description_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [197, 144, 87, 228],
                    (targets, values, calldatas, description_hash),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
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
        ///Calls the contract's `proposalDeadline` (0xc01f9e37) function
        pub fn proposal_deadline(
            &self,
            proposal_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([192, 31, 158, 55], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalEta` (0xab58fb8e) function
        pub fn proposal_eta(
            &self,
            proposal_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([171, 88, 251, 142], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalSnapshot` (0x2d63f693) function
        pub fn proposal_snapshot(
            &self,
            proposal_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([45, 99, 246, 147], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalThreshold` (0xb58131b0) function
        pub fn proposal_threshold(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([181, 129, 49, 176], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalVotes` (0x544ffc9c) function
        pub fn proposal_votes(
            &self,
            proposal_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([84, 79, 252, 156], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `propose` (0x7d5e81e2) function
        pub fn propose(
            &self,
            targets: ::std::vec::Vec<::ethers::core::types::Address>,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
            calldatas: ::std::vec::Vec<::ethers::core::types::Bytes>,
            description: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [125, 94, 129, 226],
                    (targets, values, calldatas, description),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queue` (0x160cbed7) function
        pub fn queue(
            &self,
            targets: ::std::vec::Vec<::ethers::core::types::Address>,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
            calldatas: ::std::vec::Vec<::ethers::core::types::Bytes>,
            description_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [22, 12, 190, 215],
                    (targets, values, calldatas, description_hash),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorum` (0xf8ce560a) function
        pub fn quorum(
            &self,
            block_number: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([248, 206, 86, 10], block_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorumDenominator` (0x97c3d334) function
        pub fn quorum_denominator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([151, 195, 211, 52], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorumNumerator` (0x60c4247f) function
        pub fn quorum_numerator_with_block_number(
            &self,
            block_number: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([96, 196, 36, 127], block_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorumNumerator` (0xa7713a70) function
        pub fn quorum_numerator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([167, 113, 58, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `relay` (0xc28bc2fa) function
        pub fn relay(
            &self,
            target: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 139, 194, 250], (target, value, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setProposalThreshold` (0xece40cc1) function
        pub fn set_proposal_threshold(
            &self,
            new_proposal_threshold: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([236, 228, 12, 193], new_proposal_threshold)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setVotingDelay` (0x70b0f660) function
        pub fn set_voting_delay(
            &self,
            new_voting_delay: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 176, 246, 96], new_voting_delay)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setVotingPeriod` (0xea0217cf) function
        pub fn set_voting_period(
            &self,
            new_voting_period: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 2, 23, 207], new_voting_period)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `state` (0x3e4f49e6) function
        pub fn state(
            &self,
            proposal_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([62, 79, 73, 230], proposal_id)
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
        ///Calls the contract's `timelock` (0xd33219b4) function
        pub fn timelock(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([211, 50, 25, 180], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token` (0xfc0c546a) function
        pub fn token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([252, 12, 84, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateQuorumNumerator` (0x06f3f9e6) function
        pub fn update_quorum_numerator(
            &self,
            new_quorum_numerator: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([6, 243, 249, 230], new_quorum_numerator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateTimelock` (0xa890c910) function
        pub fn update_timelock(
            &self,
            new_timelock: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([168, 144, 201, 16], new_timelock)
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
        ///Calls the contract's `votingDelay` (0x3932abb1) function
        pub fn voting_delay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([57, 50, 171, 177], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `votingPeriod` (0x02a251a3) function
        pub fn voting_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([2, 162, 81, 163], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ProposalCanceled` event
        pub fn proposal_canceled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProposalCanceledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProposalCreated` event
        pub fn proposal_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProposalCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProposalExecuted` event
        pub fn proposal_executed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProposalExecutedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProposalQueued` event
        pub fn proposal_queued_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProposalQueuedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProposalThresholdSet` event
        pub fn proposal_threshold_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProposalThresholdSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `QuorumNumeratorUpdated` event
        pub fn quorum_numerator_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            QuorumNumeratorUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TimelockChange` event
        pub fn timelock_change_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TimelockChangeFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `VoteCast` event
        pub fn vote_cast_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VoteCastFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `VoteCastWithParams` event
        pub fn vote_cast_with_params_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VoteCastWithParamsFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `VotingDelaySet` event
        pub fn voting_delay_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VotingDelaySetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `VotingPeriodSet` event
        pub fn voting_period_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VotingPeriodSetFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AlongsideGovernorEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AlongsideGovernor<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Empty` with signature `Empty()` and selector `0x3db2a12a`
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
    #[etherror(name = "Empty", abi = "Empty()")]
    pub struct Empty;
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
    #[ethevent(name = "ProposalCanceled", abi = "ProposalCanceled(uint256)")]
    pub struct ProposalCanceledFilter {
        pub proposal_id: ::ethers::core::types::U256,
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
        name = "ProposalCreated",
        abi = "ProposalCreated(uint256,address,address[],uint256[],string[],bytes[],uint256,uint256,string)"
    )]
    pub struct ProposalCreatedFilter {
        pub proposal_id: ::ethers::core::types::U256,
        pub proposer: ::ethers::core::types::Address,
        pub targets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
        pub signatures: ::std::vec::Vec<::std::string::String>,
        pub calldatas: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub start_block: ::ethers::core::types::U256,
        pub end_block: ::ethers::core::types::U256,
        pub description: ::std::string::String,
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
    #[ethevent(name = "ProposalExecuted", abi = "ProposalExecuted(uint256)")]
    pub struct ProposalExecutedFilter {
        pub proposal_id: ::ethers::core::types::U256,
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
    #[ethevent(name = "ProposalQueued", abi = "ProposalQueued(uint256,uint256)")]
    pub struct ProposalQueuedFilter {
        pub proposal_id: ::ethers::core::types::U256,
        pub eta: ::ethers::core::types::U256,
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
        name = "ProposalThresholdSet",
        abi = "ProposalThresholdSet(uint256,uint256)"
    )]
    pub struct ProposalThresholdSetFilter {
        pub old_proposal_threshold: ::ethers::core::types::U256,
        pub new_proposal_threshold: ::ethers::core::types::U256,
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
        name = "QuorumNumeratorUpdated",
        abi = "QuorumNumeratorUpdated(uint256,uint256)"
    )]
    pub struct QuorumNumeratorUpdatedFilter {
        pub old_quorum_numerator: ::ethers::core::types::U256,
        pub new_quorum_numerator: ::ethers::core::types::U256,
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
    #[ethevent(name = "TimelockChange", abi = "TimelockChange(address,address)")]
    pub struct TimelockChangeFilter {
        pub old_timelock: ::ethers::core::types::Address,
        pub new_timelock: ::ethers::core::types::Address,
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
        name = "VoteCast",
        abi = "VoteCast(address,uint256,uint8,uint256,string)"
    )]
    pub struct VoteCastFilter {
        #[ethevent(indexed)]
        pub voter: ::ethers::core::types::Address,
        pub proposal_id: ::ethers::core::types::U256,
        pub support: u8,
        pub weight: ::ethers::core::types::U256,
        pub reason: ::std::string::String,
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
        name = "VoteCastWithParams",
        abi = "VoteCastWithParams(address,uint256,uint8,uint256,string,bytes)"
    )]
    pub struct VoteCastWithParamsFilter {
        #[ethevent(indexed)]
        pub voter: ::ethers::core::types::Address,
        pub proposal_id: ::ethers::core::types::U256,
        pub support: u8,
        pub weight: ::ethers::core::types::U256,
        pub reason: ::std::string::String,
        pub params: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "VotingDelaySet", abi = "VotingDelaySet(uint256,uint256)")]
    pub struct VotingDelaySetFilter {
        pub old_voting_delay: ::ethers::core::types::U256,
        pub new_voting_delay: ::ethers::core::types::U256,
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
    #[ethevent(name = "VotingPeriodSet", abi = "VotingPeriodSet(uint256,uint256)")]
    pub struct VotingPeriodSetFilter {
        pub old_voting_period: ::ethers::core::types::U256,
        pub new_voting_period: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AlongsideGovernorEvents {
        ProposalCanceledFilter(ProposalCanceledFilter),
        ProposalCreatedFilter(ProposalCreatedFilter),
        ProposalExecutedFilter(ProposalExecutedFilter),
        ProposalQueuedFilter(ProposalQueuedFilter),
        ProposalThresholdSetFilter(ProposalThresholdSetFilter),
        QuorumNumeratorUpdatedFilter(QuorumNumeratorUpdatedFilter),
        TimelockChangeFilter(TimelockChangeFilter),
        VoteCastFilter(VoteCastFilter),
        VoteCastWithParamsFilter(VoteCastWithParamsFilter),
        VotingDelaySetFilter(VotingDelaySetFilter),
        VotingPeriodSetFilter(VotingPeriodSetFilter),
    }
    impl ::ethers::contract::EthLogDecode for AlongsideGovernorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ProposalCanceledFilter::decode_log(log) {
                return Ok(AlongsideGovernorEvents::ProposalCanceledFilter(decoded));
            }
            if let Ok(decoded) = ProposalCreatedFilter::decode_log(log) {
                return Ok(AlongsideGovernorEvents::ProposalCreatedFilter(decoded));
            }
            if let Ok(decoded) = ProposalExecutedFilter::decode_log(log) {
                return Ok(AlongsideGovernorEvents::ProposalExecutedFilter(decoded));
            }
            if let Ok(decoded) = ProposalQueuedFilter::decode_log(log) {
                return Ok(AlongsideGovernorEvents::ProposalQueuedFilter(decoded));
            }
            if let Ok(decoded) = ProposalThresholdSetFilter::decode_log(log) {
                return Ok(AlongsideGovernorEvents::ProposalThresholdSetFilter(decoded));
            }
            if let Ok(decoded) = QuorumNumeratorUpdatedFilter::decode_log(log) {
                return Ok(
                    AlongsideGovernorEvents::QuorumNumeratorUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = TimelockChangeFilter::decode_log(log) {
                return Ok(AlongsideGovernorEvents::TimelockChangeFilter(decoded));
            }
            if let Ok(decoded) = VoteCastFilter::decode_log(log) {
                return Ok(AlongsideGovernorEvents::VoteCastFilter(decoded));
            }
            if let Ok(decoded) = VoteCastWithParamsFilter::decode_log(log) {
                return Ok(AlongsideGovernorEvents::VoteCastWithParamsFilter(decoded));
            }
            if let Ok(decoded) = VotingDelaySetFilter::decode_log(log) {
                return Ok(AlongsideGovernorEvents::VotingDelaySetFilter(decoded));
            }
            if let Ok(decoded) = VotingPeriodSetFilter::decode_log(log) {
                return Ok(AlongsideGovernorEvents::VotingPeriodSetFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AlongsideGovernorEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ProposalCanceledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposalCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposalExecutedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposalQueuedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposalThresholdSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuorumNumeratorUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TimelockChangeFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VoteCastFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::VoteCastWithParamsFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VotingDelaySetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VotingPeriodSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ProposalCanceledFilter> for AlongsideGovernorEvents {
        fn from(value: ProposalCanceledFilter) -> Self {
            Self::ProposalCanceledFilter(value)
        }
    }
    impl ::core::convert::From<ProposalCreatedFilter> for AlongsideGovernorEvents {
        fn from(value: ProposalCreatedFilter) -> Self {
            Self::ProposalCreatedFilter(value)
        }
    }
    impl ::core::convert::From<ProposalExecutedFilter> for AlongsideGovernorEvents {
        fn from(value: ProposalExecutedFilter) -> Self {
            Self::ProposalExecutedFilter(value)
        }
    }
    impl ::core::convert::From<ProposalQueuedFilter> for AlongsideGovernorEvents {
        fn from(value: ProposalQueuedFilter) -> Self {
            Self::ProposalQueuedFilter(value)
        }
    }
    impl ::core::convert::From<ProposalThresholdSetFilter> for AlongsideGovernorEvents {
        fn from(value: ProposalThresholdSetFilter) -> Self {
            Self::ProposalThresholdSetFilter(value)
        }
    }
    impl ::core::convert::From<QuorumNumeratorUpdatedFilter>
    for AlongsideGovernorEvents {
        fn from(value: QuorumNumeratorUpdatedFilter) -> Self {
            Self::QuorumNumeratorUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<TimelockChangeFilter> for AlongsideGovernorEvents {
        fn from(value: TimelockChangeFilter) -> Self {
            Self::TimelockChangeFilter(value)
        }
    }
    impl ::core::convert::From<VoteCastFilter> for AlongsideGovernorEvents {
        fn from(value: VoteCastFilter) -> Self {
            Self::VoteCastFilter(value)
        }
    }
    impl ::core::convert::From<VoteCastWithParamsFilter> for AlongsideGovernorEvents {
        fn from(value: VoteCastWithParamsFilter) -> Self {
            Self::VoteCastWithParamsFilter(value)
        }
    }
    impl ::core::convert::From<VotingDelaySetFilter> for AlongsideGovernorEvents {
        fn from(value: VotingDelaySetFilter) -> Self {
            Self::VotingDelaySetFilter(value)
        }
    }
    impl ::core::convert::From<VotingPeriodSetFilter> for AlongsideGovernorEvents {
        fn from(value: VotingPeriodSetFilter) -> Self {
            Self::VotingPeriodSetFilter(value)
        }
    }
    ///Container type for all input parameters for the `BALLOT_TYPEHASH` function with signature `BALLOT_TYPEHASH()` and selector `0xdeaaa7cc`
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
    #[ethcall(name = "BALLOT_TYPEHASH", abi = "BALLOT_TYPEHASH()")]
    pub struct BallotTypehashCall;
    ///Container type for all input parameters for the `COUNTING_MODE` function with signature `COUNTING_MODE()` and selector `0xdd4e2ba5`
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
    #[ethcall(name = "COUNTING_MODE", abi = "COUNTING_MODE()")]
    pub struct CountingModeCall;
    ///Container type for all input parameters for the `EXTENDED_BALLOT_TYPEHASH` function with signature `EXTENDED_BALLOT_TYPEHASH()` and selector `0x2fe3e261`
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
    #[ethcall(name = "EXTENDED_BALLOT_TYPEHASH", abi = "EXTENDED_BALLOT_TYPEHASH()")]
    pub struct ExtendedBallotTypehashCall;
    ///Container type for all input parameters for the `castVote` function with signature `castVote(uint256,uint8)` and selector `0x56781388`
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
    #[ethcall(name = "castVote", abi = "castVote(uint256,uint8)")]
    pub struct CastVoteCall {
        pub proposal_id: ::ethers::core::types::U256,
        pub support: u8,
    }
    ///Container type for all input parameters for the `castVoteBySig` function with signature `castVoteBySig(uint256,uint8,uint8,bytes32,bytes32)` and selector `0x3bccf4fd`
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
        name = "castVoteBySig",
        abi = "castVoteBySig(uint256,uint8,uint8,bytes32,bytes32)"
    )]
    pub struct CastVoteBySigCall {
        pub proposal_id: ::ethers::core::types::U256,
        pub support: u8,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `castVoteWithReason` function with signature `castVoteWithReason(uint256,uint8,string)` and selector `0x7b3c71d3`
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
        name = "castVoteWithReason",
        abi = "castVoteWithReason(uint256,uint8,string)"
    )]
    pub struct CastVoteWithReasonCall {
        pub proposal_id: ::ethers::core::types::U256,
        pub support: u8,
        pub reason: ::std::string::String,
    }
    ///Container type for all input parameters for the `castVoteWithReasonAndParams` function with signature `castVoteWithReasonAndParams(uint256,uint8,string,bytes)` and selector `0x5f398a14`
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
        name = "castVoteWithReasonAndParams",
        abi = "castVoteWithReasonAndParams(uint256,uint8,string,bytes)"
    )]
    pub struct CastVoteWithReasonAndParamsCall {
        pub proposal_id: ::ethers::core::types::U256,
        pub support: u8,
        pub reason: ::std::string::String,
        pub params: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `castVoteWithReasonAndParamsBySig` function with signature `castVoteWithReasonAndParamsBySig(uint256,uint8,string,bytes,uint8,bytes32,bytes32)` and selector `0x03420181`
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
        name = "castVoteWithReasonAndParamsBySig",
        abi = "castVoteWithReasonAndParamsBySig(uint256,uint8,string,bytes,uint8,bytes32,bytes32)"
    )]
    pub struct CastVoteWithReasonAndParamsBySigCall {
        pub proposal_id: ::ethers::core::types::U256,
        pub support: u8,
        pub reason: ::std::string::String,
        pub params: ::ethers::core::types::Bytes,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `execute` function with signature `execute(address[],uint256[],bytes[],bytes32)` and selector `0x2656227d`
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
    #[ethcall(name = "execute", abi = "execute(address[],uint256[],bytes[],bytes32)")]
    pub struct ExecuteCall {
        pub targets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
        pub calldatas: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub description_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `getVotes` function with signature `getVotes(address,uint256)` and selector `0xeb9019d4`
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
    #[ethcall(name = "getVotes", abi = "getVotes(address,uint256)")]
    pub struct GetVotesCall {
        pub account: ::ethers::core::types::Address,
        pub block_number: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getVotesWithParams` function with signature `getVotesWithParams(address,uint256,bytes)` and selector `0x9a802a6d`
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
        name = "getVotesWithParams",
        abi = "getVotesWithParams(address,uint256,bytes)"
    )]
    pub struct GetVotesWithParamsCall {
        pub account: ::ethers::core::types::Address,
        pub block_number: ::ethers::core::types::U256,
        pub params: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `hasVoted` function with signature `hasVoted(uint256,address)` and selector `0x43859632`
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
    #[ethcall(name = "hasVoted", abi = "hasVoted(uint256,address)")]
    pub struct HasVotedCall {
        pub proposal_id: ::ethers::core::types::U256,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hashProposal` function with signature `hashProposal(address[],uint256[],bytes[],bytes32)` and selector `0xc59057e4`
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
        name = "hashProposal",
        abi = "hashProposal(address[],uint256[],bytes[],bytes32)"
    )]
    pub struct HashProposalCall {
        pub targets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
        pub calldatas: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub description_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
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
    ///Container type for all input parameters for the `proposalDeadline` function with signature `proposalDeadline(uint256)` and selector `0xc01f9e37`
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
    #[ethcall(name = "proposalDeadline", abi = "proposalDeadline(uint256)")]
    pub struct ProposalDeadlineCall {
        pub proposal_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `proposalEta` function with signature `proposalEta(uint256)` and selector `0xab58fb8e`
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
    #[ethcall(name = "proposalEta", abi = "proposalEta(uint256)")]
    pub struct ProposalEtaCall {
        pub proposal_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `proposalSnapshot` function with signature `proposalSnapshot(uint256)` and selector `0x2d63f693`
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
    #[ethcall(name = "proposalSnapshot", abi = "proposalSnapshot(uint256)")]
    pub struct ProposalSnapshotCall {
        pub proposal_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `proposalThreshold` function with signature `proposalThreshold()` and selector `0xb58131b0`
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
    #[ethcall(name = "proposalThreshold", abi = "proposalThreshold()")]
    pub struct ProposalThresholdCall;
    ///Container type for all input parameters for the `proposalVotes` function with signature `proposalVotes(uint256)` and selector `0x544ffc9c`
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
    #[ethcall(name = "proposalVotes", abi = "proposalVotes(uint256)")]
    pub struct ProposalVotesCall {
        pub proposal_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `propose` function with signature `propose(address[],uint256[],bytes[],string)` and selector `0x7d5e81e2`
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
    #[ethcall(name = "propose", abi = "propose(address[],uint256[],bytes[],string)")]
    pub struct ProposeCall {
        pub targets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
        pub calldatas: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub description: ::std::string::String,
    }
    ///Container type for all input parameters for the `queue` function with signature `queue(address[],uint256[],bytes[],bytes32)` and selector `0x160cbed7`
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
    #[ethcall(name = "queue", abi = "queue(address[],uint256[],bytes[],bytes32)")]
    pub struct QueueCall {
        pub targets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
        pub calldatas: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub description_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `quorum` function with signature `quorum(uint256)` and selector `0xf8ce560a`
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
    #[ethcall(name = "quorum", abi = "quorum(uint256)")]
    pub struct QuorumCall {
        pub block_number: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `quorumDenominator` function with signature `quorumDenominator()` and selector `0x97c3d334`
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
    #[ethcall(name = "quorumDenominator", abi = "quorumDenominator()")]
    pub struct QuorumDenominatorCall;
    ///Container type for all input parameters for the `quorumNumerator` function with signature `quorumNumerator(uint256)` and selector `0x60c4247f`
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
    #[ethcall(name = "quorumNumerator", abi = "quorumNumerator(uint256)")]
    pub struct QuorumNumeratorWithBlockNumberCall {
        pub block_number: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `quorumNumerator` function with signature `quorumNumerator()` and selector `0xa7713a70`
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
    #[ethcall(name = "quorumNumerator", abi = "quorumNumerator()")]
    pub struct QuorumNumeratorCall;
    ///Container type for all input parameters for the `relay` function with signature `relay(address,uint256,bytes)` and selector `0xc28bc2fa`
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
    #[ethcall(name = "relay", abi = "relay(address,uint256,bytes)")]
    pub struct RelayCall {
        pub target: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setProposalThreshold` function with signature `setProposalThreshold(uint256)` and selector `0xece40cc1`
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
    #[ethcall(name = "setProposalThreshold", abi = "setProposalThreshold(uint256)")]
    pub struct SetProposalThresholdCall {
        pub new_proposal_threshold: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setVotingDelay` function with signature `setVotingDelay(uint256)` and selector `0x70b0f660`
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
    #[ethcall(name = "setVotingDelay", abi = "setVotingDelay(uint256)")]
    pub struct SetVotingDelayCall {
        pub new_voting_delay: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setVotingPeriod` function with signature `setVotingPeriod(uint256)` and selector `0xea0217cf`
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
    #[ethcall(name = "setVotingPeriod", abi = "setVotingPeriod(uint256)")]
    pub struct SetVotingPeriodCall {
        pub new_voting_period: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `state` function with signature `state(uint256)` and selector `0x3e4f49e6`
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
    #[ethcall(name = "state", abi = "state(uint256)")]
    pub struct StateCall {
        pub proposal_id: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `timelock` function with signature `timelock()` and selector `0xd33219b4`
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
    #[ethcall(name = "timelock", abi = "timelock()")]
    pub struct TimelockCall;
    ///Container type for all input parameters for the `token` function with signature `token()` and selector `0xfc0c546a`
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
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    ///Container type for all input parameters for the `updateQuorumNumerator` function with signature `updateQuorumNumerator(uint256)` and selector `0x06f3f9e6`
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
    #[ethcall(name = "updateQuorumNumerator", abi = "updateQuorumNumerator(uint256)")]
    pub struct UpdateQuorumNumeratorCall {
        pub new_quorum_numerator: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateTimelock` function with signature `updateTimelock(address)` and selector `0xa890c910`
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
    #[ethcall(name = "updateTimelock", abi = "updateTimelock(address)")]
    pub struct UpdateTimelockCall {
        pub new_timelock: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `votingDelay` function with signature `votingDelay()` and selector `0x3932abb1`
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
    #[ethcall(name = "votingDelay", abi = "votingDelay()")]
    pub struct VotingDelayCall;
    ///Container type for all input parameters for the `votingPeriod` function with signature `votingPeriod()` and selector `0x02a251a3`
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
    #[ethcall(name = "votingPeriod", abi = "votingPeriod()")]
    pub struct VotingPeriodCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AlongsideGovernorCalls {
        BallotTypehash(BallotTypehashCall),
        CountingMode(CountingModeCall),
        ExtendedBallotTypehash(ExtendedBallotTypehashCall),
        CastVote(CastVoteCall),
        CastVoteBySig(CastVoteBySigCall),
        CastVoteWithReason(CastVoteWithReasonCall),
        CastVoteWithReasonAndParams(CastVoteWithReasonAndParamsCall),
        CastVoteWithReasonAndParamsBySig(CastVoteWithReasonAndParamsBySigCall),
        Execute(ExecuteCall),
        GetVotes(GetVotesCall),
        GetVotesWithParams(GetVotesWithParamsCall),
        HasVoted(HasVotedCall),
        HashProposal(HashProposalCall),
        Name(NameCall),
        OnERC1155BatchReceived(OnERC1155BatchReceivedCall),
        OnERC1155Received(OnERC1155ReceivedCall),
        OnERC721Received(OnERC721ReceivedCall),
        ProposalDeadline(ProposalDeadlineCall),
        ProposalEta(ProposalEtaCall),
        ProposalSnapshot(ProposalSnapshotCall),
        ProposalThreshold(ProposalThresholdCall),
        ProposalVotes(ProposalVotesCall),
        Propose(ProposeCall),
        Queue(QueueCall),
        Quorum(QuorumCall),
        QuorumDenominator(QuorumDenominatorCall),
        QuorumNumeratorWithBlockNumber(QuorumNumeratorWithBlockNumberCall),
        QuorumNumerator(QuorumNumeratorCall),
        Relay(RelayCall),
        SetProposalThreshold(SetProposalThresholdCall),
        SetVotingDelay(SetVotingDelayCall),
        SetVotingPeriod(SetVotingPeriodCall),
        State(StateCall),
        SupportsInterface(SupportsInterfaceCall),
        Timelock(TimelockCall),
        Token(TokenCall),
        UpdateQuorumNumerator(UpdateQuorumNumeratorCall),
        UpdateTimelock(UpdateTimelockCall),
        Version(VersionCall),
        VotingDelay(VotingDelayCall),
        VotingPeriod(VotingPeriodCall),
    }
    impl ::ethers::core::abi::AbiDecode for AlongsideGovernorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BallotTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BallotTypehash(decoded));
            }
            if let Ok(decoded) = <CountingModeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CountingMode(decoded));
            }
            if let Ok(decoded) = <ExtendedBallotTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExtendedBallotTypehash(decoded));
            }
            if let Ok(decoded) = <CastVoteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CastVote(decoded));
            }
            if let Ok(decoded) = <CastVoteBySigCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CastVoteBySig(decoded));
            }
            if let Ok(decoded) = <CastVoteWithReasonCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CastVoteWithReason(decoded));
            }
            if let Ok(decoded) = <CastVoteWithReasonAndParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CastVoteWithReasonAndParams(decoded));
            }
            if let Ok(decoded) = <CastVoteWithReasonAndParamsBySigCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CastVoteWithReasonAndParamsBySig(decoded));
            }
            if let Ok(decoded) = <ExecuteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded) = <GetVotesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetVotes(decoded));
            }
            if let Ok(decoded) = <GetVotesWithParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetVotesWithParams(decoded));
            }
            if let Ok(decoded) = <HasVotedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasVoted(decoded));
            }
            if let Ok(decoded) = <HashProposalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HashProposal(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
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
            if let Ok(decoded) = <ProposalDeadlineCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProposalDeadline(decoded));
            }
            if let Ok(decoded) = <ProposalEtaCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProposalEta(decoded));
            }
            if let Ok(decoded) = <ProposalSnapshotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProposalSnapshot(decoded));
            }
            if let Ok(decoded) = <ProposalThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProposalThreshold(decoded));
            }
            if let Ok(decoded) = <ProposalVotesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProposalVotes(decoded));
            }
            if let Ok(decoded) = <ProposeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Propose(decoded));
            }
            if let Ok(decoded) = <QueueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Queue(decoded));
            }
            if let Ok(decoded) = <QuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Quorum(decoded));
            }
            if let Ok(decoded) = <QuorumDenominatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuorumDenominator(decoded));
            }
            if let Ok(decoded) = <QuorumNumeratorWithBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuorumNumeratorWithBlockNumber(decoded));
            }
            if let Ok(decoded) = <QuorumNumeratorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuorumNumerator(decoded));
            }
            if let Ok(decoded) = <RelayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Relay(decoded));
            }
            if let Ok(decoded) = <SetProposalThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetProposalThreshold(decoded));
            }
            if let Ok(decoded) = <SetVotingDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetVotingDelay(decoded));
            }
            if let Ok(decoded) = <SetVotingPeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetVotingPeriod(decoded));
            }
            if let Ok(decoded) = <StateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::State(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <TimelockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Timelock(decoded));
            }
            if let Ok(decoded) = <TokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Token(decoded));
            }
            if let Ok(decoded) = <UpdateQuorumNumeratorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateQuorumNumerator(decoded));
            }
            if let Ok(decoded) = <UpdateTimelockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateTimelock(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            if let Ok(decoded) = <VotingDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VotingDelay(decoded));
            }
            if let Ok(decoded) = <VotingPeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VotingPeriod(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AlongsideGovernorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BallotTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CountingMode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExtendedBallotTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CastVote(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CastVoteBySig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CastVoteWithReason(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CastVoteWithReasonAndParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CastVoteWithReasonAndParamsBySig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetVotes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVotesWithParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasVoted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HashProposal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnERC1155BatchReceived(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC1155Received(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC721Received(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposalDeadline(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposalEta(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposalSnapshot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposalThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposalVotes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Propose(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Queue(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Quorum(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QuorumDenominator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuorumNumeratorWithBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuorumNumerator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Relay(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetProposalThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetVotingDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetVotingPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::State(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Timelock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Token(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateQuorumNumerator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateTimelock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VotingDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VotingPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AlongsideGovernorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BallotTypehash(element) => ::core::fmt::Display::fmt(element, f),
                Self::CountingMode(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExtendedBallotTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CastVote(element) => ::core::fmt::Display::fmt(element, f),
                Self::CastVoteBySig(element) => ::core::fmt::Display::fmt(element, f),
                Self::CastVoteWithReason(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CastVoteWithReasonAndParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CastVoteWithReasonAndParamsBySig(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVotes(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVotesWithParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HasVoted(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashProposal(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC1155BatchReceived(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnERC1155Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC721Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalDeadline(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalEta(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalSnapshot(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalThreshold(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalVotes(element) => ::core::fmt::Display::fmt(element, f),
                Self::Propose(element) => ::core::fmt::Display::fmt(element, f),
                Self::Queue(element) => ::core::fmt::Display::fmt(element, f),
                Self::Quorum(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumDenominator(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumNumeratorWithBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuorumNumerator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Relay(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProposalThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetVotingDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetVotingPeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::State(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Timelock(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateQuorumNumerator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateTimelock(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::VotingDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::VotingPeriod(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BallotTypehashCall> for AlongsideGovernorCalls {
        fn from(value: BallotTypehashCall) -> Self {
            Self::BallotTypehash(value)
        }
    }
    impl ::core::convert::From<CountingModeCall> for AlongsideGovernorCalls {
        fn from(value: CountingModeCall) -> Self {
            Self::CountingMode(value)
        }
    }
    impl ::core::convert::From<ExtendedBallotTypehashCall> for AlongsideGovernorCalls {
        fn from(value: ExtendedBallotTypehashCall) -> Self {
            Self::ExtendedBallotTypehash(value)
        }
    }
    impl ::core::convert::From<CastVoteCall> for AlongsideGovernorCalls {
        fn from(value: CastVoteCall) -> Self {
            Self::CastVote(value)
        }
    }
    impl ::core::convert::From<CastVoteBySigCall> for AlongsideGovernorCalls {
        fn from(value: CastVoteBySigCall) -> Self {
            Self::CastVoteBySig(value)
        }
    }
    impl ::core::convert::From<CastVoteWithReasonCall> for AlongsideGovernorCalls {
        fn from(value: CastVoteWithReasonCall) -> Self {
            Self::CastVoteWithReason(value)
        }
    }
    impl ::core::convert::From<CastVoteWithReasonAndParamsCall>
    for AlongsideGovernorCalls {
        fn from(value: CastVoteWithReasonAndParamsCall) -> Self {
            Self::CastVoteWithReasonAndParams(value)
        }
    }
    impl ::core::convert::From<CastVoteWithReasonAndParamsBySigCall>
    for AlongsideGovernorCalls {
        fn from(value: CastVoteWithReasonAndParamsBySigCall) -> Self {
            Self::CastVoteWithReasonAndParamsBySig(value)
        }
    }
    impl ::core::convert::From<ExecuteCall> for AlongsideGovernorCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<GetVotesCall> for AlongsideGovernorCalls {
        fn from(value: GetVotesCall) -> Self {
            Self::GetVotes(value)
        }
    }
    impl ::core::convert::From<GetVotesWithParamsCall> for AlongsideGovernorCalls {
        fn from(value: GetVotesWithParamsCall) -> Self {
            Self::GetVotesWithParams(value)
        }
    }
    impl ::core::convert::From<HasVotedCall> for AlongsideGovernorCalls {
        fn from(value: HasVotedCall) -> Self {
            Self::HasVoted(value)
        }
    }
    impl ::core::convert::From<HashProposalCall> for AlongsideGovernorCalls {
        fn from(value: HashProposalCall) -> Self {
            Self::HashProposal(value)
        }
    }
    impl ::core::convert::From<NameCall> for AlongsideGovernorCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OnERC1155BatchReceivedCall> for AlongsideGovernorCalls {
        fn from(value: OnERC1155BatchReceivedCall) -> Self {
            Self::OnERC1155BatchReceived(value)
        }
    }
    impl ::core::convert::From<OnERC1155ReceivedCall> for AlongsideGovernorCalls {
        fn from(value: OnERC1155ReceivedCall) -> Self {
            Self::OnERC1155Received(value)
        }
    }
    impl ::core::convert::From<OnERC721ReceivedCall> for AlongsideGovernorCalls {
        fn from(value: OnERC721ReceivedCall) -> Self {
            Self::OnERC721Received(value)
        }
    }
    impl ::core::convert::From<ProposalDeadlineCall> for AlongsideGovernorCalls {
        fn from(value: ProposalDeadlineCall) -> Self {
            Self::ProposalDeadline(value)
        }
    }
    impl ::core::convert::From<ProposalEtaCall> for AlongsideGovernorCalls {
        fn from(value: ProposalEtaCall) -> Self {
            Self::ProposalEta(value)
        }
    }
    impl ::core::convert::From<ProposalSnapshotCall> for AlongsideGovernorCalls {
        fn from(value: ProposalSnapshotCall) -> Self {
            Self::ProposalSnapshot(value)
        }
    }
    impl ::core::convert::From<ProposalThresholdCall> for AlongsideGovernorCalls {
        fn from(value: ProposalThresholdCall) -> Self {
            Self::ProposalThreshold(value)
        }
    }
    impl ::core::convert::From<ProposalVotesCall> for AlongsideGovernorCalls {
        fn from(value: ProposalVotesCall) -> Self {
            Self::ProposalVotes(value)
        }
    }
    impl ::core::convert::From<ProposeCall> for AlongsideGovernorCalls {
        fn from(value: ProposeCall) -> Self {
            Self::Propose(value)
        }
    }
    impl ::core::convert::From<QueueCall> for AlongsideGovernorCalls {
        fn from(value: QueueCall) -> Self {
            Self::Queue(value)
        }
    }
    impl ::core::convert::From<QuorumCall> for AlongsideGovernorCalls {
        fn from(value: QuorumCall) -> Self {
            Self::Quorum(value)
        }
    }
    impl ::core::convert::From<QuorumDenominatorCall> for AlongsideGovernorCalls {
        fn from(value: QuorumDenominatorCall) -> Self {
            Self::QuorumDenominator(value)
        }
    }
    impl ::core::convert::From<QuorumNumeratorWithBlockNumberCall>
    for AlongsideGovernorCalls {
        fn from(value: QuorumNumeratorWithBlockNumberCall) -> Self {
            Self::QuorumNumeratorWithBlockNumber(value)
        }
    }
    impl ::core::convert::From<QuorumNumeratorCall> for AlongsideGovernorCalls {
        fn from(value: QuorumNumeratorCall) -> Self {
            Self::QuorumNumerator(value)
        }
    }
    impl ::core::convert::From<RelayCall> for AlongsideGovernorCalls {
        fn from(value: RelayCall) -> Self {
            Self::Relay(value)
        }
    }
    impl ::core::convert::From<SetProposalThresholdCall> for AlongsideGovernorCalls {
        fn from(value: SetProposalThresholdCall) -> Self {
            Self::SetProposalThreshold(value)
        }
    }
    impl ::core::convert::From<SetVotingDelayCall> for AlongsideGovernorCalls {
        fn from(value: SetVotingDelayCall) -> Self {
            Self::SetVotingDelay(value)
        }
    }
    impl ::core::convert::From<SetVotingPeriodCall> for AlongsideGovernorCalls {
        fn from(value: SetVotingPeriodCall) -> Self {
            Self::SetVotingPeriod(value)
        }
    }
    impl ::core::convert::From<StateCall> for AlongsideGovernorCalls {
        fn from(value: StateCall) -> Self {
            Self::State(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for AlongsideGovernorCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<TimelockCall> for AlongsideGovernorCalls {
        fn from(value: TimelockCall) -> Self {
            Self::Timelock(value)
        }
    }
    impl ::core::convert::From<TokenCall> for AlongsideGovernorCalls {
        fn from(value: TokenCall) -> Self {
            Self::Token(value)
        }
    }
    impl ::core::convert::From<UpdateQuorumNumeratorCall> for AlongsideGovernorCalls {
        fn from(value: UpdateQuorumNumeratorCall) -> Self {
            Self::UpdateQuorumNumerator(value)
        }
    }
    impl ::core::convert::From<UpdateTimelockCall> for AlongsideGovernorCalls {
        fn from(value: UpdateTimelockCall) -> Self {
            Self::UpdateTimelock(value)
        }
    }
    impl ::core::convert::From<VersionCall> for AlongsideGovernorCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<VotingDelayCall> for AlongsideGovernorCalls {
        fn from(value: VotingDelayCall) -> Self {
            Self::VotingDelay(value)
        }
    }
    impl ::core::convert::From<VotingPeriodCall> for AlongsideGovernorCalls {
        fn from(value: VotingPeriodCall) -> Self {
            Self::VotingPeriod(value)
        }
    }
    ///Container type for all return fields from the `BALLOT_TYPEHASH` function with signature `BALLOT_TYPEHASH()` and selector `0xdeaaa7cc`
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
    pub struct BallotTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `COUNTING_MODE` function with signature `COUNTING_MODE()` and selector `0xdd4e2ba5`
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
    pub struct CountingModeReturn(pub ::std::string::String);
    ///Container type for all return fields from the `EXTENDED_BALLOT_TYPEHASH` function with signature `EXTENDED_BALLOT_TYPEHASH()` and selector `0x2fe3e261`
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
    pub struct ExtendedBallotTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `castVote` function with signature `castVote(uint256,uint8)` and selector `0x56781388`
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
    pub struct CastVoteReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `castVoteBySig` function with signature `castVoteBySig(uint256,uint8,uint8,bytes32,bytes32)` and selector `0x3bccf4fd`
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
    pub struct CastVoteBySigReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `castVoteWithReason` function with signature `castVoteWithReason(uint256,uint8,string)` and selector `0x7b3c71d3`
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
    pub struct CastVoteWithReasonReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `castVoteWithReasonAndParams` function with signature `castVoteWithReasonAndParams(uint256,uint8,string,bytes)` and selector `0x5f398a14`
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
    pub struct CastVoteWithReasonAndParamsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `castVoteWithReasonAndParamsBySig` function with signature `castVoteWithReasonAndParamsBySig(uint256,uint8,string,bytes,uint8,bytes32,bytes32)` and selector `0x03420181`
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
    pub struct CastVoteWithReasonAndParamsBySigReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `execute` function with signature `execute(address[],uint256[],bytes[],bytes32)` and selector `0x2656227d`
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
    pub struct ExecuteReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getVotes` function with signature `getVotes(address,uint256)` and selector `0xeb9019d4`
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
    pub struct GetVotesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getVotesWithParams` function with signature `getVotesWithParams(address,uint256,bytes)` and selector `0x9a802a6d`
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
    pub struct GetVotesWithParamsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `hasVoted` function with signature `hasVoted(uint256,address)` and selector `0x43859632`
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
    pub struct HasVotedReturn(pub bool);
    ///Container type for all return fields from the `hashProposal` function with signature `hashProposal(address[],uint256[],bytes[],bytes32)` and selector `0xc59057e4`
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
    pub struct HashProposalReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
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
    ///Container type for all return fields from the `proposalDeadline` function with signature `proposalDeadline(uint256)` and selector `0xc01f9e37`
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
    pub struct ProposalDeadlineReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `proposalEta` function with signature `proposalEta(uint256)` and selector `0xab58fb8e`
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
    pub struct ProposalEtaReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `proposalSnapshot` function with signature `proposalSnapshot(uint256)` and selector `0x2d63f693`
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
    pub struct ProposalSnapshotReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `proposalThreshold` function with signature `proposalThreshold()` and selector `0xb58131b0`
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
    pub struct ProposalThresholdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `proposalVotes` function with signature `proposalVotes(uint256)` and selector `0x544ffc9c`
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
    pub struct ProposalVotesReturn {
        pub against_votes: ::ethers::core::types::U256,
        pub for_votes: ::ethers::core::types::U256,
        pub abstain_votes: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `propose` function with signature `propose(address[],uint256[],bytes[],string)` and selector `0x7d5e81e2`
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
    pub struct ProposeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `queue` function with signature `queue(address[],uint256[],bytes[],bytes32)` and selector `0x160cbed7`
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
    pub struct QueueReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `quorum` function with signature `quorum(uint256)` and selector `0xf8ce560a`
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
    pub struct QuorumReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `quorumDenominator` function with signature `quorumDenominator()` and selector `0x97c3d334`
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
    pub struct QuorumDenominatorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `quorumNumerator` function with signature `quorumNumerator(uint256)` and selector `0x60c4247f`
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
    pub struct QuorumNumeratorWithBlockNumberReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `quorumNumerator` function with signature `quorumNumerator()` and selector `0xa7713a70`
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
    pub struct QuorumNumeratorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `state` function with signature `state(uint256)` and selector `0x3e4f49e6`
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
    pub struct StateReturn(pub u8);
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
    ///Container type for all return fields from the `timelock` function with signature `timelock()` and selector `0xd33219b4`
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
    pub struct TimelockReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `token` function with signature `token()` and selector `0xfc0c546a`
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
    pub struct TokenReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `votingDelay` function with signature `votingDelay()` and selector `0x3932abb1`
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
    pub struct VotingDelayReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `votingPeriod` function with signature `votingPeriod()` and selector `0x02a251a3`
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
    pub struct VotingPeriodReturn(pub ::ethers::core::types::U256);
}
