pub use compish_price_oracle::*;
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
pub mod compish_price_oracle {
    const _: () = {
        ::core::include_bytes!(
            "/home/joss/dev/liquidator/liquidator_v1/abi/compound_price_oracle.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned(
                            "anchorToleranceMantissa_",
                        ),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("anchorPeriod_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint32"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("configs"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Tuple(
                                    ::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Bool,
                                    ],
                                ),
                            ),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "struct UniswapConfig.TokenConfig[]",
                            ),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ETH_BASE_UNIT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ETH_BASE_UNIT"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EXP_SCALE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("EXP_SCALE"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MAX_INTEGER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_INTEGER"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MAX_TOKENS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_TOKENS"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("acceptOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("acceptOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("activateFailover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("activateFailover"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbolHash"),
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
                (
                    ::std::borrow::ToOwned::to_owned("anchorPeriod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("anchorPeriod"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deactivateFailover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deactivateFailover"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbolHash"),
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
                (
                    ::std::borrow::ToOwned::to_owned("getTokenConfig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTokenConfig"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("i"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct UniswapConfig.TokenConfig",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTokenConfigByCToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTokenConfigByCToken",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct UniswapConfig.TokenConfig",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTokenConfigByReporter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTokenConfigByReporter",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reporter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct UniswapConfig.TokenConfig",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTokenConfigBySymbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTokenConfigBySymbol",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct UniswapConfig.TokenConfig",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTokenConfigBySymbolHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTokenConfigBySymbolHash",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbolHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct UniswapConfig.TokenConfig",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTokenConfigByUnderlying"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTokenConfigByUnderlying",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlying"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct UniswapConfig.TokenConfig",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
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
                (
                    ::std::borrow::ToOwned::to_owned("lowerBoundAnchorRatio"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lowerBoundAnchorRatio",
                            ),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("numTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("numTokens"),
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
                    ::std::borrow::ToOwned::to_owned("pokeFailedOverPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "pokeFailedOverPrice",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbolHash"),
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
                (
                    ::std::borrow::ToOwned::to_owned("price"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("price"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                (
                    ::std::borrow::ToOwned::to_owned("prices"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("prices"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        248usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint248"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("failoverActive"),
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
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                    ::std::borrow::ToOwned::to_owned("upperBoundAnchorRatio"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "upperBoundAnchorRatio",
                            ),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("currentAnswer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("FailoverActivated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FailoverActivated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("symbolHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailoverDeactivated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FailoverDeactivated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("symbolHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferRequested"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferRequested",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PriceGuarded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PriceGuarded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("symbolHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reporterPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("anchorPrice"),
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
                    ::std::borrow::ToOwned::to_owned("PriceUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PriceUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("symbolHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static COMPISHPRICEORACLE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct CompishPriceOracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CompishPriceOracle<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CompishPriceOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CompishPriceOracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CompishPriceOracle<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CompishPriceOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CompishPriceOracle<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    COMPISHPRICEORACLE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `ETH_BASE_UNIT` (0x152810fb) function
        pub fn eth_base_unit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([21, 40, 16, 251], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `EXP_SCALE` (0xbbba205d) function
        pub fn exp_scale(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([187, 186, 32, 93], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_INTEGER` (0x78eadb1a) function
        pub fn max_integer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([120, 234, 219, 26], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_TOKENS` (0xf47c84c5) function
        pub fn max_tokens(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([244, 124, 132, 197], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `acceptOwnership` (0x79ba5097) function
        pub fn accept_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 186, 80, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `activateFailover` (0x5f396923) function
        pub fn activate_failover(
            &self,
            symbol_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 57, 105, 35], symbol_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `anchorPeriod` (0xe9206d78) function
        pub fn anchor_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([233, 32, 109, 120], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deactivateFailover` (0x8185b81c) function
        pub fn deactivate_failover(
            &self,
            symbol_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 133, 184, 28], symbol_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTokenConfig` (0x8a003888) function
        pub fn get_token_config(
            &self,
            i: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, TokenConfig> {
            self.0
                .method_hash([138, 0, 56, 136], i)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTokenConfigByCToken` (0x9f599631) function
        pub fn get_token_config_by_c_token(
            &self,
            c_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, TokenConfig> {
            self.0
                .method_hash([159, 89, 150, 49], c_token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTokenConfigByReporter` (0x29feb7bc) function
        pub fn get_token_config_by_reporter(
            &self,
            reporter: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, TokenConfig> {
            self.0
                .method_hash([41, 254, 183, 188], reporter)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTokenConfigBySymbol` (0x276c2cba) function
        pub fn get_token_config_by_symbol(
            &self,
            symbol: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, TokenConfig> {
            self.0
                .method_hash([39, 108, 44, 186], symbol)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTokenConfigBySymbolHash` (0x1a125204) function
        pub fn get_token_config_by_symbol_hash(
            &self,
            symbol_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, TokenConfig> {
            self.0
                .method_hash([26, 18, 82, 4], symbol_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTokenConfigByUnderlying` (0x4da21942) function
        pub fn get_token_config_by_underlying(
            &self,
            underlying: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, TokenConfig> {
            self.0
                .method_hash([77, 162, 25, 66], underlying)
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
        ///Calls the contract's `lowerBoundAnchorRatio` (0x92b84357) function
        pub fn lower_bound_anchor_ratio(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([146, 184, 67, 87], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `numTokens` (0x8e499bcf) function
        pub fn num_tokens(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([142, 73, 155, 207], ())
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
        ///Calls the contract's `pokeFailedOverPrice` (0x68f9a97f) function
        pub fn poke_failed_over_price(
            &self,
            symbol_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([104, 249, 169, 127], symbol_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `price` (0xfe2c6198) function
        pub fn price(
            &self,
            symbol: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([254, 44, 97, 152], symbol)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prices` (0x60846bc6) function
        pub fn prices(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, bool),
        > {
            self.0
                .method_hash([96, 132, 107, 198], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], to)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upperBoundAnchorRatio` (0x24105209) function
        pub fn upper_bound_anchor_ratio(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([36, 16, 82, 9], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validate` (0xbeed9b51) function
        pub fn validate(
            &self,
            p0: ::ethers::core::types::U256,
            p1: ::ethers::core::types::I256,
            p2: ::ethers::core::types::U256,
            current_answer: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([190, 237, 155, 81], (p0, p1, p2, current_answer))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `FailoverActivated` event
        pub fn failover_activated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FailoverActivatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FailoverDeactivated` event
        pub fn failover_deactivated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FailoverDeactivatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferRequested` event
        pub fn ownership_transfer_requested_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferRequestedFilter,
        > {
            self.0.event()
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
        ///Gets the contract's `PriceGuarded` event
        pub fn price_guarded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PriceGuardedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PriceUpdated` event
        pub fn price_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PriceUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CompishPriceOracleEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CompishPriceOracle<M> {
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
    #[ethevent(name = "FailoverActivated", abi = "FailoverActivated(bytes32)")]
    pub struct FailoverActivatedFilter {
        #[ethevent(indexed)]
        pub symbol_hash: [u8; 32],
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
    #[ethevent(name = "FailoverDeactivated", abi = "FailoverDeactivated(bytes32)")]
    pub struct FailoverDeactivatedFilter {
        #[ethevent(indexed)]
        pub symbol_hash: [u8; 32],
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
        name = "OwnershipTransferRequested",
        abi = "OwnershipTransferRequested(address,address)"
    )]
    pub struct OwnershipTransferRequestedFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
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
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
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
    #[ethevent(name = "PriceGuarded", abi = "PriceGuarded(bytes32,uint256,uint256)")]
    pub struct PriceGuardedFilter {
        #[ethevent(indexed)]
        pub symbol_hash: [u8; 32],
        pub reporter_price: ::ethers::core::types::U256,
        pub anchor_price: ::ethers::core::types::U256,
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
    #[ethevent(name = "PriceUpdated", abi = "PriceUpdated(bytes32,uint256)")]
    pub struct PriceUpdatedFilter {
        #[ethevent(indexed)]
        pub symbol_hash: [u8; 32],
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CompishPriceOracleEvents {
        FailoverActivatedFilter(FailoverActivatedFilter),
        FailoverDeactivatedFilter(FailoverDeactivatedFilter),
        OwnershipTransferRequestedFilter(OwnershipTransferRequestedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PriceGuardedFilter(PriceGuardedFilter),
        PriceUpdatedFilter(PriceUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for CompishPriceOracleEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = FailoverActivatedFilter::decode_log(log) {
                return Ok(CompishPriceOracleEvents::FailoverActivatedFilter(decoded));
            }
            if let Ok(decoded) = FailoverDeactivatedFilter::decode_log(log) {
                return Ok(CompishPriceOracleEvents::FailoverDeactivatedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferRequestedFilter::decode_log(log) {
                return Ok(
                    CompishPriceOracleEvents::OwnershipTransferRequestedFilter(decoded),
                );
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(CompishPriceOracleEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PriceGuardedFilter::decode_log(log) {
                return Ok(CompishPriceOracleEvents::PriceGuardedFilter(decoded));
            }
            if let Ok(decoded) = PriceUpdatedFilter::decode_log(log) {
                return Ok(CompishPriceOracleEvents::PriceUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CompishPriceOracleEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FailoverActivatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailoverDeactivatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferRequestedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PriceGuardedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PriceUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<FailoverActivatedFilter> for CompishPriceOracleEvents {
        fn from(value: FailoverActivatedFilter) -> Self {
            Self::FailoverActivatedFilter(value)
        }
    }
    impl ::core::convert::From<FailoverDeactivatedFilter> for CompishPriceOracleEvents {
        fn from(value: FailoverDeactivatedFilter) -> Self {
            Self::FailoverDeactivatedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferRequestedFilter>
    for CompishPriceOracleEvents {
        fn from(value: OwnershipTransferRequestedFilter) -> Self {
            Self::OwnershipTransferRequestedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for CompishPriceOracleEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PriceGuardedFilter> for CompishPriceOracleEvents {
        fn from(value: PriceGuardedFilter) -> Self {
            Self::PriceGuardedFilter(value)
        }
    }
    impl ::core::convert::From<PriceUpdatedFilter> for CompishPriceOracleEvents {
        fn from(value: PriceUpdatedFilter) -> Self {
            Self::PriceUpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `ETH_BASE_UNIT` function with signature `ETH_BASE_UNIT()` and selector `0x152810fb`
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
    #[ethcall(name = "ETH_BASE_UNIT", abi = "ETH_BASE_UNIT()")]
    pub struct EthBaseUnitCall;
    ///Container type for all input parameters for the `EXP_SCALE` function with signature `EXP_SCALE()` and selector `0xbbba205d`
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
    #[ethcall(name = "EXP_SCALE", abi = "EXP_SCALE()")]
    pub struct ExpScaleCall;
    ///Container type for all input parameters for the `MAX_INTEGER` function with signature `MAX_INTEGER()` and selector `0x78eadb1a`
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
    #[ethcall(name = "MAX_INTEGER", abi = "MAX_INTEGER()")]
    pub struct MaxIntegerCall;
    ///Container type for all input parameters for the `MAX_TOKENS` function with signature `MAX_TOKENS()` and selector `0xf47c84c5`
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
    #[ethcall(name = "MAX_TOKENS", abi = "MAX_TOKENS()")]
    pub struct MaxTokensCall;
    ///Container type for all input parameters for the `acceptOwnership` function with signature `acceptOwnership()` and selector `0x79ba5097`
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
    #[ethcall(name = "acceptOwnership", abi = "acceptOwnership()")]
    pub struct AcceptOwnershipCall;
    ///Container type for all input parameters for the `activateFailover` function with signature `activateFailover(bytes32)` and selector `0x5f396923`
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
    #[ethcall(name = "activateFailover", abi = "activateFailover(bytes32)")]
    pub struct ActivateFailoverCall {
        pub symbol_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `anchorPeriod` function with signature `anchorPeriod()` and selector `0xe9206d78`
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
    #[ethcall(name = "anchorPeriod", abi = "anchorPeriod()")]
    pub struct AnchorPeriodCall;
    ///Container type for all input parameters for the `deactivateFailover` function with signature `deactivateFailover(bytes32)` and selector `0x8185b81c`
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
    #[ethcall(name = "deactivateFailover", abi = "deactivateFailover(bytes32)")]
    pub struct DeactivateFailoverCall {
        pub symbol_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `getTokenConfig` function with signature `getTokenConfig(uint256)` and selector `0x8a003888`
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
    #[ethcall(name = "getTokenConfig", abi = "getTokenConfig(uint256)")]
    pub struct GetTokenConfigCall {
        pub i: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getTokenConfigByCToken` function with signature `getTokenConfigByCToken(address)` and selector `0x9f599631`
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
    #[ethcall(name = "getTokenConfigByCToken", abi = "getTokenConfigByCToken(address)")]
    pub struct GetTokenConfigByCTokenCall {
        pub c_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getTokenConfigByReporter` function with signature `getTokenConfigByReporter(address)` and selector `0x29feb7bc`
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
        name = "getTokenConfigByReporter",
        abi = "getTokenConfigByReporter(address)"
    )]
    pub struct GetTokenConfigByReporterCall {
        pub reporter: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getTokenConfigBySymbol` function with signature `getTokenConfigBySymbol(string)` and selector `0x276c2cba`
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
    #[ethcall(name = "getTokenConfigBySymbol", abi = "getTokenConfigBySymbol(string)")]
    pub struct GetTokenConfigBySymbolCall {
        pub symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the `getTokenConfigBySymbolHash` function with signature `getTokenConfigBySymbolHash(bytes32)` and selector `0x1a125204`
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
        name = "getTokenConfigBySymbolHash",
        abi = "getTokenConfigBySymbolHash(bytes32)"
    )]
    pub struct GetTokenConfigBySymbolHashCall {
        pub symbol_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `getTokenConfigByUnderlying` function with signature `getTokenConfigByUnderlying(address)` and selector `0x4da21942`
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
        name = "getTokenConfigByUnderlying",
        abi = "getTokenConfigByUnderlying(address)"
    )]
    pub struct GetTokenConfigByUnderlyingCall {
        pub underlying: ::ethers::core::types::Address,
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
        Hash
    )]
    #[ethcall(name = "getUnderlyingPrice", abi = "getUnderlyingPrice(address)")]
    pub struct GetUnderlyingPriceCall {
        pub c_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `lowerBoundAnchorRatio` function with signature `lowerBoundAnchorRatio()` and selector `0x92b84357`
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
    #[ethcall(name = "lowerBoundAnchorRatio", abi = "lowerBoundAnchorRatio()")]
    pub struct LowerBoundAnchorRatioCall;
    ///Container type for all input parameters for the `numTokens` function with signature `numTokens()` and selector `0x8e499bcf`
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
    #[ethcall(name = "numTokens", abi = "numTokens()")]
    pub struct NumTokensCall;
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
    ///Container type for all input parameters for the `pokeFailedOverPrice` function with signature `pokeFailedOverPrice(bytes32)` and selector `0x68f9a97f`
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
    #[ethcall(name = "pokeFailedOverPrice", abi = "pokeFailedOverPrice(bytes32)")]
    pub struct PokeFailedOverPriceCall {
        pub symbol_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `price` function with signature `price(string)` and selector `0xfe2c6198`
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
    #[ethcall(name = "price", abi = "price(string)")]
    pub struct PriceCall {
        pub symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the `prices` function with signature `prices(bytes32)` and selector `0x60846bc6`
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
    #[ethcall(name = "prices", abi = "prices(bytes32)")]
    pub struct PricesCall(pub [u8; 32]);
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
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `upperBoundAnchorRatio` function with signature `upperBoundAnchorRatio()` and selector `0x24105209`
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
    #[ethcall(name = "upperBoundAnchorRatio", abi = "upperBoundAnchorRatio()")]
    pub struct UpperBoundAnchorRatioCall;
    ///Container type for all input parameters for the `validate` function with signature `validate(uint256,int256,uint256,int256)` and selector `0xbeed9b51`
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
    #[ethcall(name = "validate", abi = "validate(uint256,int256,uint256,int256)")]
    pub struct ValidateCall {
        pub p0: ::ethers::core::types::U256,
        pub p1: ::ethers::core::types::I256,
        pub p2: ::ethers::core::types::U256,
        pub current_answer: ::ethers::core::types::I256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CompishPriceOracleCalls {
        EthBaseUnit(EthBaseUnitCall),
        ExpScale(ExpScaleCall),
        MaxInteger(MaxIntegerCall),
        MaxTokens(MaxTokensCall),
        AcceptOwnership(AcceptOwnershipCall),
        ActivateFailover(ActivateFailoverCall),
        AnchorPeriod(AnchorPeriodCall),
        DeactivateFailover(DeactivateFailoverCall),
        GetTokenConfig(GetTokenConfigCall),
        GetTokenConfigByCToken(GetTokenConfigByCTokenCall),
        GetTokenConfigByReporter(GetTokenConfigByReporterCall),
        GetTokenConfigBySymbol(GetTokenConfigBySymbolCall),
        GetTokenConfigBySymbolHash(GetTokenConfigBySymbolHashCall),
        GetTokenConfigByUnderlying(GetTokenConfigByUnderlyingCall),
        GetUnderlyingPrice(GetUnderlyingPriceCall),
        LowerBoundAnchorRatio(LowerBoundAnchorRatioCall),
        NumTokens(NumTokensCall),
        Owner(OwnerCall),
        PokeFailedOverPrice(PokeFailedOverPriceCall),
        Price(PriceCall),
        Prices(PricesCall),
        TransferOwnership(TransferOwnershipCall),
        UpperBoundAnchorRatio(UpperBoundAnchorRatioCall),
        Validate(ValidateCall),
    }
    impl ::ethers::core::abi::AbiDecode for CompishPriceOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <EthBaseUnitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EthBaseUnit(decoded));
            }
            if let Ok(decoded) = <ExpScaleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpScale(decoded));
            }
            if let Ok(decoded) = <MaxIntegerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxInteger(decoded));
            }
            if let Ok(decoded) = <MaxTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxTokens(decoded));
            }
            if let Ok(decoded) = <AcceptOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AcceptOwnership(decoded));
            }
            if let Ok(decoded) = <ActivateFailoverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ActivateFailover(decoded));
            }
            if let Ok(decoded) = <AnchorPeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AnchorPeriod(decoded));
            }
            if let Ok(decoded) = <DeactivateFailoverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeactivateFailover(decoded));
            }
            if let Ok(decoded) = <GetTokenConfigCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTokenConfig(decoded));
            }
            if let Ok(decoded) = <GetTokenConfigByCTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTokenConfigByCToken(decoded));
            }
            if let Ok(decoded) = <GetTokenConfigByReporterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTokenConfigByReporter(decoded));
            }
            if let Ok(decoded) = <GetTokenConfigBySymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTokenConfigBySymbol(decoded));
            }
            if let Ok(decoded) = <GetTokenConfigBySymbolHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTokenConfigBySymbolHash(decoded));
            }
            if let Ok(decoded) = <GetTokenConfigByUnderlyingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTokenConfigByUnderlying(decoded));
            }
            if let Ok(decoded) = <GetUnderlyingPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetUnderlyingPrice(decoded));
            }
            if let Ok(decoded) = <LowerBoundAnchorRatioCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LowerBoundAnchorRatio(decoded));
            }
            if let Ok(decoded) = <NumTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NumTokens(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PokeFailedOverPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PokeFailedOverPrice(decoded));
            }
            if let Ok(decoded) = <PriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Price(decoded));
            }
            if let Ok(decoded) = <PricesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Prices(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UpperBoundAnchorRatioCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpperBoundAnchorRatio(decoded));
            }
            if let Ok(decoded) = <ValidateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Validate(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CompishPriceOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::EthBaseUnit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpScale(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxInteger(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AcceptOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ActivateFailover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AnchorPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeactivateFailover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokenConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokenConfigByCToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokenConfigByReporter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokenConfigBySymbol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokenConfigBySymbolHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokenConfigByUnderlying(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUnderlyingPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LowerBoundAnchorRatio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NumTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PokeFailedOverPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Price(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Prices(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpperBoundAnchorRatio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Validate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CompishPriceOracleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EthBaseUnit(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpScale(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxInteger(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::AcceptOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::ActivateFailover(element) => ::core::fmt::Display::fmt(element, f),
                Self::AnchorPeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeactivateFailover(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTokenConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTokenConfigByCToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTokenConfigByReporter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTokenConfigBySymbol(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTokenConfigBySymbolHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTokenConfigByUnderlying(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetUnderlyingPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LowerBoundAnchorRatio(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NumTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PokeFailedOverPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Price(element) => ::core::fmt::Display::fmt(element, f),
                Self::Prices(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpperBoundAnchorRatio(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Validate(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EthBaseUnitCall> for CompishPriceOracleCalls {
        fn from(value: EthBaseUnitCall) -> Self {
            Self::EthBaseUnit(value)
        }
    }
    impl ::core::convert::From<ExpScaleCall> for CompishPriceOracleCalls {
        fn from(value: ExpScaleCall) -> Self {
            Self::ExpScale(value)
        }
    }
    impl ::core::convert::From<MaxIntegerCall> for CompishPriceOracleCalls {
        fn from(value: MaxIntegerCall) -> Self {
            Self::MaxInteger(value)
        }
    }
    impl ::core::convert::From<MaxTokensCall> for CompishPriceOracleCalls {
        fn from(value: MaxTokensCall) -> Self {
            Self::MaxTokens(value)
        }
    }
    impl ::core::convert::From<AcceptOwnershipCall> for CompishPriceOracleCalls {
        fn from(value: AcceptOwnershipCall) -> Self {
            Self::AcceptOwnership(value)
        }
    }
    impl ::core::convert::From<ActivateFailoverCall> for CompishPriceOracleCalls {
        fn from(value: ActivateFailoverCall) -> Self {
            Self::ActivateFailover(value)
        }
    }
    impl ::core::convert::From<AnchorPeriodCall> for CompishPriceOracleCalls {
        fn from(value: AnchorPeriodCall) -> Self {
            Self::AnchorPeriod(value)
        }
    }
    impl ::core::convert::From<DeactivateFailoverCall> for CompishPriceOracleCalls {
        fn from(value: DeactivateFailoverCall) -> Self {
            Self::DeactivateFailover(value)
        }
    }
    impl ::core::convert::From<GetTokenConfigCall> for CompishPriceOracleCalls {
        fn from(value: GetTokenConfigCall) -> Self {
            Self::GetTokenConfig(value)
        }
    }
    impl ::core::convert::From<GetTokenConfigByCTokenCall> for CompishPriceOracleCalls {
        fn from(value: GetTokenConfigByCTokenCall) -> Self {
            Self::GetTokenConfigByCToken(value)
        }
    }
    impl ::core::convert::From<GetTokenConfigByReporterCall>
    for CompishPriceOracleCalls {
        fn from(value: GetTokenConfigByReporterCall) -> Self {
            Self::GetTokenConfigByReporter(value)
        }
    }
    impl ::core::convert::From<GetTokenConfigBySymbolCall> for CompishPriceOracleCalls {
        fn from(value: GetTokenConfigBySymbolCall) -> Self {
            Self::GetTokenConfigBySymbol(value)
        }
    }
    impl ::core::convert::From<GetTokenConfigBySymbolHashCall>
    for CompishPriceOracleCalls {
        fn from(value: GetTokenConfigBySymbolHashCall) -> Self {
            Self::GetTokenConfigBySymbolHash(value)
        }
    }
    impl ::core::convert::From<GetTokenConfigByUnderlyingCall>
    for CompishPriceOracleCalls {
        fn from(value: GetTokenConfigByUnderlyingCall) -> Self {
            Self::GetTokenConfigByUnderlying(value)
        }
    }
    impl ::core::convert::From<GetUnderlyingPriceCall> for CompishPriceOracleCalls {
        fn from(value: GetUnderlyingPriceCall) -> Self {
            Self::GetUnderlyingPrice(value)
        }
    }
    impl ::core::convert::From<LowerBoundAnchorRatioCall> for CompishPriceOracleCalls {
        fn from(value: LowerBoundAnchorRatioCall) -> Self {
            Self::LowerBoundAnchorRatio(value)
        }
    }
    impl ::core::convert::From<NumTokensCall> for CompishPriceOracleCalls {
        fn from(value: NumTokensCall) -> Self {
            Self::NumTokens(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for CompishPriceOracleCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PokeFailedOverPriceCall> for CompishPriceOracleCalls {
        fn from(value: PokeFailedOverPriceCall) -> Self {
            Self::PokeFailedOverPrice(value)
        }
    }
    impl ::core::convert::From<PriceCall> for CompishPriceOracleCalls {
        fn from(value: PriceCall) -> Self {
            Self::Price(value)
        }
    }
    impl ::core::convert::From<PricesCall> for CompishPriceOracleCalls {
        fn from(value: PricesCall) -> Self {
            Self::Prices(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for CompishPriceOracleCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpperBoundAnchorRatioCall> for CompishPriceOracleCalls {
        fn from(value: UpperBoundAnchorRatioCall) -> Self {
            Self::UpperBoundAnchorRatio(value)
        }
    }
    impl ::core::convert::From<ValidateCall> for CompishPriceOracleCalls {
        fn from(value: ValidateCall) -> Self {
            Self::Validate(value)
        }
    }
    ///Container type for all return fields from the `ETH_BASE_UNIT` function with signature `ETH_BASE_UNIT()` and selector `0x152810fb`
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
    pub struct EthBaseUnitReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `EXP_SCALE` function with signature `EXP_SCALE()` and selector `0xbbba205d`
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
    pub struct ExpScaleReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_INTEGER` function with signature `MAX_INTEGER()` and selector `0x78eadb1a`
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
    pub struct MaxIntegerReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_TOKENS` function with signature `MAX_TOKENS()` and selector `0xf47c84c5`
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
    pub struct MaxTokensReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `anchorPeriod` function with signature `anchorPeriod()` and selector `0xe9206d78`
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
    pub struct AnchorPeriodReturn(pub u32);
    ///Container type for all return fields from the `getTokenConfig` function with signature `getTokenConfig(uint256)` and selector `0x8a003888`
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
    pub struct GetTokenConfigReturn(pub TokenConfig);
    ///Container type for all return fields from the `getTokenConfigByCToken` function with signature `getTokenConfigByCToken(address)` and selector `0x9f599631`
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
    pub struct GetTokenConfigByCTokenReturn(pub TokenConfig);
    ///Container type for all return fields from the `getTokenConfigByReporter` function with signature `getTokenConfigByReporter(address)` and selector `0x29feb7bc`
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
    pub struct GetTokenConfigByReporterReturn(pub TokenConfig);
    ///Container type for all return fields from the `getTokenConfigBySymbol` function with signature `getTokenConfigBySymbol(string)` and selector `0x276c2cba`
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
    pub struct GetTokenConfigBySymbolReturn(pub TokenConfig);
    ///Container type for all return fields from the `getTokenConfigBySymbolHash` function with signature `getTokenConfigBySymbolHash(bytes32)` and selector `0x1a125204`
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
    pub struct GetTokenConfigBySymbolHashReturn(pub TokenConfig);
    ///Container type for all return fields from the `getTokenConfigByUnderlying` function with signature `getTokenConfigByUnderlying(address)` and selector `0x4da21942`
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
    pub struct GetTokenConfigByUnderlyingReturn(pub TokenConfig);
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
    ///Container type for all return fields from the `lowerBoundAnchorRatio` function with signature `lowerBoundAnchorRatio()` and selector `0x92b84357`
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
    pub struct LowerBoundAnchorRatioReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `numTokens` function with signature `numTokens()` and selector `0x8e499bcf`
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
    pub struct NumTokensReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `price` function with signature `price(string)` and selector `0xfe2c6198`
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
    pub struct PriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `prices` function with signature `prices(bytes32)` and selector `0x60846bc6`
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
    pub struct PricesReturn {
        pub price: ::ethers::core::types::U256,
        pub failover_active: bool,
    }
    ///Container type for all return fields from the `upperBoundAnchorRatio` function with signature `upperBoundAnchorRatio()` and selector `0x24105209`
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
    pub struct UpperBoundAnchorRatioReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `validate` function with signature `validate(uint256,int256,uint256,int256)` and selector `0xbeed9b51`
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
    pub struct ValidateReturn {
        pub valid: bool,
    }
    ///`TokenConfig(address,address,bytes32,uint256,uint8,uint256,address,address,uint256,bool)`
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
    pub struct TokenConfig {
        pub c_token: ::ethers::core::types::Address,
        pub underlying: ::ethers::core::types::Address,
        pub symbol_hash: [u8; 32],
        pub base_unit: ::ethers::core::types::U256,
        pub price_source: u8,
        pub fixed_price: ::ethers::core::types::U256,
        pub uniswap_market: ::ethers::core::types::Address,
        pub reporter: ::ethers::core::types::Address,
        pub reporter_multiplier: ::ethers::core::types::U256,
        pub is_uniswap_reversed: bool,
    }
}
