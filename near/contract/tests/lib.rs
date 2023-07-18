pub mod context;
pub mod utils;
pub use context::*;
pub use utils::*;

mod configuration;
mod errors;

pub const SHOW_LOGS: bool = false;
pub const SHOW_DEFAULT_OUTPUT: bool = false;

pub const CONTRACT: &[u8] = include_bytes!("../../res/hapi_core_near.wasm");

pub const INITIAL_USER_BALANCE: u128 = 1000;
pub const INITIAL_NEAR_USER_BALANCE: u128 = 10000000000000000000000000; // 10 near
