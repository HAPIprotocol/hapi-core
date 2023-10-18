mod address;
mod asset;
mod case;
mod configuration;
pub mod context;
pub mod errors;
mod reporter;
mod utils;
pub use configuration::*;
pub use errors::*;
pub use utils::*;

pub const SHOW_LOGS: bool = false;
pub const SHOW_DEFAULT_OUTPUT: bool = false;

pub const CONTRACT: &[u8] = include_bytes!("../../res/hapi_core_near.wasm");

pub const INITIAL_USER_BALANCE: u128 = 1000;
pub const INITIAL_NEAR_USER_BALANCE: u128 = 10000000000000000000000000; // 10 near
