pub use issuance::*;
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
pub mod issuance {
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
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("issue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("issue"),
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
                    ::std::borrow::ToOwned::to_owned("quote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quote"),
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
                (
                    ::std::borrow::ToOwned::to_owned("redeem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("redeem"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ISSUANCE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\r\x108\x03\x80a\r\x10\x839\x81\x01`@\x81\x90Ra\0/\x91a\0@V[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0pV[`\0` \x82\x84\x03\x12\x15a\0RW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0iW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x0C:a\0\xD6`\09`\0\x81\x81`\xA7\x01R\x81\x81`\xE3\x01R\x81\x81a\x01j\x01R\x81\x81a\x02\xB8\x01R\x81\x81a\x03\\\x01R\x81\x81a\x03\xC3\x01R\x81\x81a\x047\x01R\x81\x81a\x04\xBE\x01R\x81\x81a\x06\xB8\x01R\x81\x81a\x074\x01R\x81\x81a\x07\xA1\x01Ra\x07\xFE\x01Ra\x0C:`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\xCC\x87+f\x14a\0QW\x80c\xDB\0ju\x14a\0fW\x80c\xED\x1B\xD7l\x14a\0yW\x80c\xFB\xFAw\xCF\x14a\0\xA2W[`\0\x80\xFD[a\0da\0_6`\x04a\t\x0FV[a\0\xE1V[\0[a\0da\0t6`\x04a\t\x0FV[a\x045V[a\0\x8Ca\0\x876`\x04a\t\x0FV[a\x07\xF8V[`@Qa\0\x99\x91\x90a\t(V[`@Q\x80\x91\x03\x90\xF3[a\0\xC9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x99V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA7\xD03w`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01e\x91\x90a\t\x80V[P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x81\xA0\xC8U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xC6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x01\xEE\x91\x90\x81\x01\x90a\n\tV[\x90P`\0\x81Q\x11a\x02;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq\x13\x9B\xC8\x1D\x1B\xDA\xD9[\x9C\xC8\x1A[\x88\x1D\x98][\x1D`r\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0[\x81Q\x81\x10\x15a\x03?W`\0a\x02p\x83\x83\x81Q\x81\x10a\x02^Wa\x02^a\n\xECV[` \x02` \x01\x01Q` \x01Q\x85a\x08\xE9V[a\x02{\x90`\x01a\x0B\x18V[\x90P\x82\x82\x81Q\x81\x10a\x02\x8FWa\x02\x8Fa\n\xECV[` \x90\x81\x02\x91\x90\x91\x01\x01QQ`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`$\x83\x01R`D\x82\x01\x84\x90R\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x035\x91\x90a\x0B0V[PP`\x01\x01a\x02>V[P`@Qco\x0F\x17u`\xE1\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDE\x1E.\xEA\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\xA8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xBCW=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cE\t\x01~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\x1AW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x04.W=`\0\x80>=`\0\xFD[PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA7\xD03w`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xB9\x91\x90a\t\x80V[P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x81\xA0\xC8U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05B\x91\x90\x81\x01\x90a\n\tV[\x90P`\0\x81Q\x11a\x05\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq\x13\x9B\xC8\x1D\x1B\xDA\xD9[\x9C\xC8\x1A[\x88\x1D\x98][\x1D`r\x1B`D\x82\x01R`d\x01a\x022V[`\0\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xA6Wa\x05\xA6a\t\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xF1W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x05\xC4W\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x06\x9BW`\0a\x06)\x84\x83\x81Q\x81\x10a\x06\x17Wa\x06\x17a\n\xECV[` \x02` \x01\x01Q` \x01Q\x86a\x08\xE9V[\x90P`@Q\x80``\x01`@R\x80\x85\x84\x81Q\x81\x10a\x06HWa\x06Ha\n\xECV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x013`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81RP\x83\x83\x81Q\x81\x10a\x06\x87Wa\x06\x87a\n\xECV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x05\xF7V[P`@Qc\x01\xB6\xAC\xAF`\xE6\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cm\xAB+\xC0\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x04W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\x18W=`\0\x80>=`\0\xFD[PP`@Q`\x01b\x1F\xB1;`\xE3\x1B\x03\x19\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xFF\x02v(\x91Pa\x07k\x90\x84\x90`\x04\x01a\x0BRV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x85W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\x99W=`\0\x80>=`\0\xFD[PPPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cE\t\x01~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\x1AW`\0\x80\xFD[```\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x81\xA0\xC8U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x08\x82\x91\x90\x81\x01\x90a\n\tV[\x90P`\0[\x81Q\x81\x10\x15a\x08\xE2Wa\x08\xA5\x82\x82\x81Q\x81\x10a\x02^Wa\x02^a\n\xECV[a\x08\xB0\x90`\x01a\x0B\x18V[\x82\x82\x81Q\x81\x10a\x08\xC2Wa\x08\xC2a\n\xECV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x01R\x80a\x08\xDA\x81a\x0B\xAAV[\x91PPa\x08\x87V[P\x92\x91PPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x08\xFE\x83\x85a\x0B\xC3V[a\t\x08\x91\x90a\x0B\xE2V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\t!W`\0\x80\xFD[P5\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\tsW\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x86\x01Q\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\tEV[P\x91\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\t\x92W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\t\xD2Wa\t\xD2a\t\x99V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\x01Wa\n\x01a\t\x99V[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\n\x1CW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n4W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\nHW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\nZWa\nZa\t\x99V[a\nh\x84\x82`\x05\x1B\x01a\t\xD8V[\x81\x81R\x84\x81\x01\x92P`\x06\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x87\x82\x11\x15a\n\x88W`\0\x80\xFD[\x92\x84\x01\x92[\x81\x84\x10\x15a\n\xE1W`@\x84\x89\x03\x12\x15a\n\xA6W`\0\x80\x81\xFD[a\n\xAEa\t\xAFV[\x84Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\xC6W`\0\x80\x81\xFD[\x81R\x84\x86\x01Q\x86\x82\x01R\x83R`@\x90\x93\x01\x92\x91\x84\x01\x91a\n\x8DV[\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x0B+Wa\x0B+a\x0B\x02V[P\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x0BBW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\t\x08W`\0\x80\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\tsW\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x86R\x87\x82\x01Q\x16\x87\x86\x01R\x85\x01Q\x85\x85\x01R``\x90\x93\x01\x92\x90\x85\x01\x90`\x01\x01a\x0BoV[`\0`\x01\x82\x01a\x0B\xBCWa\x0B\xBCa\x0B\x02V[P`\x01\x01\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x0B\xDDWa\x0B\xDDa\x0B\x02V[P\x02\x90V[`\0\x82a\x0B\xFFWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \x94\x9F\xCBB\xF2\x95\x96\x86$3abq\x88\x987N\0\xE6\xD3\xFF\xB1\x1C\xF9\xF6\xDE\xE3\x07p\0iTdsolcC\0\x08\x0F\x003";
    /// The bytecode of the contract.
    pub static ISSUANCE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\xCC\x87+f\x14a\0QW\x80c\xDB\0ju\x14a\0fW\x80c\xED\x1B\xD7l\x14a\0yW\x80c\xFB\xFAw\xCF\x14a\0\xA2W[`\0\x80\xFD[a\0da\0_6`\x04a\t\x0FV[a\0\xE1V[\0[a\0da\0t6`\x04a\t\x0FV[a\x045V[a\0\x8Ca\0\x876`\x04a\t\x0FV[a\x07\xF8V[`@Qa\0\x99\x91\x90a\t(V[`@Q\x80\x91\x03\x90\xF3[a\0\xC9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x99V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA7\xD03w`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01e\x91\x90a\t\x80V[P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x81\xA0\xC8U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xC6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x01\xEE\x91\x90\x81\x01\x90a\n\tV[\x90P`\0\x81Q\x11a\x02;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq\x13\x9B\xC8\x1D\x1B\xDA\xD9[\x9C\xC8\x1A[\x88\x1D\x98][\x1D`r\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0[\x81Q\x81\x10\x15a\x03?W`\0a\x02p\x83\x83\x81Q\x81\x10a\x02^Wa\x02^a\n\xECV[` \x02` \x01\x01Q` \x01Q\x85a\x08\xE9V[a\x02{\x90`\x01a\x0B\x18V[\x90P\x82\x82\x81Q\x81\x10a\x02\x8FWa\x02\x8Fa\n\xECV[` \x90\x81\x02\x91\x90\x91\x01\x01QQ`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`$\x83\x01R`D\x82\x01\x84\x90R\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x035\x91\x90a\x0B0V[PP`\x01\x01a\x02>V[P`@Qco\x0F\x17u`\xE1\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDE\x1E.\xEA\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\xA8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xBCW=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cE\t\x01~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\x1AW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x04.W=`\0\x80>=`\0\xFD[PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA7\xD03w`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xB9\x91\x90a\t\x80V[P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x81\xA0\xC8U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05B\x91\x90\x81\x01\x90a\n\tV[\x90P`\0\x81Q\x11a\x05\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq\x13\x9B\xC8\x1D\x1B\xDA\xD9[\x9C\xC8\x1A[\x88\x1D\x98][\x1D`r\x1B`D\x82\x01R`d\x01a\x022V[`\0\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xA6Wa\x05\xA6a\t\x99V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xF1W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x05\xC4W\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x06\x9BW`\0a\x06)\x84\x83\x81Q\x81\x10a\x06\x17Wa\x06\x17a\n\xECV[` \x02` \x01\x01Q` \x01Q\x86a\x08\xE9V[\x90P`@Q\x80``\x01`@R\x80\x85\x84\x81Q\x81\x10a\x06HWa\x06Ha\n\xECV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x013`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81RP\x83\x83\x81Q\x81\x10a\x06\x87Wa\x06\x87a\n\xECV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x05\xF7V[P`@Qc\x01\xB6\xAC\xAF`\xE6\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x84\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cm\xAB+\xC0\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x04W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\x18W=`\0\x80>=`\0\xFD[PP`@Q`\x01b\x1F\xB1;`\xE3\x1B\x03\x19\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xFF\x02v(\x91Pa\x07k\x90\x84\x90`\x04\x01a\x0BRV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x85W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\x99W=`\0\x80>=`\0\xFD[PPPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cE\t\x01~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\x1AW`\0\x80\xFD[```\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x81\xA0\xC8U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x08\x82\x91\x90\x81\x01\x90a\n\tV[\x90P`\0[\x81Q\x81\x10\x15a\x08\xE2Wa\x08\xA5\x82\x82\x81Q\x81\x10a\x02^Wa\x02^a\n\xECV[a\x08\xB0\x90`\x01a\x0B\x18V[\x82\x82\x81Q\x81\x10a\x08\xC2Wa\x08\xC2a\n\xECV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x01R\x80a\x08\xDA\x81a\x0B\xAAV[\x91PPa\x08\x87V[P\x92\x91PPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x08\xFE\x83\x85a\x0B\xC3V[a\t\x08\x91\x90a\x0B\xE2V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\t!W`\0\x80\xFD[P5\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\tsW\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x86\x01Q\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\tEV[P\x91\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\t\x92W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\t\xD2Wa\t\xD2a\t\x99V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\x01Wa\n\x01a\t\x99V[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\n\x1CW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n4W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\nHW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\nZWa\nZa\t\x99V[a\nh\x84\x82`\x05\x1B\x01a\t\xD8V[\x81\x81R\x84\x81\x01\x92P`\x06\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x87\x82\x11\x15a\n\x88W`\0\x80\xFD[\x92\x84\x01\x92[\x81\x84\x10\x15a\n\xE1W`@\x84\x89\x03\x12\x15a\n\xA6W`\0\x80\x81\xFD[a\n\xAEa\t\xAFV[\x84Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\xC6W`\0\x80\x81\xFD[\x81R\x84\x86\x01Q\x86\x82\x01R\x83R`@\x90\x93\x01\x92\x91\x84\x01\x91a\n\x8DV[\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x0B+Wa\x0B+a\x0B\x02V[P\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x0BBW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\t\x08W`\0\x80\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\tsW\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x86R\x87\x82\x01Q\x16\x87\x86\x01R\x85\x01Q\x85\x85\x01R``\x90\x93\x01\x92\x90\x85\x01\x90`\x01\x01a\x0BoV[`\0`\x01\x82\x01a\x0B\xBCWa\x0B\xBCa\x0B\x02V[P`\x01\x01\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x0B\xDDWa\x0B\xDDa\x0B\x02V[P\x02\x90V[`\0\x82a\x0B\xFFWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \x94\x9F\xCBB\xF2\x95\x96\x86$3abq\x88\x987N\0\xE6\xD3\xFF\xB1\x1C\xF9\xF6\xDE\xE3\x07p\0iTdsolcC\0\x08\x0F\x003";
    /// The deployed bytecode of the contract.
    pub static ISSUANCE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Issuance<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Issuance<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Issuance<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Issuance<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Issuance<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Issuance)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Issuance<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ISSUANCE_ABI.clone(),
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
                ISSUANCE_ABI.clone(),
                ISSUANCE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `issue` (0xcc872b66) function
        pub fn issue(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([204, 135, 43, 102], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quote` (0xed1bd76c) function
        pub fn quote(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<TokenInfo>> {
            self.0
                .method_hash([237, 27, 215, 108], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redeem` (0xdb006a75) function
        pub fn redeem(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 0, 106, 117], amount)
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Issuance<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `issue` function with signature `issue(uint256)` and selector `0xcc872b66`
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
    #[ethcall(name = "issue", abi = "issue(uint256)")]
    pub struct IssueCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `quote` function with signature `quote(uint256)` and selector `0xed1bd76c`
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
    #[ethcall(name = "quote", abi = "quote(uint256)")]
    pub struct QuoteCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `redeem` function with signature `redeem(uint256)` and selector `0xdb006a75`
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
    #[ethcall(name = "redeem", abi = "redeem(uint256)")]
    pub struct RedeemCall {
        pub amount: ::ethers::core::types::U256,
    }
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IssuanceCalls {
        Issue(IssueCall),
        Quote(QuoteCall),
        Redeem(RedeemCall),
        Vault(VaultCall),
    }
    impl ::ethers::core::abi::AbiDecode for IssuanceCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <IssueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Issue(decoded));
            }
            if let Ok(decoded) = <QuoteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Quote(decoded));
            }
            if let Ok(decoded) = <RedeemCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Redeem(decoded));
            }
            if let Ok(decoded) = <VaultCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Vault(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IssuanceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Issue(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Quote(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Redeem(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Vault(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IssuanceCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Issue(element) => ::core::fmt::Display::fmt(element, f),
                Self::Quote(element) => ::core::fmt::Display::fmt(element, f),
                Self::Redeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::Vault(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IssueCall> for IssuanceCalls {
        fn from(value: IssueCall) -> Self {
            Self::Issue(value)
        }
    }
    impl ::core::convert::From<QuoteCall> for IssuanceCalls {
        fn from(value: QuoteCall) -> Self {
            Self::Quote(value)
        }
    }
    impl ::core::convert::From<RedeemCall> for IssuanceCalls {
        fn from(value: RedeemCall) -> Self {
            Self::Redeem(value)
        }
    }
    impl ::core::convert::From<VaultCall> for IssuanceCalls {
        fn from(value: VaultCall) -> Self {
            Self::Vault(value)
        }
    }
    ///Container type for all return fields from the `quote` function with signature `quote(uint256)` and selector `0xed1bd76c`
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
    pub struct QuoteReturn(pub ::std::vec::Vec<TokenInfo>);
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
}
