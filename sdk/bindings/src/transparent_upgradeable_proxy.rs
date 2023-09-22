pub use transparent_upgradeable_proxy::*;
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
pub mod transparent_upgradeable_proxy {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_logic"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("admin_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_data"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes"),
                        ),
                    },
                ],
            }),
            functions: ::std::collections::BTreeMap::new(),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BeaconUpgraded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("BeaconUpgraded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("beacon"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Upgraded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Upgraded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
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
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static TRANSPARENTUPGRADEABLEPROXY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`@Qb\0\x0E\xD08\x03\x80b\0\x0E\xD0\x839\x81\x01`@\x81\x90Rb\0\0&\x91b\0\x04\xA0V[\x82\x81b\0\x006\x82\x82`\0b\0\0MV[Pb\0\0D\x90P\x82b\0\0\x8AV[PPPb\0\x05\xD3V[b\0\0X\x83b\0\0\xE5V[`\0\x82Q\x11\x80b\0\0fWP\x80[\x15b\0\0\x85Wb\0\0\x83\x83\x83b\0\x01'` \x1Bb\0\x01i\x17` \x1CV[P[PPPV[\x7F~dMyB/\x17\xC0\x1EH\x94\xB5\xF4\xF5\x88\xD31\xEB\xFA(e=B\xAE\x83-\xC5\x9E8\xC9y\x8Fb\0\0\xB5b\0\x01VV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x84\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1b\0\0\xE2\x81b\0\x01\x8FV[PV[b\0\0\xF0\x81b\0\x02DV[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[``b\0\x01O\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01b\0\x0E\xA9`'\x919b\0\x02\xF8V[\x93\x92PPPV[`\0b\0\x01\x80`\0\x80Q` b\0\x0E\x89\x839\x81Q\x91R`\0\x1Bb\0\x03w` \x1Bb\0\x01\x95\x17` \x1CV[T`\x01`\x01`\xA0\x1B\x03\x16\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\x01\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC1967: new admin is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x80b\0\x02#`\0\x80Q` b\0\x0E\x89\x839\x81Q\x91R`\0\x1Bb\0\x03w` \x1Bb\0\x01\x95\x17` \x1CV[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[b\0\x02Z\x81b\0\x03z` \x1Bb\0\x01\x98\x17` \x1CV[b\0\x02\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01Rl\x1B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x9A\x1B`d\x82\x01R`\x84\x01b\0\x01\xF1V[\x80b\0\x02#\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`\0\x1Bb\0\x03w` \x1Bb\0\x01\x95\x17` \x1CV[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qb\0\x03\x17\x91\x90b\0\x05\x80V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14b\0\x03TW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x03YV[``\x91P[P\x90\x92P\x90Pb\0\x03m\x86\x83\x83\x87b\0\x03\x89V[\x96\x95PPPPPPV[\x90V[`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[``\x83\x15b\0\x03\xFDW\x82Q`\0\x03b\0\x03\xF5W`\x01`\x01`\xA0\x1B\x03\x85\x16;b\0\x03\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01b\0\x01\xF1V[P\x81b\0\x04\tV[b\0\x04\t\x83\x83b\0\x04\x11V[\x94\x93PPPPV[\x81Q\x15b\0\x04\"W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x01\xF1\x91\x90b\0\x05\x9EV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x04VW`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15b\0\x04\x8EW\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x04tV[\x83\x81\x11\x15b\0\0\x83WPP`\0\x91\x01RV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x04\xB6W`\0\x80\xFD[b\0\x04\xC1\x84b\0\x04>V[\x92Pb\0\x04\xD1` \x85\x01b\0\x04>V[`@\x85\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x04\xEFW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12b\0\x05\x04W`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0\x05\x19Wb\0\x05\x19b\0\x04[V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15b\0\x05DWb\0\x05Db\0\x04[V[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15b\0\x05^W`\0\x80\xFD[b\0\x05q\x83` \x83\x01` \x88\x01b\0\x04qV[\x80\x95PPPPPP\x92P\x92P\x92V[`\0\x82Qb\0\x05\x94\x81\x84` \x87\x01b\0\x04qV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Rb\0\x05\xBF\x81`@\x85\x01` \x87\x01b\0\x04qV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[a\x08\xA6\x80b\0\x05\xE3`\09`\0\xF3\xFE`\x80`@R6a\0\x13Wa\0\x11a\0\x17V[\0[a\0\x11[a\0\x1Fa\x01\xA7V[`\x01`\x01`\xA0\x1B\x03\x163\x03a\x01_W```\x01`\x01`\xE0\x1B\x03\x19`\x005\x16cd\xD3\x18\r`\xE1\x1B\x81\x01a\0ZWa\0Sa\x01\xDAV[\x91Pa\x01WV[cXp\x86\xBD`\xE1\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\0zWa\0Sa\x021V[c\x07\r|i`\xE4\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\0\x9AWa\0Sa\x02wV[b\x1E\xB9o`\xE6\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\0\xB9Wa\0Sa\x02\xA8V[c\xA3\x9F%\xE5`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\0\xD9Wa\0Sa\x02\xE8V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R\x7FTransparentUpgradeableProxy: adm`D\x82\x01R\x7Fin cannot fallback to proxy targ`d\x82\x01Ra\x19]`\xF2\x1B`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[\x81Q` \x83\x01\xF3[a\x01ga\x02\xFCV[V[``a\x01\x8E\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a\x08J`'\x919a\x03\x0CV[\x93\x92PPPV[\x90V[`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[`\0\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03[T`\x01`\x01`\xA0\x1B\x03\x16\x91\x90PV[``a\x01\xE4a\x03\x84V[`\0a\x01\xF36`\x04\x81\x84a\x06\x95V[\x81\x01\x90a\x02\0\x91\x90a\x06\xDBV[\x90Pa\x02\x1D\x81`@Q\x80` \x01`@R\x80`\0\x81RP`\0a\x03\x8FV[PP`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[```\0\x80a\x02C6`\x04\x81\x84a\x06\x95V[\x81\x01\x90a\x02P\x91\x90a\x07\x0CV[\x91P\x91Pa\x02`\x82\x82`\x01a\x03\x8FV[`@Q\x80` \x01`@R\x80`\0\x81RP\x92PPP\x90V[``a\x02\x81a\x03\x84V[`\0a\x02\x906`\x04\x81\x84a\x06\x95V[\x81\x01\x90a\x02\x9D\x91\x90a\x06\xDBV[\x90Pa\x02\x1D\x81a\x03\xBBV[``a\x02\xB2a\x03\x84V[`\0a\x02\xBCa\x01\xA7V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x83\x16` \x82\x01R\x91\x92P\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x90V[``a\x02\xF2a\x03\x84V[`\0a\x02\xBCa\x04\x12V[a\x01ga\x03\x07a\x04\x12V[a\x04!V[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa\x03)\x91\x90a\x07\xFAV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x03dW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x03iV[``\x91P[P\x91P\x91Pa\x03z\x86\x83\x83\x87a\x04EV[\x96\x95PPPPPPV[4\x15a\x01gW`\0\x80\xFD[a\x03\x98\x83a\x04\xC6V[`\0\x82Q\x11\x80a\x03\xA5WP\x80[\x15a\x03\xB6Wa\x03\xB4\x83\x83a\x01iV[P[PPPV[\x7F~dMyB/\x17\xC0\x1EH\x94\xB5\xF4\xF5\x88\xD31\xEB\xFA(e=B\xAE\x83-\xC5\x9E8\xC9y\x8Fa\x03\xE4a\x01\xA7V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x84\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1a\x04\x0F\x81a\x05\x06V[PV[`\0a\x04\x1Ca\x05\xAFV[\x90P\x90V[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80\x80\x15a\x04@W=`\0\xF3[=`\0\xFD[``\x83\x15a\x04\xB4W\x82Q`\0\x03a\x04\xADW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x04\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x01NV[P\x81a\x04\xBEV[a\x04\xBE\x83\x83a\x05\xD7V[\x94\x93PPPPV[a\x04\xCF\x81a\x06\x01V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC1967: new admin is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x01NV[\x80\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[`\0\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCa\x01\xCBV[\x81Q\x15a\x05\xE7W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01N\x91\x90a\x08\x16V[`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x06nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01Rl\x1B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x9A\x1B`d\x82\x01R`\x84\x01a\x01NV[\x80\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCa\x05\x8EV[`\0\x80\x85\x85\x11\x15a\x06\xA5W`\0\x80\xFD[\x83\x86\x11\x15a\x06\xB2W`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\xD6W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x06\xEDW`\0\x80\xFD[a\x01\x8E\x82a\x06\xBFV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x07\x1FW`\0\x80\xFD[a\x07(\x83a\x06\xBFV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07EW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x07YW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x07kWa\x07ka\x06\xF6V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x07\x93Wa\x07\x93a\x06\xF6V[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x07\xACW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\x07\xE9W\x81\x81\x01Q\x83\x82\x01R` \x01a\x07\xD1V[\x83\x81\x11\x15a\x03\xB4WPP`\0\x91\x01RV[`\0\x82Qa\x08\x0C\x81\x84` \x87\x01a\x07\xCEV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x085\x81`@\x85\x01` \x87\x01a\x07\xCEV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFEAddress: low-level delegate call failed\xA2dipfsX\"\x12 \xB0\xC5\x1B)\x12\xC0\x0C8b%\xB7\xDCT\0\xD3\xC7\\\xA0)\x91\x05\x01\x9EBZ\x82\xE9\xBD\x03y\x90\xC3dsolcC\0\x08\x0F\x003\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03Address: low-level delegate call failed";
    /// The bytecode of the contract.
    pub static TRANSPARENTUPGRADEABLEPROXY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R6a\0\x13Wa\0\x11a\0\x17V[\0[a\0\x11[a\0\x1Fa\x01\xA7V[`\x01`\x01`\xA0\x1B\x03\x163\x03a\x01_W```\x01`\x01`\xE0\x1B\x03\x19`\x005\x16cd\xD3\x18\r`\xE1\x1B\x81\x01a\0ZWa\0Sa\x01\xDAV[\x91Pa\x01WV[cXp\x86\xBD`\xE1\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\0zWa\0Sa\x021V[c\x07\r|i`\xE4\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\0\x9AWa\0Sa\x02wV[b\x1E\xB9o`\xE6\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\0\xB9Wa\0Sa\x02\xA8V[c\xA3\x9F%\xE5`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\0\xD9Wa\0Sa\x02\xE8V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R\x7FTransparentUpgradeableProxy: adm`D\x82\x01R\x7Fin cannot fallback to proxy targ`d\x82\x01Ra\x19]`\xF2\x1B`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[\x81Q` \x83\x01\xF3[a\x01ga\x02\xFCV[V[``a\x01\x8E\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a\x08J`'\x919a\x03\x0CV[\x93\x92PPPV[\x90V[`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[`\0\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03[T`\x01`\x01`\xA0\x1B\x03\x16\x91\x90PV[``a\x01\xE4a\x03\x84V[`\0a\x01\xF36`\x04\x81\x84a\x06\x95V[\x81\x01\x90a\x02\0\x91\x90a\x06\xDBV[\x90Pa\x02\x1D\x81`@Q\x80` \x01`@R\x80`\0\x81RP`\0a\x03\x8FV[PP`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[```\0\x80a\x02C6`\x04\x81\x84a\x06\x95V[\x81\x01\x90a\x02P\x91\x90a\x07\x0CV[\x91P\x91Pa\x02`\x82\x82`\x01a\x03\x8FV[`@Q\x80` \x01`@R\x80`\0\x81RP\x92PPP\x90V[``a\x02\x81a\x03\x84V[`\0a\x02\x906`\x04\x81\x84a\x06\x95V[\x81\x01\x90a\x02\x9D\x91\x90a\x06\xDBV[\x90Pa\x02\x1D\x81a\x03\xBBV[``a\x02\xB2a\x03\x84V[`\0a\x02\xBCa\x01\xA7V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x83\x16` \x82\x01R\x91\x92P\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x90V[``a\x02\xF2a\x03\x84V[`\0a\x02\xBCa\x04\x12V[a\x01ga\x03\x07a\x04\x12V[a\x04!V[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa\x03)\x91\x90a\x07\xFAV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x03dW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x03iV[``\x91P[P\x91P\x91Pa\x03z\x86\x83\x83\x87a\x04EV[\x96\x95PPPPPPV[4\x15a\x01gW`\0\x80\xFD[a\x03\x98\x83a\x04\xC6V[`\0\x82Q\x11\x80a\x03\xA5WP\x80[\x15a\x03\xB6Wa\x03\xB4\x83\x83a\x01iV[P[PPPV[\x7F~dMyB/\x17\xC0\x1EH\x94\xB5\xF4\xF5\x88\xD31\xEB\xFA(e=B\xAE\x83-\xC5\x9E8\xC9y\x8Fa\x03\xE4a\x01\xA7V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x84\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1a\x04\x0F\x81a\x05\x06V[PV[`\0a\x04\x1Ca\x05\xAFV[\x90P\x90V[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80\x80\x15a\x04@W=`\0\xF3[=`\0\xFD[``\x83\x15a\x04\xB4W\x82Q`\0\x03a\x04\xADW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x04\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x01NV[P\x81a\x04\xBEV[a\x04\xBE\x83\x83a\x05\xD7V[\x94\x93PPPPV[a\x04\xCF\x81a\x06\x01V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC1967: new admin is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x01NV[\x80\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[`\0\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCa\x01\xCBV[\x81Q\x15a\x05\xE7W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01N\x91\x90a\x08\x16V[`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x06nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01Rl\x1B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x9A\x1B`d\x82\x01R`\x84\x01a\x01NV[\x80\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCa\x05\x8EV[`\0\x80\x85\x85\x11\x15a\x06\xA5W`\0\x80\xFD[\x83\x86\x11\x15a\x06\xB2W`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\xD6W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x06\xEDW`\0\x80\xFD[a\x01\x8E\x82a\x06\xBFV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x07\x1FW`\0\x80\xFD[a\x07(\x83a\x06\xBFV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07EW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x07YW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x07kWa\x07ka\x06\xF6V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x07\x93Wa\x07\x93a\x06\xF6V[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x07\xACW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\x07\xE9W\x81\x81\x01Q\x83\x82\x01R` \x01a\x07\xD1V[\x83\x81\x11\x15a\x03\xB4WPP`\0\x91\x01RV[`\0\x82Qa\x08\x0C\x81\x84` \x87\x01a\x07\xCEV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x085\x81`@\x85\x01` \x87\x01a\x07\xCEV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFEAddress: low-level delegate call failed\xA2dipfsX\"\x12 \xB0\xC5\x1B)\x12\xC0\x0C8b%\xB7\xDCT\0\xD3\xC7\\\xA0)\x91\x05\x01\x9EBZ\x82\xE9\xBD\x03y\x90\xC3dsolcC\0\x08\x0F\x003";
    /// The deployed bytecode of the contract.
    pub static TRANSPARENTUPGRADEABLEPROXY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct TransparentUpgradeableProxy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TransparentUpgradeableProxy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TransparentUpgradeableProxy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TransparentUpgradeableProxy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TransparentUpgradeableProxy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(TransparentUpgradeableProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TransparentUpgradeableProxy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    TRANSPARENTUPGRADEABLEPROXY_ABI.clone(),
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
                TRANSPARENTUPGRADEABLEPROXY_ABI.clone(),
                TRANSPARENTUPGRADEABLEPROXY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Gets the contract's `AdminChanged` event
        pub fn admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `BeaconUpgraded` event
        pub fn beacon_upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BeaconUpgradedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpgradedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransparentUpgradeableProxyEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for TransparentUpgradeableProxy<M> {
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
    #[ethevent(name = "AdminChanged", abi = "AdminChanged(address,address)")]
    pub struct AdminChangedFilter {
        pub previous_admin: ::ethers::core::types::Address,
        pub new_admin: ::ethers::core::types::Address,
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
    #[ethevent(name = "BeaconUpgraded", abi = "BeaconUpgraded(address)")]
    pub struct BeaconUpgradedFilter {
        #[ethevent(indexed)]
        pub beacon: ::ethers::core::types::Address,
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TransparentUpgradeableProxyEvents {
        AdminChangedFilter(AdminChangedFilter),
        BeaconUpgradedFilter(BeaconUpgradedFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for TransparentUpgradeableProxyEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(
                    TransparentUpgradeableProxyEvents::AdminChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = BeaconUpgradedFilter::decode_log(log) {
                return Ok(
                    TransparentUpgradeableProxyEvents::BeaconUpgradedFilter(decoded),
                );
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(TransparentUpgradeableProxyEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for TransparentUpgradeableProxyEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeaconUpgradedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminChangedFilter>
    for TransparentUpgradeableProxyEvents {
        fn from(value: AdminChangedFilter) -> Self {
            Self::AdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<BeaconUpgradedFilter>
    for TransparentUpgradeableProxyEvents {
        fn from(value: BeaconUpgradedFilter) -> Self {
            Self::BeaconUpgradedFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for TransparentUpgradeableProxyEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
}
