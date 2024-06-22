pub use iron_bank_price_oracle::*;
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
pub mod iron_bank_price_oracle {
    const _: () = {
        ::core::include_bytes!(
            "/home/joss/dev/liquidator/liquidator_v1/abi/ironbank_price_oracle.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("admin_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("v1PriceOracle_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("ethUsdAggregator_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("_setAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_setAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_admin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_setAggregators"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_setAggregators"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cTokenAddresses"),
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
                                    name: ::std::borrow::ToOwned::to_owned("sources"),
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
                                    name: ::std::borrow::ToOwned::to_owned("bases"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum PriceOracleProxyUSD.AggregatorBase[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_setGuardian"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_setGuardian"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_guardian"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("admin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("admin"),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("aggregators"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("aggregators"),
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
                                    name: ::std::borrow::ToOwned::to_owned("source"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract AggregatorV3Interface",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("base"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum PriceOracleProxyUSD.AggregatorBase",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ethUsdAggregator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ethUsdAggregator"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract AggregatorV3Interface",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getUnderlyingPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getUnderlyingPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract CToken"),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("guardian"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("guardian"),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("v1PriceOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("v1PriceOracle"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract V1PriceOracleInterface",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AggregatorUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AggregatorUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("cTokenAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("source"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("base"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("admin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetGuardian"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetGuardian"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("guardian"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
    pub static IRONBANKPRICEORACLE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IronBankPriceOracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IronBankPriceOracle<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IronBankPriceOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IronBankPriceOracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IronBankPriceOracle<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IronBankPriceOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IronBankPriceOracle<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IRONBANKPRICEORACLE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `_setAdmin` (0x3a74a767) function
        pub fn set_admin(
            &self,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([58, 116, 167, 103], admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_setAggregators` (0x05631658) function
        pub fn set_aggregators(
            &self,
            c_token_addresses: ::std::vec::Vec<::ethers::core::types::Address>,
            sources: ::std::vec::Vec<::ethers::core::types::Address>,
            bases: ::std::vec::Vec<u8>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 99, 22, 88], (c_token_addresses, sources, bases))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_setGuardian` (0xe38e8c0f) function
        pub fn set_guardian(
            &self,
            guardian: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 142, 140, 15], guardian)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `admin` (0xf851a440) function
        pub fn admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `aggregators` (0x112cdab9) function
        pub fn aggregators(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, u8),
        > {
            self.0
                .method_hash([17, 44, 218, 185], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ethUsdAggregator` (0x10236026) function
        pub fn eth_usd_aggregator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([16, 35, 96, 38], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUnderlyingPrice` (0xfc57d4df) function
        pub fn get_underlying_price(
            &self,
            c_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([252, 87, 212, 223], c_token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `guardian` (0x452a9320) function
        pub fn guardian(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([69, 42, 147, 32], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `v1PriceOracle` (0xfe10c98d) function
        pub fn v_1_price_oracle(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([254, 16, 201, 141], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AggregatorUpdated` event
        pub fn aggregator_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AggregatorUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetAdmin` event
        pub fn set_admin_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetAdminFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetGuardian` event
        pub fn set_guardian_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetGuardianFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IronBankPriceOracleEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IronBankPriceOracle<M> {
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
        name = "AggregatorUpdated",
        abi = "AggregatorUpdated(address,address,uint8)"
    )]
    pub struct AggregatorUpdatedFilter {
        pub c_token_address: ::ethers::core::types::Address,
        pub source: ::ethers::core::types::Address,
        pub base: u8,
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
    #[ethevent(name = "SetAdmin", abi = "SetAdmin(address)")]
    pub struct SetAdminFilter {
        pub admin: ::ethers::core::types::Address,
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
    #[ethevent(name = "SetGuardian", abi = "SetGuardian(address)")]
    pub struct SetGuardianFilter {
        pub guardian: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IronBankPriceOracleEvents {
        AggregatorUpdatedFilter(AggregatorUpdatedFilter),
        SetAdminFilter(SetAdminFilter),
        SetGuardianFilter(SetGuardianFilter),
    }
    impl ::ethers::contract::EthLogDecode for IronBankPriceOracleEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AggregatorUpdatedFilter::decode_log(log) {
                return Ok(IronBankPriceOracleEvents::AggregatorUpdatedFilter(decoded));
            }
            if let Ok(decoded) = SetAdminFilter::decode_log(log) {
                return Ok(IronBankPriceOracleEvents::SetAdminFilter(decoded));
            }
            if let Ok(decoded) = SetGuardianFilter::decode_log(log) {
                return Ok(IronBankPriceOracleEvents::SetGuardianFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IronBankPriceOracleEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AggregatorUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetAdminFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetGuardianFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AggregatorUpdatedFilter> for IronBankPriceOracleEvents {
        fn from(value: AggregatorUpdatedFilter) -> Self {
            Self::AggregatorUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<SetAdminFilter> for IronBankPriceOracleEvents {
        fn from(value: SetAdminFilter) -> Self {
            Self::SetAdminFilter(value)
        }
    }
    impl ::core::convert::From<SetGuardianFilter> for IronBankPriceOracleEvents {
        fn from(value: SetGuardianFilter) -> Self {
            Self::SetGuardianFilter(value)
        }
    }
    ///Container type for all input parameters for the `_setAdmin` function with signature `_setAdmin(address)` and selector `0x3a74a767`
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
    #[ethcall(name = "_setAdmin", abi = "_setAdmin(address)")]
    pub struct SetAdminCall {
        pub admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `_setAggregators` function with signature `_setAggregators(address[],address[],uint8[])` and selector `0x05631658`
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
        name = "_setAggregators",
        abi = "_setAggregators(address[],address[],uint8[])"
    )]
    pub struct SetAggregatorsCall {
        pub c_token_addresses: ::std::vec::Vec<::ethers::core::types::Address>,
        pub sources: ::std::vec::Vec<::ethers::core::types::Address>,
        pub bases: ::std::vec::Vec<u8>,
    }
    ///Container type for all input parameters for the `_setGuardian` function with signature `_setGuardian(address)` and selector `0xe38e8c0f`
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
    #[ethcall(name = "_setGuardian", abi = "_setGuardian(address)")]
    pub struct SetGuardianCall {
        pub guardian: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `admin` function with signature `admin()` and selector `0xf851a440`
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
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
    ///Container type for all input parameters for the `aggregators` function with signature `aggregators(address)` and selector `0x112cdab9`
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
    #[ethcall(name = "aggregators", abi = "aggregators(address)")]
    pub struct AggregatorsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `ethUsdAggregator` function with signature `ethUsdAggregator()` and selector `0x10236026`
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
    #[ethcall(name = "ethUsdAggregator", abi = "ethUsdAggregator()")]
    pub struct EthUsdAggregatorCall;
    ///Container type for all input parameters for the `getUnderlyingPrice` function with signature `getUnderlyingPrice(address)` and selector `0xfc57d4df`
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
    #[ethcall(name = "getUnderlyingPrice", abi = "getUnderlyingPrice(address)")]
    pub struct GetUnderlyingPriceCall {
        pub c_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `guardian` function with signature `guardian()` and selector `0x452a9320`
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
    #[ethcall(name = "guardian", abi = "guardian()")]
    pub struct GuardianCall;
    ///Container type for all input parameters for the `v1PriceOracle` function with signature `v1PriceOracle()` and selector `0xfe10c98d`
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
    #[ethcall(name = "v1PriceOracle", abi = "v1PriceOracle()")]
    pub struct V1PriceOracleCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IronBankPriceOracleCalls {
        SetAdmin(SetAdminCall),
        SetAggregators(SetAggregatorsCall),
        SetGuardian(SetGuardianCall),
        Admin(AdminCall),
        Aggregators(AggregatorsCall),
        EthUsdAggregator(EthUsdAggregatorCall),
        GetUnderlyingPrice(GetUnderlyingPriceCall),
        Guardian(GuardianCall),
        V1PriceOracle(V1PriceOracleCall),
    }
    impl ::ethers::core::abi::AbiDecode for IronBankPriceOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <SetAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetAdmin(decoded));
            }
            if let Ok(decoded) = <SetAggregatorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetAggregators(decoded));
            }
            if let Ok(decoded) = <SetGuardianCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetGuardian(decoded));
            }
            if let Ok(decoded) = <AdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Admin(decoded));
            }
            if let Ok(decoded) = <AggregatorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Aggregators(decoded));
            }
            if let Ok(decoded) = <EthUsdAggregatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EthUsdAggregator(decoded));
            }
            if let Ok(decoded) = <GetUnderlyingPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetUnderlyingPrice(decoded));
            }
            if let Ok(decoded) = <GuardianCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Guardian(decoded));
            }
            if let Ok(decoded) = <V1PriceOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::V1PriceOracle(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IronBankPriceOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::SetAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAggregators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetGuardian(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Admin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Aggregators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EthUsdAggregator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUnderlyingPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Guardian(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::V1PriceOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IronBankPriceOracleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::SetAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAggregators(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetGuardian(element) => ::core::fmt::Display::fmt(element, f),
                Self::Admin(element) => ::core::fmt::Display::fmt(element, f),
                Self::Aggregators(element) => ::core::fmt::Display::fmt(element, f),
                Self::EthUsdAggregator(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUnderlyingPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Guardian(element) => ::core::fmt::Display::fmt(element, f),
                Self::V1PriceOracle(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<SetAdminCall> for IronBankPriceOracleCalls {
        fn from(value: SetAdminCall) -> Self {
            Self::SetAdmin(value)
        }
    }
    impl ::core::convert::From<SetAggregatorsCall> for IronBankPriceOracleCalls {
        fn from(value: SetAggregatorsCall) -> Self {
            Self::SetAggregators(value)
        }
    }
    impl ::core::convert::From<SetGuardianCall> for IronBankPriceOracleCalls {
        fn from(value: SetGuardianCall) -> Self {
            Self::SetGuardian(value)
        }
    }
    impl ::core::convert::From<AdminCall> for IronBankPriceOracleCalls {
        fn from(value: AdminCall) -> Self {
            Self::Admin(value)
        }
    }
    impl ::core::convert::From<AggregatorsCall> for IronBankPriceOracleCalls {
        fn from(value: AggregatorsCall) -> Self {
            Self::Aggregators(value)
        }
    }
    impl ::core::convert::From<EthUsdAggregatorCall> for IronBankPriceOracleCalls {
        fn from(value: EthUsdAggregatorCall) -> Self {
            Self::EthUsdAggregator(value)
        }
    }
    impl ::core::convert::From<GetUnderlyingPriceCall> for IronBankPriceOracleCalls {
        fn from(value: GetUnderlyingPriceCall) -> Self {
            Self::GetUnderlyingPrice(value)
        }
    }
    impl ::core::convert::From<GuardianCall> for IronBankPriceOracleCalls {
        fn from(value: GuardianCall) -> Self {
            Self::Guardian(value)
        }
    }
    impl ::core::convert::From<V1PriceOracleCall> for IronBankPriceOracleCalls {
        fn from(value: V1PriceOracleCall) -> Self {
            Self::V1PriceOracle(value)
        }
    }
    ///Container type for all return fields from the `admin` function with signature `admin()` and selector `0xf851a440`
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
    pub struct AdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `aggregators` function with signature `aggregators(address)` and selector `0x112cdab9`
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
    pub struct AggregatorsReturn {
        pub source: ::ethers::core::types::Address,
        pub base: u8,
    }
    ///Container type for all return fields from the `ethUsdAggregator` function with signature `ethUsdAggregator()` and selector `0x10236026`
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
    pub struct EthUsdAggregatorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getUnderlyingPrice` function with signature `getUnderlyingPrice(address)` and selector `0xfc57d4df`
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
    pub struct GetUnderlyingPriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `guardian` function with signature `guardian()` and selector `0x452a9320`
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
    pub struct GuardianReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `v1PriceOracle` function with signature `v1PriceOracle()` and selector `0xfe10c98d`
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
    pub struct V1PriceOracleReturn(pub ::ethers::core::types::Address);
}
