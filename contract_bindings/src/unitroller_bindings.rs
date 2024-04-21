pub use unitroller::*;
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
pub mod unitroller {
    const _: () = {
        ::core::include_bytes!("/home/joss/dev/liquidator/liquidator_v1/abi/unitroller.json",);
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("_acceptAdmin"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("_acceptAdmin"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::Some(false),
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_acceptImplementation"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("_acceptImplementation",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::Some(false),
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_setPendingAdmin"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("_setPendingAdmin"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newPendingAdmin"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::Some(false),
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_setPendingImplementation"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("_setPendingImplementation",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newPendingImplementation",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::Some(false),
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("admin"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("admin"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::Some(true),
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("comptrollerImplementation"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("comptrollerImplementation",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::Some(true),
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("implementation"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("implementation"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::Some(true),
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pendingAdmin"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pendingAdmin"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::Some(true),
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pendingComptrollerImplementation"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pendingComptrollerImplementation",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::Some(true),
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Failure"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Failure"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("error"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("info"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("detail"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewAdmin"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("NewAdmin"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("oldAdmin"),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewImplementation"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("NewImplementation"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("oldImplementation"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newImplementation"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewPendingAdmin"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("NewPendingAdmin"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("oldPendingAdmin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newPendingAdmin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewPendingImplementation"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("NewPendingImplementation",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("oldPendingImplementation",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newPendingImplementation",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static UNITROLLER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct Unitroller<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Unitroller<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Unitroller<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Unitroller<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Unitroller<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Unitroller))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Unitroller<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                UNITROLLER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `_acceptAdmin` (0xe9c714f2) function
        pub fn accept_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([233, 199, 20, 242], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_acceptImplementation` (0xc1e80334) function
        pub fn accept_implementation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([193, 232, 3, 52], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_setPendingAdmin` (0xb71d1a0c) function
        pub fn set_pending_admin(
            &self,
            new_pending_admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([183, 29, 26, 12], new_pending_admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_setPendingImplementation` (0xe992a041) function
        pub fn set_pending_implementation(
            &self,
            new_pending_implementation: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([233, 146, 160, 65], new_pending_implementation)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `admin` (0xf851a440) function
        pub fn admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `comptrollerImplementation` (0xbb82aa5e) function
        pub fn comptroller_implementation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([187, 130, 170, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `implementation` (0x5c60da1b) function
        pub fn implementation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([92, 96, 218, 27], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingAdmin` (0x26782247) function
        pub fn pending_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([38, 120, 34, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingComptrollerImplementation` (0xdcfbc0c7) function
        pub fn pending_comptroller_implementation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([220, 251, 192, 199], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Failure` event
        pub fn failure_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FailureFilter> {
            self.0.event()
        }
        ///Gets the contract's `NewAdmin` event
        pub fn new_admin_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewAdminFilter> {
            self.0.event()
        }
        ///Gets the contract's `NewImplementation` event
        pub fn new_implementation_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewImplementationFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `NewPendingAdmin` event
        pub fn new_pending_admin_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewPendingAdminFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `NewPendingImplementation` event
        pub fn new_pending_implementation_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewPendingImplementationFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UnitrollerEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Unitroller<M> {
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
        Hash,
    )]
    #[ethevent(name = "Failure", abi = "Failure(uint256,uint256,uint256)")]
    pub struct FailureFilter {
        pub error: ::ethers::core::types::U256,
        pub info: ::ethers::core::types::U256,
        pub detail: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "NewAdmin", abi = "NewAdmin(address,address)")]
    pub struct NewAdminFilter {
        pub old_admin: ::ethers::core::types::Address,
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
        Hash,
    )]
    #[ethevent(name = "NewImplementation", abi = "NewImplementation(address,address)")]
    pub struct NewImplementationFilter {
        pub old_implementation: ::ethers::core::types::Address,
        pub new_implementation: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "NewPendingAdmin", abi = "NewPendingAdmin(address,address)")]
    pub struct NewPendingAdminFilter {
        pub old_pending_admin: ::ethers::core::types::Address,
        pub new_pending_admin: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "NewPendingImplementation",
        abi = "NewPendingImplementation(address,address)"
    )]
    pub struct NewPendingImplementationFilter {
        pub old_pending_implementation: ::ethers::core::types::Address,
        pub new_pending_implementation: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum UnitrollerEvents {
        FailureFilter(FailureFilter),
        NewAdminFilter(NewAdminFilter),
        NewImplementationFilter(NewImplementationFilter),
        NewPendingAdminFilter(NewPendingAdminFilter),
        NewPendingImplementationFilter(NewPendingImplementationFilter),
    }
    impl ::ethers::contract::EthLogDecode for UnitrollerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = FailureFilter::decode_log(log) {
                return Ok(UnitrollerEvents::FailureFilter(decoded));
            }
            if let Ok(decoded) = NewAdminFilter::decode_log(log) {
                return Ok(UnitrollerEvents::NewAdminFilter(decoded));
            }
            if let Ok(decoded) = NewImplementationFilter::decode_log(log) {
                return Ok(UnitrollerEvents::NewImplementationFilter(decoded));
            }
            if let Ok(decoded) = NewPendingAdminFilter::decode_log(log) {
                return Ok(UnitrollerEvents::NewPendingAdminFilter(decoded));
            }
            if let Ok(decoded) = NewPendingImplementationFilter::decode_log(log) {
                return Ok(UnitrollerEvents::NewPendingImplementationFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for UnitrollerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FailureFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewAdminFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewImplementationFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewPendingAdminFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewPendingImplementationFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<FailureFilter> for UnitrollerEvents {
        fn from(value: FailureFilter) -> Self {
            Self::FailureFilter(value)
        }
    }
    impl ::core::convert::From<NewAdminFilter> for UnitrollerEvents {
        fn from(value: NewAdminFilter) -> Self {
            Self::NewAdminFilter(value)
        }
    }
    impl ::core::convert::From<NewImplementationFilter> for UnitrollerEvents {
        fn from(value: NewImplementationFilter) -> Self {
            Self::NewImplementationFilter(value)
        }
    }
    impl ::core::convert::From<NewPendingAdminFilter> for UnitrollerEvents {
        fn from(value: NewPendingAdminFilter) -> Self {
            Self::NewPendingAdminFilter(value)
        }
    }
    impl ::core::convert::From<NewPendingImplementationFilter> for UnitrollerEvents {
        fn from(value: NewPendingImplementationFilter) -> Self {
            Self::NewPendingImplementationFilter(value)
        }
    }
    ///Container type for all input parameters for the `_acceptAdmin` function with signature `_acceptAdmin()` and selector `0xe9c714f2`
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
    #[ethcall(name = "_acceptAdmin", abi = "_acceptAdmin()")]
    pub struct AcceptAdminCall;
    ///Container type for all input parameters for the `_acceptImplementation` function with signature `_acceptImplementation()` and selector `0xc1e80334`
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
    #[ethcall(name = "_acceptImplementation", abi = "_acceptImplementation()")]
    pub struct AcceptImplementationCall;
    ///Container type for all input parameters for the `_setPendingAdmin` function with signature `_setPendingAdmin(address)` and selector `0xb71d1a0c`
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
    #[ethcall(name = "_setPendingAdmin", abi = "_setPendingAdmin(address)")]
    pub struct SetPendingAdminCall {
        pub new_pending_admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `_setPendingImplementation` function with signature `_setPendingImplementation(address)` and selector `0xe992a041`
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
    #[ethcall(
        name = "_setPendingImplementation",
        abi = "_setPendingImplementation(address)"
    )]
    pub struct SetPendingImplementationCall {
        pub new_pending_implementation: ::ethers::core::types::Address,
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
        Hash,
    )]
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
    ///Container type for all input parameters for the `comptrollerImplementation` function with signature `comptrollerImplementation()` and selector `0xbb82aa5e`
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
    #[ethcall(
        name = "comptrollerImplementation",
        abi = "comptrollerImplementation()"
    )]
    pub struct ComptrollerImplementationCall;
    ///Container type for all input parameters for the `implementation` function with signature `implementation()` and selector `0x5c60da1b`
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
    #[ethcall(name = "implementation", abi = "implementation()")]
    pub struct ImplementationCall;
    ///Container type for all input parameters for the `pendingAdmin` function with signature `pendingAdmin()` and selector `0x26782247`
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
    #[ethcall(name = "pendingAdmin", abi = "pendingAdmin()")]
    pub struct PendingAdminCall;
    ///Container type for all input parameters for the `pendingComptrollerImplementation` function with signature `pendingComptrollerImplementation()` and selector `0xdcfbc0c7`
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
    #[ethcall(
        name = "pendingComptrollerImplementation",
        abi = "pendingComptrollerImplementation()"
    )]
    pub struct PendingComptrollerImplementationCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum UnitrollerCalls {
        AcceptAdmin(AcceptAdminCall),
        AcceptImplementation(AcceptImplementationCall),
        SetPendingAdmin(SetPendingAdminCall),
        SetPendingImplementation(SetPendingImplementationCall),
        Admin(AdminCall),
        ComptrollerImplementation(ComptrollerImplementationCall),
        Implementation(ImplementationCall),
        PendingAdmin(PendingAdminCall),
        PendingComptrollerImplementation(PendingComptrollerImplementationCall),
    }
    impl ::ethers::core::abi::AbiDecode for UnitrollerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AcceptAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AcceptAdmin(decoded));
            }
            if let Ok(decoded) =
                <AcceptImplementationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AcceptImplementation(decoded));
            }
            if let Ok(decoded) =
                <SetPendingAdminCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetPendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <SetPendingImplementationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetPendingImplementation(decoded));
            }
            if let Ok(decoded) = <AdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Admin(decoded));
            }
            if let Ok(decoded) =
                <ComptrollerImplementationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ComptrollerImplementation(decoded));
            }
            if let Ok(decoded) =
                <ImplementationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Implementation(decoded));
            }
            if let Ok(decoded) = <PendingAdminCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <PendingComptrollerImplementationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::PendingComptrollerImplementation(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for UnitrollerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AcceptAdmin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AcceptImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPendingAdmin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetPendingImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Admin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ComptrollerImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Implementation(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PendingAdmin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PendingComptrollerImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for UnitrollerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AcceptAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::AcceptImplementation(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPendingAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPendingImplementation(element) => ::core::fmt::Display::fmt(element, f),
                Self::Admin(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComptrollerImplementation(element) => ::core::fmt::Display::fmt(element, f),
                Self::Implementation(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingComptrollerImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AcceptAdminCall> for UnitrollerCalls {
        fn from(value: AcceptAdminCall) -> Self {
            Self::AcceptAdmin(value)
        }
    }
    impl ::core::convert::From<AcceptImplementationCall> for UnitrollerCalls {
        fn from(value: AcceptImplementationCall) -> Self {
            Self::AcceptImplementation(value)
        }
    }
    impl ::core::convert::From<SetPendingAdminCall> for UnitrollerCalls {
        fn from(value: SetPendingAdminCall) -> Self {
            Self::SetPendingAdmin(value)
        }
    }
    impl ::core::convert::From<SetPendingImplementationCall> for UnitrollerCalls {
        fn from(value: SetPendingImplementationCall) -> Self {
            Self::SetPendingImplementation(value)
        }
    }
    impl ::core::convert::From<AdminCall> for UnitrollerCalls {
        fn from(value: AdminCall) -> Self {
            Self::Admin(value)
        }
    }
    impl ::core::convert::From<ComptrollerImplementationCall> for UnitrollerCalls {
        fn from(value: ComptrollerImplementationCall) -> Self {
            Self::ComptrollerImplementation(value)
        }
    }
    impl ::core::convert::From<ImplementationCall> for UnitrollerCalls {
        fn from(value: ImplementationCall) -> Self {
            Self::Implementation(value)
        }
    }
    impl ::core::convert::From<PendingAdminCall> for UnitrollerCalls {
        fn from(value: PendingAdminCall) -> Self {
            Self::PendingAdmin(value)
        }
    }
    impl ::core::convert::From<PendingComptrollerImplementationCall> for UnitrollerCalls {
        fn from(value: PendingComptrollerImplementationCall) -> Self {
            Self::PendingComptrollerImplementation(value)
        }
    }
    ///Container type for all return fields from the `_acceptAdmin` function with signature `_acceptAdmin()` and selector `0xe9c714f2`
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
    pub struct AcceptAdminReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `_acceptImplementation` function with signature `_acceptImplementation()` and selector `0xc1e80334`
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
    pub struct AcceptImplementationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `_setPendingAdmin` function with signature `_setPendingAdmin(address)` and selector `0xb71d1a0c`
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
    pub struct SetPendingAdminReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `_setPendingImplementation` function with signature `_setPendingImplementation(address)` and selector `0xe992a041`
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
    pub struct SetPendingImplementationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `admin` function with signature `admin()` and selector `0xf851a440`
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
    pub struct AdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `comptrollerImplementation` function with signature `comptrollerImplementation()` and selector `0xbb82aa5e`
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
    pub struct ComptrollerImplementationReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `implementation` function with signature `implementation()` and selector `0x5c60da1b`
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
    pub struct ImplementationReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pendingAdmin` function with signature `pendingAdmin()` and selector `0x26782247`
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
    pub struct PendingAdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pendingComptrollerImplementation` function with signature `pendingComptrollerImplementation()` and selector `0xdcfbc0c7`
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
    pub struct PendingComptrollerImplementationReturn(pub ::ethers::core::types::Address);
}
