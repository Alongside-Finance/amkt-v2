pub use active_bounty::*;
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
pub mod active_bounty {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_authority"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("authority"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("authority"),
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
                    ::std::borrow::ToOwned::to_owned("setHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setHash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bountyHash"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ActiveBountyAuth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ActiveBountyAuth"),
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
    pub static ACTIVEBOUNTY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x01\xE38\x03\x80a\x01\xE3\x839\x81\x01`@\x81\x90Ra\0/\x91a\0@V[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0pV[`\0` \x82\x84\x03\x12\x15a\0RW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0iW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x01Sa\0\x90`\09`\0\x81\x81``\x01R`\xC1\x01Ra\x01S`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\x0CLB\x85\x14a\0FW\x80c\xBF~!O\x14a\0[W\x80c\xC5\xAB\x93\xA1\x14a\0\x9FW[`\0\x80\xFD[a\0Ya\0T6`\x04a\x01\x04V[a\0\xB6V[\0[a\0\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA8`\0T\x81V[`@Q\x90\x81R` \x01a\0\x96V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\0\xFFW`@Qc\x0C\x08\x0E\x8B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0UV[`\0` \x82\x84\x03\x12\x15a\x01\x16W`\0\x80\xFD[P5\x91\x90PV\xFE\xA2dipfsX\"\x12 \x1B\x98lhD\xF2\x05\xDC,}\x83\xDEi\xF9}\xB1(\xB8\xA9\xC4\xDE\x03\x8Ar\x9D\xF7\xB6L%I}\xBDdsolcC\0\x08\x0F\x003";
    /// The bytecode of the contract.
    pub static ACTIVEBOUNTY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\x0CLB\x85\x14a\0FW\x80c\xBF~!O\x14a\0[W\x80c\xC5\xAB\x93\xA1\x14a\0\x9FW[`\0\x80\xFD[a\0Ya\0T6`\x04a\x01\x04V[a\0\xB6V[\0[a\0\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA8`\0T\x81V[`@Q\x90\x81R` \x01a\0\x96V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\0\xFFW`@Qc\x0C\x08\x0E\x8B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0UV[`\0` \x82\x84\x03\x12\x15a\x01\x16W`\0\x80\xFD[P5\x91\x90PV\xFE\xA2dipfsX\"\x12 \x1B\x98lhD\xF2\x05\xDC,}\x83\xDEi\xF9}\xB1(\xB8\xA9\xC4\xDE\x03\x8Ar\x9D\xF7\xB6L%I}\xBDdsolcC\0\x08\x0F\x003";
    /// The deployed bytecode of the contract.
    pub static ACTIVEBOUNTY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ActiveBounty<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ActiveBounty<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ActiveBounty<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ActiveBounty<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ActiveBounty<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ActiveBounty))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ActiveBounty<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ACTIVEBOUNTY_ABI.clone(),
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
                ACTIVEBOUNTY_ABI.clone(),
                ACTIVEBOUNTY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `activeBounty` (0xc5ab93a1) function
        pub fn active_bounty(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([197, 171, 147, 161], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `authority` (0xbf7e214f) function
        pub fn authority(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([191, 126, 33, 79], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setHash` (0x0c4c4285) function
        pub fn set_hash(
            &self,
            bounty_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([12, 76, 66, 133], bounty_hash)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ActiveBounty<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ActiveBountyAuth` with signature `ActiveBountyAuth()` and selector `0x18101d16`
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
    #[etherror(name = "ActiveBountyAuth", abi = "ActiveBountyAuth()")]
    pub struct ActiveBountyAuth;
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
    ///Container type for all input parameters for the `authority` function with signature `authority()` and selector `0xbf7e214f`
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
    #[ethcall(name = "authority", abi = "authority()")]
    pub struct AuthorityCall;
    ///Container type for all input parameters for the `setHash` function with signature `setHash(bytes32)` and selector `0x0c4c4285`
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
    #[ethcall(name = "setHash", abi = "setHash(bytes32)")]
    pub struct SetHashCall {
        pub bounty_hash: [u8; 32],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ActiveBountyCalls {
        ActiveBounty(ActiveBountyCall),
        Authority(AuthorityCall),
        SetHash(SetHashCall),
    }
    impl ::ethers::core::abi::AbiDecode for ActiveBountyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ActiveBountyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ActiveBounty(decoded));
            }
            if let Ok(decoded)
                = <AuthorityCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Authority(decoded));
            }
            if let Ok(decoded)
                = <SetHashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetHash(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ActiveBountyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ActiveBounty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Authority(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetHash(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ActiveBountyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ActiveBounty(element) => ::core::fmt::Display::fmt(element, f),
                Self::Authority(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetHash(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ActiveBountyCall> for ActiveBountyCalls {
        fn from(value: ActiveBountyCall) -> Self {
            Self::ActiveBounty(value)
        }
    }
    impl ::core::convert::From<AuthorityCall> for ActiveBountyCalls {
        fn from(value: AuthorityCall) -> Self {
            Self::Authority(value)
        }
    }
    impl ::core::convert::From<SetHashCall> for ActiveBountyCalls {
        fn from(value: SetHashCall) -> Self {
            Self::SetHash(value)
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
    pub struct ActiveBountyReturn(pub [u8; 32]);
    ///Container type for all return fields from the `authority` function with signature `authority()` and selector `0xbf7e214f`
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
    pub struct AuthorityReturn(pub ::ethers::core::types::Address);
}
