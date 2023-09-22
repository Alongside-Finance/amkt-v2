pub use proxy_admin::*;
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
pub mod proxy_admin {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("changeProxyAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("changeProxyAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract ITransparentUpgradeableProxy",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
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
                    ::std::borrow::ToOwned::to_owned("getProxyAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getProxyAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract ITransparentUpgradeableProxy",
                                        ),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("getProxyImplementation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getProxyImplementation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract ITransparentUpgradeableProxy",
                                        ),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("upgrade"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upgrade"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract ITransparentUpgradeableProxy",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
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
                    ::std::borrow::ToOwned::to_owned("upgradeAndCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upgradeAndCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract ITransparentUpgradeableProxy",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static PROXYADMIN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\0\x1A3a\0\x1FV[a\0oV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[a\x06\x9A\x80a\0~`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0{W`\x005`\xE0\x1C\x80c\x96#`\x9D\x11a\0NW\x80c\x96#`\x9D\x14a\x01\x11W\x80c\x99\xA8\x8E\xC4\x14a\x01$W\x80c\xF2\xFD\xE3\x8B\x14a\x01DW\x80c\xF3\xB7\xDE\xAD\x14a\x01dW`\0\x80\xFD[\x80c N\x1Cz\x14a\0\x80W\x80cqP\x18\xA6\x14a\0\xBCW\x80c~\xFF'^\x14a\0\xD3W\x80c\x8D\xA5\xCB[\x14a\0\xF3W[`\0\x80\xFD[4\x80\x15a\0\x8CW`\0\x80\xFD[Pa\0\xA0a\0\x9B6`\x04a\x04\x99V[a\x01\x84V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xC8W`\0\x80\xFD[Pa\0\xD1a\x02\x15V[\0[4\x80\x15a\0\xDFW`\0\x80\xFD[Pa\0\xD1a\0\xEE6`\x04a\x04\xBDV[a\x02)V[4\x80\x15a\0\xFFW`\0\x80\xFD[P`\0T`\x01`\x01`\xA0\x1B\x03\x16a\0\xA0V[a\0\xD1a\x01\x1F6`\x04a\x05\x0CV[a\x02\x91V[4\x80\x15a\x010W`\0\x80\xFD[Pa\0\xD1a\x01?6`\x04a\x04\xBDV[a\x03\0V[4\x80\x15a\x01PW`\0\x80\xFD[Pa\0\xD1a\x01_6`\x04a\x04\x99V[a\x036V[4\x80\x15a\x01pW`\0\x80\xFD[Pa\0\xA0a\x01\x7F6`\x04a\x04\x99V[a\x03\xB4V[`\0\x80`\0\x83`\x01`\x01`\xA0\x1B\x03\x16`@Qa\x01\xAA\x90c\\`\xDA\x1B`\xE0\x1B\x81R`\x04\x01\x90V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x01\xE5W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\xEAV[``\x91P[P\x91P\x91P\x81a\x01\xF9W`\0\x80\xFD[\x80\x80` \x01\x90Q\x81\x01\x90a\x02\r\x91\x90a\x05\xE2V[\x94\x93PPPPV[a\x02\x1Da\x03\xDAV[a\x02'`\0a\x044V[V[a\x021a\x03\xDAV[`@Qc\x08\xF2\x83\x97`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x83\x16\x90c\x8F(9p\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02uW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\x89W=`\0\x80>=`\0\xFD[PPPPPPV[a\x02\x99a\x03\xDAV[`@Qc'\x8FyC`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cO\x1E\xF2\x86\x904\x90a\x02\xC9\x90\x86\x90\x86\x90`\x04\x01a\x05\xFFV[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x02\xE2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xF6W=`\0\x80>=`\0\xFD[PPPPPPPPV[a\x03\x08a\x03\xDAV[`@Qc\x1B,\xE7\xF3`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x83\x16\x90c6Y\xCF\xE6\x90`$\x01a\x02[V[a\x03>a\x03\xDAV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x03\xB1\x81a\x044V[PV[`\0\x80`\0\x83`\x01`\x01`\xA0\x1B\x03\x16`@Qa\x01\xAA\x90c\x03\xE1F\x91`\xE6\x1B\x81R`\x04\x01\x90V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x03\x9FV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\xB1W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x04\xABW`\0\x80\xFD[\x815a\x04\xB6\x81a\x04\x84V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x04\xD0W`\0\x80\xFD[\x825a\x04\xDB\x81a\x04\x84V[\x91P` \x83\x015a\x04\xEB\x81a\x04\x84V[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x05!W`\0\x80\xFD[\x835a\x05,\x81a\x04\x84V[\x92P` \x84\x015a\x05<\x81a\x04\x84V[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05YW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x05mW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x05\x7FWa\x05\x7Fa\x04\xF6V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x05\xA7Wa\x05\xA7a\x04\xF6V[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a\x05\xC0W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x05\xF4W`\0\x80\xFD[\x81Qa\x04\xB6\x81a\x04\x84V[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15a\x06;W\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01a\x06\x1FV[\x81\x81\x11\x15a\x06MW`\0``\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \xE6\x04\x83x\xF6W>)\x1A\xBEy\xEBc\xAD\x9C|\xBC7 \xFB\xFA\xD9\xE3\xC2\xB9\x9B\xABD7(\x0B<dsolcC\0\x08\x0F\x003";
    /// The bytecode of the contract.
    pub static PROXYADMIN_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0{W`\x005`\xE0\x1C\x80c\x96#`\x9D\x11a\0NW\x80c\x96#`\x9D\x14a\x01\x11W\x80c\x99\xA8\x8E\xC4\x14a\x01$W\x80c\xF2\xFD\xE3\x8B\x14a\x01DW\x80c\xF3\xB7\xDE\xAD\x14a\x01dW`\0\x80\xFD[\x80c N\x1Cz\x14a\0\x80W\x80cqP\x18\xA6\x14a\0\xBCW\x80c~\xFF'^\x14a\0\xD3W\x80c\x8D\xA5\xCB[\x14a\0\xF3W[`\0\x80\xFD[4\x80\x15a\0\x8CW`\0\x80\xFD[Pa\0\xA0a\0\x9B6`\x04a\x04\x99V[a\x01\x84V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xC8W`\0\x80\xFD[Pa\0\xD1a\x02\x15V[\0[4\x80\x15a\0\xDFW`\0\x80\xFD[Pa\0\xD1a\0\xEE6`\x04a\x04\xBDV[a\x02)V[4\x80\x15a\0\xFFW`\0\x80\xFD[P`\0T`\x01`\x01`\xA0\x1B\x03\x16a\0\xA0V[a\0\xD1a\x01\x1F6`\x04a\x05\x0CV[a\x02\x91V[4\x80\x15a\x010W`\0\x80\xFD[Pa\0\xD1a\x01?6`\x04a\x04\xBDV[a\x03\0V[4\x80\x15a\x01PW`\0\x80\xFD[Pa\0\xD1a\x01_6`\x04a\x04\x99V[a\x036V[4\x80\x15a\x01pW`\0\x80\xFD[Pa\0\xA0a\x01\x7F6`\x04a\x04\x99V[a\x03\xB4V[`\0\x80`\0\x83`\x01`\x01`\xA0\x1B\x03\x16`@Qa\x01\xAA\x90c\\`\xDA\x1B`\xE0\x1B\x81R`\x04\x01\x90V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x01\xE5W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\xEAV[``\x91P[P\x91P\x91P\x81a\x01\xF9W`\0\x80\xFD[\x80\x80` \x01\x90Q\x81\x01\x90a\x02\r\x91\x90a\x05\xE2V[\x94\x93PPPPV[a\x02\x1Da\x03\xDAV[a\x02'`\0a\x044V[V[a\x021a\x03\xDAV[`@Qc\x08\xF2\x83\x97`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x83\x16\x90c\x8F(9p\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02uW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\x89W=`\0\x80>=`\0\xFD[PPPPPPV[a\x02\x99a\x03\xDAV[`@Qc'\x8FyC`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cO\x1E\xF2\x86\x904\x90a\x02\xC9\x90\x86\x90\x86\x90`\x04\x01a\x05\xFFV[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x02\xE2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xF6W=`\0\x80>=`\0\xFD[PPPPPPPPV[a\x03\x08a\x03\xDAV[`@Qc\x1B,\xE7\xF3`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x83\x16\x90c6Y\xCF\xE6\x90`$\x01a\x02[V[a\x03>a\x03\xDAV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x03\xB1\x81a\x044V[PV[`\0\x80`\0\x83`\x01`\x01`\xA0\x1B\x03\x16`@Qa\x01\xAA\x90c\x03\xE1F\x91`\xE6\x1B\x81R`\x04\x01\x90V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x03\x9FV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\xB1W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x04\xABW`\0\x80\xFD[\x815a\x04\xB6\x81a\x04\x84V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x04\xD0W`\0\x80\xFD[\x825a\x04\xDB\x81a\x04\x84V[\x91P` \x83\x015a\x04\xEB\x81a\x04\x84V[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x05!W`\0\x80\xFD[\x835a\x05,\x81a\x04\x84V[\x92P` \x84\x015a\x05<\x81a\x04\x84V[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05YW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x05mW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x05\x7FWa\x05\x7Fa\x04\xF6V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x05\xA7Wa\x05\xA7a\x04\xF6V[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a\x05\xC0W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x05\xF4W`\0\x80\xFD[\x81Qa\x04\xB6\x81a\x04\x84V[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15a\x06;W\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01a\x06\x1FV[\x81\x81\x11\x15a\x06MW`\0``\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \xE6\x04\x83x\xF6W>)\x1A\xBEy\xEBc\xAD\x9C|\xBC7 \xFB\xFA\xD9\xE3\xC2\xB9\x9B\xABD7(\x0B<dsolcC\0\x08\x0F\x003";
    /// The deployed bytecode of the contract.
    pub static PROXYADMIN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ProxyAdmin<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ProxyAdmin<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ProxyAdmin<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ProxyAdmin<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ProxyAdmin<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ProxyAdmin)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ProxyAdmin<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PROXYADMIN_ABI.clone(),
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
                PROXYADMIN_ABI.clone(),
                PROXYADMIN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `changeProxyAdmin` (0x7eff275e) function
        pub fn change_proxy_admin(
            &self,
            proxy: ::ethers::core::types::Address,
            new_admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([126, 255, 39, 94], (proxy, new_admin))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProxyAdmin` (0xf3b7dead) function
        pub fn get_proxy_admin(
            &self,
            proxy: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([243, 183, 222, 173], proxy)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProxyImplementation` (0x204e1c7a) function
        pub fn get_proxy_implementation(
            &self,
            proxy: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([32, 78, 28, 122], proxy)
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
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
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
        ///Calls the contract's `upgrade` (0x99a88ec4) function
        pub fn upgrade(
            &self,
            proxy: ::ethers::core::types::Address,
            implementation: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 168, 142, 196], (proxy, implementation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeAndCall` (0x9623609d) function
        pub fn upgrade_and_call(
            &self,
            proxy: ::ethers::core::types::Address,
            implementation: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 35, 96, 157], (proxy, implementation, data))
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
    for ProxyAdmin<M> {
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `changeProxyAdmin` function with signature `changeProxyAdmin(address,address)` and selector `0x7eff275e`
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
    #[ethcall(name = "changeProxyAdmin", abi = "changeProxyAdmin(address,address)")]
    pub struct ChangeProxyAdminCall {
        pub proxy: ::ethers::core::types::Address,
        pub new_admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getProxyAdmin` function with signature `getProxyAdmin(address)` and selector `0xf3b7dead`
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
    #[ethcall(name = "getProxyAdmin", abi = "getProxyAdmin(address)")]
    pub struct GetProxyAdminCall {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getProxyImplementation` function with signature `getProxyImplementation(address)` and selector `0x204e1c7a`
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
    #[ethcall(name = "getProxyImplementation", abi = "getProxyImplementation(address)")]
    pub struct GetProxyImplementationCall {
        pub proxy: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `upgrade` function with signature `upgrade(address,address)` and selector `0x99a88ec4`
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
    #[ethcall(name = "upgrade", abi = "upgrade(address,address)")]
    pub struct UpgradeCall {
        pub proxy: ::ethers::core::types::Address,
        pub implementation: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `upgradeAndCall` function with signature `upgradeAndCall(address,address,bytes)` and selector `0x9623609d`
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
    #[ethcall(name = "upgradeAndCall", abi = "upgradeAndCall(address,address,bytes)")]
    pub struct UpgradeAndCallCall {
        pub proxy: ::ethers::core::types::Address,
        pub implementation: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ProxyAdminCalls {
        ChangeProxyAdmin(ChangeProxyAdminCall),
        GetProxyAdmin(GetProxyAdminCall),
        GetProxyImplementation(GetProxyImplementationCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
        Upgrade(UpgradeCall),
        UpgradeAndCall(UpgradeAndCallCall),
    }
    impl ::ethers::core::abi::AbiDecode for ProxyAdminCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ChangeProxyAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeProxyAdmin(decoded));
            }
            if let Ok(decoded) = <GetProxyAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetProxyAdmin(decoded));
            }
            if let Ok(decoded) = <GetProxyImplementationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetProxyImplementation(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UpgradeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Upgrade(decoded));
            }
            if let Ok(decoded) = <UpgradeAndCallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpgradeAndCall(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ProxyAdminCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ChangeProxyAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProxyAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProxyImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Upgrade(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpgradeAndCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ProxyAdminCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ChangeProxyAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetProxyAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetProxyImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Upgrade(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeAndCall(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ChangeProxyAdminCall> for ProxyAdminCalls {
        fn from(value: ChangeProxyAdminCall) -> Self {
            Self::ChangeProxyAdmin(value)
        }
    }
    impl ::core::convert::From<GetProxyAdminCall> for ProxyAdminCalls {
        fn from(value: GetProxyAdminCall) -> Self {
            Self::GetProxyAdmin(value)
        }
    }
    impl ::core::convert::From<GetProxyImplementationCall> for ProxyAdminCalls {
        fn from(value: GetProxyImplementationCall) -> Self {
            Self::GetProxyImplementation(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for ProxyAdminCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for ProxyAdminCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for ProxyAdminCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpgradeCall> for ProxyAdminCalls {
        fn from(value: UpgradeCall) -> Self {
            Self::Upgrade(value)
        }
    }
    impl ::core::convert::From<UpgradeAndCallCall> for ProxyAdminCalls {
        fn from(value: UpgradeAndCallCall) -> Self {
            Self::UpgradeAndCall(value)
        }
    }
    ///Container type for all return fields from the `getProxyAdmin` function with signature `getProxyAdmin(address)` and selector `0xf3b7dead`
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
    pub struct GetProxyAdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getProxyImplementation` function with signature `getProxyImplementation(address)` and selector `0x204e1c7a`
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
    pub struct GetProxyImplementationReturn(pub ::ethers::core::types::Address);
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
}
