pub use invokeable_bounty::*;
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
pub mod invokeable_bounty {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_vault"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_activeBounty"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_version"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("activeBounty"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("activeBounty"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IActiveBounty"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("completedBounties"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("completedBounties"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("fulfillBounty"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fulfillBounty"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bounty"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Bounty"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callback"),
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
                    ::std::borrow::ToOwned::to_owned("hashBounty"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hashBounty"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bounty"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Bounty"),
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
                    ::std::borrow::ToOwned::to_owned("quoteBounty"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quoteBounty"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bounty"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Bounty"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("outs"),
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
                (
                    ::std::borrow::ToOwned::to_owned("reentrancyLock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reentrancyLock"),
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
                    ::std::borrow::ToOwned::to_owned("vault"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("vault"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IVault"),
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
                    ::std::borrow::ToOwned::to_owned("BountyAMKTSupplyChange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BountyAMKTSupplyChange",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BountyAlreadyCompleted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BountyAlreadyCompleted",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BountyInvalidHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("BountyInvalidHash"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BountyMustIncludeAllUnderlyings"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BountyMustIncludeAllUnderlyings",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BountyPastDeadline"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("BountyPastDeadline"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BountyReentrant"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("BountyReentrant"),
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
    pub static INVOKEABLEBOUNTY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R`\x01\x80U4\x80\x15b\0\0\x16W`\0\x80\xFD[P`@Qb\0\x1Aq8\x03\x80b\0\x1Aq\x839\x81\x01`@\x81\x90Rb\0\09\x91b\0\0\xE2V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\xA0\x81\x90R`@\x80Qcs\xE8\n\xF9`\xE1\x1B\x81R\x90Qc\xE7\xD0\x15\xF2\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\0\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\0\xAA\x91\x90b\0\x01*V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80R\x91\x90\x91\x16`\xC0R`\xE0RPb\0\x01QV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xDFW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\0\xF8W`\0\x80\xFD[\x83Qb\0\x01\x05\x81b\0\0\xC9V[` \x85\x01Q\x90\x93Pb\0\x01\x18\x81b\0\0\xC9V[\x80\x92PP`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15b\0\x01=W`\0\x80\xFD[\x81Qb\0\x01J\x81b\0\0\xC9V[\x93\x92PPPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x18wb\0\x01\xFA`\09`\0\x81\x81`\xDA\x01Ra\x0CE\x01R`\0\x81\x81a\x01I\x01Ra\x04\xBB\x01R`\0\x81\x81a\x01\xC2\x01R\x81\x81a\x01\xEB\x01R\x81\x81a\x05\xB0\x01R\x81\x81a\x067\x01R\x81\x81a\x07y\x01R\x81\x81a\x083\x01R\x81\x81a\t\xDC\x01R\x81\x81a\n\xBD\x01R\x81\x81a\x0B>\x01R\x81\x81a\x0B\xDA\x01R\x81\x81a\x0E`\x01Ra\x0F\r\x01R`\0\x81\x81a\x01\x9B\x01R\x81\x81a\x02t\x01R\x81\x81a\x06\xC0\x01Ra\t\x03\x01Ra\x18w`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x93W`\x005`\xE0\x1C\x80c\xB42X\x9D\x11a\0fW\x80c\xB42X\x9D\x14a\x01\x11W\x80c\xC5\xAB\x93\xA1\x14a\x01DW\x80c\xD5J2@\x14a\x01\x83W\x80c\xE7\xD0\x15\xF2\x14a\x01\x96W\x80c\xFB\xFAw\xCF\x14a\x01\xBDW`\0\x80\xFD[\x80c$F\xD7\x9F\x14a\0\x98W\x80c6\xDC\x91\x03\x14a\0\xB4W\x80cT\xFDMP\x14a\0\xD5W\x80c\x9F\xCA\xD8\x9A\x14a\0\xFCW[`\0\x80\xFD[a\0\xA1`\x01T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xC7a\0\xC26`\x04a\x12\tV[a\x01\xE4V[`@Qa\0\xAB\x92\x91\x90a\x12\x9BV[a\0\xA1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x0Fa\x01\n6`\x04a\x14~V[a\x04\x83V[\0[a\x014a\x01\x1F6`\x04a\x14\xCFV[`\0` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xABV[a\x01k\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xABV[a\0\xA1a\x01\x916`\x04a\x14\xE8V[a\x0C?V[a\x01k\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01k\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[``\x80`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x1B>\xD7\"`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02GW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02k\x91\x90a\x15$V[PP\x91PP`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xF4\x91\x90a\x15ZV[\x90P`\0a\x03\x02\x86\x80a\x15sV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03NWa\x03?`@\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90a\x15\xC3V[\x81R` \x01\x90`\x01\x01\x90a\x03\"V[PPPPP\x90P`\0\x80a\x03{`@Q\x80``\x01`@R\x80\x85\x81R` \x01\x86\x81R` \x01\x87\x81RPa\x0C\xD1V[PP\x91P\x91P\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03\x9AWa\x03\x9Aa\x12\xC9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\xDFW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x03\xB8W\x90P[P\x96P`\0[\x82Q\x81\x10\x15a\x04vW`@Q\x80`@\x01`@R\x80\x84\x83\x81Q\x81\x10a\x04\x0BWa\x04\x0Ba\x15\xDFV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x83\x81Q\x81\x10a\x047Wa\x047a\x15\xDFV[` \x02` \x01\x01Q`@\x01Q\x81RP\x88\x82\x81Q\x81\x10a\x04XWa\x04Xa\x15\xDFV[` \x02` \x01\x01\x81\x90RP\x80\x80a\x04n\x90a\x16\x0BV[\x91PPa\x03\xE5V[P\x95\x97\x95\x96PPPPPPV[`\x01\x80T\x11\x15a\x04\xA6W`@Qc\xA3\xF8Q\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01U`\0a\x04\xB6\x83a\x0C?V[\x90P\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC5\xAB\x93\xA1`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x17W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05;\x91\x90a\x15ZV[\x14a\x05YW`@QceHQe`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81R` \x81\x90R`@\x90 T`\xFF\x16\x15a\x05\x89W`@Qc&\x18._`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82` \x01QB\x11\x15a\x05\xAEW`@Qc\xA0\x92\xEAQ`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA7\xD03w`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\x0EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x062\x91\x90a\x15ZV[P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x1B>\xD7\"`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x93W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xB7\x91\x90a\x15$V[PP\x91PP`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07@\x91\x90a\x15ZV[\x90P`\0\x80`\0\x80a\x07o`@Q\x80``\x01`@R\x80\x8B`\0\x01Q\x81R` \x01\x87\x81R` \x01\x88\x81RPa\x0C\xD1V[\x93P\x93P\x93P\x93P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c \x7F\xAA}`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xF9\x91\x90a\x15ZV[\x81\x10\x15a\x08\x19W`@Qc&\xC7\x9F\xFD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q`\x01b\x1F\xB1;`\xE3\x1B\x03\x19\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xFF\x02v(\x90a\x08h\x90\x87\x90`\x04\x01a\x16$V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x82W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\x96W=`\0\x80>=`\0\xFD[PPPP\x87\x15a\t\0W3c\xB7\x9D\xEA\xB4\x84a\x08\xB0\x87a\x10\xE6V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\xCD\x92\x91\x90a\x12\x9BV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\xE7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xFBW=`\0\x80>=`\0\xFD[PPPP[\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t_W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x83\x91\x90a\x15ZV[\x14a\t\xA1W`@Qc\xB9\xBF/\x03`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x83Q\x81\x10\x15a\n\xA6W\x83\x81\x81Q\x81\x10a\t\xBFWa\t\xBFa\x15\xDFV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x85\x81Q\x81\x10a\n\rWa\n\ra\x15\xDFV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x92\x90\x91\x16`$\x83\x01R`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\noW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x93\x91\x90a\x16\x89V[P\x80a\n\x9E\x81a\x16\x0BV[\x91PPa\t\xA4V[P`@Qb\x1A)\xE3`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90b\xD1O\x18\x90a\n\xF1\x90\x85\x90`\x04\x01a\x16\xA6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\x0BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\x1FW=`\0\x80>=`\0\xFD[PP`@Qc\x03\xF7\x13/`\xE6\x1B\x81Rg\r\xE0\xB6\xB3\xA7d\0\0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92Pc\xFD\xC4\xCB\xC0\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\x8CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\xA0W=`\0\x80>=`\0\xFD[PPP`\0\x97\x88RPPP` \x85\x90RPP`@\x80\x84 \x80T`\xFF\x19\x16`\x01\x17\x90U\x80Qc\"\x84\x80\xBF`\xE1\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x94cE\t\x01~\x94P`\x04\x80\x84\x01\x94P\x90\x92\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x0C\x1FW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0C3W=`\0\x80>=`\0\xFD[PP`\x01\x80UPPPPV[`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82`@Q` \x01a\x0C\x8D\x91\x90a\x16\xF9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0C\xB4\x92\x91\x90a\x17{V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[``\x80```\0\x84`\0\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\xF5Wa\x0C\xF5a\x12\xC9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r@W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\r\x13W\x90P[P\x85QQ\x90\x94P`\x01`\x01`@\x1B\x03\x81\x11\x15a\r^Wa\r^a\x12\xC9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\xA3W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r|W\x90P[P\x85QQ\x90\x93P`\x01`\x01`@\x1B\x03\x81\x11\x15a\r\xC1Wa\r\xC1a\x12\xC9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\x06W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\xDFW\x90P[P\x91P`\0\x80`\0\x80[\x88QQ\x81\x10\x15a\x10\xD7W`\0\x89`\0\x01Q\x82\x81Q\x81\x10a\x0E2Wa\x0E2a\x15\xDFV[` \x90\x81\x02\x91\x90\x91\x01\x01QQ`@Qc\xB2\x13\xE3\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xB2\x13\xE3\xF7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xA9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xCD\x91\x90a\x16\x89V[\x15a\x0E\xE0W\x85a\x0E\xDC\x81a\x16\x0BV[\x96PP[`\0\x8A`\0\x01Q\x83\x81Q\x81\x10a\x0E\xF8Wa\x0E\xF8a\x15\xDFV[` \x02` \x01\x01Q` \x01Q\x90P`\0a\x0F\xB1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c}\xD0\x9F\xEC\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Ff\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x83W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xA7\x91\x90a\x15ZV[\x8D`@\x01Qa\x11\xE3V[\x90P\x81\x81\x11\x15a\x10\x1CW`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R3` \x82\x01R\x90\x81\x01a\x0F\xF1a\x0F\xE7\x85\x85a\x17\xE9V[\x8F` \x01Qa\x11\xE3V[\x81RP\x8B\x88\x81Q\x81\x10a\x10\x06Wa\x10\x06a\x15\xDFV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x90\x96\x01\x95a\x10\x80V[\x80\x82\x11\x15a\x10xW`@Q\x80`@\x01`@R\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x10M\x83\x85a\x0F\xE7\x91\x90a\x17\xE9V[\x81RP\x8A\x87\x81Q\x81\x10a\x10bWa\x10ba\x15\xDFV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x90\x95\x01\x94a\x10\x80V[PPPa\x10\xC5V[`@Q\x80`@\x01`@R\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81RP\x89\x86\x81Q\x81\x10a\x10\xB0Wa\x10\xB0a\x15\xDFV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x90\x92\x01\x91P[\x80a\x10\xCF\x81a\x16\x0BV[\x91PPa\x0E\x10V[P\x91\x86R\x84R\x82R\x91\x93P\x91\x93V[``\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\x01Wa\x11\x01a\x12\xC9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11FW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x11\x1FW\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x11\xDDW`@Q\x80`@\x01`@R\x80\x84\x83\x81Q\x81\x10a\x11rWa\x11ra\x15\xDFV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x83\x81Q\x81\x10a\x11\x9EWa\x11\x9Ea\x15\xDFV[` \x02` \x01\x01Q`@\x01Q\x81RP\x82\x82\x81Q\x81\x10a\x11\xBFWa\x11\xBFa\x15\xDFV[` \x02` \x01\x01\x81\x90RP\x80\x80a\x11\xD5\x90a\x16\x0BV[\x91PPa\x11LV[P\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x11\xF8\x83\x85a\x18\0V[a\x12\x02\x91\x90a\x18\x1FV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x12\x1BW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x121W`\0\x80\xFD[\x82\x01``\x81\x85\x03\x12\x15a\x12\x02W`\0\x80\xFD[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x12\x90Wa\x12}\x87\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[`@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x12WV[P\x94\x95\x94PPPPPV[`@\x81R`\0a\x12\xAE`@\x83\x01\x85a\x12CV[\x82\x81\x03` \x84\x01Ra\x12\xC0\x81\x85a\x12CV[\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x13\x01Wa\x13\x01a\x12\xC9V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x13/Wa\x13/a\x12\xC9V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x13IW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x13kWa\x13ka\x12\xC9V[`@R\x90P\x80\x825`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13\x88W`\0\x80\xFD[\x81R` \x92\x83\x015\x92\x01\x91\x90\x91R\x91\x90PV[`\0``\x82\x84\x03\x12\x15a\x13\xADW`\0\x80\xFD[a\x13\xB5a\x12\xDFV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x13\xCEW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x13\xE2W`\0\x80\xFD[\x815` \x82\x82\x11\x15a\x13\xF6Wa\x13\xF6a\x12\xC9V[a\x14\x04\x81\x83`\x05\x1B\x01a\x13\x07V[\x82\x81R\x81\x81\x01\x93P`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x87\x83\x11\x15a\x14$W`\0\x80\xFD[\x93\x81\x01\x93[\x82\x85\x10\x15a\x14MWa\x14;\x88\x86a\x137V[\x84R\x81\x84\x01\x93P`@\x85\x01\x94Pa\x14)V[\x80\x86RP\x80\x86\x015\x81\x86\x01RPPPP`@\x82\x015`@\x82\x01R\x92\x91PPV[\x80\x15\x15\x81\x14a\x14{W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\x91W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xA7W`\0\x80\xFD[a\x14\xB3\x85\x82\x86\x01a\x13\x9BV[\x92PP` \x83\x015a\x14\xC4\x81a\x14mV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x14\xE1W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x14\xFAW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\x10W`\0\x80\xFD[a\x15\x1C\x84\x82\x85\x01a\x13\x9BV[\x94\x93PPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x15:W`\0\x80\xFD[PP\x82Q` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Q\x91\x96\x90\x95P\x90\x92P\x90PV[`\0` \x82\x84\x03\x12\x15a\x15lW`\0\x80\xFD[PQ\x91\x90PV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x15\x8AW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x15\xA4W`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a\x15\xBCW`\0\x80\xFD[\x92P\x92\x90PV[`\0`@\x82\x84\x03\x12\x15a\x15\xD5W`\0\x80\xFD[a\x12\x02\x83\x83a\x137V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x16\x1DWa\x16\x1Da\x15\xF5V[P`\x01\x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x16|W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x86R\x87\x82\x01Q\x16\x87\x86\x01R\x85\x01Q\x85\x85\x01R``\x90\x93\x01\x92\x90\x85\x01\x90`\x01\x01a\x16AV[P\x91\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x16\x9BW`\0\x80\xFD[\x81Qa\x12\x02\x81a\x14mV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x16|Wa\x16\xE9\x84\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x16\xC3V[` \x80\x82R\x82Q``\x83\x83\x01R\x80Q`\x80\x84\x01\x81\x90R`\0\x92\x91\x82\x01\x90\x83\x90`\xA0\x86\x01\x90[\x80\x83\x10\x15a\x17ZWa\x17D\x82\x85Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[`@\x82\x01\x91P\x84\x84\x01\x93P`\x01\x83\x01\x92Pa\x17\x1EV[P\x83\x87\x01Q`@\x87\x01R`@\x87\x01Q``\x87\x01R\x80\x94PPPPP\x92\x91PPV[\x7Falongside::invoker::bounty\0\0\0\0\0\0\x81R`\0\x83Q`\0[\x81\x81\x10\x15a\x17\xC2W` \x81\x87\x01\x81\x01Q`\x1A\x86\x84\x01\x01R\x01a\x17\xA5V[\x81\x81\x11\x15a\x17\xD4W`\0`\x1A\x83\x86\x01\x01R[P`\x1A\x92\x01\x91\x82\x01\x92\x90\x92R`:\x01\x92\x91PPV[`\0\x82\x82\x10\x15a\x17\xFBWa\x17\xFBa\x15\xF5V[P\x03\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x18\x1AWa\x18\x1Aa\x15\xF5V[P\x02\x90V[`\0\x82a\x18<WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 E\xEA\xC1\x8A\x8A\xDE[\t\xDAI\xE1\xD0\xF4L\xBE\xAB\x87q@\xE9\xD7,wQQ\xF4\xE8\x9A\r\xB7\xE46dsolcC\0\x08\x0F\x003";
    /// The bytecode of the contract.
    pub static INVOKEABLEBOUNTY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x93W`\x005`\xE0\x1C\x80c\xB42X\x9D\x11a\0fW\x80c\xB42X\x9D\x14a\x01\x11W\x80c\xC5\xAB\x93\xA1\x14a\x01DW\x80c\xD5J2@\x14a\x01\x83W\x80c\xE7\xD0\x15\xF2\x14a\x01\x96W\x80c\xFB\xFAw\xCF\x14a\x01\xBDW`\0\x80\xFD[\x80c$F\xD7\x9F\x14a\0\x98W\x80c6\xDC\x91\x03\x14a\0\xB4W\x80cT\xFDMP\x14a\0\xD5W\x80c\x9F\xCA\xD8\x9A\x14a\0\xFCW[`\0\x80\xFD[a\0\xA1`\x01T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xC7a\0\xC26`\x04a\x12\tV[a\x01\xE4V[`@Qa\0\xAB\x92\x91\x90a\x12\x9BV[a\0\xA1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x0Fa\x01\n6`\x04a\x14~V[a\x04\x83V[\0[a\x014a\x01\x1F6`\x04a\x14\xCFV[`\0` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xABV[a\x01k\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xABV[a\0\xA1a\x01\x916`\x04a\x14\xE8V[a\x0C?V[a\x01k\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01k\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[``\x80`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x1B>\xD7\"`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02GW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02k\x91\x90a\x15$V[PP\x91PP`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xF4\x91\x90a\x15ZV[\x90P`\0a\x03\x02\x86\x80a\x15sV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x03NWa\x03?`@\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90a\x15\xC3V[\x81R` \x01\x90`\x01\x01\x90a\x03\"V[PPPPP\x90P`\0\x80a\x03{`@Q\x80``\x01`@R\x80\x85\x81R` \x01\x86\x81R` \x01\x87\x81RPa\x0C\xD1V[PP\x91P\x91P\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03\x9AWa\x03\x9Aa\x12\xC9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\xDFW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x03\xB8W\x90P[P\x96P`\0[\x82Q\x81\x10\x15a\x04vW`@Q\x80`@\x01`@R\x80\x84\x83\x81Q\x81\x10a\x04\x0BWa\x04\x0Ba\x15\xDFV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x83\x81Q\x81\x10a\x047Wa\x047a\x15\xDFV[` \x02` \x01\x01Q`@\x01Q\x81RP\x88\x82\x81Q\x81\x10a\x04XWa\x04Xa\x15\xDFV[` \x02` \x01\x01\x81\x90RP\x80\x80a\x04n\x90a\x16\x0BV[\x91PPa\x03\xE5V[P\x95\x97\x95\x96PPPPPPV[`\x01\x80T\x11\x15a\x04\xA6W`@Qc\xA3\xF8Q\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01U`\0a\x04\xB6\x83a\x0C?V[\x90P\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC5\xAB\x93\xA1`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x17W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05;\x91\x90a\x15ZV[\x14a\x05YW`@QceHQe`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81R` \x81\x90R`@\x90 T`\xFF\x16\x15a\x05\x89W`@Qc&\x18._`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82` \x01QB\x11\x15a\x05\xAEW`@Qc\xA0\x92\xEAQ`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA7\xD03w`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\x0EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x062\x91\x90a\x15ZV[P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x1B>\xD7\"`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x93W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xB7\x91\x90a\x15$V[PP\x91PP`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07@\x91\x90a\x15ZV[\x90P`\0\x80`\0\x80a\x07o`@Q\x80``\x01`@R\x80\x8B`\0\x01Q\x81R` \x01\x87\x81R` \x01\x88\x81RPa\x0C\xD1V[\x93P\x93P\x93P\x93P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c \x7F\xAA}`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xF9\x91\x90a\x15ZV[\x81\x10\x15a\x08\x19W`@Qc&\xC7\x9F\xFD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q`\x01b\x1F\xB1;`\xE3\x1B\x03\x19\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xFF\x02v(\x90a\x08h\x90\x87\x90`\x04\x01a\x16$V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x82W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\x96W=`\0\x80>=`\0\xFD[PPPP\x87\x15a\t\0W3c\xB7\x9D\xEA\xB4\x84a\x08\xB0\x87a\x10\xE6V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\xCD\x92\x91\x90a\x12\x9BV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\xE7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xFBW=`\0\x80>=`\0\xFD[PPPP[\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t_W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x83\x91\x90a\x15ZV[\x14a\t\xA1W`@Qc\xB9\xBF/\x03`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x83Q\x81\x10\x15a\n\xA6W\x83\x81\x81Q\x81\x10a\t\xBFWa\t\xBFa\x15\xDFV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x85\x81Q\x81\x10a\n\rWa\n\ra\x15\xDFV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x92\x90\x91\x16`$\x83\x01R`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\noW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x93\x91\x90a\x16\x89V[P\x80a\n\x9E\x81a\x16\x0BV[\x91PPa\t\xA4V[P`@Qb\x1A)\xE3`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90b\xD1O\x18\x90a\n\xF1\x90\x85\x90`\x04\x01a\x16\xA6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\x0BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\x1FW=`\0\x80>=`\0\xFD[PP`@Qc\x03\xF7\x13/`\xE6\x1B\x81Rg\r\xE0\xB6\xB3\xA7d\0\0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92Pc\xFD\xC4\xCB\xC0\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\x8CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\xA0W=`\0\x80>=`\0\xFD[PPP`\0\x97\x88RPPP` \x85\x90RPP`@\x80\x84 \x80T`\xFF\x19\x16`\x01\x17\x90U\x80Qc\"\x84\x80\xBF`\xE1\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x94cE\t\x01~\x94P`\x04\x80\x84\x01\x94P\x90\x92\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x0C\x1FW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0C3W=`\0\x80>=`\0\xFD[PP`\x01\x80UPPPPV[`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x82`@Q` \x01a\x0C\x8D\x91\x90a\x16\xF9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0C\xB4\x92\x91\x90a\x17{V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[``\x80```\0\x84`\0\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\xF5Wa\x0C\xF5a\x12\xC9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r@W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\r\x13W\x90P[P\x85QQ\x90\x94P`\x01`\x01`@\x1B\x03\x81\x11\x15a\r^Wa\r^a\x12\xC9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\xA3W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r|W\x90P[P\x85QQ\x90\x93P`\x01`\x01`@\x1B\x03\x81\x11\x15a\r\xC1Wa\r\xC1a\x12\xC9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\x06W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\xDFW\x90P[P\x91P`\0\x80`\0\x80[\x88QQ\x81\x10\x15a\x10\xD7W`\0\x89`\0\x01Q\x82\x81Q\x81\x10a\x0E2Wa\x0E2a\x15\xDFV[` \x90\x81\x02\x91\x90\x91\x01\x01QQ`@Qc\xB2\x13\xE3\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xB2\x13\xE3\xF7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xA9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xCD\x91\x90a\x16\x89V[\x15a\x0E\xE0W\x85a\x0E\xDC\x81a\x16\x0BV[\x96PP[`\0\x8A`\0\x01Q\x83\x81Q\x81\x10a\x0E\xF8Wa\x0E\xF8a\x15\xDFV[` \x02` \x01\x01Q` \x01Q\x90P`\0a\x0F\xB1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c}\xD0\x9F\xEC\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Ff\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x83W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xA7\x91\x90a\x15ZV[\x8D`@\x01Qa\x11\xE3V[\x90P\x81\x81\x11\x15a\x10\x1CW`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R3` \x82\x01R\x90\x81\x01a\x0F\xF1a\x0F\xE7\x85\x85a\x17\xE9V[\x8F` \x01Qa\x11\xE3V[\x81RP\x8B\x88\x81Q\x81\x10a\x10\x06Wa\x10\x06a\x15\xDFV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x90\x96\x01\x95a\x10\x80V[\x80\x82\x11\x15a\x10xW`@Q\x80`@\x01`@R\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x10M\x83\x85a\x0F\xE7\x91\x90a\x17\xE9V[\x81RP\x8A\x87\x81Q\x81\x10a\x10bWa\x10ba\x15\xDFV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x90\x95\x01\x94a\x10\x80V[PPPa\x10\xC5V[`@Q\x80`@\x01`@R\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81RP\x89\x86\x81Q\x81\x10a\x10\xB0Wa\x10\xB0a\x15\xDFV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x90\x92\x01\x91P[\x80a\x10\xCF\x81a\x16\x0BV[\x91PPa\x0E\x10V[P\x91\x86R\x84R\x82R\x91\x93P\x91\x93V[``\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\x01Wa\x11\x01a\x12\xC9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11FW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x11\x1FW\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x11\xDDW`@Q\x80`@\x01`@R\x80\x84\x83\x81Q\x81\x10a\x11rWa\x11ra\x15\xDFV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x83\x81Q\x81\x10a\x11\x9EWa\x11\x9Ea\x15\xDFV[` \x02` \x01\x01Q`@\x01Q\x81RP\x82\x82\x81Q\x81\x10a\x11\xBFWa\x11\xBFa\x15\xDFV[` \x02` \x01\x01\x81\x90RP\x80\x80a\x11\xD5\x90a\x16\x0BV[\x91PPa\x11LV[P\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x11\xF8\x83\x85a\x18\0V[a\x12\x02\x91\x90a\x18\x1FV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x12\x1BW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x121W`\0\x80\xFD[\x82\x01``\x81\x85\x03\x12\x15a\x12\x02W`\0\x80\xFD[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x12\x90Wa\x12}\x87\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[`@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x12WV[P\x94\x95\x94PPPPPV[`@\x81R`\0a\x12\xAE`@\x83\x01\x85a\x12CV[\x82\x81\x03` \x84\x01Ra\x12\xC0\x81\x85a\x12CV[\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x13\x01Wa\x13\x01a\x12\xC9V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x13/Wa\x13/a\x12\xC9V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x13IW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x13kWa\x13ka\x12\xC9V[`@R\x90P\x80\x825`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13\x88W`\0\x80\xFD[\x81R` \x92\x83\x015\x92\x01\x91\x90\x91R\x91\x90PV[`\0``\x82\x84\x03\x12\x15a\x13\xADW`\0\x80\xFD[a\x13\xB5a\x12\xDFV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x13\xCEW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x13\xE2W`\0\x80\xFD[\x815` \x82\x82\x11\x15a\x13\xF6Wa\x13\xF6a\x12\xC9V[a\x14\x04\x81\x83`\x05\x1B\x01a\x13\x07V[\x82\x81R\x81\x81\x01\x93P`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x87\x83\x11\x15a\x14$W`\0\x80\xFD[\x93\x81\x01\x93[\x82\x85\x10\x15a\x14MWa\x14;\x88\x86a\x137V[\x84R\x81\x84\x01\x93P`@\x85\x01\x94Pa\x14)V[\x80\x86RP\x80\x86\x015\x81\x86\x01RPPPP`@\x82\x015`@\x82\x01R\x92\x91PPV[\x80\x15\x15\x81\x14a\x14{W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\x91W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xA7W`\0\x80\xFD[a\x14\xB3\x85\x82\x86\x01a\x13\x9BV[\x92PP` \x83\x015a\x14\xC4\x81a\x14mV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x14\xE1W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x14\xFAW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\x10W`\0\x80\xFD[a\x15\x1C\x84\x82\x85\x01a\x13\x9BV[\x94\x93PPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x15:W`\0\x80\xFD[PP\x82Q` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Q\x91\x96\x90\x95P\x90\x92P\x90PV[`\0` \x82\x84\x03\x12\x15a\x15lW`\0\x80\xFD[PQ\x91\x90PV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x15\x8AW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x15\xA4W`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a\x15\xBCW`\0\x80\xFD[\x92P\x92\x90PV[`\0`@\x82\x84\x03\x12\x15a\x15\xD5W`\0\x80\xFD[a\x12\x02\x83\x83a\x137V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x16\x1DWa\x16\x1Da\x15\xF5V[P`\x01\x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x16|W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x86R\x87\x82\x01Q\x16\x87\x86\x01R\x85\x01Q\x85\x85\x01R``\x90\x93\x01\x92\x90\x85\x01\x90`\x01\x01a\x16AV[P\x91\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x16\x9BW`\0\x80\xFD[\x81Qa\x12\x02\x81a\x14mV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x16|Wa\x16\xE9\x84\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x16\xC3V[` \x80\x82R\x82Q``\x83\x83\x01R\x80Q`\x80\x84\x01\x81\x90R`\0\x92\x91\x82\x01\x90\x83\x90`\xA0\x86\x01\x90[\x80\x83\x10\x15a\x17ZWa\x17D\x82\x85Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[`@\x82\x01\x91P\x84\x84\x01\x93P`\x01\x83\x01\x92Pa\x17\x1EV[P\x83\x87\x01Q`@\x87\x01R`@\x87\x01Q``\x87\x01R\x80\x94PPPPP\x92\x91PPV[\x7Falongside::invoker::bounty\0\0\0\0\0\0\x81R`\0\x83Q`\0[\x81\x81\x10\x15a\x17\xC2W` \x81\x87\x01\x81\x01Q`\x1A\x86\x84\x01\x01R\x01a\x17\xA5V[\x81\x81\x11\x15a\x17\xD4W`\0`\x1A\x83\x86\x01\x01R[P`\x1A\x92\x01\x91\x82\x01\x92\x90\x92R`:\x01\x92\x91PPV[`\0\x82\x82\x10\x15a\x17\xFBWa\x17\xFBa\x15\xF5V[P\x03\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x18\x1AWa\x18\x1Aa\x15\xF5V[P\x02\x90V[`\0\x82a\x18<WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 E\xEA\xC1\x8A\x8A\xDE[\t\xDAI\xE1\xD0\xF4L\xBE\xAB\x87q@\xE9\xD7,wQQ\xF4\xE8\x9A\r\xB7\xE46dsolcC\0\x08\x0F\x003";
    /// The deployed bytecode of the contract.
    pub static INVOKEABLEBOUNTY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct InvokeableBounty<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for InvokeableBounty<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for InvokeableBounty<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for InvokeableBounty<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for InvokeableBounty<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(InvokeableBounty))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> InvokeableBounty<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    INVOKEABLEBOUNTY_ABI.clone(),
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
                INVOKEABLEBOUNTY_ABI.clone(),
                INVOKEABLEBOUNTY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `activeBounty` (0xc5ab93a1) function
        pub fn active_bounty(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([197, 171, 147, 161], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `completedBounties` (0xb432589d) function
        pub fn completed_bounties(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([180, 50, 88, 157], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fulfillBounty` (0x9fcad89a) function
        pub fn fulfill_bounty(
            &self,
            bounty: Bounty,
            callback: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 202, 216, 154], (bounty, callback))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hashBounty` (0xd54a3240) function
        pub fn hash_bounty(
            &self,
            bounty: Bounty,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([213, 74, 50, 64], (bounty,))
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
        ///Calls the contract's `quoteBounty` (0x36dc9103) function
        pub fn quote_bounty(
            &self,
            bounty: Bounty,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<TokenInfo>, ::std::vec::Vec<TokenInfo>),
        > {
            self.0
                .method_hash([54, 220, 145, 3], (bounty,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reentrancyLock` (0x2446d79f) function
        pub fn reentrancy_lock(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([36, 70, 215, 159], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vault` (0xfbfa77cf) function
        pub fn vault(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([251, 250, 119, 207], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `version` (0x54fd4d50) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for InvokeableBounty<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `BountyAMKTSupplyChange` with signature `BountyAMKTSupplyChange()` and selector `0xb9bf2f03`
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
    #[etherror(name = "BountyAMKTSupplyChange", abi = "BountyAMKTSupplyChange()")]
    pub struct BountyAMKTSupplyChange;
    ///Custom Error type `BountyAlreadyCompleted` with signature `BountyAlreadyCompleted()` and selector `0x4c305cbe`
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
    #[etherror(name = "BountyAlreadyCompleted", abi = "BountyAlreadyCompleted()")]
    pub struct BountyAlreadyCompleted;
    ///Custom Error type `BountyInvalidHash` with signature `BountyInvalidHash()` and selector `0x65485165`
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
    #[etherror(name = "BountyInvalidHash", abi = "BountyInvalidHash()")]
    pub struct BountyInvalidHash;
    ///Custom Error type `BountyMustIncludeAllUnderlyings` with signature `BountyMustIncludeAllUnderlyings()` and selector `0x4d8f3ffa`
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
    #[etherror(
        name = "BountyMustIncludeAllUnderlyings",
        abi = "BountyMustIncludeAllUnderlyings()"
    )]
    pub struct BountyMustIncludeAllUnderlyings;
    ///Custom Error type `BountyPastDeadline` with signature `BountyPastDeadline()` and selector `0xa092ea51`
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
    #[etherror(name = "BountyPastDeadline", abi = "BountyPastDeadline()")]
    pub struct BountyPastDeadline;
    ///Custom Error type `BountyReentrant` with signature `BountyReentrant()` and selector `0xa3f851df`
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
    #[etherror(name = "BountyReentrant", abi = "BountyReentrant()")]
    pub struct BountyReentrant;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum InvokeableBountyErrors {
        BountyAMKTSupplyChange(BountyAMKTSupplyChange),
        BountyAlreadyCompleted(BountyAlreadyCompleted),
        BountyInvalidHash(BountyInvalidHash),
        BountyMustIncludeAllUnderlyings(BountyMustIncludeAllUnderlyings),
        BountyPastDeadline(BountyPastDeadline),
        BountyReentrant(BountyReentrant),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for InvokeableBountyErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <BountyAMKTSupplyChange as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BountyAMKTSupplyChange(decoded));
            }
            if let Ok(decoded)
                = <BountyAlreadyCompleted as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BountyAlreadyCompleted(decoded));
            }
            if let Ok(decoded)
                = <BountyInvalidHash as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BountyInvalidHash(decoded));
            }
            if let Ok(decoded)
                = <BountyMustIncludeAllUnderlyings as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BountyMustIncludeAllUnderlyings(decoded));
            }
            if let Ok(decoded)
                = <BountyPastDeadline as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BountyPastDeadline(decoded));
            }
            if let Ok(decoded)
                = <BountyReentrant as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BountyReentrant(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for InvokeableBountyErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BountyAMKTSupplyChange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BountyAlreadyCompleted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BountyInvalidHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BountyMustIncludeAllUnderlyings(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BountyPastDeadline(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BountyReentrant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for InvokeableBountyErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BountyAMKTSupplyChange as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BountyAlreadyCompleted as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BountyInvalidHash as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BountyMustIncludeAllUnderlyings as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BountyPastDeadline as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BountyReentrant as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for InvokeableBountyErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BountyAMKTSupplyChange(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BountyAlreadyCompleted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BountyInvalidHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::BountyMustIncludeAllUnderlyings(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BountyPastDeadline(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BountyReentrant(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for InvokeableBountyErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BountyAMKTSupplyChange> for InvokeableBountyErrors {
        fn from(value: BountyAMKTSupplyChange) -> Self {
            Self::BountyAMKTSupplyChange(value)
        }
    }
    impl ::core::convert::From<BountyAlreadyCompleted> for InvokeableBountyErrors {
        fn from(value: BountyAlreadyCompleted) -> Self {
            Self::BountyAlreadyCompleted(value)
        }
    }
    impl ::core::convert::From<BountyInvalidHash> for InvokeableBountyErrors {
        fn from(value: BountyInvalidHash) -> Self {
            Self::BountyInvalidHash(value)
        }
    }
    impl ::core::convert::From<BountyMustIncludeAllUnderlyings>
    for InvokeableBountyErrors {
        fn from(value: BountyMustIncludeAllUnderlyings) -> Self {
            Self::BountyMustIncludeAllUnderlyings(value)
        }
    }
    impl ::core::convert::From<BountyPastDeadline> for InvokeableBountyErrors {
        fn from(value: BountyPastDeadline) -> Self {
            Self::BountyPastDeadline(value)
        }
    }
    impl ::core::convert::From<BountyReentrant> for InvokeableBountyErrors {
        fn from(value: BountyReentrant) -> Self {
            Self::BountyReentrant(value)
        }
    }
    ///Container type for all input parameters for the `activeBounty` function with signature `activeBounty()` and selector `0xc5ab93a1`
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
    #[ethcall(name = "activeBounty", abi = "activeBounty()")]
    pub struct ActiveBountyCall;
    ///Container type for all input parameters for the `completedBounties` function with signature `completedBounties(bytes32)` and selector `0xb432589d`
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
    #[ethcall(name = "completedBounties", abi = "completedBounties(bytes32)")]
    pub struct CompletedBountiesCall(pub [u8; 32]);
    ///Container type for all input parameters for the `fulfillBounty` function with signature `fulfillBounty(((address,uint256)[],uint256,bytes32),bool)` and selector `0x9fcad89a`
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
        name = "fulfillBounty",
        abi = "fulfillBounty(((address,uint256)[],uint256,bytes32),bool)"
    )]
    pub struct FulfillBountyCall {
        pub bounty: Bounty,
        pub callback: bool,
    }
    ///Container type for all input parameters for the `hashBounty` function with signature `hashBounty(((address,uint256)[],uint256,bytes32))` and selector `0xd54a3240`
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
        name = "hashBounty",
        abi = "hashBounty(((address,uint256)[],uint256,bytes32))"
    )]
    pub struct HashBountyCall {
        pub bounty: Bounty,
    }
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
    ///Container type for all input parameters for the `quoteBounty` function with signature `quoteBounty(((address,uint256)[],uint256,bytes32))` and selector `0x36dc9103`
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
        name = "quoteBounty",
        abi = "quoteBounty(((address,uint256)[],uint256,bytes32))"
    )]
    pub struct QuoteBountyCall {
        pub bounty: Bounty,
    }
    ///Container type for all input parameters for the `reentrancyLock` function with signature `reentrancyLock()` and selector `0x2446d79f`
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
    #[ethcall(name = "reentrancyLock", abi = "reentrancyLock()")]
    pub struct ReentrancyLockCall;
    ///Container type for all input parameters for the `vault` function with signature `vault()` and selector `0xfbfa77cf`
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
    #[ethcall(name = "vault", abi = "vault()")]
    pub struct VaultCall;
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum InvokeableBountyCalls {
        ActiveBounty(ActiveBountyCall),
        CompletedBounties(CompletedBountiesCall),
        FulfillBounty(FulfillBountyCall),
        HashBounty(HashBountyCall),
        IndexToken(IndexTokenCall),
        QuoteBounty(QuoteBountyCall),
        ReentrancyLock(ReentrancyLockCall),
        Vault(VaultCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for InvokeableBountyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ActiveBountyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ActiveBounty(decoded));
            }
            if let Ok(decoded)
                = <CompletedBountiesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CompletedBounties(decoded));
            }
            if let Ok(decoded)
                = <FulfillBountyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FulfillBounty(decoded));
            }
            if let Ok(decoded)
                = <HashBountyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HashBounty(decoded));
            }
            if let Ok(decoded)
                = <IndexTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IndexToken(decoded));
            }
            if let Ok(decoded)
                = <QuoteBountyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QuoteBounty(decoded));
            }
            if let Ok(decoded)
                = <ReentrancyLockCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ReentrancyLock(decoded));
            }
            if let Ok(decoded)
                = <VaultCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Vault(decoded));
            }
            if let Ok(decoded)
                = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for InvokeableBountyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ActiveBounty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompletedBounties(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FulfillBounty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HashBounty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IndexToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteBounty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReentrancyLock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Vault(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for InvokeableBountyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ActiveBounty(element) => ::core::fmt::Display::fmt(element, f),
                Self::CompletedBounties(element) => ::core::fmt::Display::fmt(element, f),
                Self::FulfillBounty(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashBounty(element) => ::core::fmt::Display::fmt(element, f),
                Self::IndexToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuoteBounty(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReentrancyLock(element) => ::core::fmt::Display::fmt(element, f),
                Self::Vault(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ActiveBountyCall> for InvokeableBountyCalls {
        fn from(value: ActiveBountyCall) -> Self {
            Self::ActiveBounty(value)
        }
    }
    impl ::core::convert::From<CompletedBountiesCall> for InvokeableBountyCalls {
        fn from(value: CompletedBountiesCall) -> Self {
            Self::CompletedBounties(value)
        }
    }
    impl ::core::convert::From<FulfillBountyCall> for InvokeableBountyCalls {
        fn from(value: FulfillBountyCall) -> Self {
            Self::FulfillBounty(value)
        }
    }
    impl ::core::convert::From<HashBountyCall> for InvokeableBountyCalls {
        fn from(value: HashBountyCall) -> Self {
            Self::HashBounty(value)
        }
    }
    impl ::core::convert::From<IndexTokenCall> for InvokeableBountyCalls {
        fn from(value: IndexTokenCall) -> Self {
            Self::IndexToken(value)
        }
    }
    impl ::core::convert::From<QuoteBountyCall> for InvokeableBountyCalls {
        fn from(value: QuoteBountyCall) -> Self {
            Self::QuoteBounty(value)
        }
    }
    impl ::core::convert::From<ReentrancyLockCall> for InvokeableBountyCalls {
        fn from(value: ReentrancyLockCall) -> Self {
            Self::ReentrancyLock(value)
        }
    }
    impl ::core::convert::From<VaultCall> for InvokeableBountyCalls {
        fn from(value: VaultCall) -> Self {
            Self::Vault(value)
        }
    }
    impl ::core::convert::From<VersionCall> for InvokeableBountyCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `activeBounty` function with signature `activeBounty()` and selector `0xc5ab93a1`
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
    pub struct ActiveBountyReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `completedBounties` function with signature `completedBounties(bytes32)` and selector `0xb432589d`
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
    pub struct CompletedBountiesReturn(pub bool);
    ///Container type for all return fields from the `hashBounty` function with signature `hashBounty(((address,uint256)[],uint256,bytes32))` and selector `0xd54a3240`
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
    pub struct HashBountyReturn {
        pub hash: [u8; 32],
    }
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
    ///Container type for all return fields from the `quoteBounty` function with signature `quoteBounty(((address,uint256)[],uint256,bytes32))` and selector `0x36dc9103`
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
    pub struct QuoteBountyReturn {
        pub outs: ::std::vec::Vec<TokenInfo>,
        pub p1: ::std::vec::Vec<TokenInfo>,
    }
    ///Container type for all return fields from the `reentrancyLock` function with signature `reentrancyLock()` and selector `0x2446d79f`
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
    pub struct ReentrancyLockReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `vault` function with signature `vault()` and selector `0xfbfa77cf`
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
    pub struct VaultReturn(pub ::ethers::core::types::Address);
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
    pub struct VersionReturn(pub ::ethers::core::types::U256);
    ///`Bounty((address,uint256)[],uint256,bytes32)`
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
    pub struct Bounty {
        pub infos: ::std::vec::Vec<TokenInfo>,
        pub deadline: ::ethers::core::types::U256,
        pub salt: [u8; 32],
    }
}
