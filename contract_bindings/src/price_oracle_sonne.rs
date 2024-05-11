pub use sonne_price_oracle::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod sonne_price_oracle {
    const _: () = {
        ::core::include_bytes!(
            "/home/joss/dev/liquidator/liquidator_v1/abi/sonne_price_oracle.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("symbols_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(::ethers::core::abi::ethabi::ParamType::String,),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string[]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("feeds_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(::ethers::core::abi::ethabi::ParamType::Address,),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IAggregatorV3[]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("baseUnits_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(::ethers::core::abi::ethabi::ParamType::Uint(
                                256usize
                            ),),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256[]"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("baseUnits"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("baseUnits"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPrice"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("cToken"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract CToken"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getUnderlyingPrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getUnderlyingPrice"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("cToken"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract CToken"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isPriceOracle"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isPriceOracle"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("priceFeeds"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("priceFeeds"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IAggregatorV3"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SONNEPRICEORACLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct SonnePriceOracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SonnePriceOracle<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SonnePriceOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SonnePriceOracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SonnePriceOracle<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SonnePriceOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SonnePriceOracle<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                SONNEPRICEORACLE_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `baseUnits` (0x51003683) function
        pub fn base_units(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([81, 0, 54, 131], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPrice` (0x41976e09) function
        pub fn get_price(
            &self,
            c_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([65, 151, 110, 9], c_token)
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
        ///Calls the contract's `isPriceOracle` (0x66331bba) function
        pub fn is_price_oracle(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([102, 51, 27, 186], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `priceFeeds` (0xcb8ae86c) function
        pub fn price_feeds(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([203, 138, 232, 108], p0)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for SonnePriceOracle<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `baseUnits` function with signature `baseUnits(string)` and selector `0x51003683`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "baseUnits", abi = "baseUnits(string)")]
    pub struct BaseUnitsCall(pub ::std::string::String);
    ///Container type for all input parameters for the `getPrice` function with signature `getPrice(address)` and selector `0x41976e09`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPrice", abi = "getPrice(address)")]
    pub struct GetPriceCall {
        pub c_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getUnderlyingPrice` function with signature `getUnderlyingPrice(address)` and selector `0xfc57d4df`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getUnderlyingPrice", abi = "getUnderlyingPrice(address)")]
    pub struct GetUnderlyingPriceCall {
        pub c_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isPriceOracle` function with signature `isPriceOracle()` and selector `0x66331bba`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isPriceOracle", abi = "isPriceOracle()")]
    pub struct IsPriceOracleCall;
    ///Container type for all input parameters for the `priceFeeds` function with signature `priceFeeds(string)` and selector `0xcb8ae86c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "priceFeeds", abi = "priceFeeds(string)")]
    pub struct PriceFeedsCall(pub ::std::string::String);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SonnePriceOracleCalls {
        BaseUnits(BaseUnitsCall),
        GetPrice(GetPriceCall),
        GetUnderlyingPrice(GetUnderlyingPriceCall),
        IsPriceOracle(IsPriceOracleCall),
        PriceFeeds(PriceFeedsCall),
    }
    impl ::ethers::core::abi::AbiDecode for SonnePriceOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BaseUnitsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BaseUnits(decoded));
            }
            if let Ok(decoded) = <GetPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPrice(decoded));
            }
            if let Ok(decoded) =
                <GetUnderlyingPriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetUnderlyingPrice(decoded));
            }
            if let Ok(decoded) = <IsPriceOracleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsPriceOracle(decoded));
            }
            if let Ok(decoded) = <PriceFeedsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PriceFeeds(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SonnePriceOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BaseUnits(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetUnderlyingPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsPriceOracle(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PriceFeeds(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for SonnePriceOracleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BaseUnits(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUnderlyingPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPriceOracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::PriceFeeds(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BaseUnitsCall> for SonnePriceOracleCalls {
        fn from(value: BaseUnitsCall) -> Self {
            Self::BaseUnits(value)
        }
    }
    impl ::core::convert::From<GetPriceCall> for SonnePriceOracleCalls {
        fn from(value: GetPriceCall) -> Self {
            Self::GetPrice(value)
        }
    }
    impl ::core::convert::From<GetUnderlyingPriceCall> for SonnePriceOracleCalls {
        fn from(value: GetUnderlyingPriceCall) -> Self {
            Self::GetUnderlyingPrice(value)
        }
    }
    impl ::core::convert::From<IsPriceOracleCall> for SonnePriceOracleCalls {
        fn from(value: IsPriceOracleCall) -> Self {
            Self::IsPriceOracle(value)
        }
    }
    impl ::core::convert::From<PriceFeedsCall> for SonnePriceOracleCalls {
        fn from(value: PriceFeedsCall) -> Self {
            Self::PriceFeeds(value)
        }
    }
    ///Container type for all return fields from the `baseUnits` function with signature `baseUnits(string)` and selector `0x51003683`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BaseUnitsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPrice` function with signature `getPrice(address)` and selector `0x41976e09`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetPriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getUnderlyingPrice` function with signature `getUnderlyingPrice(address)` and selector `0xfc57d4df`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetUnderlyingPriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `isPriceOracle` function with signature `isPriceOracle()` and selector `0x66331bba`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsPriceOracleReturn(pub bool);
    ///Container type for all return fields from the `priceFeeds` function with signature `priceFeeds(string)` and selector `0xcb8ae86c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PriceFeedsReturn(pub ::ethers::core::types::Address);
}
