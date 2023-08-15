pub use comptroller::*;
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
pub mod comptroller {
    const _: () = {
        ::core::include_bytes!("/home/joss/dev/liquidator_v1/abi/comptroller.json");
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("_backPopulate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_backPopulate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_oracle"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract PriceOracle"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pauseGuardian"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_borrowCapGuardian",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_closeFactorMantissa",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_compRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_maxAssets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_compAddress"),
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
                    ::std::borrow::ToOwned::to_owned("_become"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_become"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("unitroller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Unitroller"),
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
                    ::std::borrow::ToOwned::to_owned("_borrowGuardianPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_borrowGuardianPaused",
                            ),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_grantComp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_grantComp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("_mintGuardianPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_mintGuardianPaused",
                            ),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_setBorrowCapGuardian"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_setBorrowCapGuardian",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newBorrowCapGuardian",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("_setBorrowPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_setBorrowPaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract CToken"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("state"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                (
                    ::std::borrow::ToOwned::to_owned("_setCloseFactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_setCloseFactor"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newCloseFactorMantissa",
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_setCollateralFactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_setCollateralFactor",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract CToken"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newCollateralFactorMantissa",
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_setCompSpeeds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_setCompSpeeds"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract CToken[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("supplySpeeds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrowSpeeds"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_setContributorCompSpeed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_setContributorCompSpeed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("contributor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("compSpeed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("_setLiquidationIncentive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_setLiquidationIncentive",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newLiquidationIncentiveMantissa",
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_setMarketBorrowCaps"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_setMarketBorrowCaps",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract CToken[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newBorrowCaps"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_setMintPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_setMintPaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract CToken"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("state"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                (
                    ::std::borrow::ToOwned::to_owned("_setPauseGuardian"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_setPauseGuardian"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPauseGuardian"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_setPriceOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_setPriceOracle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOracle"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract PriceOracle"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_setSeizePaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_setSeizePaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("state"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                (
                    ::std::borrow::ToOwned::to_owned("_setTransferPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_setTransferPaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("state"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                (
                    ::std::borrow::ToOwned::to_owned("_supportMarket"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_supportMarket"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("accountAssets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("accountAssets"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract CToken"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allMarkets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allMarkets"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract CToken"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("borrowAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("borrowAllowed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrowAmount"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("borrowCapGuardian"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("borrowCapGuardian"),
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
                    ::std::borrow::ToOwned::to_owned("borrowCaps"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("borrowCaps"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("borrowGuardianPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "borrowGuardianPaused",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("borrowVerify"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("borrowVerify"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrowAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("checkMembership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkMembership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("claimComp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimComp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("holder"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract CToken[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimComp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("holders"),
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
                                    name: ::std::borrow::ToOwned::to_owned("cTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract CToken[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrowers"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("suppliers"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimComp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("holder"),
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
                    ::std::borrow::ToOwned::to_owned("closeFactorMantissa"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "closeFactorMantissa",
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
                    ::std::borrow::ToOwned::to_owned("compAccrued"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("compAccrued"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("compBorrowSpeeds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("compBorrowSpeeds"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("compBorrowState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("compBorrowState"),
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
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        224usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint224"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("block"),
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
                    ::std::borrow::ToOwned::to_owned("compBorrowerIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("compBorrowerIndex"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("compContributorSpeeds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "compContributorSpeeds",
                            ),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("compInitialIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("compInitialIndex"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        224usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint224"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("compRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("compRate"),
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
                    ::std::borrow::ToOwned::to_owned("compReceivable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("compReceivable"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("compSpeeds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("compSpeeds"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("compSupplierIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("compSupplierIndex"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("compSupplySpeeds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("compSupplySpeeds"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("compSupplyState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("compSupplyState"),
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
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        224usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint224"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("block"),
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
                    ::std::borrow::ToOwned::to_owned("comptrollerImplementation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "comptrollerImplementation",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("enterMarkets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("enterMarkets"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("exitMarket"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exitMarket"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cTokenAddress"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("fixBadAccruals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fixBadAccruals"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("affectedUsers"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAccountLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getAccountLiquidity",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("getAllMarkets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAllMarkets"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract CToken[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAssetsIn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAssetsIn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract CToken[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBlockNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBlockNumber"),
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
                    ::std::borrow::ToOwned::to_owned("getCompAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getCompAddress"),
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
                    ::std::borrow::ToOwned::to_owned("getHypotheticalAccountLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getHypotheticalAccountLiquidity",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cTokenModify"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("redeemTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrowAmount"),
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
                    ::std::borrow::ToOwned::to_owned("isComptroller"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isComptroller"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isDeprecated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isDeprecated"),
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
                    ::std::borrow::ToOwned::to_owned("lastContributorBlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastContributorBlock",
                            ),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liquidateBorrowAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "liquidateBorrowAllowed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cTokenBorrowed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cTokenCollateral"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("repayAmount"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liquidateBorrowVerify"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "liquidateBorrowVerify",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cTokenBorrowed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cTokenCollateral"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("actualRepayAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("seizeTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("liquidateCalculateSeizeTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "liquidateCalculateSeizeTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cTokenBorrowed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cTokenCollateral"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("actualRepayAmount"),
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
                    ::std::borrow::ToOwned::to_owned("liquidationIncentiveMantissa"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "liquidationIncentiveMantissa",
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
                    ::std::borrow::ToOwned::to_owned("markets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("markets"),
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
                                    name: ::std::borrow::ToOwned::to_owned("isListed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "collateralFactorMantissa",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("isComped"),
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
                    ::std::borrow::ToOwned::to_owned("maxAssets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxAssets"),
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
                    ::std::borrow::ToOwned::to_owned("mintAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mintAllowed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mintAmount"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mintGuardianPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mintGuardianPaused"),
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
                    ::std::borrow::ToOwned::to_owned("mintVerify"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mintVerify"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("actualMintAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mintTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("oracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("oracle"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract PriceOracle"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pauseGuardian"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pauseGuardian"),
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
                    ::std::borrow::ToOwned::to_owned("pendingAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pendingAdmin"),
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
                    ::std::borrow::ToOwned::to_owned("pendingComptrollerImplementation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "pendingComptrollerImplementation",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("proposal65FixExecuted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "proposal65FixExecuted",
                            ),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("redeemAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("redeemAllowed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("redeemer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("redeemTokens"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("redeemVerify"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("redeemVerify"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("redeemer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("redeemAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("redeemTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("repayBorrowAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("repayBorrowAllowed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("repayAmount"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("repayBorrowVerify"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("repayBorrowVerify"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("actualRepayAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrowerIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("seizeAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("seizeAllowed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cTokenCollateral"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cTokenBorrowed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("seizeTokens"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("seizeGuardianPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "seizeGuardianPaused",
                            ),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("seizeVerify"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("seizeVerify"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cTokenCollateral"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cTokenBorrowed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("seizeTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("transferAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferAllowed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("src"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dst"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("transferTokens"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferGuardianPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "transferGuardianPaused",
                            ),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferVerify"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferVerify"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("src"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dst"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("transferTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("updateContributorRewards"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateContributorRewards",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("contributor"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ActionPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ActionPaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("action"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pauseState"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ActionPaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("action"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pauseState"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CompAccruedAdjusted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CompAccruedAdjusted",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldCompAccrued"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newCompAccrued"),
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
                    ::std::borrow::ToOwned::to_owned("CompBorrowSpeedUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CompBorrowSpeedUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newSpeed"),
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
                    ::std::borrow::ToOwned::to_owned("CompGranted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("CompGranted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("CompReceivableUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CompReceivableUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldCompReceivable"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newCompReceivable"),
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
                    ::std::borrow::ToOwned::to_owned("CompSupplySpeedUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CompSupplySpeedUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newSpeed"),
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
                    ::std::borrow::ToOwned::to_owned("ContributorCompSpeedUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ContributorCompSpeedUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("contributor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newSpeed"),
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
                    ::std::borrow::ToOwned::to_owned("DistributedBorrowerComp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DistributedBorrowerComp",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("borrower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("compDelta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("compBorrowIndex"),
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
                    ::std::borrow::ToOwned::to_owned("DistributedSupplierComp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DistributedSupplierComp",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("supplier"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("compDelta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("compSupplyIndex"),
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
                    ::std::borrow::ToOwned::to_owned("MarketEntered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MarketEntered"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MarketExited"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MarketExited"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MarketListed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MarketListed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewBorrowCap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NewBorrowCap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newBorrowCap"),
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
                    ::std::borrow::ToOwned::to_owned("NewBorrowCapGuardian"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NewBorrowCapGuardian",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "oldBorrowCapGuardian",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newBorrowCapGuardian",
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
                    ::std::borrow::ToOwned::to_owned("NewCloseFactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NewCloseFactor"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "oldCloseFactorMantissa",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newCloseFactorMantissa",
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
                    ::std::borrow::ToOwned::to_owned("NewCollateralFactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NewCollateralFactor",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("cToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "oldCollateralFactorMantissa",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newCollateralFactorMantissa",
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
                    ::std::borrow::ToOwned::to_owned("NewLiquidationIncentive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NewLiquidationIncentive",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "oldLiquidationIncentiveMantissa",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newLiquidationIncentiveMantissa",
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
                    ::std::borrow::ToOwned::to_owned("NewPauseGuardian"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NewPauseGuardian"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldPauseGuardian"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPauseGuardian"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewPriceOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NewPriceOracle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldPriceOracle"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPriceOracle"),
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
    pub static COMPTROLLER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct Comptroller<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Comptroller<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Comptroller<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Comptroller<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Comptroller<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Comptroller))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Comptroller<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    COMPTROLLER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `_backPopulate` (0x0753a198) function
        pub fn back_populate(
            &self,
            oracle: ::ethers::core::types::Address,
            pause_guardian: ::ethers::core::types::Address,
            borrow_cap_guardian: ::ethers::core::types::Address,
            close_factor_mantissa: ::ethers::core::types::U256,
            comp_rate: ::ethers::core::types::U256,
            max_assets: ::ethers::core::types::U256,
            comp_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [7, 83, 161, 152],
                    (
                        oracle,
                        pause_guardian,
                        borrow_cap_guardian,
                        close_factor_mantissa,
                        comp_rate,
                        max_assets,
                        comp_address,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_become` (0x1d504dc6) function
        pub fn become_(
            &self,
            unitroller: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 80, 77, 198], unitroller)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_borrowGuardianPaused` (0xe6653f3d) function
        pub fn _borrow_guardian_paused(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([230, 101, 63, 61], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_grantComp` (0x27efe3cb) function
        pub fn grant_comp(
            &self,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([39, 239, 227, 203], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_mintGuardianPaused` (0x3c94786f) function
        pub fn _mint_guardian_paused(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([60, 148, 120, 111], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_setBorrowCapGuardian` (0x391957d7) function
        pub fn set_borrow_cap_guardian(
            &self,
            new_borrow_cap_guardian: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 25, 87, 215], new_borrow_cap_guardian)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_setBorrowPaused` (0x18c882a5) function
        pub fn set_borrow_paused(
            &self,
            c_token: ::ethers::core::types::Address,
            state: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([24, 200, 130, 165], (c_token, state))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_setCloseFactor` (0x317b0b77) function
        pub fn set_close_factor(
            &self,
            new_close_factor_mantissa: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([49, 123, 11, 119], new_close_factor_mantissa)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_setCollateralFactor` (0xe4028eee) function
        pub fn set_collateral_factor(
            &self,
            c_token: ::ethers::core::types::Address,
            new_collateral_factor_mantissa: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [228, 2, 142, 238],
                    (c_token, new_collateral_factor_mantissa),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_setCompSpeeds` (0xa8b43948) function
        pub fn set_comp_speeds(
            &self,
            c_tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            supply_speeds: ::std::vec::Vec<::ethers::core::types::U256>,
            borrow_speeds: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [168, 180, 57, 72],
                    (c_tokens, supply_speeds, borrow_speeds),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_setContributorCompSpeed` (0x598ee1cb) function
        pub fn set_contributor_comp_speed(
            &self,
            contributor: ::ethers::core::types::Address,
            comp_speed: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 142, 225, 203], (contributor, comp_speed))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_setLiquidationIncentive` (0x4fd42e17) function
        pub fn set_liquidation_incentive(
            &self,
            new_liquidation_incentive_mantissa: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([79, 212, 46, 23], new_liquidation_incentive_mantissa)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_setMarketBorrowCaps` (0x607ef6c1) function
        pub fn set_market_borrow_caps(
            &self,
            c_tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            new_borrow_caps: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([96, 126, 246, 193], (c_tokens, new_borrow_caps))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_setMintPaused` (0x3bcf7ec1) function
        pub fn set_mint_paused(
            &self,
            c_token: ::ethers::core::types::Address,
            state: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([59, 207, 126, 193], (c_token, state))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_setPauseGuardian` (0x5f5af1aa) function
        pub fn set_pause_guardian(
            &self,
            new_pause_guardian: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([95, 90, 241, 170], new_pause_guardian)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_setPriceOracle` (0x55ee1fe1) function
        pub fn set_price_oracle(
            &self,
            new_oracle: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([85, 238, 31, 225], new_oracle)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_setSeizePaused` (0x2d70db78) function
        pub fn set_seize_paused(
            &self,
            state: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([45, 112, 219, 120], state)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_setTransferPaused` (0x8ebf6364) function
        pub fn set_transfer_paused(
            &self,
            state: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([142, 191, 99, 100], state)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_supportMarket` (0xa76b3fda) function
        pub fn support_market(
            &self,
            c_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([167, 107, 63, 218], c_token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `accountAssets` (0xdce15449) function
        pub fn account_assets(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([220, 225, 84, 73], (p0, p1))
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
        ///Calls the contract's `allMarkets` (0x52d84d1e) function
        pub fn all_markets(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([82, 216, 77, 30], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `borrowAllowed` (0xda3d454c) function
        pub fn borrow_allowed(
            &self,
            c_token: ::ethers::core::types::Address,
            borrower: ::ethers::core::types::Address,
            borrow_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([218, 61, 69, 76], (c_token, borrower, borrow_amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `borrowCapGuardian` (0x21af4569) function
        pub fn borrow_cap_guardian(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([33, 175, 69, 105], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `borrowCaps` (0x4a584432) function
        pub fn borrow_caps(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([74, 88, 68, 50], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `borrowGuardianPaused` (0x6d154ea5) function
        pub fn borrow_guardian_paused(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([109, 21, 78, 165], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `borrowVerify` (0x5c778605) function
        pub fn borrow_verify(
            &self,
            c_token: ::ethers::core::types::Address,
            borrower: ::ethers::core::types::Address,
            borrow_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 119, 134, 5], (c_token, borrower, borrow_amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkMembership` (0x929fe9a1) function
        pub fn check_membership(
            &self,
            account: ::ethers::core::types::Address,
            c_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([146, 159, 233, 161], (account, c_token))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimComp` (0x1c3db2e0) function
        pub fn claim_comp_with_c_tokens(
            &self,
            holder: ::ethers::core::types::Address,
            c_tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 61, 178, 224], (holder, c_tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimComp` (0x6810dfa6) function
        pub fn claim_comp_with_holders(
            &self,
            holders: ::std::vec::Vec<::ethers::core::types::Address>,
            c_tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            borrowers: bool,
            suppliers: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [104, 16, 223, 166],
                    (holders, c_tokens, borrowers, suppliers),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimComp` (0xe9af0292) function
        pub fn claim_comp(
            &self,
            holder: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 175, 2, 146], holder)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `closeFactorMantissa` (0xe8755446) function
        pub fn close_factor_mantissa(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([232, 117, 84, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `compAccrued` (0xcc7ebdc4) function
        pub fn comp_accrued(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([204, 126, 189, 196], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `compBorrowSpeeds` (0xf4a433c0) function
        pub fn comp_borrow_speeds(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([244, 164, 51, 192], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `compBorrowState` (0x8c57804e) function
        pub fn comp_borrow_state(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, u32),
        > {
            self.0
                .method_hash([140, 87, 128, 78], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `compBorrowerIndex` (0xca0af043) function
        pub fn comp_borrower_index(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([202, 10, 240, 67], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `compContributorSpeeds` (0x986ab838) function
        pub fn comp_contributor_speeds(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([152, 106, 184, 56], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `compInitialIndex` (0xa7f0e231) function
        pub fn comp_initial_index(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([167, 240, 226, 49], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `compRate` (0xaa900754) function
        pub fn comp_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([170, 144, 7, 84], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `compReceivable` (0x85b7beb8) function
        pub fn comp_receivable(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([133, 183, 190, 184], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `compSpeeds` (0x1d7b33d7) function
        pub fn comp_speeds(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([29, 123, 51, 215], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `compSupplierIndex` (0xb21be7fd) function
        pub fn comp_supplier_index(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([178, 27, 231, 253], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `compSupplySpeeds` (0x6aa875b5) function
        pub fn comp_supply_speeds(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([106, 168, 117, 181], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `compSupplyState` (0x6b79c38d) function
        pub fn comp_supply_state(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, u32),
        > {
            self.0
                .method_hash([107, 121, 195, 141], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `comptrollerImplementation` (0xbb82aa5e) function
        pub fn comptroller_implementation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([187, 130, 170, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enterMarkets` (0xc2998238) function
        pub fn enter_markets(
            &self,
            c_tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([194, 153, 130, 56], c_tokens)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exitMarket` (0xede4edd0) function
        pub fn exit_market(
            &self,
            c_token_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([237, 228, 237, 208], c_token_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fixBadAccruals` (0x16b95e8f) function
        pub fn fix_bad_accruals(
            &self,
            affected_users: ::std::vec::Vec<::ethers::core::types::Address>,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 185, 94, 143], (affected_users, amounts))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAccountLiquidity` (0x5ec88c79) function
        pub fn get_account_liquidity(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([94, 200, 140, 121], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllMarkets` (0xb0772d0b) function
        pub fn get_all_markets(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([176, 119, 45, 11], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAssetsIn` (0xabfceffc) function
        pub fn get_assets_in(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([171, 252, 239, 252], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBlockNumber` (0x42cbb15c) function
        pub fn get_block_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([66, 203, 177, 92], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCompAddress` (0x9d1b5a0a) function
        pub fn get_comp_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([157, 27, 90, 10], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getHypotheticalAccountLiquidity` (0x4e79238f) function
        pub fn get_hypothetical_account_liquidity(
            &self,
            account: ::ethers::core::types::Address,
            c_token_modify: ::ethers::core::types::Address,
            redeem_tokens: ::ethers::core::types::U256,
            borrow_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [78, 121, 35, 143],
                    (account, c_token_modify, redeem_tokens, borrow_amount),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isComptroller` (0x007e3dd2) function
        pub fn is_comptroller(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([0, 126, 61, 210], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isDeprecated` (0x94543c15) function
        pub fn is_deprecated(
            &self,
            c_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([148, 84, 60, 21], c_token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastContributorBlock` (0xbea6b8b8) function
        pub fn last_contributor_block(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([190, 166, 184, 184], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidateBorrowAllowed` (0x5fc7e71e) function
        pub fn liquidate_borrow_allowed(
            &self,
            c_token_borrowed: ::ethers::core::types::Address,
            c_token_collateral: ::ethers::core::types::Address,
            liquidator: ::ethers::core::types::Address,
            borrower: ::ethers::core::types::Address,
            repay_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [95, 199, 231, 30],
                    (
                        c_token_borrowed,
                        c_token_collateral,
                        liquidator,
                        borrower,
                        repay_amount,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidateBorrowVerify` (0x47ef3b3b) function
        pub fn liquidate_borrow_verify(
            &self,
            c_token_borrowed: ::ethers::core::types::Address,
            c_token_collateral: ::ethers::core::types::Address,
            liquidator: ::ethers::core::types::Address,
            borrower: ::ethers::core::types::Address,
            actual_repay_amount: ::ethers::core::types::U256,
            seize_tokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [71, 239, 59, 59],
                    (
                        c_token_borrowed,
                        c_token_collateral,
                        liquidator,
                        borrower,
                        actual_repay_amount,
                        seize_tokens,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidateCalculateSeizeTokens` (0xc488847b) function
        pub fn liquidate_calculate_seize_tokens(
            &self,
            c_token_borrowed: ::ethers::core::types::Address,
            c_token_collateral: ::ethers::core::types::Address,
            actual_repay_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [196, 136, 132, 123],
                    (c_token_borrowed, c_token_collateral, actual_repay_amount),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidationIncentiveMantissa` (0x4ada90af) function
        pub fn liquidation_incentive_mantissa(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([74, 218, 144, 175], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `markets` (0x8e8f294b) function
        pub fn markets(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::U256, bool),
        > {
            self.0
                .method_hash([142, 143, 41, 75], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxAssets` (0x94b2294b) function
        pub fn max_assets(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([148, 178, 41, 75], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintAllowed` (0x4ef4c3e1) function
        pub fn mint_allowed(
            &self,
            c_token: ::ethers::core::types::Address,
            minter: ::ethers::core::types::Address,
            mint_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([78, 244, 195, 225], (c_token, minter, mint_amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintGuardianPaused` (0x731f0c2b) function
        pub fn mint_guardian_paused(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([115, 31, 12, 43], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintVerify` (0x41c728b9) function
        pub fn mint_verify(
            &self,
            c_token: ::ethers::core::types::Address,
            minter: ::ethers::core::types::Address,
            actual_mint_amount: ::ethers::core::types::U256,
            mint_tokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [65, 199, 40, 185],
                    (c_token, minter, actual_mint_amount, mint_tokens),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `oracle` (0x7dc0d1d0) function
        pub fn oracle(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([125, 192, 209, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pauseGuardian` (0x24a3d622) function
        pub fn pause_guardian(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([36, 163, 214, 34], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingAdmin` (0x26782247) function
        pub fn pending_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([38, 120, 34, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingComptrollerImplementation` (0xdcfbc0c7) function
        pub fn pending_comptroller_implementation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([220, 251, 192, 199], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposal65FixExecuted` (0xf00a7a92) function
        pub fn proposal_65_fix_executed(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([240, 10, 122, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redeemAllowed` (0xeabe7d91) function
        pub fn redeem_allowed(
            &self,
            c_token: ::ethers::core::types::Address,
            redeemer: ::ethers::core::types::Address,
            redeem_tokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([234, 190, 125, 145], (c_token, redeemer, redeem_tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redeemVerify` (0x51dff989) function
        pub fn redeem_verify(
            &self,
            c_token: ::ethers::core::types::Address,
            redeemer: ::ethers::core::types::Address,
            redeem_amount: ::ethers::core::types::U256,
            redeem_tokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [81, 223, 249, 137],
                    (c_token, redeemer, redeem_amount, redeem_tokens),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `repayBorrowAllowed` (0x24008a62) function
        pub fn repay_borrow_allowed(
            &self,
            c_token: ::ethers::core::types::Address,
            payer: ::ethers::core::types::Address,
            borrower: ::ethers::core::types::Address,
            repay_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([36, 0, 138, 98], (c_token, payer, borrower, repay_amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `repayBorrowVerify` (0x1ededc91) function
        pub fn repay_borrow_verify(
            &self,
            c_token: ::ethers::core::types::Address,
            payer: ::ethers::core::types::Address,
            borrower: ::ethers::core::types::Address,
            actual_repay_amount: ::ethers::core::types::U256,
            borrower_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [30, 222, 220, 145],
                    (c_token, payer, borrower, actual_repay_amount, borrower_index),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `seizeAllowed` (0xd02f7351) function
        pub fn seize_allowed(
            &self,
            c_token_collateral: ::ethers::core::types::Address,
            c_token_borrowed: ::ethers::core::types::Address,
            liquidator: ::ethers::core::types::Address,
            borrower: ::ethers::core::types::Address,
            seize_tokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [208, 47, 115, 81],
                    (
                        c_token_collateral,
                        c_token_borrowed,
                        liquidator,
                        borrower,
                        seize_tokens,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `seizeGuardianPaused` (0xac0b0bb7) function
        pub fn seize_guardian_paused(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([172, 11, 11, 183], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `seizeVerify` (0x6d35bf91) function
        pub fn seize_verify(
            &self,
            c_token_collateral: ::ethers::core::types::Address,
            c_token_borrowed: ::ethers::core::types::Address,
            liquidator: ::ethers::core::types::Address,
            borrower: ::ethers::core::types::Address,
            seize_tokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [109, 53, 191, 145],
                    (
                        c_token_collateral,
                        c_token_borrowed,
                        liquidator,
                        borrower,
                        seize_tokens,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferAllowed` (0xbdcdc258) function
        pub fn transfer_allowed(
            &self,
            c_token: ::ethers::core::types::Address,
            src: ::ethers::core::types::Address,
            dst: ::ethers::core::types::Address,
            transfer_tokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([189, 205, 194, 88], (c_token, src, dst, transfer_tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferGuardianPaused` (0x87f76303) function
        pub fn transfer_guardian_paused(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([135, 247, 99, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferVerify` (0x6a56947e) function
        pub fn transfer_verify(
            &self,
            c_token: ::ethers::core::types::Address,
            src: ::ethers::core::types::Address,
            dst: ::ethers::core::types::Address,
            transfer_tokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([106, 86, 148, 126], (c_token, src, dst, transfer_tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateContributorRewards` (0x741b2525) function
        pub fn update_contributor_rewards(
            &self,
            contributor: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([116, 27, 37, 37], contributor)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ActionPaused` event
        pub fn action_paused_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ActionPaused1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ActionPaused` event
        pub fn action_paused_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ActionPaused2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `CompAccruedAdjusted` event
        pub fn comp_accrued_adjusted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CompAccruedAdjustedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `CompBorrowSpeedUpdated` event
        pub fn comp_borrow_speed_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CompBorrowSpeedUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `CompGranted` event
        pub fn comp_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CompGrantedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `CompReceivableUpdated` event
        pub fn comp_receivable_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CompReceivableUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `CompSupplySpeedUpdated` event
        pub fn comp_supply_speed_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CompSupplySpeedUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ContributorCompSpeedUpdated` event
        pub fn contributor_comp_speed_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ContributorCompSpeedUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DistributedBorrowerComp` event
        pub fn distributed_borrower_comp_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DistributedBorrowerCompFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DistributedSupplierComp` event
        pub fn distributed_supplier_comp_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DistributedSupplierCompFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Failure` event
        pub fn failure_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FailureFilter> {
            self.0.event()
        }
        ///Gets the contract's `MarketEntered` event
        pub fn market_entered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MarketEnteredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MarketExited` event
        pub fn market_exited_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MarketExitedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MarketListed` event
        pub fn market_listed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MarketListedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewBorrowCap` event
        pub fn new_borrow_cap_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewBorrowCapFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewBorrowCapGuardian` event
        pub fn new_borrow_cap_guardian_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewBorrowCapGuardianFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewCloseFactor` event
        pub fn new_close_factor_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewCloseFactorFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewCollateralFactor` event
        pub fn new_collateral_factor_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewCollateralFactorFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewLiquidationIncentive` event
        pub fn new_liquidation_incentive_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewLiquidationIncentiveFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewPauseGuardian` event
        pub fn new_pause_guardian_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewPauseGuardianFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewPriceOracle` event
        pub fn new_price_oracle_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewPriceOracleFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ComptrollerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Comptroller<M> {
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
    #[ethevent(name = "ActionPaused", abi = "ActionPaused(string,bool)")]
    pub struct ActionPaused1Filter {
        pub action: ::std::string::String,
        pub pause_state: bool,
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
    #[ethevent(name = "ActionPaused", abi = "ActionPaused(address,string,bool)")]
    pub struct ActionPaused2Filter {
        pub c_token: ::ethers::core::types::Address,
        pub action: ::std::string::String,
        pub pause_state: bool,
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
        name = "CompAccruedAdjusted",
        abi = "CompAccruedAdjusted(address,uint256,uint256)"
    )]
    pub struct CompAccruedAdjustedFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        pub old_comp_accrued: ::ethers::core::types::U256,
        pub new_comp_accrued: ::ethers::core::types::U256,
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
        name = "CompBorrowSpeedUpdated",
        abi = "CompBorrowSpeedUpdated(address,uint256)"
    )]
    pub struct CompBorrowSpeedUpdatedFilter {
        #[ethevent(indexed)]
        pub c_token: ::ethers::core::types::Address,
        pub new_speed: ::ethers::core::types::U256,
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
    #[ethevent(name = "CompGranted", abi = "CompGranted(address,uint256)")]
    pub struct CompGrantedFilter {
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
        name = "CompReceivableUpdated",
        abi = "CompReceivableUpdated(address,uint256,uint256)"
    )]
    pub struct CompReceivableUpdatedFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        pub old_comp_receivable: ::ethers::core::types::U256,
        pub new_comp_receivable: ::ethers::core::types::U256,
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
        name = "CompSupplySpeedUpdated",
        abi = "CompSupplySpeedUpdated(address,uint256)"
    )]
    pub struct CompSupplySpeedUpdatedFilter {
        #[ethevent(indexed)]
        pub c_token: ::ethers::core::types::Address,
        pub new_speed: ::ethers::core::types::U256,
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
        name = "ContributorCompSpeedUpdated",
        abi = "ContributorCompSpeedUpdated(address,uint256)"
    )]
    pub struct ContributorCompSpeedUpdatedFilter {
        #[ethevent(indexed)]
        pub contributor: ::ethers::core::types::Address,
        pub new_speed: ::ethers::core::types::U256,
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
        name = "DistributedBorrowerComp",
        abi = "DistributedBorrowerComp(address,address,uint256,uint256)"
    )]
    pub struct DistributedBorrowerCompFilter {
        #[ethevent(indexed)]
        pub c_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub borrower: ::ethers::core::types::Address,
        pub comp_delta: ::ethers::core::types::U256,
        pub comp_borrow_index: ::ethers::core::types::U256,
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
        name = "DistributedSupplierComp",
        abi = "DistributedSupplierComp(address,address,uint256,uint256)"
    )]
    pub struct DistributedSupplierCompFilter {
        #[ethevent(indexed)]
        pub c_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub supplier: ::ethers::core::types::Address,
        pub comp_delta: ::ethers::core::types::U256,
        pub comp_supply_index: ::ethers::core::types::U256,
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
    #[ethevent(name = "MarketEntered", abi = "MarketEntered(address,address)")]
    pub struct MarketEnteredFilter {
        pub c_token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
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
    #[ethevent(name = "MarketExited", abi = "MarketExited(address,address)")]
    pub struct MarketExitedFilter {
        pub c_token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
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
    #[ethevent(name = "MarketListed", abi = "MarketListed(address)")]
    pub struct MarketListedFilter {
        pub c_token: ::ethers::core::types::Address,
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
    #[ethevent(name = "NewBorrowCap", abi = "NewBorrowCap(address,uint256)")]
    pub struct NewBorrowCapFilter {
        #[ethevent(indexed)]
        pub c_token: ::ethers::core::types::Address,
        pub new_borrow_cap: ::ethers::core::types::U256,
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
        name = "NewBorrowCapGuardian",
        abi = "NewBorrowCapGuardian(address,address)"
    )]
    pub struct NewBorrowCapGuardianFilter {
        pub old_borrow_cap_guardian: ::ethers::core::types::Address,
        pub new_borrow_cap_guardian: ::ethers::core::types::Address,
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
    #[ethevent(name = "NewCloseFactor", abi = "NewCloseFactor(uint256,uint256)")]
    pub struct NewCloseFactorFilter {
        pub old_close_factor_mantissa: ::ethers::core::types::U256,
        pub new_close_factor_mantissa: ::ethers::core::types::U256,
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
        name = "NewCollateralFactor",
        abi = "NewCollateralFactor(address,uint256,uint256)"
    )]
    pub struct NewCollateralFactorFilter {
        pub c_token: ::ethers::core::types::Address,
        pub old_collateral_factor_mantissa: ::ethers::core::types::U256,
        pub new_collateral_factor_mantissa: ::ethers::core::types::U256,
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
        name = "NewLiquidationIncentive",
        abi = "NewLiquidationIncentive(uint256,uint256)"
    )]
    pub struct NewLiquidationIncentiveFilter {
        pub old_liquidation_incentive_mantissa: ::ethers::core::types::U256,
        pub new_liquidation_incentive_mantissa: ::ethers::core::types::U256,
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
    #[ethevent(name = "NewPauseGuardian", abi = "NewPauseGuardian(address,address)")]
    pub struct NewPauseGuardianFilter {
        pub old_pause_guardian: ::ethers::core::types::Address,
        pub new_pause_guardian: ::ethers::core::types::Address,
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
    #[ethevent(name = "NewPriceOracle", abi = "NewPriceOracle(address,address)")]
    pub struct NewPriceOracleFilter {
        pub old_price_oracle: ::ethers::core::types::Address,
        pub new_price_oracle: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ComptrollerEvents {
        ActionPaused1Filter(ActionPaused1Filter),
        ActionPaused2Filter(ActionPaused2Filter),
        CompAccruedAdjustedFilter(CompAccruedAdjustedFilter),
        CompBorrowSpeedUpdatedFilter(CompBorrowSpeedUpdatedFilter),
        CompGrantedFilter(CompGrantedFilter),
        CompReceivableUpdatedFilter(CompReceivableUpdatedFilter),
        CompSupplySpeedUpdatedFilter(CompSupplySpeedUpdatedFilter),
        ContributorCompSpeedUpdatedFilter(ContributorCompSpeedUpdatedFilter),
        DistributedBorrowerCompFilter(DistributedBorrowerCompFilter),
        DistributedSupplierCompFilter(DistributedSupplierCompFilter),
        FailureFilter(FailureFilter),
        MarketEnteredFilter(MarketEnteredFilter),
        MarketExitedFilter(MarketExitedFilter),
        MarketListedFilter(MarketListedFilter),
        NewBorrowCapFilter(NewBorrowCapFilter),
        NewBorrowCapGuardianFilter(NewBorrowCapGuardianFilter),
        NewCloseFactorFilter(NewCloseFactorFilter),
        NewCollateralFactorFilter(NewCollateralFactorFilter),
        NewLiquidationIncentiveFilter(NewLiquidationIncentiveFilter),
        NewPauseGuardianFilter(NewPauseGuardianFilter),
        NewPriceOracleFilter(NewPriceOracleFilter),
    }
    impl ::ethers::contract::EthLogDecode for ComptrollerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ActionPaused1Filter::decode_log(log) {
                return Ok(ComptrollerEvents::ActionPaused1Filter(decoded));
            }
            if let Ok(decoded) = ActionPaused2Filter::decode_log(log) {
                return Ok(ComptrollerEvents::ActionPaused2Filter(decoded));
            }
            if let Ok(decoded) = CompAccruedAdjustedFilter::decode_log(log) {
                return Ok(ComptrollerEvents::CompAccruedAdjustedFilter(decoded));
            }
            if let Ok(decoded) = CompBorrowSpeedUpdatedFilter::decode_log(log) {
                return Ok(ComptrollerEvents::CompBorrowSpeedUpdatedFilter(decoded));
            }
            if let Ok(decoded) = CompGrantedFilter::decode_log(log) {
                return Ok(ComptrollerEvents::CompGrantedFilter(decoded));
            }
            if let Ok(decoded) = CompReceivableUpdatedFilter::decode_log(log) {
                return Ok(ComptrollerEvents::CompReceivableUpdatedFilter(decoded));
            }
            if let Ok(decoded) = CompSupplySpeedUpdatedFilter::decode_log(log) {
                return Ok(ComptrollerEvents::CompSupplySpeedUpdatedFilter(decoded));
            }
            if let Ok(decoded) = ContributorCompSpeedUpdatedFilter::decode_log(log) {
                return Ok(ComptrollerEvents::ContributorCompSpeedUpdatedFilter(decoded));
            }
            if let Ok(decoded) = DistributedBorrowerCompFilter::decode_log(log) {
                return Ok(ComptrollerEvents::DistributedBorrowerCompFilter(decoded));
            }
            if let Ok(decoded) = DistributedSupplierCompFilter::decode_log(log) {
                return Ok(ComptrollerEvents::DistributedSupplierCompFilter(decoded));
            }
            if let Ok(decoded) = FailureFilter::decode_log(log) {
                return Ok(ComptrollerEvents::FailureFilter(decoded));
            }
            if let Ok(decoded) = MarketEnteredFilter::decode_log(log) {
                return Ok(ComptrollerEvents::MarketEnteredFilter(decoded));
            }
            if let Ok(decoded) = MarketExitedFilter::decode_log(log) {
                return Ok(ComptrollerEvents::MarketExitedFilter(decoded));
            }
            if let Ok(decoded) = MarketListedFilter::decode_log(log) {
                return Ok(ComptrollerEvents::MarketListedFilter(decoded));
            }
            if let Ok(decoded) = NewBorrowCapFilter::decode_log(log) {
                return Ok(ComptrollerEvents::NewBorrowCapFilter(decoded));
            }
            if let Ok(decoded) = NewBorrowCapGuardianFilter::decode_log(log) {
                return Ok(ComptrollerEvents::NewBorrowCapGuardianFilter(decoded));
            }
            if let Ok(decoded) = NewCloseFactorFilter::decode_log(log) {
                return Ok(ComptrollerEvents::NewCloseFactorFilter(decoded));
            }
            if let Ok(decoded) = NewCollateralFactorFilter::decode_log(log) {
                return Ok(ComptrollerEvents::NewCollateralFactorFilter(decoded));
            }
            if let Ok(decoded) = NewLiquidationIncentiveFilter::decode_log(log) {
                return Ok(ComptrollerEvents::NewLiquidationIncentiveFilter(decoded));
            }
            if let Ok(decoded) = NewPauseGuardianFilter::decode_log(log) {
                return Ok(ComptrollerEvents::NewPauseGuardianFilter(decoded));
            }
            if let Ok(decoded) = NewPriceOracleFilter::decode_log(log) {
                return Ok(ComptrollerEvents::NewPriceOracleFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ComptrollerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ActionPaused1Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ActionPaused2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CompAccruedAdjustedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CompBorrowSpeedUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CompGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CompReceivableUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CompSupplySpeedUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ContributorCompSpeedUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DistributedBorrowerCompFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DistributedSupplierCompFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailureFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MarketEnteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MarketExitedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MarketListedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewBorrowCapFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewBorrowCapGuardianFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewCloseFactorFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewCollateralFactorFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewLiquidationIncentiveFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewPauseGuardianFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewPriceOracleFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ActionPaused1Filter> for ComptrollerEvents {
        fn from(value: ActionPaused1Filter) -> Self {
            Self::ActionPaused1Filter(value)
        }
    }
    impl ::core::convert::From<ActionPaused2Filter> for ComptrollerEvents {
        fn from(value: ActionPaused2Filter) -> Self {
            Self::ActionPaused2Filter(value)
        }
    }
    impl ::core::convert::From<CompAccruedAdjustedFilter> for ComptrollerEvents {
        fn from(value: CompAccruedAdjustedFilter) -> Self {
            Self::CompAccruedAdjustedFilter(value)
        }
    }
    impl ::core::convert::From<CompBorrowSpeedUpdatedFilter> for ComptrollerEvents {
        fn from(value: CompBorrowSpeedUpdatedFilter) -> Self {
            Self::CompBorrowSpeedUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<CompGrantedFilter> for ComptrollerEvents {
        fn from(value: CompGrantedFilter) -> Self {
            Self::CompGrantedFilter(value)
        }
    }
    impl ::core::convert::From<CompReceivableUpdatedFilter> for ComptrollerEvents {
        fn from(value: CompReceivableUpdatedFilter) -> Self {
            Self::CompReceivableUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<CompSupplySpeedUpdatedFilter> for ComptrollerEvents {
        fn from(value: CompSupplySpeedUpdatedFilter) -> Self {
            Self::CompSupplySpeedUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<ContributorCompSpeedUpdatedFilter> for ComptrollerEvents {
        fn from(value: ContributorCompSpeedUpdatedFilter) -> Self {
            Self::ContributorCompSpeedUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<DistributedBorrowerCompFilter> for ComptrollerEvents {
        fn from(value: DistributedBorrowerCompFilter) -> Self {
            Self::DistributedBorrowerCompFilter(value)
        }
    }
    impl ::core::convert::From<DistributedSupplierCompFilter> for ComptrollerEvents {
        fn from(value: DistributedSupplierCompFilter) -> Self {
            Self::DistributedSupplierCompFilter(value)
        }
    }
    impl ::core::convert::From<FailureFilter> for ComptrollerEvents {
        fn from(value: FailureFilter) -> Self {
            Self::FailureFilter(value)
        }
    }
    impl ::core::convert::From<MarketEnteredFilter> for ComptrollerEvents {
        fn from(value: MarketEnteredFilter) -> Self {
            Self::MarketEnteredFilter(value)
        }
    }
    impl ::core::convert::From<MarketExitedFilter> for ComptrollerEvents {
        fn from(value: MarketExitedFilter) -> Self {
            Self::MarketExitedFilter(value)
        }
    }
    impl ::core::convert::From<MarketListedFilter> for ComptrollerEvents {
        fn from(value: MarketListedFilter) -> Self {
            Self::MarketListedFilter(value)
        }
    }
    impl ::core::convert::From<NewBorrowCapFilter> for ComptrollerEvents {
        fn from(value: NewBorrowCapFilter) -> Self {
            Self::NewBorrowCapFilter(value)
        }
    }
    impl ::core::convert::From<NewBorrowCapGuardianFilter> for ComptrollerEvents {
        fn from(value: NewBorrowCapGuardianFilter) -> Self {
            Self::NewBorrowCapGuardianFilter(value)
        }
    }
    impl ::core::convert::From<NewCloseFactorFilter> for ComptrollerEvents {
        fn from(value: NewCloseFactorFilter) -> Self {
            Self::NewCloseFactorFilter(value)
        }
    }
    impl ::core::convert::From<NewCollateralFactorFilter> for ComptrollerEvents {
        fn from(value: NewCollateralFactorFilter) -> Self {
            Self::NewCollateralFactorFilter(value)
        }
    }
    impl ::core::convert::From<NewLiquidationIncentiveFilter> for ComptrollerEvents {
        fn from(value: NewLiquidationIncentiveFilter) -> Self {
            Self::NewLiquidationIncentiveFilter(value)
        }
    }
    impl ::core::convert::From<NewPauseGuardianFilter> for ComptrollerEvents {
        fn from(value: NewPauseGuardianFilter) -> Self {
            Self::NewPauseGuardianFilter(value)
        }
    }
    impl ::core::convert::From<NewPriceOracleFilter> for ComptrollerEvents {
        fn from(value: NewPriceOracleFilter) -> Self {
            Self::NewPriceOracleFilter(value)
        }
    }
    ///Container type for all input parameters for the `_backPopulate` function with signature `_backPopulate(address,address,address,uint256,uint256,uint256,address)` and selector `0x0753a198`
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
        name = "_backPopulate",
        abi = "_backPopulate(address,address,address,uint256,uint256,uint256,address)"
    )]
    pub struct BackPopulateCall {
        pub oracle: ::ethers::core::types::Address,
        pub pause_guardian: ::ethers::core::types::Address,
        pub borrow_cap_guardian: ::ethers::core::types::Address,
        pub close_factor_mantissa: ::ethers::core::types::U256,
        pub comp_rate: ::ethers::core::types::U256,
        pub max_assets: ::ethers::core::types::U256,
        pub comp_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `_become` function with signature `_become(address)` and selector `0x1d504dc6`
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
    #[ethcall(name = "_become", abi = "_become(address)")]
    pub struct BecomeCall {
        pub unitroller: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `_borrowGuardianPaused` function with signature `_borrowGuardianPaused()` and selector `0xe6653f3d`
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
    #[ethcall(name = "_borrowGuardianPaused", abi = "_borrowGuardianPaused()")]
    pub struct _BorrowGuardianPausedCall;
    ///Container type for all input parameters for the `_grantComp` function with signature `_grantComp(address,uint256)` and selector `0x27efe3cb`
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
    #[ethcall(name = "_grantComp", abi = "_grantComp(address,uint256)")]
    pub struct GrantCompCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `_mintGuardianPaused` function with signature `_mintGuardianPaused()` and selector `0x3c94786f`
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
    #[ethcall(name = "_mintGuardianPaused", abi = "_mintGuardianPaused()")]
    pub struct _MintGuardianPausedCall;
    ///Container type for all input parameters for the `_setBorrowCapGuardian` function with signature `_setBorrowCapGuardian(address)` and selector `0x391957d7`
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
    #[ethcall(name = "_setBorrowCapGuardian", abi = "_setBorrowCapGuardian(address)")]
    pub struct SetBorrowCapGuardianCall {
        pub new_borrow_cap_guardian: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `_setBorrowPaused` function with signature `_setBorrowPaused(address,bool)` and selector `0x18c882a5`
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
    #[ethcall(name = "_setBorrowPaused", abi = "_setBorrowPaused(address,bool)")]
    pub struct SetBorrowPausedCall {
        pub c_token: ::ethers::core::types::Address,
        pub state: bool,
    }
    ///Container type for all input parameters for the `_setCloseFactor` function with signature `_setCloseFactor(uint256)` and selector `0x317b0b77`
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
    #[ethcall(name = "_setCloseFactor", abi = "_setCloseFactor(uint256)")]
    pub struct SetCloseFactorCall {
        pub new_close_factor_mantissa: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `_setCollateralFactor` function with signature `_setCollateralFactor(address,uint256)` and selector `0xe4028eee`
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
        name = "_setCollateralFactor",
        abi = "_setCollateralFactor(address,uint256)"
    )]
    pub struct SetCollateralFactorCall {
        pub c_token: ::ethers::core::types::Address,
        pub new_collateral_factor_mantissa: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `_setCompSpeeds` function with signature `_setCompSpeeds(address[],uint256[],uint256[])` and selector `0xa8b43948`
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
        name = "_setCompSpeeds",
        abi = "_setCompSpeeds(address[],uint256[],uint256[])"
    )]
    pub struct SetCompSpeedsCall {
        pub c_tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub supply_speeds: ::std::vec::Vec<::ethers::core::types::U256>,
        pub borrow_speeds: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `_setContributorCompSpeed` function with signature `_setContributorCompSpeed(address,uint256)` and selector `0x598ee1cb`
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
        name = "_setContributorCompSpeed",
        abi = "_setContributorCompSpeed(address,uint256)"
    )]
    pub struct SetContributorCompSpeedCall {
        pub contributor: ::ethers::core::types::Address,
        pub comp_speed: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `_setLiquidationIncentive` function with signature `_setLiquidationIncentive(uint256)` and selector `0x4fd42e17`
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
        name = "_setLiquidationIncentive",
        abi = "_setLiquidationIncentive(uint256)"
    )]
    pub struct SetLiquidationIncentiveCall {
        pub new_liquidation_incentive_mantissa: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `_setMarketBorrowCaps` function with signature `_setMarketBorrowCaps(address[],uint256[])` and selector `0x607ef6c1`
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
        name = "_setMarketBorrowCaps",
        abi = "_setMarketBorrowCaps(address[],uint256[])"
    )]
    pub struct SetMarketBorrowCapsCall {
        pub c_tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub new_borrow_caps: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `_setMintPaused` function with signature `_setMintPaused(address,bool)` and selector `0x3bcf7ec1`
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
    #[ethcall(name = "_setMintPaused", abi = "_setMintPaused(address,bool)")]
    pub struct SetMintPausedCall {
        pub c_token: ::ethers::core::types::Address,
        pub state: bool,
    }
    ///Container type for all input parameters for the `_setPauseGuardian` function with signature `_setPauseGuardian(address)` and selector `0x5f5af1aa`
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
    #[ethcall(name = "_setPauseGuardian", abi = "_setPauseGuardian(address)")]
    pub struct SetPauseGuardianCall {
        pub new_pause_guardian: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `_setPriceOracle` function with signature `_setPriceOracle(address)` and selector `0x55ee1fe1`
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
    #[ethcall(name = "_setPriceOracle", abi = "_setPriceOracle(address)")]
    pub struct SetPriceOracleCall {
        pub new_oracle: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `_setSeizePaused` function with signature `_setSeizePaused(bool)` and selector `0x2d70db78`
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
    #[ethcall(name = "_setSeizePaused", abi = "_setSeizePaused(bool)")]
    pub struct SetSeizePausedCall {
        pub state: bool,
    }
    ///Container type for all input parameters for the `_setTransferPaused` function with signature `_setTransferPaused(bool)` and selector `0x8ebf6364`
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
    #[ethcall(name = "_setTransferPaused", abi = "_setTransferPaused(bool)")]
    pub struct SetTransferPausedCall {
        pub state: bool,
    }
    ///Container type for all input parameters for the `_supportMarket` function with signature `_supportMarket(address)` and selector `0xa76b3fda`
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
    #[ethcall(name = "_supportMarket", abi = "_supportMarket(address)")]
    pub struct SupportMarketCall {
        pub c_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `accountAssets` function with signature `accountAssets(address,uint256)` and selector `0xdce15449`
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
    #[ethcall(name = "accountAssets", abi = "accountAssets(address,uint256)")]
    pub struct AccountAssetsCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
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
    ///Container type for all input parameters for the `allMarkets` function with signature `allMarkets(uint256)` and selector `0x52d84d1e`
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
    #[ethcall(name = "allMarkets", abi = "allMarkets(uint256)")]
    pub struct AllMarketsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `borrowAllowed` function with signature `borrowAllowed(address,address,uint256)` and selector `0xda3d454c`
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
    #[ethcall(name = "borrowAllowed", abi = "borrowAllowed(address,address,uint256)")]
    pub struct BorrowAllowedCall {
        pub c_token: ::ethers::core::types::Address,
        pub borrower: ::ethers::core::types::Address,
        pub borrow_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `borrowCapGuardian` function with signature `borrowCapGuardian()` and selector `0x21af4569`
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
    #[ethcall(name = "borrowCapGuardian", abi = "borrowCapGuardian()")]
    pub struct BorrowCapGuardianCall;
    ///Container type for all input parameters for the `borrowCaps` function with signature `borrowCaps(address)` and selector `0x4a584432`
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
    #[ethcall(name = "borrowCaps", abi = "borrowCaps(address)")]
    pub struct BorrowCapsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `borrowGuardianPaused` function with signature `borrowGuardianPaused(address)` and selector `0x6d154ea5`
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
    #[ethcall(name = "borrowGuardianPaused", abi = "borrowGuardianPaused(address)")]
    pub struct BorrowGuardianPausedCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `borrowVerify` function with signature `borrowVerify(address,address,uint256)` and selector `0x5c778605`
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
    #[ethcall(name = "borrowVerify", abi = "borrowVerify(address,address,uint256)")]
    pub struct BorrowVerifyCall {
        pub c_token: ::ethers::core::types::Address,
        pub borrower: ::ethers::core::types::Address,
        pub borrow_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `checkMembership` function with signature `checkMembership(address,address)` and selector `0x929fe9a1`
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
    #[ethcall(name = "checkMembership", abi = "checkMembership(address,address)")]
    pub struct CheckMembershipCall {
        pub account: ::ethers::core::types::Address,
        pub c_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `claimComp` function with signature `claimComp(address,address[])` and selector `0x1c3db2e0`
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
    #[ethcall(name = "claimComp", abi = "claimComp(address,address[])")]
    pub struct ClaimCompWithCTokensCall {
        pub holder: ::ethers::core::types::Address,
        pub c_tokens: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `claimComp` function with signature `claimComp(address[],address[],bool,bool)` and selector `0x6810dfa6`
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
    #[ethcall(name = "claimComp", abi = "claimComp(address[],address[],bool,bool)")]
    pub struct ClaimCompWithHoldersCall {
        pub holders: ::std::vec::Vec<::ethers::core::types::Address>,
        pub c_tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub borrowers: bool,
        pub suppliers: bool,
    }
    ///Container type for all input parameters for the `claimComp` function with signature `claimComp(address)` and selector `0xe9af0292`
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
    #[ethcall(name = "claimComp", abi = "claimComp(address)")]
    pub struct ClaimCompCall {
        pub holder: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `closeFactorMantissa` function with signature `closeFactorMantissa()` and selector `0xe8755446`
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
    #[ethcall(name = "closeFactorMantissa", abi = "closeFactorMantissa()")]
    pub struct CloseFactorMantissaCall;
    ///Container type for all input parameters for the `compAccrued` function with signature `compAccrued(address)` and selector `0xcc7ebdc4`
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
    #[ethcall(name = "compAccrued", abi = "compAccrued(address)")]
    pub struct CompAccruedCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `compBorrowSpeeds` function with signature `compBorrowSpeeds(address)` and selector `0xf4a433c0`
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
    #[ethcall(name = "compBorrowSpeeds", abi = "compBorrowSpeeds(address)")]
    pub struct CompBorrowSpeedsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `compBorrowState` function with signature `compBorrowState(address)` and selector `0x8c57804e`
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
    #[ethcall(name = "compBorrowState", abi = "compBorrowState(address)")]
    pub struct CompBorrowStateCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `compBorrowerIndex` function with signature `compBorrowerIndex(address,address)` and selector `0xca0af043`
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
    #[ethcall(name = "compBorrowerIndex", abi = "compBorrowerIndex(address,address)")]
    pub struct CompBorrowerIndexCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `compContributorSpeeds` function with signature `compContributorSpeeds(address)` and selector `0x986ab838`
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
    #[ethcall(name = "compContributorSpeeds", abi = "compContributorSpeeds(address)")]
    pub struct CompContributorSpeedsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `compInitialIndex` function with signature `compInitialIndex()` and selector `0xa7f0e231`
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
    #[ethcall(name = "compInitialIndex", abi = "compInitialIndex()")]
    pub struct CompInitialIndexCall;
    ///Container type for all input parameters for the `compRate` function with signature `compRate()` and selector `0xaa900754`
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
    #[ethcall(name = "compRate", abi = "compRate()")]
    pub struct CompRateCall;
    ///Container type for all input parameters for the `compReceivable` function with signature `compReceivable(address)` and selector `0x85b7beb8`
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
    #[ethcall(name = "compReceivable", abi = "compReceivable(address)")]
    pub struct CompReceivableCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `compSpeeds` function with signature `compSpeeds(address)` and selector `0x1d7b33d7`
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
    #[ethcall(name = "compSpeeds", abi = "compSpeeds(address)")]
    pub struct CompSpeedsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `compSupplierIndex` function with signature `compSupplierIndex(address,address)` and selector `0xb21be7fd`
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
    #[ethcall(name = "compSupplierIndex", abi = "compSupplierIndex(address,address)")]
    pub struct CompSupplierIndexCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `compSupplySpeeds` function with signature `compSupplySpeeds(address)` and selector `0x6aa875b5`
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
    #[ethcall(name = "compSupplySpeeds", abi = "compSupplySpeeds(address)")]
    pub struct CompSupplySpeedsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `compSupplyState` function with signature `compSupplyState(address)` and selector `0x6b79c38d`
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
    #[ethcall(name = "compSupplyState", abi = "compSupplyState(address)")]
    pub struct CompSupplyStateCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `comptrollerImplementation` function with signature `comptrollerImplementation()` and selector `0xbb82aa5e`
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
    #[ethcall(name = "comptrollerImplementation", abi = "comptrollerImplementation()")]
    pub struct ComptrollerImplementationCall;
    ///Container type for all input parameters for the `enterMarkets` function with signature `enterMarkets(address[])` and selector `0xc2998238`
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
    #[ethcall(name = "enterMarkets", abi = "enterMarkets(address[])")]
    pub struct EnterMarketsCall {
        pub c_tokens: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `exitMarket` function with signature `exitMarket(address)` and selector `0xede4edd0`
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
    #[ethcall(name = "exitMarket", abi = "exitMarket(address)")]
    pub struct ExitMarketCall {
        pub c_token_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `fixBadAccruals` function with signature `fixBadAccruals(address[],uint256[])` and selector `0x16b95e8f`
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
    #[ethcall(name = "fixBadAccruals", abi = "fixBadAccruals(address[],uint256[])")]
    pub struct FixBadAccrualsCall {
        pub affected_users: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `getAccountLiquidity` function with signature `getAccountLiquidity(address)` and selector `0x5ec88c79`
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
    #[ethcall(name = "getAccountLiquidity", abi = "getAccountLiquidity(address)")]
    pub struct GetAccountLiquidityCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getAllMarkets` function with signature `getAllMarkets()` and selector `0xb0772d0b`
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
    #[ethcall(name = "getAllMarkets", abi = "getAllMarkets()")]
    pub struct GetAllMarketsCall;
    ///Container type for all input parameters for the `getAssetsIn` function with signature `getAssetsIn(address)` and selector `0xabfceffc`
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
    #[ethcall(name = "getAssetsIn", abi = "getAssetsIn(address)")]
    pub struct GetAssetsInCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getBlockNumber` function with signature `getBlockNumber()` and selector `0x42cbb15c`
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
    #[ethcall(name = "getBlockNumber", abi = "getBlockNumber()")]
    pub struct GetBlockNumberCall;
    ///Container type for all input parameters for the `getCompAddress` function with signature `getCompAddress()` and selector `0x9d1b5a0a`
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
    #[ethcall(name = "getCompAddress", abi = "getCompAddress()")]
    pub struct GetCompAddressCall;
    ///Container type for all input parameters for the `getHypotheticalAccountLiquidity` function with signature `getHypotheticalAccountLiquidity(address,address,uint256,uint256)` and selector `0x4e79238f`
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
        name = "getHypotheticalAccountLiquidity",
        abi = "getHypotheticalAccountLiquidity(address,address,uint256,uint256)"
    )]
    pub struct GetHypotheticalAccountLiquidityCall {
        pub account: ::ethers::core::types::Address,
        pub c_token_modify: ::ethers::core::types::Address,
        pub redeem_tokens: ::ethers::core::types::U256,
        pub borrow_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isComptroller` function with signature `isComptroller()` and selector `0x007e3dd2`
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
    #[ethcall(name = "isComptroller", abi = "isComptroller()")]
    pub struct IsComptrollerCall;
    ///Container type for all input parameters for the `isDeprecated` function with signature `isDeprecated(address)` and selector `0x94543c15`
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
    #[ethcall(name = "isDeprecated", abi = "isDeprecated(address)")]
    pub struct IsDeprecatedCall {
        pub c_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `lastContributorBlock` function with signature `lastContributorBlock(address)` and selector `0xbea6b8b8`
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
    #[ethcall(name = "lastContributorBlock", abi = "lastContributorBlock(address)")]
    pub struct LastContributorBlockCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `liquidateBorrowAllowed` function with signature `liquidateBorrowAllowed(address,address,address,address,uint256)` and selector `0x5fc7e71e`
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
        name = "liquidateBorrowAllowed",
        abi = "liquidateBorrowAllowed(address,address,address,address,uint256)"
    )]
    pub struct LiquidateBorrowAllowedCall {
        pub c_token_borrowed: ::ethers::core::types::Address,
        pub c_token_collateral: ::ethers::core::types::Address,
        pub liquidator: ::ethers::core::types::Address,
        pub borrower: ::ethers::core::types::Address,
        pub repay_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `liquidateBorrowVerify` function with signature `liquidateBorrowVerify(address,address,address,address,uint256,uint256)` and selector `0x47ef3b3b`
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
        name = "liquidateBorrowVerify",
        abi = "liquidateBorrowVerify(address,address,address,address,uint256,uint256)"
    )]
    pub struct LiquidateBorrowVerifyCall {
        pub c_token_borrowed: ::ethers::core::types::Address,
        pub c_token_collateral: ::ethers::core::types::Address,
        pub liquidator: ::ethers::core::types::Address,
        pub borrower: ::ethers::core::types::Address,
        pub actual_repay_amount: ::ethers::core::types::U256,
        pub seize_tokens: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `liquidateCalculateSeizeTokens` function with signature `liquidateCalculateSeizeTokens(address,address,uint256)` and selector `0xc488847b`
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
        name = "liquidateCalculateSeizeTokens",
        abi = "liquidateCalculateSeizeTokens(address,address,uint256)"
    )]
    pub struct LiquidateCalculateSeizeTokensCall {
        pub c_token_borrowed: ::ethers::core::types::Address,
        pub c_token_collateral: ::ethers::core::types::Address,
        pub actual_repay_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `liquidationIncentiveMantissa` function with signature `liquidationIncentiveMantissa()` and selector `0x4ada90af`
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
        name = "liquidationIncentiveMantissa",
        abi = "liquidationIncentiveMantissa()"
    )]
    pub struct LiquidationIncentiveMantissaCall;
    ///Container type for all input parameters for the `markets` function with signature `markets(address)` and selector `0x8e8f294b`
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
    #[ethcall(name = "markets", abi = "markets(address)")]
    pub struct MarketsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `maxAssets` function with signature `maxAssets()` and selector `0x94b2294b`
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
    #[ethcall(name = "maxAssets", abi = "maxAssets()")]
    pub struct MaxAssetsCall;
    ///Container type for all input parameters for the `mintAllowed` function with signature `mintAllowed(address,address,uint256)` and selector `0x4ef4c3e1`
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
    #[ethcall(name = "mintAllowed", abi = "mintAllowed(address,address,uint256)")]
    pub struct MintAllowedCall {
        pub c_token: ::ethers::core::types::Address,
        pub minter: ::ethers::core::types::Address,
        pub mint_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `mintGuardianPaused` function with signature `mintGuardianPaused(address)` and selector `0x731f0c2b`
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
    #[ethcall(name = "mintGuardianPaused", abi = "mintGuardianPaused(address)")]
    pub struct MintGuardianPausedCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `mintVerify` function with signature `mintVerify(address,address,uint256,uint256)` and selector `0x41c728b9`
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
    #[ethcall(name = "mintVerify", abi = "mintVerify(address,address,uint256,uint256)")]
    pub struct MintVerifyCall {
        pub c_token: ::ethers::core::types::Address,
        pub minter: ::ethers::core::types::Address,
        pub actual_mint_amount: ::ethers::core::types::U256,
        pub mint_tokens: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `oracle` function with signature `oracle()` and selector `0x7dc0d1d0`
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
    #[ethcall(name = "oracle", abi = "oracle()")]
    pub struct OracleCall;
    ///Container type for all input parameters for the `pauseGuardian` function with signature `pauseGuardian()` and selector `0x24a3d622`
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
    #[ethcall(name = "pauseGuardian", abi = "pauseGuardian()")]
    pub struct PauseGuardianCall;
    ///Container type for all input parameters for the `pendingAdmin` function with signature `pendingAdmin()` and selector `0x26782247`
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
        Hash
    )]
    #[ethcall(
        name = "pendingComptrollerImplementation",
        abi = "pendingComptrollerImplementation()"
    )]
    pub struct PendingComptrollerImplementationCall;
    ///Container type for all input parameters for the `proposal65FixExecuted` function with signature `proposal65FixExecuted()` and selector `0xf00a7a92`
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
    #[ethcall(name = "proposal65FixExecuted", abi = "proposal65FixExecuted()")]
    pub struct Proposal65FixExecutedCall;
    ///Container type for all input parameters for the `redeemAllowed` function with signature `redeemAllowed(address,address,uint256)` and selector `0xeabe7d91`
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
    #[ethcall(name = "redeemAllowed", abi = "redeemAllowed(address,address,uint256)")]
    pub struct RedeemAllowedCall {
        pub c_token: ::ethers::core::types::Address,
        pub redeemer: ::ethers::core::types::Address,
        pub redeem_tokens: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `redeemVerify` function with signature `redeemVerify(address,address,uint256,uint256)` and selector `0x51dff989`
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
        name = "redeemVerify",
        abi = "redeemVerify(address,address,uint256,uint256)"
    )]
    pub struct RedeemVerifyCall {
        pub c_token: ::ethers::core::types::Address,
        pub redeemer: ::ethers::core::types::Address,
        pub redeem_amount: ::ethers::core::types::U256,
        pub redeem_tokens: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `repayBorrowAllowed` function with signature `repayBorrowAllowed(address,address,address,uint256)` and selector `0x24008a62`
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
        name = "repayBorrowAllowed",
        abi = "repayBorrowAllowed(address,address,address,uint256)"
    )]
    pub struct RepayBorrowAllowedCall {
        pub c_token: ::ethers::core::types::Address,
        pub payer: ::ethers::core::types::Address,
        pub borrower: ::ethers::core::types::Address,
        pub repay_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `repayBorrowVerify` function with signature `repayBorrowVerify(address,address,address,uint256,uint256)` and selector `0x1ededc91`
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
        name = "repayBorrowVerify",
        abi = "repayBorrowVerify(address,address,address,uint256,uint256)"
    )]
    pub struct RepayBorrowVerifyCall {
        pub c_token: ::ethers::core::types::Address,
        pub payer: ::ethers::core::types::Address,
        pub borrower: ::ethers::core::types::Address,
        pub actual_repay_amount: ::ethers::core::types::U256,
        pub borrower_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `seizeAllowed` function with signature `seizeAllowed(address,address,address,address,uint256)` and selector `0xd02f7351`
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
        name = "seizeAllowed",
        abi = "seizeAllowed(address,address,address,address,uint256)"
    )]
    pub struct SeizeAllowedCall {
        pub c_token_collateral: ::ethers::core::types::Address,
        pub c_token_borrowed: ::ethers::core::types::Address,
        pub liquidator: ::ethers::core::types::Address,
        pub borrower: ::ethers::core::types::Address,
        pub seize_tokens: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `seizeGuardianPaused` function with signature `seizeGuardianPaused()` and selector `0xac0b0bb7`
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
    #[ethcall(name = "seizeGuardianPaused", abi = "seizeGuardianPaused()")]
    pub struct SeizeGuardianPausedCall;
    ///Container type for all input parameters for the `seizeVerify` function with signature `seizeVerify(address,address,address,address,uint256)` and selector `0x6d35bf91`
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
        name = "seizeVerify",
        abi = "seizeVerify(address,address,address,address,uint256)"
    )]
    pub struct SeizeVerifyCall {
        pub c_token_collateral: ::ethers::core::types::Address,
        pub c_token_borrowed: ::ethers::core::types::Address,
        pub liquidator: ::ethers::core::types::Address,
        pub borrower: ::ethers::core::types::Address,
        pub seize_tokens: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferAllowed` function with signature `transferAllowed(address,address,address,uint256)` and selector `0xbdcdc258`
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
        name = "transferAllowed",
        abi = "transferAllowed(address,address,address,uint256)"
    )]
    pub struct TransferAllowedCall {
        pub c_token: ::ethers::core::types::Address,
        pub src: ::ethers::core::types::Address,
        pub dst: ::ethers::core::types::Address,
        pub transfer_tokens: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferGuardianPaused` function with signature `transferGuardianPaused()` and selector `0x87f76303`
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
    #[ethcall(name = "transferGuardianPaused", abi = "transferGuardianPaused()")]
    pub struct TransferGuardianPausedCall;
    ///Container type for all input parameters for the `transferVerify` function with signature `transferVerify(address,address,address,uint256)` and selector `0x6a56947e`
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
        name = "transferVerify",
        abi = "transferVerify(address,address,address,uint256)"
    )]
    pub struct TransferVerifyCall {
        pub c_token: ::ethers::core::types::Address,
        pub src: ::ethers::core::types::Address,
        pub dst: ::ethers::core::types::Address,
        pub transfer_tokens: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateContributorRewards` function with signature `updateContributorRewards(address)` and selector `0x741b2525`
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
        name = "updateContributorRewards",
        abi = "updateContributorRewards(address)"
    )]
    pub struct UpdateContributorRewardsCall {
        pub contributor: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ComptrollerCalls {
        BackPopulate(BackPopulateCall),
        Become(BecomeCall),
        _BorrowGuardianPaused(_BorrowGuardianPausedCall),
        GrantComp(GrantCompCall),
        _MintGuardianPaused(_MintGuardianPausedCall),
        SetBorrowCapGuardian(SetBorrowCapGuardianCall),
        SetBorrowPaused(SetBorrowPausedCall),
        SetCloseFactor(SetCloseFactorCall),
        SetCollateralFactor(SetCollateralFactorCall),
        SetCompSpeeds(SetCompSpeedsCall),
        SetContributorCompSpeed(SetContributorCompSpeedCall),
        SetLiquidationIncentive(SetLiquidationIncentiveCall),
        SetMarketBorrowCaps(SetMarketBorrowCapsCall),
        SetMintPaused(SetMintPausedCall),
        SetPauseGuardian(SetPauseGuardianCall),
        SetPriceOracle(SetPriceOracleCall),
        SetSeizePaused(SetSeizePausedCall),
        SetTransferPaused(SetTransferPausedCall),
        SupportMarket(SupportMarketCall),
        AccountAssets(AccountAssetsCall),
        Admin(AdminCall),
        AllMarkets(AllMarketsCall),
        BorrowAllowed(BorrowAllowedCall),
        BorrowCapGuardian(BorrowCapGuardianCall),
        BorrowCaps(BorrowCapsCall),
        BorrowGuardianPaused(BorrowGuardianPausedCall),
        BorrowVerify(BorrowVerifyCall),
        CheckMembership(CheckMembershipCall),
        ClaimCompWithCTokens(ClaimCompWithCTokensCall),
        ClaimCompWithHolders(ClaimCompWithHoldersCall),
        ClaimComp(ClaimCompCall),
        CloseFactorMantissa(CloseFactorMantissaCall),
        CompAccrued(CompAccruedCall),
        CompBorrowSpeeds(CompBorrowSpeedsCall),
        CompBorrowState(CompBorrowStateCall),
        CompBorrowerIndex(CompBorrowerIndexCall),
        CompContributorSpeeds(CompContributorSpeedsCall),
        CompInitialIndex(CompInitialIndexCall),
        CompRate(CompRateCall),
        CompReceivable(CompReceivableCall),
        CompSpeeds(CompSpeedsCall),
        CompSupplierIndex(CompSupplierIndexCall),
        CompSupplySpeeds(CompSupplySpeedsCall),
        CompSupplyState(CompSupplyStateCall),
        ComptrollerImplementation(ComptrollerImplementationCall),
        EnterMarkets(EnterMarketsCall),
        ExitMarket(ExitMarketCall),
        FixBadAccruals(FixBadAccrualsCall),
        GetAccountLiquidity(GetAccountLiquidityCall),
        GetAllMarkets(GetAllMarketsCall),
        GetAssetsIn(GetAssetsInCall),
        GetBlockNumber(GetBlockNumberCall),
        GetCompAddress(GetCompAddressCall),
        GetHypotheticalAccountLiquidity(GetHypotheticalAccountLiquidityCall),
        IsComptroller(IsComptrollerCall),
        IsDeprecated(IsDeprecatedCall),
        LastContributorBlock(LastContributorBlockCall),
        LiquidateBorrowAllowed(LiquidateBorrowAllowedCall),
        LiquidateBorrowVerify(LiquidateBorrowVerifyCall),
        LiquidateCalculateSeizeTokens(LiquidateCalculateSeizeTokensCall),
        LiquidationIncentiveMantissa(LiquidationIncentiveMantissaCall),
        Markets(MarketsCall),
        MaxAssets(MaxAssetsCall),
        MintAllowed(MintAllowedCall),
        MintGuardianPaused(MintGuardianPausedCall),
        MintVerify(MintVerifyCall),
        Oracle(OracleCall),
        PauseGuardian(PauseGuardianCall),
        PendingAdmin(PendingAdminCall),
        PendingComptrollerImplementation(PendingComptrollerImplementationCall),
        Proposal65FixExecuted(Proposal65FixExecutedCall),
        RedeemAllowed(RedeemAllowedCall),
        RedeemVerify(RedeemVerifyCall),
        RepayBorrowAllowed(RepayBorrowAllowedCall),
        RepayBorrowVerify(RepayBorrowVerifyCall),
        SeizeAllowed(SeizeAllowedCall),
        SeizeGuardianPaused(SeizeGuardianPausedCall),
        SeizeVerify(SeizeVerifyCall),
        TransferAllowed(TransferAllowedCall),
        TransferGuardianPaused(TransferGuardianPausedCall),
        TransferVerify(TransferVerifyCall),
        UpdateContributorRewards(UpdateContributorRewardsCall),
    }
    impl ::ethers::core::abi::AbiDecode for ComptrollerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <BackPopulateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BackPopulate(decoded));
            }
            if let Ok(decoded)
                = <BecomeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Become(decoded));
            }
            if let Ok(decoded)
                = <_BorrowGuardianPausedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::_BorrowGuardianPaused(decoded));
            }
            if let Ok(decoded)
                = <GrantCompCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GrantComp(decoded));
            }
            if let Ok(decoded)
                = <_MintGuardianPausedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::_MintGuardianPaused(decoded));
            }
            if let Ok(decoded)
                = <SetBorrowCapGuardianCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetBorrowCapGuardian(decoded));
            }
            if let Ok(decoded)
                = <SetBorrowPausedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetBorrowPaused(decoded));
            }
            if let Ok(decoded)
                = <SetCloseFactorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetCloseFactor(decoded));
            }
            if let Ok(decoded)
                = <SetCollateralFactorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetCollateralFactor(decoded));
            }
            if let Ok(decoded)
                = <SetCompSpeedsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetCompSpeeds(decoded));
            }
            if let Ok(decoded)
                = <SetContributorCompSpeedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetContributorCompSpeed(decoded));
            }
            if let Ok(decoded)
                = <SetLiquidationIncentiveCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetLiquidationIncentive(decoded));
            }
            if let Ok(decoded)
                = <SetMarketBorrowCapsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetMarketBorrowCaps(decoded));
            }
            if let Ok(decoded)
                = <SetMintPausedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetMintPaused(decoded));
            }
            if let Ok(decoded)
                = <SetPauseGuardianCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetPauseGuardian(decoded));
            }
            if let Ok(decoded)
                = <SetPriceOracleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetPriceOracle(decoded));
            }
            if let Ok(decoded)
                = <SetSeizePausedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetSeizePaused(decoded));
            }
            if let Ok(decoded)
                = <SetTransferPausedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetTransferPaused(decoded));
            }
            if let Ok(decoded)
                = <SupportMarketCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SupportMarket(decoded));
            }
            if let Ok(decoded)
                = <AccountAssetsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AccountAssets(decoded));
            }
            if let Ok(decoded)
                = <AdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Admin(decoded));
            }
            if let Ok(decoded)
                = <AllMarketsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AllMarkets(decoded));
            }
            if let Ok(decoded)
                = <BorrowAllowedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BorrowAllowed(decoded));
            }
            if let Ok(decoded)
                = <BorrowCapGuardianCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BorrowCapGuardian(decoded));
            }
            if let Ok(decoded)
                = <BorrowCapsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BorrowCaps(decoded));
            }
            if let Ok(decoded)
                = <BorrowGuardianPausedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BorrowGuardianPaused(decoded));
            }
            if let Ok(decoded)
                = <BorrowVerifyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BorrowVerify(decoded));
            }
            if let Ok(decoded)
                = <CheckMembershipCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CheckMembership(decoded));
            }
            if let Ok(decoded)
                = <ClaimCompWithCTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ClaimCompWithCTokens(decoded));
            }
            if let Ok(decoded)
                = <ClaimCompWithHoldersCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ClaimCompWithHolders(decoded));
            }
            if let Ok(decoded)
                = <ClaimCompCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ClaimComp(decoded));
            }
            if let Ok(decoded)
                = <CloseFactorMantissaCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CloseFactorMantissa(decoded));
            }
            if let Ok(decoded)
                = <CompAccruedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CompAccrued(decoded));
            }
            if let Ok(decoded)
                = <CompBorrowSpeedsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CompBorrowSpeeds(decoded));
            }
            if let Ok(decoded)
                = <CompBorrowStateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CompBorrowState(decoded));
            }
            if let Ok(decoded)
                = <CompBorrowerIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CompBorrowerIndex(decoded));
            }
            if let Ok(decoded)
                = <CompContributorSpeedsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CompContributorSpeeds(decoded));
            }
            if let Ok(decoded)
                = <CompInitialIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CompInitialIndex(decoded));
            }
            if let Ok(decoded)
                = <CompRateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CompRate(decoded));
            }
            if let Ok(decoded)
                = <CompReceivableCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CompReceivable(decoded));
            }
            if let Ok(decoded)
                = <CompSpeedsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CompSpeeds(decoded));
            }
            if let Ok(decoded)
                = <CompSupplierIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CompSupplierIndex(decoded));
            }
            if let Ok(decoded)
                = <CompSupplySpeedsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CompSupplySpeeds(decoded));
            }
            if let Ok(decoded)
                = <CompSupplyStateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CompSupplyState(decoded));
            }
            if let Ok(decoded)
                = <ComptrollerImplementationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ComptrollerImplementation(decoded));
            }
            if let Ok(decoded)
                = <EnterMarketsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EnterMarkets(decoded));
            }
            if let Ok(decoded)
                = <ExitMarketCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExitMarket(decoded));
            }
            if let Ok(decoded)
                = <FixBadAccrualsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FixBadAccruals(decoded));
            }
            if let Ok(decoded)
                = <GetAccountLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetAccountLiquidity(decoded));
            }
            if let Ok(decoded)
                = <GetAllMarketsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAllMarkets(decoded));
            }
            if let Ok(decoded)
                = <GetAssetsInCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAssetsIn(decoded));
            }
            if let Ok(decoded)
                = <GetBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetBlockNumber(decoded));
            }
            if let Ok(decoded)
                = <GetCompAddressCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetCompAddress(decoded));
            }
            if let Ok(decoded)
                = <GetHypotheticalAccountLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetHypotheticalAccountLiquidity(decoded));
            }
            if let Ok(decoded)
                = <IsComptrollerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsComptroller(decoded));
            }
            if let Ok(decoded)
                = <IsDeprecatedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsDeprecated(decoded));
            }
            if let Ok(decoded)
                = <LastContributorBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::LastContributorBlock(decoded));
            }
            if let Ok(decoded)
                = <LiquidateBorrowAllowedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::LiquidateBorrowAllowed(decoded));
            }
            if let Ok(decoded)
                = <LiquidateBorrowVerifyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::LiquidateBorrowVerify(decoded));
            }
            if let Ok(decoded)
                = <LiquidateCalculateSeizeTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::LiquidateCalculateSeizeTokens(decoded));
            }
            if let Ok(decoded)
                = <LiquidationIncentiveMantissaCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::LiquidationIncentiveMantissa(decoded));
            }
            if let Ok(decoded)
                = <MarketsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Markets(decoded));
            }
            if let Ok(decoded)
                = <MaxAssetsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaxAssets(decoded));
            }
            if let Ok(decoded)
                = <MintAllowedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MintAllowed(decoded));
            }
            if let Ok(decoded)
                = <MintGuardianPausedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MintGuardianPaused(decoded));
            }
            if let Ok(decoded)
                = <MintVerifyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MintVerify(decoded));
            }
            if let Ok(decoded)
                = <OracleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Oracle(decoded));
            }
            if let Ok(decoded)
                = <PauseGuardianCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PauseGuardian(decoded));
            }
            if let Ok(decoded)
                = <PendingAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PendingAdmin(decoded));
            }
            if let Ok(decoded)
                = <PendingComptrollerImplementationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PendingComptrollerImplementation(decoded));
            }
            if let Ok(decoded)
                = <Proposal65FixExecutedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::Proposal65FixExecuted(decoded));
            }
            if let Ok(decoded)
                = <RedeemAllowedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RedeemAllowed(decoded));
            }
            if let Ok(decoded)
                = <RedeemVerifyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RedeemVerify(decoded));
            }
            if let Ok(decoded)
                = <RepayBorrowAllowedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RepayBorrowAllowed(decoded));
            }
            if let Ok(decoded)
                = <RepayBorrowVerifyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RepayBorrowVerify(decoded));
            }
            if let Ok(decoded)
                = <SeizeAllowedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SeizeAllowed(decoded));
            }
            if let Ok(decoded)
                = <SeizeGuardianPausedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SeizeGuardianPaused(decoded));
            }
            if let Ok(decoded)
                = <SeizeVerifyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SeizeVerify(decoded));
            }
            if let Ok(decoded)
                = <TransferAllowedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferAllowed(decoded));
            }
            if let Ok(decoded)
                = <TransferGuardianPausedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferGuardianPaused(decoded));
            }
            if let Ok(decoded)
                = <TransferVerifyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferVerify(decoded));
            }
            if let Ok(decoded)
                = <UpdateContributorRewardsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UpdateContributorRewards(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ComptrollerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BackPopulate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Become(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::_BorrowGuardianPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantComp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::_MintGuardianPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetBorrowCapGuardian(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetBorrowPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetCloseFactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetCollateralFactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetCompSpeeds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetContributorCompSpeed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetLiquidationIncentive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMarketBorrowCaps(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMintPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPauseGuardian(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPriceOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSeizePaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTransferPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportMarket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccountAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Admin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AllMarkets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BorrowAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BorrowCapGuardian(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BorrowCaps(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BorrowGuardianPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BorrowVerify(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckMembership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimCompWithCTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimCompWithHolders(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimComp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CloseFactorMantissa(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompAccrued(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompBorrowSpeeds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompBorrowState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompBorrowerIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompContributorSpeeds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompInitialIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompReceivable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompSpeeds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompSupplierIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompSupplySpeeds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompSupplyState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComptrollerImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnterMarkets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExitMarket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FixBadAccruals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAccountLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAllMarkets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAssetsIn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCompAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetHypotheticalAccountLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsComptroller(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsDeprecated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastContributorBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidateBorrowAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidateBorrowVerify(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidateCalculateSeizeTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidationIncentiveMantissa(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Markets(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintGuardianPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintVerify(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Oracle(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauseGuardian(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PendingAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PendingComptrollerImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Proposal65FixExecuted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RedeemAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RedeemVerify(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RepayBorrowAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RepayBorrowVerify(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SeizeAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SeizeGuardianPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SeizeVerify(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferGuardianPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferVerify(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateContributorRewards(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ComptrollerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BackPopulate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Become(element) => ::core::fmt::Display::fmt(element, f),
                Self::_BorrowGuardianPaused(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GrantComp(element) => ::core::fmt::Display::fmt(element, f),
                Self::_MintGuardianPaused(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetBorrowCapGuardian(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetBorrowPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetCloseFactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetCollateralFactor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetCompSpeeds(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetContributorCompSpeed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetLiquidationIncentive(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetMarketBorrowCaps(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetMintPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauseGuardian(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPriceOracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSeizePaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTransferPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportMarket(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccountAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::Admin(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllMarkets(element) => ::core::fmt::Display::fmt(element, f),
                Self::BorrowAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::BorrowCapGuardian(element) => ::core::fmt::Display::fmt(element, f),
                Self::BorrowCaps(element) => ::core::fmt::Display::fmt(element, f),
                Self::BorrowGuardianPaused(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BorrowVerify(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckMembership(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimCompWithCTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ClaimCompWithHolders(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ClaimComp(element) => ::core::fmt::Display::fmt(element, f),
                Self::CloseFactorMantissa(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CompAccrued(element) => ::core::fmt::Display::fmt(element, f),
                Self::CompBorrowSpeeds(element) => ::core::fmt::Display::fmt(element, f),
                Self::CompBorrowState(element) => ::core::fmt::Display::fmt(element, f),
                Self::CompBorrowerIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::CompContributorSpeeds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CompInitialIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::CompRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::CompReceivable(element) => ::core::fmt::Display::fmt(element, f),
                Self::CompSpeeds(element) => ::core::fmt::Display::fmt(element, f),
                Self::CompSupplierIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::CompSupplySpeeds(element) => ::core::fmt::Display::fmt(element, f),
                Self::CompSupplyState(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComptrollerImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnterMarkets(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExitMarket(element) => ::core::fmt::Display::fmt(element, f),
                Self::FixBadAccruals(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAccountLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetAllMarkets(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAssetsIn(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBlockNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCompAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetHypotheticalAccountLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsComptroller(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDeprecated(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastContributorBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidateBorrowAllowed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidateBorrowVerify(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidateCalculateSeizeTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidationIncentiveMantissa(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Markets(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintGuardianPaused(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MintVerify(element) => ::core::fmt::Display::fmt(element, f),
                Self::Oracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseGuardian(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingComptrollerImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Proposal65FixExecuted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RedeemAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::RedeemVerify(element) => ::core::fmt::Display::fmt(element, f),
                Self::RepayBorrowAllowed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RepayBorrowVerify(element) => ::core::fmt::Display::fmt(element, f),
                Self::SeizeAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::SeizeGuardianPaused(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SeizeVerify(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferGuardianPaused(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferVerify(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateContributorRewards(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<BackPopulateCall> for ComptrollerCalls {
        fn from(value: BackPopulateCall) -> Self {
            Self::BackPopulate(value)
        }
    }
    impl ::core::convert::From<BecomeCall> for ComptrollerCalls {
        fn from(value: BecomeCall) -> Self {
            Self::Become(value)
        }
    }
    impl ::core::convert::From<_BorrowGuardianPausedCall> for ComptrollerCalls {
        fn from(value: _BorrowGuardianPausedCall) -> Self {
            Self::_BorrowGuardianPaused(value)
        }
    }
    impl ::core::convert::From<GrantCompCall> for ComptrollerCalls {
        fn from(value: GrantCompCall) -> Self {
            Self::GrantComp(value)
        }
    }
    impl ::core::convert::From<_MintGuardianPausedCall> for ComptrollerCalls {
        fn from(value: _MintGuardianPausedCall) -> Self {
            Self::_MintGuardianPaused(value)
        }
    }
    impl ::core::convert::From<SetBorrowCapGuardianCall> for ComptrollerCalls {
        fn from(value: SetBorrowCapGuardianCall) -> Self {
            Self::SetBorrowCapGuardian(value)
        }
    }
    impl ::core::convert::From<SetBorrowPausedCall> for ComptrollerCalls {
        fn from(value: SetBorrowPausedCall) -> Self {
            Self::SetBorrowPaused(value)
        }
    }
    impl ::core::convert::From<SetCloseFactorCall> for ComptrollerCalls {
        fn from(value: SetCloseFactorCall) -> Self {
            Self::SetCloseFactor(value)
        }
    }
    impl ::core::convert::From<SetCollateralFactorCall> for ComptrollerCalls {
        fn from(value: SetCollateralFactorCall) -> Self {
            Self::SetCollateralFactor(value)
        }
    }
    impl ::core::convert::From<SetCompSpeedsCall> for ComptrollerCalls {
        fn from(value: SetCompSpeedsCall) -> Self {
            Self::SetCompSpeeds(value)
        }
    }
    impl ::core::convert::From<SetContributorCompSpeedCall> for ComptrollerCalls {
        fn from(value: SetContributorCompSpeedCall) -> Self {
            Self::SetContributorCompSpeed(value)
        }
    }
    impl ::core::convert::From<SetLiquidationIncentiveCall> for ComptrollerCalls {
        fn from(value: SetLiquidationIncentiveCall) -> Self {
            Self::SetLiquidationIncentive(value)
        }
    }
    impl ::core::convert::From<SetMarketBorrowCapsCall> for ComptrollerCalls {
        fn from(value: SetMarketBorrowCapsCall) -> Self {
            Self::SetMarketBorrowCaps(value)
        }
    }
    impl ::core::convert::From<SetMintPausedCall> for ComptrollerCalls {
        fn from(value: SetMintPausedCall) -> Self {
            Self::SetMintPaused(value)
        }
    }
    impl ::core::convert::From<SetPauseGuardianCall> for ComptrollerCalls {
        fn from(value: SetPauseGuardianCall) -> Self {
            Self::SetPauseGuardian(value)
        }
    }
    impl ::core::convert::From<SetPriceOracleCall> for ComptrollerCalls {
        fn from(value: SetPriceOracleCall) -> Self {
            Self::SetPriceOracle(value)
        }
    }
    impl ::core::convert::From<SetSeizePausedCall> for ComptrollerCalls {
        fn from(value: SetSeizePausedCall) -> Self {
            Self::SetSeizePaused(value)
        }
    }
    impl ::core::convert::From<SetTransferPausedCall> for ComptrollerCalls {
        fn from(value: SetTransferPausedCall) -> Self {
            Self::SetTransferPaused(value)
        }
    }
    impl ::core::convert::From<SupportMarketCall> for ComptrollerCalls {
        fn from(value: SupportMarketCall) -> Self {
            Self::SupportMarket(value)
        }
    }
    impl ::core::convert::From<AccountAssetsCall> for ComptrollerCalls {
        fn from(value: AccountAssetsCall) -> Self {
            Self::AccountAssets(value)
        }
    }
    impl ::core::convert::From<AdminCall> for ComptrollerCalls {
        fn from(value: AdminCall) -> Self {
            Self::Admin(value)
        }
    }
    impl ::core::convert::From<AllMarketsCall> for ComptrollerCalls {
        fn from(value: AllMarketsCall) -> Self {
            Self::AllMarkets(value)
        }
    }
    impl ::core::convert::From<BorrowAllowedCall> for ComptrollerCalls {
        fn from(value: BorrowAllowedCall) -> Self {
            Self::BorrowAllowed(value)
        }
    }
    impl ::core::convert::From<BorrowCapGuardianCall> for ComptrollerCalls {
        fn from(value: BorrowCapGuardianCall) -> Self {
            Self::BorrowCapGuardian(value)
        }
    }
    impl ::core::convert::From<BorrowCapsCall> for ComptrollerCalls {
        fn from(value: BorrowCapsCall) -> Self {
            Self::BorrowCaps(value)
        }
    }
    impl ::core::convert::From<BorrowGuardianPausedCall> for ComptrollerCalls {
        fn from(value: BorrowGuardianPausedCall) -> Self {
            Self::BorrowGuardianPaused(value)
        }
    }
    impl ::core::convert::From<BorrowVerifyCall> for ComptrollerCalls {
        fn from(value: BorrowVerifyCall) -> Self {
            Self::BorrowVerify(value)
        }
    }
    impl ::core::convert::From<CheckMembershipCall> for ComptrollerCalls {
        fn from(value: CheckMembershipCall) -> Self {
            Self::CheckMembership(value)
        }
    }
    impl ::core::convert::From<ClaimCompWithCTokensCall> for ComptrollerCalls {
        fn from(value: ClaimCompWithCTokensCall) -> Self {
            Self::ClaimCompWithCTokens(value)
        }
    }
    impl ::core::convert::From<ClaimCompWithHoldersCall> for ComptrollerCalls {
        fn from(value: ClaimCompWithHoldersCall) -> Self {
            Self::ClaimCompWithHolders(value)
        }
    }
    impl ::core::convert::From<ClaimCompCall> for ComptrollerCalls {
        fn from(value: ClaimCompCall) -> Self {
            Self::ClaimComp(value)
        }
    }
    impl ::core::convert::From<CloseFactorMantissaCall> for ComptrollerCalls {
        fn from(value: CloseFactorMantissaCall) -> Self {
            Self::CloseFactorMantissa(value)
        }
    }
    impl ::core::convert::From<CompAccruedCall> for ComptrollerCalls {
        fn from(value: CompAccruedCall) -> Self {
            Self::CompAccrued(value)
        }
    }
    impl ::core::convert::From<CompBorrowSpeedsCall> for ComptrollerCalls {
        fn from(value: CompBorrowSpeedsCall) -> Self {
            Self::CompBorrowSpeeds(value)
        }
    }
    impl ::core::convert::From<CompBorrowStateCall> for ComptrollerCalls {
        fn from(value: CompBorrowStateCall) -> Self {
            Self::CompBorrowState(value)
        }
    }
    impl ::core::convert::From<CompBorrowerIndexCall> for ComptrollerCalls {
        fn from(value: CompBorrowerIndexCall) -> Self {
            Self::CompBorrowerIndex(value)
        }
    }
    impl ::core::convert::From<CompContributorSpeedsCall> for ComptrollerCalls {
        fn from(value: CompContributorSpeedsCall) -> Self {
            Self::CompContributorSpeeds(value)
        }
    }
    impl ::core::convert::From<CompInitialIndexCall> for ComptrollerCalls {
        fn from(value: CompInitialIndexCall) -> Self {
            Self::CompInitialIndex(value)
        }
    }
    impl ::core::convert::From<CompRateCall> for ComptrollerCalls {
        fn from(value: CompRateCall) -> Self {
            Self::CompRate(value)
        }
    }
    impl ::core::convert::From<CompReceivableCall> for ComptrollerCalls {
        fn from(value: CompReceivableCall) -> Self {
            Self::CompReceivable(value)
        }
    }
    impl ::core::convert::From<CompSpeedsCall> for ComptrollerCalls {
        fn from(value: CompSpeedsCall) -> Self {
            Self::CompSpeeds(value)
        }
    }
    impl ::core::convert::From<CompSupplierIndexCall> for ComptrollerCalls {
        fn from(value: CompSupplierIndexCall) -> Self {
            Self::CompSupplierIndex(value)
        }
    }
    impl ::core::convert::From<CompSupplySpeedsCall> for ComptrollerCalls {
        fn from(value: CompSupplySpeedsCall) -> Self {
            Self::CompSupplySpeeds(value)
        }
    }
    impl ::core::convert::From<CompSupplyStateCall> for ComptrollerCalls {
        fn from(value: CompSupplyStateCall) -> Self {
            Self::CompSupplyState(value)
        }
    }
    impl ::core::convert::From<ComptrollerImplementationCall> for ComptrollerCalls {
        fn from(value: ComptrollerImplementationCall) -> Self {
            Self::ComptrollerImplementation(value)
        }
    }
    impl ::core::convert::From<EnterMarketsCall> for ComptrollerCalls {
        fn from(value: EnterMarketsCall) -> Self {
            Self::EnterMarkets(value)
        }
    }
    impl ::core::convert::From<ExitMarketCall> for ComptrollerCalls {
        fn from(value: ExitMarketCall) -> Self {
            Self::ExitMarket(value)
        }
    }
    impl ::core::convert::From<FixBadAccrualsCall> for ComptrollerCalls {
        fn from(value: FixBadAccrualsCall) -> Self {
            Self::FixBadAccruals(value)
        }
    }
    impl ::core::convert::From<GetAccountLiquidityCall> for ComptrollerCalls {
        fn from(value: GetAccountLiquidityCall) -> Self {
            Self::GetAccountLiquidity(value)
        }
    }
    impl ::core::convert::From<GetAllMarketsCall> for ComptrollerCalls {
        fn from(value: GetAllMarketsCall) -> Self {
            Self::GetAllMarkets(value)
        }
    }
    impl ::core::convert::From<GetAssetsInCall> for ComptrollerCalls {
        fn from(value: GetAssetsInCall) -> Self {
            Self::GetAssetsIn(value)
        }
    }
    impl ::core::convert::From<GetBlockNumberCall> for ComptrollerCalls {
        fn from(value: GetBlockNumberCall) -> Self {
            Self::GetBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetCompAddressCall> for ComptrollerCalls {
        fn from(value: GetCompAddressCall) -> Self {
            Self::GetCompAddress(value)
        }
    }
    impl ::core::convert::From<GetHypotheticalAccountLiquidityCall>
    for ComptrollerCalls {
        fn from(value: GetHypotheticalAccountLiquidityCall) -> Self {
            Self::GetHypotheticalAccountLiquidity(value)
        }
    }
    impl ::core::convert::From<IsComptrollerCall> for ComptrollerCalls {
        fn from(value: IsComptrollerCall) -> Self {
            Self::IsComptroller(value)
        }
    }
    impl ::core::convert::From<IsDeprecatedCall> for ComptrollerCalls {
        fn from(value: IsDeprecatedCall) -> Self {
            Self::IsDeprecated(value)
        }
    }
    impl ::core::convert::From<LastContributorBlockCall> for ComptrollerCalls {
        fn from(value: LastContributorBlockCall) -> Self {
            Self::LastContributorBlock(value)
        }
    }
    impl ::core::convert::From<LiquidateBorrowAllowedCall> for ComptrollerCalls {
        fn from(value: LiquidateBorrowAllowedCall) -> Self {
            Self::LiquidateBorrowAllowed(value)
        }
    }
    impl ::core::convert::From<LiquidateBorrowVerifyCall> for ComptrollerCalls {
        fn from(value: LiquidateBorrowVerifyCall) -> Self {
            Self::LiquidateBorrowVerify(value)
        }
    }
    impl ::core::convert::From<LiquidateCalculateSeizeTokensCall> for ComptrollerCalls {
        fn from(value: LiquidateCalculateSeizeTokensCall) -> Self {
            Self::LiquidateCalculateSeizeTokens(value)
        }
    }
    impl ::core::convert::From<LiquidationIncentiveMantissaCall> for ComptrollerCalls {
        fn from(value: LiquidationIncentiveMantissaCall) -> Self {
            Self::LiquidationIncentiveMantissa(value)
        }
    }
    impl ::core::convert::From<MarketsCall> for ComptrollerCalls {
        fn from(value: MarketsCall) -> Self {
            Self::Markets(value)
        }
    }
    impl ::core::convert::From<MaxAssetsCall> for ComptrollerCalls {
        fn from(value: MaxAssetsCall) -> Self {
            Self::MaxAssets(value)
        }
    }
    impl ::core::convert::From<MintAllowedCall> for ComptrollerCalls {
        fn from(value: MintAllowedCall) -> Self {
            Self::MintAllowed(value)
        }
    }
    impl ::core::convert::From<MintGuardianPausedCall> for ComptrollerCalls {
        fn from(value: MintGuardianPausedCall) -> Self {
            Self::MintGuardianPaused(value)
        }
    }
    impl ::core::convert::From<MintVerifyCall> for ComptrollerCalls {
        fn from(value: MintVerifyCall) -> Self {
            Self::MintVerify(value)
        }
    }
    impl ::core::convert::From<OracleCall> for ComptrollerCalls {
        fn from(value: OracleCall) -> Self {
            Self::Oracle(value)
        }
    }
    impl ::core::convert::From<PauseGuardianCall> for ComptrollerCalls {
        fn from(value: PauseGuardianCall) -> Self {
            Self::PauseGuardian(value)
        }
    }
    impl ::core::convert::From<PendingAdminCall> for ComptrollerCalls {
        fn from(value: PendingAdminCall) -> Self {
            Self::PendingAdmin(value)
        }
    }
    impl ::core::convert::From<PendingComptrollerImplementationCall>
    for ComptrollerCalls {
        fn from(value: PendingComptrollerImplementationCall) -> Self {
            Self::PendingComptrollerImplementation(value)
        }
    }
    impl ::core::convert::From<Proposal65FixExecutedCall> for ComptrollerCalls {
        fn from(value: Proposal65FixExecutedCall) -> Self {
            Self::Proposal65FixExecuted(value)
        }
    }
    impl ::core::convert::From<RedeemAllowedCall> for ComptrollerCalls {
        fn from(value: RedeemAllowedCall) -> Self {
            Self::RedeemAllowed(value)
        }
    }
    impl ::core::convert::From<RedeemVerifyCall> for ComptrollerCalls {
        fn from(value: RedeemVerifyCall) -> Self {
            Self::RedeemVerify(value)
        }
    }
    impl ::core::convert::From<RepayBorrowAllowedCall> for ComptrollerCalls {
        fn from(value: RepayBorrowAllowedCall) -> Self {
            Self::RepayBorrowAllowed(value)
        }
    }
    impl ::core::convert::From<RepayBorrowVerifyCall> for ComptrollerCalls {
        fn from(value: RepayBorrowVerifyCall) -> Self {
            Self::RepayBorrowVerify(value)
        }
    }
    impl ::core::convert::From<SeizeAllowedCall> for ComptrollerCalls {
        fn from(value: SeizeAllowedCall) -> Self {
            Self::SeizeAllowed(value)
        }
    }
    impl ::core::convert::From<SeizeGuardianPausedCall> for ComptrollerCalls {
        fn from(value: SeizeGuardianPausedCall) -> Self {
            Self::SeizeGuardianPaused(value)
        }
    }
    impl ::core::convert::From<SeizeVerifyCall> for ComptrollerCalls {
        fn from(value: SeizeVerifyCall) -> Self {
            Self::SeizeVerify(value)
        }
    }
    impl ::core::convert::From<TransferAllowedCall> for ComptrollerCalls {
        fn from(value: TransferAllowedCall) -> Self {
            Self::TransferAllowed(value)
        }
    }
    impl ::core::convert::From<TransferGuardianPausedCall> for ComptrollerCalls {
        fn from(value: TransferGuardianPausedCall) -> Self {
            Self::TransferGuardianPaused(value)
        }
    }
    impl ::core::convert::From<TransferVerifyCall> for ComptrollerCalls {
        fn from(value: TransferVerifyCall) -> Self {
            Self::TransferVerify(value)
        }
    }
    impl ::core::convert::From<UpdateContributorRewardsCall> for ComptrollerCalls {
        fn from(value: UpdateContributorRewardsCall) -> Self {
            Self::UpdateContributorRewards(value)
        }
    }
    ///Container type for all return fields from the `_borrowGuardianPaused` function with signature `_borrowGuardianPaused()` and selector `0xe6653f3d`
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
    pub struct _BorrowGuardianPausedReturn(pub bool);
    ///Container type for all return fields from the `_mintGuardianPaused` function with signature `_mintGuardianPaused()` and selector `0x3c94786f`
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
    pub struct _MintGuardianPausedReturn(pub bool);
    ///Container type for all return fields from the `_setBorrowPaused` function with signature `_setBorrowPaused(address,bool)` and selector `0x18c882a5`
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
    pub struct SetBorrowPausedReturn(pub bool);
    ///Container type for all return fields from the `_setCloseFactor` function with signature `_setCloseFactor(uint256)` and selector `0x317b0b77`
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
    pub struct SetCloseFactorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `_setCollateralFactor` function with signature `_setCollateralFactor(address,uint256)` and selector `0xe4028eee`
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
    pub struct SetCollateralFactorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `_setLiquidationIncentive` function with signature `_setLiquidationIncentive(uint256)` and selector `0x4fd42e17`
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
    pub struct SetLiquidationIncentiveReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `_setMintPaused` function with signature `_setMintPaused(address,bool)` and selector `0x3bcf7ec1`
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
    pub struct SetMintPausedReturn(pub bool);
    ///Container type for all return fields from the `_setPauseGuardian` function with signature `_setPauseGuardian(address)` and selector `0x5f5af1aa`
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
    pub struct SetPauseGuardianReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `_setPriceOracle` function with signature `_setPriceOracle(address)` and selector `0x55ee1fe1`
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
    pub struct SetPriceOracleReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `_setSeizePaused` function with signature `_setSeizePaused(bool)` and selector `0x2d70db78`
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
    pub struct SetSeizePausedReturn(pub bool);
    ///Container type for all return fields from the `_setTransferPaused` function with signature `_setTransferPaused(bool)` and selector `0x8ebf6364`
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
    pub struct SetTransferPausedReturn(pub bool);
    ///Container type for all return fields from the `_supportMarket` function with signature `_supportMarket(address)` and selector `0xa76b3fda`
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
    pub struct SupportMarketReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `accountAssets` function with signature `accountAssets(address,uint256)` and selector `0xdce15449`
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
    pub struct AccountAssetsReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `allMarkets` function with signature `allMarkets(uint256)` and selector `0x52d84d1e`
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
    pub struct AllMarketsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `borrowAllowed` function with signature `borrowAllowed(address,address,uint256)` and selector `0xda3d454c`
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
    pub struct BorrowAllowedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `borrowCapGuardian` function with signature `borrowCapGuardian()` and selector `0x21af4569`
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
    pub struct BorrowCapGuardianReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `borrowCaps` function with signature `borrowCaps(address)` and selector `0x4a584432`
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
    pub struct BorrowCapsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `borrowGuardianPaused` function with signature `borrowGuardianPaused(address)` and selector `0x6d154ea5`
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
    pub struct BorrowGuardianPausedReturn(pub bool);
    ///Container type for all return fields from the `checkMembership` function with signature `checkMembership(address,address)` and selector `0x929fe9a1`
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
    pub struct CheckMembershipReturn(pub bool);
    ///Container type for all return fields from the `closeFactorMantissa` function with signature `closeFactorMantissa()` and selector `0xe8755446`
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
    pub struct CloseFactorMantissaReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `compAccrued` function with signature `compAccrued(address)` and selector `0xcc7ebdc4`
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
    pub struct CompAccruedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `compBorrowSpeeds` function with signature `compBorrowSpeeds(address)` and selector `0xf4a433c0`
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
    pub struct CompBorrowSpeedsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `compBorrowState` function with signature `compBorrowState(address)` and selector `0x8c57804e`
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
    pub struct CompBorrowStateReturn {
        pub index: ::ethers::core::types::U256,
        pub block: u32,
    }
    ///Container type for all return fields from the `compBorrowerIndex` function with signature `compBorrowerIndex(address,address)` and selector `0xca0af043`
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
    pub struct CompBorrowerIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `compContributorSpeeds` function with signature `compContributorSpeeds(address)` and selector `0x986ab838`
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
    pub struct CompContributorSpeedsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `compInitialIndex` function with signature `compInitialIndex()` and selector `0xa7f0e231`
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
    pub struct CompInitialIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `compRate` function with signature `compRate()` and selector `0xaa900754`
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
    pub struct CompRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `compReceivable` function with signature `compReceivable(address)` and selector `0x85b7beb8`
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
    pub struct CompReceivableReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `compSpeeds` function with signature `compSpeeds(address)` and selector `0x1d7b33d7`
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
    pub struct CompSpeedsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `compSupplierIndex` function with signature `compSupplierIndex(address,address)` and selector `0xb21be7fd`
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
    pub struct CompSupplierIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `compSupplySpeeds` function with signature `compSupplySpeeds(address)` and selector `0x6aa875b5`
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
    pub struct CompSupplySpeedsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `compSupplyState` function with signature `compSupplyState(address)` and selector `0x6b79c38d`
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
    pub struct CompSupplyStateReturn {
        pub index: ::ethers::core::types::U256,
        pub block: u32,
    }
    ///Container type for all return fields from the `comptrollerImplementation` function with signature `comptrollerImplementation()` and selector `0xbb82aa5e`
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
    pub struct ComptrollerImplementationReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `enterMarkets` function with signature `enterMarkets(address[])` and selector `0xc2998238`
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
    pub struct EnterMarketsReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    ///Container type for all return fields from the `exitMarket` function with signature `exitMarket(address)` and selector `0xede4edd0`
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
    pub struct ExitMarketReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getAccountLiquidity` function with signature `getAccountLiquidity(address)` and selector `0x5ec88c79`
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
    pub struct GetAccountLiquidityReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getAllMarkets` function with signature `getAllMarkets()` and selector `0xb0772d0b`
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
    pub struct GetAllMarketsReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getAssetsIn` function with signature `getAssetsIn(address)` and selector `0xabfceffc`
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
    pub struct GetAssetsInReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getBlockNumber` function with signature `getBlockNumber()` and selector `0x42cbb15c`
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
    pub struct GetBlockNumberReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getCompAddress` function with signature `getCompAddress()` and selector `0x9d1b5a0a`
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
    pub struct GetCompAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getHypotheticalAccountLiquidity` function with signature `getHypotheticalAccountLiquidity(address,address,uint256,uint256)` and selector `0x4e79238f`
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
    pub struct GetHypotheticalAccountLiquidityReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `isComptroller` function with signature `isComptroller()` and selector `0x007e3dd2`
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
    pub struct IsComptrollerReturn(pub bool);
    ///Container type for all return fields from the `isDeprecated` function with signature `isDeprecated(address)` and selector `0x94543c15`
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
    pub struct IsDeprecatedReturn(pub bool);
    ///Container type for all return fields from the `lastContributorBlock` function with signature `lastContributorBlock(address)` and selector `0xbea6b8b8`
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
    pub struct LastContributorBlockReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `liquidateBorrowAllowed` function with signature `liquidateBorrowAllowed(address,address,address,address,uint256)` and selector `0x5fc7e71e`
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
    pub struct LiquidateBorrowAllowedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `liquidateCalculateSeizeTokens` function with signature `liquidateCalculateSeizeTokens(address,address,uint256)` and selector `0xc488847b`
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
    pub struct LiquidateCalculateSeizeTokensReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `liquidationIncentiveMantissa` function with signature `liquidationIncentiveMantissa()` and selector `0x4ada90af`
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
    pub struct LiquidationIncentiveMantissaReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `markets` function with signature `markets(address)` and selector `0x8e8f294b`
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
    pub struct MarketsReturn {
        pub is_listed: bool,
        pub collateral_factor_mantissa: ::ethers::core::types::U256,
        pub is_comped: bool,
    }
    ///Container type for all return fields from the `maxAssets` function with signature `maxAssets()` and selector `0x94b2294b`
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
    pub struct MaxAssetsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mintAllowed` function with signature `mintAllowed(address,address,uint256)` and selector `0x4ef4c3e1`
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
    pub struct MintAllowedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mintGuardianPaused` function with signature `mintGuardianPaused(address)` and selector `0x731f0c2b`
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
    pub struct MintGuardianPausedReturn(pub bool);
    ///Container type for all return fields from the `oracle` function with signature `oracle()` and selector `0x7dc0d1d0`
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
    pub struct OracleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pauseGuardian` function with signature `pauseGuardian()` and selector `0x24a3d622`
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
    pub struct PauseGuardianReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pendingAdmin` function with signature `pendingAdmin()` and selector `0x26782247`
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
        Hash
    )]
    pub struct PendingComptrollerImplementationReturn(
        pub ::ethers::core::types::Address,
    );
    ///Container type for all return fields from the `proposal65FixExecuted` function with signature `proposal65FixExecuted()` and selector `0xf00a7a92`
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
    pub struct Proposal65FixExecutedReturn(pub bool);
    ///Container type for all return fields from the `redeemAllowed` function with signature `redeemAllowed(address,address,uint256)` and selector `0xeabe7d91`
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
    pub struct RedeemAllowedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `repayBorrowAllowed` function with signature `repayBorrowAllowed(address,address,address,uint256)` and selector `0x24008a62`
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
    pub struct RepayBorrowAllowedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `seizeAllowed` function with signature `seizeAllowed(address,address,address,address,uint256)` and selector `0xd02f7351`
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
    pub struct SeizeAllowedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `seizeGuardianPaused` function with signature `seizeGuardianPaused()` and selector `0xac0b0bb7`
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
    pub struct SeizeGuardianPausedReturn(pub bool);
    ///Container type for all return fields from the `transferAllowed` function with signature `transferAllowed(address,address,address,uint256)` and selector `0xbdcdc258`
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
    pub struct TransferAllowedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transferGuardianPaused` function with signature `transferGuardianPaused()` and selector `0x87f76303`
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
    pub struct TransferGuardianPausedReturn(pub bool);
}
