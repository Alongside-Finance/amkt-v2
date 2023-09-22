pub use multiplier_helper::*;
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
pub mod multiplier_helper {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("computeMultiplier"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("computeMultiplier"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("feeScaled"),
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
                                    name: ::std::borrow::ToOwned::to_owned("trackedTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("trackedMultiplier"),
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
                                    name: ::std::borrow::ToOwned::to_owned("currentMultiplier"),
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
                    ::std::borrow::ToOwned::to_owned("MultiplierFeeTooHigh"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MultiplierFeeTooHigh",
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
    pub static MULTIPLIERHELPER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x02\xBE\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xD5\xD0N\xF3\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x01\xD6V[a\0gV[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xF3[`\0\x80`\0\x80`\0\x80`\0\x80a\0~\x8B\x8B\x8Ba\0\x93V[\x92\x9E\x91\x9DP\x9BP\x90\x99P\x97PPPPPPPPV[`\0\x80`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0\x85\x11\x15a\0\xC2W`@Qc4\xE7Z\xA1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x86\x93P`\0a\0\xD9\x86g\r\xE0\xB6\xB3\xA7d\0\0a\x02\x18V[\x90P`\0b\x01Q\x80a\0\xEB\x87Ba\x02\x18V[a\0\xF5\x91\x90a\x02/V[g\r\xE0\xB6\xB3\xA7d\0\0\x94P\x90P\x80\x15a\x01BW`\0[\x81\x81\x10\x15a\x01'Wa\x01\x1D\x85\x84a\x01\xB0V[\x94P`\x01\x01a\x01\x0BV[Pa\x015\x81b\x01Q\x80a\x02QV[a\x01?\x90\x87a\x02pV[\x95P[a\x01L\x84\x89a\x01\xB0V[\x94P`\0a\x01Z\x87Ba\x02\x18V[\x90P\x80\x15a\x01\xA0W`\0a\x01qb\x01Q\x80\x8Aa\x02/V[\x90Pa\x01\x98\x87a\x01\x81\x84\x84a\x02QV[a\x01\x93\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x02\x18V[a\x01\xB0V[\x94PPa\x01\xA4V[\x85\x93P[PPP\x93P\x93P\x93P\x93V[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x01\xC5\x83\x85a\x02QV[a\x01\xCF\x91\x90a\x02/V[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x01\xEBW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x02*Wa\x02*a\x02\x02V[P\x03\x90V[`\0\x82a\x02LWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x02kWa\x02ka\x02\x02V[P\x02\x90V[`\0\x82\x19\x82\x11\x15a\x02\x83Wa\x02\x83a\x02\x02V[P\x01\x90V\xFE\xA2dipfsX\"\x12 \x83\"*z\xB5\xAEY'0\x8D\xE5\xC3\xA24L\xF626\x07q\xF5O\xE6U\xC8\xD3\x10-\x18j\xC1\x8AdsolcC\0\x08\x0F\x003";
    /// The bytecode of the contract.
    pub static MULTIPLIERHELPER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xD5\xD0N\xF3\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x01\xD6V[a\0gV[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xF3[`\0\x80`\0\x80`\0\x80`\0\x80a\0~\x8B\x8B\x8Ba\0\x93V[\x92\x9E\x91\x9DP\x9BP\x90\x99P\x97PPPPPPPPV[`\0\x80`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0\x85\x11\x15a\0\xC2W`@Qc4\xE7Z\xA1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x86\x93P`\0a\0\xD9\x86g\r\xE0\xB6\xB3\xA7d\0\0a\x02\x18V[\x90P`\0b\x01Q\x80a\0\xEB\x87Ba\x02\x18V[a\0\xF5\x91\x90a\x02/V[g\r\xE0\xB6\xB3\xA7d\0\0\x94P\x90P\x80\x15a\x01BW`\0[\x81\x81\x10\x15a\x01'Wa\x01\x1D\x85\x84a\x01\xB0V[\x94P`\x01\x01a\x01\x0BV[Pa\x015\x81b\x01Q\x80a\x02QV[a\x01?\x90\x87a\x02pV[\x95P[a\x01L\x84\x89a\x01\xB0V[\x94P`\0a\x01Z\x87Ba\x02\x18V[\x90P\x80\x15a\x01\xA0W`\0a\x01qb\x01Q\x80\x8Aa\x02/V[\x90Pa\x01\x98\x87a\x01\x81\x84\x84a\x02QV[a\x01\x93\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x02\x18V[a\x01\xB0V[\x94PPa\x01\xA4V[\x85\x93P[PPP\x93P\x93P\x93P\x93V[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x01\xC5\x83\x85a\x02QV[a\x01\xCF\x91\x90a\x02/V[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x01\xEBW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x02*Wa\x02*a\x02\x02V[P\x03\x90V[`\0\x82a\x02LWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x02kWa\x02ka\x02\x02V[P\x02\x90V[`\0\x82\x19\x82\x11\x15a\x02\x83Wa\x02\x83a\x02\x02V[P\x01\x90V\xFE\xA2dipfsX\"\x12 \x83\"*z\xB5\xAEY'0\x8D\xE5\xC3\xA24L\xF626\x07q\xF5O\xE6U\xC8\xD3\x10-\x18j\xC1\x8AdsolcC\0\x08\x0F\x003";
    /// The deployed bytecode of the contract.
    pub static MULTIPLIERHELPER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MultiplierHelper<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MultiplierHelper<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MultiplierHelper<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MultiplierHelper<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MultiplierHelper<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MultiplierHelper))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MultiplierHelper<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MULTIPLIERHELPER_ABI.clone(),
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
                MULTIPLIERHELPER_ABI.clone(),
                MULTIPLIERHELPER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `computeMultiplier` (0xd5d04ef3) function
        pub fn compute_multiplier(
            &self,
            last_tracked_timestamp: ::ethers::core::types::U256,
            last_tracked_multiplier: ::ethers::core::types::U256,
            fee_scaled: ::ethers::core::types::U256,
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
                .method_hash(
                    [213, 208, 78, 243],
                    (last_tracked_timestamp, last_tracked_multiplier, fee_scaled),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MultiplierHelper<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `MultiplierFeeTooHigh` with signature `MultiplierFeeTooHigh()` and selector `0xd39d6a84`
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
    #[etherror(name = "MultiplierFeeTooHigh", abi = "MultiplierFeeTooHigh()")]
    pub struct MultiplierFeeTooHigh;
    ///Container type for all input parameters for the `computeMultiplier` function with signature `computeMultiplier(uint256,uint256,uint256)` and selector `0xd5d04ef3`
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
        name = "computeMultiplier",
        abi = "computeMultiplier(uint256,uint256,uint256)"
    )]
    pub struct ComputeMultiplierCall {
        pub last_tracked_timestamp: ::ethers::core::types::U256,
        pub last_tracked_multiplier: ::ethers::core::types::U256,
        pub fee_scaled: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `computeMultiplier` function with signature `computeMultiplier(uint256,uint256,uint256)` and selector `0xd5d04ef3`
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
    pub struct ComputeMultiplierReturn {
        pub tracked_timestamp: ::ethers::core::types::U256,
        pub tracked_multiplier: ::ethers::core::types::U256,
        pub new_fee_accrued: ::ethers::core::types::U256,
        pub current_multiplier: ::ethers::core::types::U256,
    }
}
