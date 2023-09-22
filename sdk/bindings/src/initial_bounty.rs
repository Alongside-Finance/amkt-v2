pub use initial_bounty::*;
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
pub mod initial_bounty {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("IS_SCRIPT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("IS_SCRIPT"),
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
                    ::std::borrow::ToOwned::to_owned("approveTokenList"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approveTokenList"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialBounty"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialBounty"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("run"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("run"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setTokenUnits"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setTokenUnits"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenUnits"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenUnits"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static INITIALBOUNTY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x04\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x0C\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15a\0-W`\0\x80\xFD[Pa\t\xB8\x80a\0=`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80cV!\xBC\x0E\x14a\0gW\x80cp\x81\xBD\x9E\x14a\0qW\x80c\x8Eb\xEDQ\x14a\0\x8FW\x80c\x94\xEF\x1F\xDE\x14a\0\xBDW\x80c\xC0@b&\x14a\0\xC5W\x80c\xF8\xCC\xBFG\x14a\0\xCDW[`\0\x80\xFD[a\0oa\0\xEAV[\0[a\0ya\x02\x05V[`@Qa\0\x86\x91\x90a\x08mV[`@Q\x80\x91\x03\x90\xF3[a\0\xAFa\0\x9D6`\x04a\x08\x87V[`\x10` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\0\x86V[a\0oa\x03xV[a\0oa\x048V[`\x0CTa\0\xDA\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\x86V[`\x10` R`\x01\x7F\x05K\x96t[y\xA8\xFF\xBD\x91\x07.\xE1\xA0N\x15\xB5\xB51E\x8E\x8C\xC1\x7F3\x90\xEF\xD0\x15~\xF1N\x81\x90U`d\x7F\x18U=\x8Fd\xAE5\xEB.r\x84M\x1A\xF4e\x89c\x0C\xBC\xF4\xDA~oy\xD8\xB0\xABNjn\x04a\x81\x90U\x7F@!\xAD\x11\xA5fFFYnr\xEB\x8F\x059\x0F\xF5\x1F=r\xD1R\x07\x03?\x007\x10\xE9\xD0\xAF,\x81\x90U\x7FCt\x15\xBBE\xCA\xDAl\xFB\xDA\x94( e%\x91^9\\\x89\xE6\xBB/\x15<\xA0.B\x91\nl\xEC\x81\x90U\x7F\xBC\xD2\x91\xA1\xF9\xB3?\x1ExD\xFBQ\xEB\xDB\xC0S^Pq\x8C\x03\xDC\xABV5\xAFr\xA5\xC7\xF8\x9F\x8D\x81\x90U\x7Fn\x98\x12q\x16\xD636\rJ5Y\x87\x9C\xA9\xB9\xBAO\xCB\x1Bg\xB9\xAF\x9A\xD3\xC0\x86A\xCCM\xBC\xEEUs\xD3\x1AY\xC8Z\xE9\xD8\xED\xEF\xECA\x1DD\x8F\x90\x84\x15q\xB8\x9C`\0R\x7F\x19\x04\xE5\xA5\"H\x0B\xAA7\xE1\xD3>\x075or\xE0\xE8S\xAB\x10\xA3d\xD3P\xF2\xD5\xB107\xE0AUV[`@\x80Q``\x80\x82\x01\x83R\x81R`\0` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\rT`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02@Wa\x02@a\x08\xB0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02\x85W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x02^W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x03\x18W`\0`\r\x82\x81T\x81\x10a\x02\xA8Wa\x02\xA8a\x08\xC6V[`\0\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x80\x83R`\x10\x82R`@\x92\x83\x90 T\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x91\x83\x01\x82\x90R\x85Q\x90\x93P\x90\x91\x90\x85\x90\x85\x90\x81\x10a\x02\xF8Wa\x02\xF8a\x08\xC6V[` \x02` \x01\x01\x81\x90RPPP\x80\x80a\x03\x10\x90a\x08\xF2V[\x91PPa\x02\x8BV[P`\0`@Q\x80``\x01`@R\x80\x83\x81R` \x01`\x11Ta\x03\xE8a\x03<\x91\x90a\t\x0BV[\x81R` \x01`\x11T`@Q` \x01a\x03V\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x90R\x94\x93PPPPV[`\0[`\rT\x81\x10\x15a\x045W`\r\x81\x81T\x81\x10a\x03\x98Wa\x03\x98a\x08\xC6V[`\0\x91\x82R` \x90\x91 \x01T`\x0ET`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Rh\x05k\xC7^-c\x10\0\0`$\x82\x01R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\xFEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\"\x91\x90a\t#V[P\x80a\x04-\x81a\x08\xF2V[\x91PPa\x03{V[PV[`\x0E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16s\xEE\xE9\xD3\xAA\xE6b\xFF\xDB\x94\xCB{\xEA\xC1\x1A\xAF\x15K\x10J[\x17\x90\x91U`\x0F\x80T\x90\x91\x16s\xD4b\x8FD\xAF\xB3\x82\xEC\xC5\xF9\x82]\xBD>\xE5\x8B\xCF\xC0\x9E\xE8\x17\x90U`@\x80Q`\xE0\x81\x01\x82Rs\"`\xFA\xC5\xE5T*w:\xA4O\xBC\xFE\xDF|\x19;\xC2\xC5\x99\x81Rs\xC0*\xAA9\xB2#\xFE\x8D\n\x0E\\O'\xEA\xD9\x08<ul\xC2` \x82\x01Rs}\x1A\xFA{q\x8F\xB8\x93\xDB0\xA3\xAB\xC0\xCF\xC6\x08\xAA\xCF\xEB\xB0\x91\x81\x01\x91\x90\x91Rs\x85\xF18\xBF\xEEN\xF8\xE5@\x89\x0C\xFBH\xF6 W\x1Dg\xED\xA3``\x82\x01RsA\x8Du\xF6Z\x02\xB3\xD5;$\x18\xFB\x8E\x1F\xE4\x93u\x9Cv\x05`\x80\x82\x01RsN\x156\x1F\xD6\xB4\xBB`\x9F\xA6<\x81\xA2\xBE\x19\xD8sqxp`\xA0\x82\x01Rs\xD3\x1AY\xC8Z\xE9\xD8\xED\xEF\xECA\x1DD\x8F\x90\x84\x15q\xB8\x9C`\xC0\x82\x01Ra\x05M\x90`\r\x90`\x07a\x07\x83V[Pa\x05Va\0\xEAV[B`\x11U`\0a\x05da\x02\x05V[`@Qc\x7F\xEC*\x8D`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x7F\xEC*\x8D\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xB3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xC7W=`\0\x80>=`\0\xFD[PPPPa\x05\xD3a\x03xV[`\x0ET`@Qc\x03U(\xC9`\xE6\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD5J2@\x90a\x06\x04\x90\x85\x90`\x04\x01a\x08mV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06E\x91\x90a\tEV[`\x0FT`@Qc\x0CLB\x85`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\x0CLB\x85\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\x8CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xA0W=`\0\x80>=`\0\xFD[PP`\x0ET`@QcO\xE5lM`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\x9F\xCA\xD8\x9A\x91Pa\x06\xD7\x90\x85\x90`\0\x90`\x04\x01a\t^V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xF1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\x05W=`\0\x80>=`\0\xFD[PPPP\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cv\xEA\xDD6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07gW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07{W=`\0\x80>=`\0\xFD[PPPPPPV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x07\xD8W\x91` \x02\x82\x01[\x82\x81\x11\x15a\x07\xD8W\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a\x07\xA3V[Pa\x07\xE4\x92\x91Pa\x07\xE8V[P\x90V[[\x80\x82\x11\x15a\x07\xE4W`\0\x81U`\x01\x01a\x07\xE9V[\x80Q``\x80\x84R\x81Q\x90\x84\x01\x81\x90R`\0\x91` \x91\x90\x82\x01\x90`\x80\x86\x01\x90\x84[\x81\x81\x10\x15a\x08MW\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x85\x01Q\x85\x84\x01R\x92\x84\x01\x92`@\x90\x92\x01\x91`\x01\x01a\x08\x1DV[PP\x82\x85\x01Q\x83\x87\x01R`@\x85\x01Q`@\x87\x01R\x80\x93PPPP\x92\x91PPV[` \x81R`\0a\x08\x80` \x83\x01\x84a\x07\xFDV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x08\x99W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\x80W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\t\x04Wa\t\x04a\x08\xDCV[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15a\t\x1EWa\t\x1Ea\x08\xDCV[P\x01\x90V[`\0` \x82\x84\x03\x12\x15a\t5W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x08\x80W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\tWW`\0\x80\xFD[PQ\x91\x90PV[`@\x81R`\0a\tq`@\x83\x01\x85a\x07\xFDV[\x90P\x82\x15\x15` \x83\x01R\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x9F\x93\xB7;_\x9C\xECJ\xA6d\x15\xE6e\xD2\n\x9BQ\\\x81\xC5\xB3\x1A,9\xE7_\xDD\xFCj\x91\x80\xCBdsolcC\0\x08\x0F\x003";
    /// The bytecode of the contract.
    pub static INITIALBOUNTY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80cV!\xBC\x0E\x14a\0gW\x80cp\x81\xBD\x9E\x14a\0qW\x80c\x8Eb\xEDQ\x14a\0\x8FW\x80c\x94\xEF\x1F\xDE\x14a\0\xBDW\x80c\xC0@b&\x14a\0\xC5W\x80c\xF8\xCC\xBFG\x14a\0\xCDW[`\0\x80\xFD[a\0oa\0\xEAV[\0[a\0ya\x02\x05V[`@Qa\0\x86\x91\x90a\x08mV[`@Q\x80\x91\x03\x90\xF3[a\0\xAFa\0\x9D6`\x04a\x08\x87V[`\x10` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\0\x86V[a\0oa\x03xV[a\0oa\x048V[`\x0CTa\0\xDA\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\x86V[`\x10` R`\x01\x7F\x05K\x96t[y\xA8\xFF\xBD\x91\x07.\xE1\xA0N\x15\xB5\xB51E\x8E\x8C\xC1\x7F3\x90\xEF\xD0\x15~\xF1N\x81\x90U`d\x7F\x18U=\x8Fd\xAE5\xEB.r\x84M\x1A\xF4e\x89c\x0C\xBC\xF4\xDA~oy\xD8\xB0\xABNjn\x04a\x81\x90U\x7F@!\xAD\x11\xA5fFFYnr\xEB\x8F\x059\x0F\xF5\x1F=r\xD1R\x07\x03?\x007\x10\xE9\xD0\xAF,\x81\x90U\x7FCt\x15\xBBE\xCA\xDAl\xFB\xDA\x94( e%\x91^9\\\x89\xE6\xBB/\x15<\xA0.B\x91\nl\xEC\x81\x90U\x7F\xBC\xD2\x91\xA1\xF9\xB3?\x1ExD\xFBQ\xEB\xDB\xC0S^Pq\x8C\x03\xDC\xABV5\xAFr\xA5\xC7\xF8\x9F\x8D\x81\x90U\x7Fn\x98\x12q\x16\xD636\rJ5Y\x87\x9C\xA9\xB9\xBAO\xCB\x1Bg\xB9\xAF\x9A\xD3\xC0\x86A\xCCM\xBC\xEEUs\xD3\x1AY\xC8Z\xE9\xD8\xED\xEF\xECA\x1DD\x8F\x90\x84\x15q\xB8\x9C`\0R\x7F\x19\x04\xE5\xA5\"H\x0B\xAA7\xE1\xD3>\x075or\xE0\xE8S\xAB\x10\xA3d\xD3P\xF2\xD5\xB107\xE0AUV[`@\x80Q``\x80\x82\x01\x83R\x81R`\0` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\rT`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02@Wa\x02@a\x08\xB0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02\x85W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x02^W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x03\x18W`\0`\r\x82\x81T\x81\x10a\x02\xA8Wa\x02\xA8a\x08\xC6V[`\0\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x80\x83R`\x10\x82R`@\x92\x83\x90 T\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x91\x83\x01\x82\x90R\x85Q\x90\x93P\x90\x91\x90\x85\x90\x85\x90\x81\x10a\x02\xF8Wa\x02\xF8a\x08\xC6V[` \x02` \x01\x01\x81\x90RPPP\x80\x80a\x03\x10\x90a\x08\xF2V[\x91PPa\x02\x8BV[P`\0`@Q\x80``\x01`@R\x80\x83\x81R` \x01`\x11Ta\x03\xE8a\x03<\x91\x90a\t\x0BV[\x81R` \x01`\x11T`@Q` \x01a\x03V\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x90R\x94\x93PPPPV[`\0[`\rT\x81\x10\x15a\x045W`\r\x81\x81T\x81\x10a\x03\x98Wa\x03\x98a\x08\xC6V[`\0\x91\x82R` \x90\x91 \x01T`\x0ET`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Rh\x05k\xC7^-c\x10\0\0`$\x82\x01R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\xFEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\"\x91\x90a\t#V[P\x80a\x04-\x81a\x08\xF2V[\x91PPa\x03{V[PV[`\x0E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16s\xEE\xE9\xD3\xAA\xE6b\xFF\xDB\x94\xCB{\xEA\xC1\x1A\xAF\x15K\x10J[\x17\x90\x91U`\x0F\x80T\x90\x91\x16s\xD4b\x8FD\xAF\xB3\x82\xEC\xC5\xF9\x82]\xBD>\xE5\x8B\xCF\xC0\x9E\xE8\x17\x90U`@\x80Q`\xE0\x81\x01\x82Rs\"`\xFA\xC5\xE5T*w:\xA4O\xBC\xFE\xDF|\x19;\xC2\xC5\x99\x81Rs\xC0*\xAA9\xB2#\xFE\x8D\n\x0E\\O'\xEA\xD9\x08<ul\xC2` \x82\x01Rs}\x1A\xFA{q\x8F\xB8\x93\xDB0\xA3\xAB\xC0\xCF\xC6\x08\xAA\xCF\xEB\xB0\x91\x81\x01\x91\x90\x91Rs\x85\xF18\xBF\xEEN\xF8\xE5@\x89\x0C\xFBH\xF6 W\x1Dg\xED\xA3``\x82\x01RsA\x8Du\xF6Z\x02\xB3\xD5;$\x18\xFB\x8E\x1F\xE4\x93u\x9Cv\x05`\x80\x82\x01RsN\x156\x1F\xD6\xB4\xBB`\x9F\xA6<\x81\xA2\xBE\x19\xD8sqxp`\xA0\x82\x01Rs\xD3\x1AY\xC8Z\xE9\xD8\xED\xEF\xECA\x1DD\x8F\x90\x84\x15q\xB8\x9C`\xC0\x82\x01Ra\x05M\x90`\r\x90`\x07a\x07\x83V[Pa\x05Va\0\xEAV[B`\x11U`\0a\x05da\x02\x05V[`@Qc\x7F\xEC*\x8D`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x7F\xEC*\x8D\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xB3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xC7W=`\0\x80>=`\0\xFD[PPPPa\x05\xD3a\x03xV[`\x0ET`@Qc\x03U(\xC9`\xE6\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD5J2@\x90a\x06\x04\x90\x85\x90`\x04\x01a\x08mV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06E\x91\x90a\tEV[`\x0FT`@Qc\x0CLB\x85`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\x0CLB\x85\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\x8CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xA0W=`\0\x80>=`\0\xFD[PP`\x0ET`@QcO\xE5lM`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\x9F\xCA\xD8\x9A\x91Pa\x06\xD7\x90\x85\x90`\0\x90`\x04\x01a\t^V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xF1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\x05W=`\0\x80>=`\0\xFD[PPPP\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cv\xEA\xDD6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07gW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07{W=`\0\x80>=`\0\xFD[PPPPPPV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x07\xD8W\x91` \x02\x82\x01[\x82\x81\x11\x15a\x07\xD8W\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a\x07\xA3V[Pa\x07\xE4\x92\x91Pa\x07\xE8V[P\x90V[[\x80\x82\x11\x15a\x07\xE4W`\0\x81U`\x01\x01a\x07\xE9V[\x80Q``\x80\x84R\x81Q\x90\x84\x01\x81\x90R`\0\x91` \x91\x90\x82\x01\x90`\x80\x86\x01\x90\x84[\x81\x81\x10\x15a\x08MW\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x85\x01Q\x85\x84\x01R\x92\x84\x01\x92`@\x90\x92\x01\x91`\x01\x01a\x08\x1DV[PP\x82\x85\x01Q\x83\x87\x01R`@\x85\x01Q`@\x87\x01R\x80\x93PPPP\x92\x91PPV[` \x81R`\0a\x08\x80` \x83\x01\x84a\x07\xFDV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x08\x99W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\x80W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\t\x04Wa\t\x04a\x08\xDCV[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15a\t\x1EWa\t\x1Ea\x08\xDCV[P\x01\x90V[`\0` \x82\x84\x03\x12\x15a\t5W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x08\x80W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\tWW`\0\x80\xFD[PQ\x91\x90PV[`@\x81R`\0a\tq`@\x83\x01\x85a\x07\xFDV[\x90P\x82\x15\x15` \x83\x01R\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x9F\x93\xB7;_\x9C\xECJ\xA6d\x15\xE6e\xD2\n\x9BQ\\\x81\xC5\xB3\x1A,9\xE7_\xDD\xFCj\x91\x80\xCBdsolcC\0\x08\x0F\x003";
    /// The deployed bytecode of the contract.
    pub static INITIALBOUNTY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct InitialBounty<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for InitialBounty<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for InitialBounty<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for InitialBounty<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for InitialBounty<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(InitialBounty))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> InitialBounty<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    INITIALBOUNTY_ABI.clone(),
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
                INITIALBOUNTY_ABI.clone(),
                INITIALBOUNTY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `IS_SCRIPT` (0xf8ccbf47) function
        pub fn is_script(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([248, 204, 191, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approveTokenList` (0x94ef1fde) function
        pub fn approve_token_list(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([148, 239, 31, 222], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialBounty` (0x7081bd9e) function
        pub fn initial_bounty(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, Bounty> {
            self.0
                .method_hash([112, 129, 189, 158], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `run` (0xc0406226) function
        pub fn run(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 64, 98, 38], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTokenUnits` (0x5621bc0e) function
        pub fn set_token_units(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([86, 33, 188, 14], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenUnits` (0x8e62ed51) function
        pub fn token_units(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([142, 98, 237, 81], p0)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for InitialBounty<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`
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
    #[ethcall(name = "IS_SCRIPT", abi = "IS_SCRIPT()")]
    pub struct IsScriptCall;
    ///Container type for all input parameters for the `approveTokenList` function with signature `approveTokenList()` and selector `0x94ef1fde`
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
    #[ethcall(name = "approveTokenList", abi = "approveTokenList()")]
    pub struct ApproveTokenListCall;
    ///Container type for all input parameters for the `initialBounty` function with signature `initialBounty()` and selector `0x7081bd9e`
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
    #[ethcall(name = "initialBounty", abi = "initialBounty()")]
    pub struct InitialBountyCall;
    ///Container type for all input parameters for the `run` function with signature `run()` and selector `0xc0406226`
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
    #[ethcall(name = "run", abi = "run()")]
    pub struct RunCall;
    ///Container type for all input parameters for the `setTokenUnits` function with signature `setTokenUnits()` and selector `0x5621bc0e`
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
    #[ethcall(name = "setTokenUnits", abi = "setTokenUnits()")]
    pub struct SetTokenUnitsCall;
    ///Container type for all input parameters for the `tokenUnits` function with signature `tokenUnits(address)` and selector `0x8e62ed51`
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
    #[ethcall(name = "tokenUnits", abi = "tokenUnits(address)")]
    pub struct TokenUnitsCall(pub ::ethers::core::types::Address);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum InitialBountyCalls {
        IsScript(IsScriptCall),
        ApproveTokenList(ApproveTokenListCall),
        InitialBounty(InitialBountyCall),
        Run(RunCall),
        SetTokenUnits(SetTokenUnitsCall),
        TokenUnits(TokenUnitsCall),
    }
    impl ::ethers::core::abi::AbiDecode for InitialBountyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <IsScriptCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsScript(decoded));
            }
            if let Ok(decoded) = <ApproveTokenListCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ApproveTokenList(decoded));
            }
            if let Ok(decoded) = <InitialBountyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InitialBounty(decoded));
            }
            if let Ok(decoded) = <RunCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Run(decoded));
            }
            if let Ok(decoded) = <SetTokenUnitsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetTokenUnits(decoded));
            }
            if let Ok(decoded) = <TokenUnitsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenUnits(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for InitialBountyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsScript(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApproveTokenList(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitialBounty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Run(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetTokenUnits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenUnits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for InitialBountyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsScript(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApproveTokenList(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitialBounty(element) => ::core::fmt::Display::fmt(element, f),
                Self::Run(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTokenUnits(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenUnits(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IsScriptCall> for InitialBountyCalls {
        fn from(value: IsScriptCall) -> Self {
            Self::IsScript(value)
        }
    }
    impl ::core::convert::From<ApproveTokenListCall> for InitialBountyCalls {
        fn from(value: ApproveTokenListCall) -> Self {
            Self::ApproveTokenList(value)
        }
    }
    impl ::core::convert::From<InitialBountyCall> for InitialBountyCalls {
        fn from(value: InitialBountyCall) -> Self {
            Self::InitialBounty(value)
        }
    }
    impl ::core::convert::From<RunCall> for InitialBountyCalls {
        fn from(value: RunCall) -> Self {
            Self::Run(value)
        }
    }
    impl ::core::convert::From<SetTokenUnitsCall> for InitialBountyCalls {
        fn from(value: SetTokenUnitsCall) -> Self {
            Self::SetTokenUnits(value)
        }
    }
    impl ::core::convert::From<TokenUnitsCall> for InitialBountyCalls {
        fn from(value: TokenUnitsCall) -> Self {
            Self::TokenUnits(value)
        }
    }
    ///Container type for all return fields from the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`
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
    pub struct IsScriptReturn(pub bool);
    ///Container type for all return fields from the `initialBounty` function with signature `initialBounty()` and selector `0x7081bd9e`
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
    pub struct InitialBountyReturn(pub Bounty);
    ///Container type for all return fields from the `tokenUnits` function with signature `tokenUnits(address)` and selector `0x8e62ed51`
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
    pub struct TokenUnitsReturn(pub ::ethers::core::types::U256);
}
