pub use initial_bounty_helper::*;
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
pub mod initial_bounty_helper {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("tokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokens"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
    pub static INITIALBOUNTYHELPER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x05\xFC\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\x9Dc\x84\x8A\x14a\x000W[`\0\x80\xFD[a\08a\0NV[`@Qa\0E\x91\x90a\x05XV[`@Q\x80\x91\x03\x90\xF3[`@\x80Q`\x0F\x80\x82Ra\x02\0\x82\x01\x90\x92R``\x91`\0\x91\x90\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0kW\x90PP\x90P`@Q\x80`@\x01`@R\x80s\"`\xFA\xC5\xE5T*w:\xA4O\xBC\xFE\xDF|\x19;\xC2\xC5\x99`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\0\x81Q\x81\x10a\0\xDAWa\0\xDAa\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80s\xC0*\xAA9\xB2#\xFE\x8D\n\x0E\\O'\xEA\xD9\x08<ul\xC2`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x01\x81Q\x81\x10a\x01+Wa\x01+a\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80sA\x8Du\xF6Z\x02\xB3\xD5;$\x18\xFB\x8E\x1F\xE4\x93u\x9Cv\x05`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x02\x81Q\x81\x10a\x01|Wa\x01|a\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80s\xD3\x1AY\xC8Z\xE9\xD8\xED\xEF\xECA\x1DD\x8F\x90\x84\x15q\xB8\x9C`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x03\x81Q\x81\x10a\x01\xCDWa\x01\xCDa\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80s}\x1A\xFA{q\x8F\xB8\x93\xDB0\xA3\xAB\xC0\xCF\xC6\x08\xAA\xCF\xEB\xB0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x04\x81Q\x81\x10a\x02\x1EWa\x02\x1Ea\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80s\x95\xADa\xB0\xA1P\xD7\x92\x19\xDC\xF6N\x1El\xC0\x1F\x0Bd\xC4\xCE`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x05\x81Q\x81\x10a\x02oWa\x02oa\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80s\x85\xF18\xBF\xEEN\xF8\xE5@\x89\x0C\xFBH\xF6 W\x1Dg\xED\xA3`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x06\x81Q\x81\x10a\x02\xC0Wa\x02\xC0a\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80sQI\x10w\x1A\xF9\xCAej\xF8@\xDF\xF8>\x82d\xEC\xF9\x86\xCA`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x07\x81Q\x81\x10a\x03\x11Wa\x03\x11a\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80s\x1F\x98@\xA8]Z\xF5\xBF\x1D\x17b\xF9%\xBD\xAD\xDCB\x01\xF9\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x08\x81Q\x81\x10a\x03bWa\x03ba\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80sZ\x98\xFC\xBE\xA5\x16\xCF\x06\x85r\x15w\x9F\xD8\x12\xCA;\xEF\x1B2`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\t\x81Q\x81\x10a\x03\xB3Wa\x03\xB3a\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80s<:\x81\xE8\x1D\xC4\x9AR*Y.v\"\xA7\xE7\x11\xC0k\xF3T`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\n\x81Q\x81\x10a\x04\x04Wa\x04\x04a\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80s\xA0\xB7>\x1F\xF0\xB8\t\x14\xABo\xE0DNe\x84\x8CL4E\x0B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x0B\x81Q\x81\x10a\x04UWa\x04Ua\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80sJ\"\x0E`\x96\xB2^\xAD\xB8\x83X\xCBD\x06\x8A2H%Fu`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x0C\x81Q\x81\x10a\x04\xA6Wa\x04\xA6a\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80s\xB5\x07!\xBC\xF8\xD6d\xC3\x04\x12\xCF\xBCl\xF7\xA1QE#J\xD1`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\r\x81Q\x81\x10a\x04\xF7Wa\x04\xF7a\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80s\x9F\x8Fr\xAA\x93\x04\xC8\xB5\x93\xD5U\xF1.\xF6X\x9C\xC3\xA5y\xA2`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x0E\x81Q\x81\x10a\x05HWa\x05Ha\x05\xB0V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x05\xA3W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x86\x01Q\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x05uV[P\x91\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12  \xD0>y%\x86,N\xDF2Qc\x1E'\xB4Q\xC2s\x87\x9B\xC4\x95-C\xA5\x92\x9DlW\xF2\x1F\x99dsolcC\0\x08\x0F\x003";
    /// The bytecode of the contract.
    pub static INITIALBOUNTYHELPER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\x9Dc\x84\x8A\x14a\x000W[`\0\x80\xFD[a\08a\0NV[`@Qa\0E\x91\x90a\x05XV[`@Q\x80\x91\x03\x90\xF3[`@\x80Q`\x0F\x80\x82Ra\x02\0\x82\x01\x90\x92R``\x91`\0\x91\x90\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0kW\x90PP\x90P`@Q\x80`@\x01`@R\x80s\"`\xFA\xC5\xE5T*w:\xA4O\xBC\xFE\xDF|\x19;\xC2\xC5\x99`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\0\x81Q\x81\x10a\0\xDAWa\0\xDAa\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80s\xC0*\xAA9\xB2#\xFE\x8D\n\x0E\\O'\xEA\xD9\x08<ul\xC2`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x01\x81Q\x81\x10a\x01+Wa\x01+a\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80sA\x8Du\xF6Z\x02\xB3\xD5;$\x18\xFB\x8E\x1F\xE4\x93u\x9Cv\x05`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x02\x81Q\x81\x10a\x01|Wa\x01|a\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80s\xD3\x1AY\xC8Z\xE9\xD8\xED\xEF\xECA\x1DD\x8F\x90\x84\x15q\xB8\x9C`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x03\x81Q\x81\x10a\x01\xCDWa\x01\xCDa\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80s}\x1A\xFA{q\x8F\xB8\x93\xDB0\xA3\xAB\xC0\xCF\xC6\x08\xAA\xCF\xEB\xB0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x04\x81Q\x81\x10a\x02\x1EWa\x02\x1Ea\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80s\x95\xADa\xB0\xA1P\xD7\x92\x19\xDC\xF6N\x1El\xC0\x1F\x0Bd\xC4\xCE`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x05\x81Q\x81\x10a\x02oWa\x02oa\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80s\x85\xF18\xBF\xEEN\xF8\xE5@\x89\x0C\xFBH\xF6 W\x1Dg\xED\xA3`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x06\x81Q\x81\x10a\x02\xC0Wa\x02\xC0a\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80sQI\x10w\x1A\xF9\xCAej\xF8@\xDF\xF8>\x82d\xEC\xF9\x86\xCA`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x07\x81Q\x81\x10a\x03\x11Wa\x03\x11a\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80s\x1F\x98@\xA8]Z\xF5\xBF\x1D\x17b\xF9%\xBD\xAD\xDCB\x01\xF9\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x08\x81Q\x81\x10a\x03bWa\x03ba\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80sZ\x98\xFC\xBE\xA5\x16\xCF\x06\x85r\x15w\x9F\xD8\x12\xCA;\xEF\x1B2`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\t\x81Q\x81\x10a\x03\xB3Wa\x03\xB3a\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80s<:\x81\xE8\x1D\xC4\x9AR*Y.v\"\xA7\xE7\x11\xC0k\xF3T`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\n\x81Q\x81\x10a\x04\x04Wa\x04\x04a\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80s\xA0\xB7>\x1F\xF0\xB8\t\x14\xABo\xE0DNe\x84\x8CL4E\x0B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x0B\x81Q\x81\x10a\x04UWa\x04Ua\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80sJ\"\x0E`\x96\xB2^\xAD\xB8\x83X\xCBD\x06\x8A2H%Fu`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x0C\x81Q\x81\x10a\x04\xA6Wa\x04\xA6a\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80s\xB5\x07!\xBC\xF8\xD6d\xC3\x04\x12\xCF\xBCl\xF7\xA1QE#J\xD1`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\r\x81Q\x81\x10a\x04\xF7Wa\x04\xF7a\x05\xB0V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80s\x9F\x8Fr\xAA\x93\x04\xC8\xB5\x93\xD5U\xF1.\xF6X\x9C\xC3\xA5y\xA2`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x81RP\x81`\x0E\x81Q\x81\x10a\x05HWa\x05Ha\x05\xB0V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x05\xA3W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x86\x01Q\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x05uV[P\x91\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12  \xD0>y%\x86,N\xDF2Qc\x1E'\xB4Q\xC2s\x87\x9B\xC4\x95-C\xA5\x92\x9DlW\xF2\x1F\x99dsolcC\0\x08\x0F\x003";
    /// The deployed bytecode of the contract.
    pub static INITIALBOUNTYHELPER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct InitialBountyHelper<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for InitialBountyHelper<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for InitialBountyHelper<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for InitialBountyHelper<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for InitialBountyHelper<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(InitialBountyHelper))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> InitialBountyHelper<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    INITIALBOUNTYHELPER_ABI.clone(),
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
                INITIALBOUNTYHELPER_ABI.clone(),
                INITIALBOUNTYHELPER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `tokens` (0x9d63848a) function
        pub fn tokens(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<TokenInfo>> {
            self.0
                .method_hash([157, 99, 132, 138], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for InitialBountyHelper<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `tokens` function with signature `tokens()` and selector `0x9d63848a`
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
    #[ethcall(name = "tokens", abi = "tokens()")]
    pub struct TokensCall;
    ///Container type for all return fields from the `tokens` function with signature `tokens()` and selector `0x9d63848a`
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
    pub struct TokensReturn(pub ::std::vec::Vec<TokenInfo>);
}
