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
                        name: ::std::borrow::ToOwned::to_owned("_poster"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("_acceptAnchorAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_acceptAnchorAdmin"),
                            inputs: ::std::vec![],
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
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_assetPrices"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_assetPrices"),
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
                                    name: ::std::borrow::ToOwned::to_owned("mantissa"),
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
                    ::std::borrow::ToOwned::to_owned("_setPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_setPaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("requestedState"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_setPendingAnchor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_setPendingAnchor"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newScaledPrice"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_setPendingAnchorAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_setPendingAnchorAdmin",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newPendingAnchorAdmin",
                                    ),
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
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("anchorAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("anchorAdmin"),
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
                    ::std::borrow::ToOwned::to_owned("anchors"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("anchors"),
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
                                    name: ::std::borrow::ToOwned::to_owned("period"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("priceMantissa"),
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
                    ::std::borrow::ToOwned::to_owned("assetPrices"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("assetPrices"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("maxSwing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxSwing"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mantissa"),
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
                    ::std::borrow::ToOwned::to_owned("maxSwingMantissa"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxSwingMantissa"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("oneHour"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("oneHour"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pendingAnchorAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pendingAnchorAdmin"),
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
                    ::std::borrow::ToOwned::to_owned("pendingAnchors"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pendingAnchors"),
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
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("poster"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("poster"),
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
                    ::std::borrow::ToOwned::to_owned("setPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "requestedPriceMantissa",
                                    ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPrices"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPrices"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "requestedPriceMantissas",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(false),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CappedPricePosted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("CappedPricePosted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "requestedPriceMantissa",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "anchorPriceMantissa",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "cappedPriceMantissa",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Failure"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Failure"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("error"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("info"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("detail"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewAnchorAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NewAnchorAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldAnchorAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAnchorAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewPendingAnchor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NewPendingAnchor"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("anchorAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldScaledPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newScaledPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewPendingAnchorAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NewPendingAnchorAdmin",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "oldPendingAnchorAdmin",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newPendingAnchorAdmin",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OracleFailure"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OracleFailure"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("msgSender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("error"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("info"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("detail"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PricePosted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PricePosted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "previousPriceMantissa",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "requestedPriceMantissa",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPriceMantissa"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetPaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newState"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
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
            fallback: true,
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
        ///Calls the contract's `_acceptAnchorAdmin` (0xccb13cbd) function
        pub fn accept_anchor_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([204, 177, 60, 189], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_assetPrices` (0x183f3444) function
        pub fn _asset_prices(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 63, 52, 68], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_setPaused` (0x26617c28) function
        pub fn set_paused(
            &self,
            requested_state: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([38, 97, 124, 40], requested_state)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_setPendingAnchor` (0xde9d0e85) function
        pub fn set_pending_anchor(
            &self,
            asset: ::ethers::core::types::Address,
            new_scaled_price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([222, 157, 14, 133], (asset, new_scaled_price))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_setPendingAnchorAdmin` (0x9964622c) function
        pub fn set_pending_anchor_admin(
            &self,
            new_pending_anchor_admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([153, 100, 98, 44], new_pending_anchor_admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `anchorAdmin` (0x08f31857) function
        pub fn anchor_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([8, 243, 24, 87], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `anchors` (0x692374e3) function
        pub fn anchors(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([105, 35, 116, 227], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assetPrices` (0x5e9a523c) function
        pub fn asset_prices(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([94, 154, 82, 60], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPrice` (0x41976e09) function
        pub fn get_price(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([65, 151, 110, 9], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxSwing` (0xc5faf1d5) function
        pub fn max_swing(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([197, 250, 241, 213], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxSwingMantissa` (0x0c9c6301) function
        pub fn max_swing_mantissa(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([12, 156, 99, 1], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `oneHour` (0xc92da302) function
        pub fn one_hour(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([201, 45, 163, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `paused` (0x5c975abb) function
        pub fn paused(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingAnchorAdmin` (0x451b1e3a) function
        pub fn pending_anchor_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([69, 27, 30, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingAnchors` (0x9e8c4d95) function
        pub fn pending_anchors(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([158, 140, 77, 149], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `poster` (0x80959721) function
        pub fn poster(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([128, 149, 151, 33], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPrice` (0x00e4768b) function
        pub fn set_price(
            &self,
            asset: ::ethers::core::types::Address,
            requested_price_mantissa: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([0, 228, 118, 139], (asset, requested_price_mantissa))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPrices` (0x4352fa9f) function
        pub fn set_prices(
            &self,
            assets: ::std::vec::Vec<::ethers::core::types::Address>,
            requested_price_mantissas: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([67, 82, 250, 159], (assets, requested_price_mantissas))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `CappedPricePosted` event
        pub fn capped_price_posted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CappedPricePostedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Failure` event
        pub fn failure_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FailureFilter> {
            self.0.event()
        }
        ///Gets the contract's `NewAnchorAdmin` event
        pub fn new_anchor_admin_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewAnchorAdminFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewPendingAnchor` event
        pub fn new_pending_anchor_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewPendingAnchorFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewPendingAnchorAdmin` event
        pub fn new_pending_anchor_admin_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewPendingAnchorAdminFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OracleFailure` event
        pub fn oracle_failure_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OracleFailureFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PricePosted` event
        pub fn price_posted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PricePostedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetPaused` event
        pub fn set_paused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetPausedFilter,
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
        name = "CappedPricePosted",
        abi = "CappedPricePosted(address,uint256,uint256,uint256)"
    )]
    pub struct CappedPricePostedFilter {
        pub asset: ::ethers::core::types::Address,
        pub requested_price_mantissa: ::ethers::core::types::U256,
        pub anchor_price_mantissa: ::ethers::core::types::U256,
        pub capped_price_mantissa: ::ethers::core::types::U256,
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
        Hash
    )]
    #[ethevent(name = "NewAnchorAdmin", abi = "NewAnchorAdmin(address,address)")]
    pub struct NewAnchorAdminFilter {
        pub old_anchor_admin: ::ethers::core::types::Address,
        pub new_anchor_admin: ::ethers::core::types::Address,
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
        name = "NewPendingAnchor",
        abi = "NewPendingAnchor(address,address,uint256,uint256)"
    )]
    pub struct NewPendingAnchorFilter {
        pub anchor_admin: ::ethers::core::types::Address,
        pub asset: ::ethers::core::types::Address,
        pub old_scaled_price: ::ethers::core::types::U256,
        pub new_scaled_price: ::ethers::core::types::U256,
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
        name = "NewPendingAnchorAdmin",
        abi = "NewPendingAnchorAdmin(address,address)"
    )]
    pub struct NewPendingAnchorAdminFilter {
        pub old_pending_anchor_admin: ::ethers::core::types::Address,
        pub new_pending_anchor_admin: ::ethers::core::types::Address,
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
        name = "OracleFailure",
        abi = "OracleFailure(address,address,uint256,uint256,uint256)"
    )]
    pub struct OracleFailureFilter {
        pub msg_sender: ::ethers::core::types::Address,
        pub asset: ::ethers::core::types::Address,
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
        Hash
    )]
    #[ethevent(
        name = "PricePosted",
        abi = "PricePosted(address,uint256,uint256,uint256)"
    )]
    pub struct PricePostedFilter {
        pub asset: ::ethers::core::types::Address,
        pub previous_price_mantissa: ::ethers::core::types::U256,
        pub requested_price_mantissa: ::ethers::core::types::U256,
        pub new_price_mantissa: ::ethers::core::types::U256,
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
    #[ethevent(name = "SetPaused", abi = "SetPaused(bool)")]
    pub struct SetPausedFilter {
        pub new_state: bool,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IronBankPriceOracleEvents {
        CappedPricePostedFilter(CappedPricePostedFilter),
        FailureFilter(FailureFilter),
        NewAnchorAdminFilter(NewAnchorAdminFilter),
        NewPendingAnchorFilter(NewPendingAnchorFilter),
        NewPendingAnchorAdminFilter(NewPendingAnchorAdminFilter),
        OracleFailureFilter(OracleFailureFilter),
        PricePostedFilter(PricePostedFilter),
        SetPausedFilter(SetPausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IronBankPriceOracleEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = CappedPricePostedFilter::decode_log(log) {
                return Ok(IronBankPriceOracleEvents::CappedPricePostedFilter(decoded));
            }
            if let Ok(decoded) = FailureFilter::decode_log(log) {
                return Ok(IronBankPriceOracleEvents::FailureFilter(decoded));
            }
            if let Ok(decoded) = NewAnchorAdminFilter::decode_log(log) {
                return Ok(IronBankPriceOracleEvents::NewAnchorAdminFilter(decoded));
            }
            if let Ok(decoded) = NewPendingAnchorFilter::decode_log(log) {
                return Ok(IronBankPriceOracleEvents::NewPendingAnchorFilter(decoded));
            }
            if let Ok(decoded) = NewPendingAnchorAdminFilter::decode_log(log) {
                return Ok(
                    IronBankPriceOracleEvents::NewPendingAnchorAdminFilter(decoded),
                );
            }
            if let Ok(decoded) = OracleFailureFilter::decode_log(log) {
                return Ok(IronBankPriceOracleEvents::OracleFailureFilter(decoded));
            }
            if let Ok(decoded) = PricePostedFilter::decode_log(log) {
                return Ok(IronBankPriceOracleEvents::PricePostedFilter(decoded));
            }
            if let Ok(decoded) = SetPausedFilter::decode_log(log) {
                return Ok(IronBankPriceOracleEvents::SetPausedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IronBankPriceOracleEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CappedPricePostedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailureFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewAnchorAdminFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewPendingAnchorFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewPendingAnchorAdminFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OracleFailureFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PricePostedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CappedPricePostedFilter> for IronBankPriceOracleEvents {
        fn from(value: CappedPricePostedFilter) -> Self {
            Self::CappedPricePostedFilter(value)
        }
    }
    impl ::core::convert::From<FailureFilter> for IronBankPriceOracleEvents {
        fn from(value: FailureFilter) -> Self {
            Self::FailureFilter(value)
        }
    }
    impl ::core::convert::From<NewAnchorAdminFilter> for IronBankPriceOracleEvents {
        fn from(value: NewAnchorAdminFilter) -> Self {
            Self::NewAnchorAdminFilter(value)
        }
    }
    impl ::core::convert::From<NewPendingAnchorFilter> for IronBankPriceOracleEvents {
        fn from(value: NewPendingAnchorFilter) -> Self {
            Self::NewPendingAnchorFilter(value)
        }
    }
    impl ::core::convert::From<NewPendingAnchorAdminFilter>
    for IronBankPriceOracleEvents {
        fn from(value: NewPendingAnchorAdminFilter) -> Self {
            Self::NewPendingAnchorAdminFilter(value)
        }
    }
    impl ::core::convert::From<OracleFailureFilter> for IronBankPriceOracleEvents {
        fn from(value: OracleFailureFilter) -> Self {
            Self::OracleFailureFilter(value)
        }
    }
    impl ::core::convert::From<PricePostedFilter> for IronBankPriceOracleEvents {
        fn from(value: PricePostedFilter) -> Self {
            Self::PricePostedFilter(value)
        }
    }
    impl ::core::convert::From<SetPausedFilter> for IronBankPriceOracleEvents {
        fn from(value: SetPausedFilter) -> Self {
            Self::SetPausedFilter(value)
        }
    }
    ///Container type for all input parameters for the `_acceptAnchorAdmin` function with signature `_acceptAnchorAdmin()` and selector `0xccb13cbd`
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
    #[ethcall(name = "_acceptAnchorAdmin", abi = "_acceptAnchorAdmin()")]
    pub struct AcceptAnchorAdminCall;
    ///Container type for all input parameters for the `_assetPrices` function with signature `_assetPrices(address)` and selector `0x183f3444`
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
    #[ethcall(name = "_assetPrices", abi = "_assetPrices(address)")]
    pub struct _AssetPricesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `_setPaused` function with signature `_setPaused(bool)` and selector `0x26617c28`
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
    #[ethcall(name = "_setPaused", abi = "_setPaused(bool)")]
    pub struct SetPausedCall {
        pub requested_state: bool,
    }
    ///Container type for all input parameters for the `_setPendingAnchor` function with signature `_setPendingAnchor(address,uint256)` and selector `0xde9d0e85`
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
    #[ethcall(name = "_setPendingAnchor", abi = "_setPendingAnchor(address,uint256)")]
    pub struct SetPendingAnchorCall {
        pub asset: ::ethers::core::types::Address,
        pub new_scaled_price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `_setPendingAnchorAdmin` function with signature `_setPendingAnchorAdmin(address)` and selector `0x9964622c`
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
    #[ethcall(name = "_setPendingAnchorAdmin", abi = "_setPendingAnchorAdmin(address)")]
    pub struct SetPendingAnchorAdminCall {
        pub new_pending_anchor_admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `anchorAdmin` function with signature `anchorAdmin()` and selector `0x08f31857`
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
    #[ethcall(name = "anchorAdmin", abi = "anchorAdmin()")]
    pub struct AnchorAdminCall;
    ///Container type for all input parameters for the `anchors` function with signature `anchors(address)` and selector `0x692374e3`
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
    #[ethcall(name = "anchors", abi = "anchors(address)")]
    pub struct AnchorsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `assetPrices` function with signature `assetPrices(address)` and selector `0x5e9a523c`
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
    #[ethcall(name = "assetPrices", abi = "assetPrices(address)")]
    pub struct AssetPricesCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPrice` function with signature `getPrice(address)` and selector `0x41976e09`
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
    #[ethcall(name = "getPrice", abi = "getPrice(address)")]
    pub struct GetPriceCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `maxSwing` function with signature `maxSwing()` and selector `0xc5faf1d5`
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
    #[ethcall(name = "maxSwing", abi = "maxSwing()")]
    pub struct MaxSwingCall;
    ///Container type for all input parameters for the `maxSwingMantissa` function with signature `maxSwingMantissa()` and selector `0x0c9c6301`
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
    #[ethcall(name = "maxSwingMantissa", abi = "maxSwingMantissa()")]
    pub struct MaxSwingMantissaCall;
    ///Container type for all input parameters for the `oneHour` function with signature `oneHour()` and selector `0xc92da302`
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
    #[ethcall(name = "oneHour", abi = "oneHour()")]
    pub struct OneHourCall;
    ///Container type for all input parameters for the `paused` function with signature `paused()` and selector `0x5c975abb`
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
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    ///Container type for all input parameters for the `pendingAnchorAdmin` function with signature `pendingAnchorAdmin()` and selector `0x451b1e3a`
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
    #[ethcall(name = "pendingAnchorAdmin", abi = "pendingAnchorAdmin()")]
    pub struct PendingAnchorAdminCall;
    ///Container type for all input parameters for the `pendingAnchors` function with signature `pendingAnchors(address)` and selector `0x9e8c4d95`
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
    #[ethcall(name = "pendingAnchors", abi = "pendingAnchors(address)")]
    pub struct PendingAnchorsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `poster` function with signature `poster()` and selector `0x80959721`
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
    #[ethcall(name = "poster", abi = "poster()")]
    pub struct PosterCall;
    ///Container type for all input parameters for the `setPrice` function with signature `setPrice(address,uint256)` and selector `0x00e4768b`
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
    #[ethcall(name = "setPrice", abi = "setPrice(address,uint256)")]
    pub struct SetPriceCall {
        pub asset: ::ethers::core::types::Address,
        pub requested_price_mantissa: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setPrices` function with signature `setPrices(address[],uint256[])` and selector `0x4352fa9f`
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
    #[ethcall(name = "setPrices", abi = "setPrices(address[],uint256[])")]
    pub struct SetPricesCall {
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub requested_price_mantissas: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IronBankPriceOracleCalls {
        AcceptAnchorAdmin(AcceptAnchorAdminCall),
        _AssetPrices(_AssetPricesCall),
        SetPaused(SetPausedCall),
        SetPendingAnchor(SetPendingAnchorCall),
        SetPendingAnchorAdmin(SetPendingAnchorAdminCall),
        AnchorAdmin(AnchorAdminCall),
        Anchors(AnchorsCall),
        AssetPrices(AssetPricesCall),
        GetPrice(GetPriceCall),
        MaxSwing(MaxSwingCall),
        MaxSwingMantissa(MaxSwingMantissaCall),
        OneHour(OneHourCall),
        Paused(PausedCall),
        PendingAnchorAdmin(PendingAnchorAdminCall),
        PendingAnchors(PendingAnchorsCall),
        Poster(PosterCall),
        SetPrice(SetPriceCall),
        SetPrices(SetPricesCall),
    }
    impl ::ethers::core::abi::AbiDecode for IronBankPriceOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AcceptAnchorAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AcceptAnchorAdmin(decoded));
            }
            if let Ok(decoded) = <_AssetPricesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::_AssetPrices(decoded));
            }
            if let Ok(decoded) = <SetPausedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPaused(decoded));
            }
            if let Ok(decoded) = <SetPendingAnchorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPendingAnchor(decoded));
            }
            if let Ok(decoded) = <SetPendingAnchorAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPendingAnchorAdmin(decoded));
            }
            if let Ok(decoded) = <AnchorAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AnchorAdmin(decoded));
            }
            if let Ok(decoded) = <AnchorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Anchors(decoded));
            }
            if let Ok(decoded) = <AssetPricesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AssetPrices(decoded));
            }
            if let Ok(decoded) = <GetPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPrice(decoded));
            }
            if let Ok(decoded) = <MaxSwingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxSwing(decoded));
            }
            if let Ok(decoded) = <MaxSwingMantissaCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxSwingMantissa(decoded));
            }
            if let Ok(decoded) = <OneHourCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OneHour(decoded));
            }
            if let Ok(decoded) = <PausedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Paused(decoded));
            }
            if let Ok(decoded) = <PendingAnchorAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PendingAnchorAdmin(decoded));
            }
            if let Ok(decoded) = <PendingAnchorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PendingAnchors(decoded));
            }
            if let Ok(decoded) = <PosterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Poster(decoded));
            }
            if let Ok(decoded) = <SetPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPrice(decoded));
            }
            if let Ok(decoded) = <SetPricesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPrices(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IronBankPriceOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AcceptAnchorAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::_AssetPrices(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPendingAnchor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPendingAnchorAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AnchorAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Anchors(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssetPrices(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxSwing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxSwingMantissa(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OneHour(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PendingAnchorAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PendingAnchors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Poster(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPrices(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IronBankPriceOracleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AcceptAnchorAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::_AssetPrices(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPendingAnchor(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPendingAnchorAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AnchorAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::Anchors(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetPrices(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxSwing(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxSwingMantissa(element) => ::core::fmt::Display::fmt(element, f),
                Self::OneHour(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingAnchorAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PendingAnchors(element) => ::core::fmt::Display::fmt(element, f),
                Self::Poster(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPrices(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AcceptAnchorAdminCall> for IronBankPriceOracleCalls {
        fn from(value: AcceptAnchorAdminCall) -> Self {
            Self::AcceptAnchorAdmin(value)
        }
    }
    impl ::core::convert::From<_AssetPricesCall> for IronBankPriceOracleCalls {
        fn from(value: _AssetPricesCall) -> Self {
            Self::_AssetPrices(value)
        }
    }
    impl ::core::convert::From<SetPausedCall> for IronBankPriceOracleCalls {
        fn from(value: SetPausedCall) -> Self {
            Self::SetPaused(value)
        }
    }
    impl ::core::convert::From<SetPendingAnchorCall> for IronBankPriceOracleCalls {
        fn from(value: SetPendingAnchorCall) -> Self {
            Self::SetPendingAnchor(value)
        }
    }
    impl ::core::convert::From<SetPendingAnchorAdminCall> for IronBankPriceOracleCalls {
        fn from(value: SetPendingAnchorAdminCall) -> Self {
            Self::SetPendingAnchorAdmin(value)
        }
    }
    impl ::core::convert::From<AnchorAdminCall> for IronBankPriceOracleCalls {
        fn from(value: AnchorAdminCall) -> Self {
            Self::AnchorAdmin(value)
        }
    }
    impl ::core::convert::From<AnchorsCall> for IronBankPriceOracleCalls {
        fn from(value: AnchorsCall) -> Self {
            Self::Anchors(value)
        }
    }
    impl ::core::convert::From<AssetPricesCall> for IronBankPriceOracleCalls {
        fn from(value: AssetPricesCall) -> Self {
            Self::AssetPrices(value)
        }
    }
    impl ::core::convert::From<GetPriceCall> for IronBankPriceOracleCalls {
        fn from(value: GetPriceCall) -> Self {
            Self::GetPrice(value)
        }
    }
    impl ::core::convert::From<MaxSwingCall> for IronBankPriceOracleCalls {
        fn from(value: MaxSwingCall) -> Self {
            Self::MaxSwing(value)
        }
    }
    impl ::core::convert::From<MaxSwingMantissaCall> for IronBankPriceOracleCalls {
        fn from(value: MaxSwingMantissaCall) -> Self {
            Self::MaxSwingMantissa(value)
        }
    }
    impl ::core::convert::From<OneHourCall> for IronBankPriceOracleCalls {
        fn from(value: OneHourCall) -> Self {
            Self::OneHour(value)
        }
    }
    impl ::core::convert::From<PausedCall> for IronBankPriceOracleCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PendingAnchorAdminCall> for IronBankPriceOracleCalls {
        fn from(value: PendingAnchorAdminCall) -> Self {
            Self::PendingAnchorAdmin(value)
        }
    }
    impl ::core::convert::From<PendingAnchorsCall> for IronBankPriceOracleCalls {
        fn from(value: PendingAnchorsCall) -> Self {
            Self::PendingAnchors(value)
        }
    }
    impl ::core::convert::From<PosterCall> for IronBankPriceOracleCalls {
        fn from(value: PosterCall) -> Self {
            Self::Poster(value)
        }
    }
    impl ::core::convert::From<SetPriceCall> for IronBankPriceOracleCalls {
        fn from(value: SetPriceCall) -> Self {
            Self::SetPrice(value)
        }
    }
    impl ::core::convert::From<SetPricesCall> for IronBankPriceOracleCalls {
        fn from(value: SetPricesCall) -> Self {
            Self::SetPrices(value)
        }
    }
    ///Container type for all return fields from the `_acceptAnchorAdmin` function with signature `_acceptAnchorAdmin()` and selector `0xccb13cbd`
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
    pub struct AcceptAnchorAdminReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `_assetPrices` function with signature `_assetPrices(address)` and selector `0x183f3444`
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
    pub struct _AssetPricesReturn {
        pub mantissa: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `_setPaused` function with signature `_setPaused(bool)` and selector `0x26617c28`
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
    pub struct SetPausedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `_setPendingAnchor` function with signature `_setPendingAnchor(address,uint256)` and selector `0xde9d0e85`
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
    pub struct SetPendingAnchorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `_setPendingAnchorAdmin` function with signature `_setPendingAnchorAdmin(address)` and selector `0x9964622c`
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
    pub struct SetPendingAnchorAdminReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `anchorAdmin` function with signature `anchorAdmin()` and selector `0x08f31857`
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
    pub struct AnchorAdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `anchors` function with signature `anchors(address)` and selector `0x692374e3`
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
    pub struct AnchorsReturn {
        pub period: ::ethers::core::types::U256,
        pub price_mantissa: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `assetPrices` function with signature `assetPrices(address)` and selector `0x5e9a523c`
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
    pub struct AssetPricesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPrice` function with signature `getPrice(address)` and selector `0x41976e09`
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
    pub struct GetPriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `maxSwing` function with signature `maxSwing()` and selector `0xc5faf1d5`
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
    pub struct MaxSwingReturn {
        pub mantissa: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `maxSwingMantissa` function with signature `maxSwingMantissa()` and selector `0x0c9c6301`
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
    pub struct MaxSwingMantissaReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `oneHour` function with signature `oneHour()` and selector `0xc92da302`
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
    pub struct OneHourReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `paused` function with signature `paused()` and selector `0x5c975abb`
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
    pub struct PausedReturn(pub bool);
    ///Container type for all return fields from the `pendingAnchorAdmin` function with signature `pendingAnchorAdmin()` and selector `0x451b1e3a`
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
    pub struct PendingAnchorAdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pendingAnchors` function with signature `pendingAnchors(address)` and selector `0x9e8c4d95`
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
    pub struct PendingAnchorsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `poster` function with signature `poster()` and selector `0x80959721`
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
    pub struct PosterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `setPrice` function with signature `setPrice(address,uint256)` and selector `0x00e4768b`
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
    pub struct SetPriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `setPrices` function with signature `setPrices(address[],uint256[])` and selector `0x4352fa9f`
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
    pub struct SetPricesReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
}
