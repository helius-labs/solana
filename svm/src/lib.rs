#![cfg_attr(RUSTC_WITH_SPECIALIZATION, feature(min_specialization))]
#![allow(clippy::arithmetic_side_effects)]

pub mod account_loader;
pub mod account_overrides;
pub mod account_rent_state;
<<<<<<< HEAD
=======
pub mod message_processor;
pub mod nonce_info;
pub mod program_loader;
pub mod runtime_config;
>>>>>>> 6631e5f4d9605821afb0e021fbd0c24e6fc46d20
pub mod transaction_account_state_info;
pub mod transaction_error_metrics;
pub mod transaction_processing_callback;
pub mod transaction_processor;
pub mod transaction_results;

#[macro_use]
extern crate solana_metrics;

#[cfg_attr(feature = "frozen-abi", macro_use)]
#[cfg(feature = "frozen-abi")]
extern crate solana_frozen_abi_macro;
