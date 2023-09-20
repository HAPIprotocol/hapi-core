mod client;
mod conversion;
mod token;

pub use client::{
    HapiCoreNear, DELAY_AFTER_TX_EXECUTION, PERIOD_CHECK_TX_STATUS, TRANSACTION_TIMEOUT,
};
pub use token::TokenContractNear;

pub const GAS_FOR_TX: u64 = 50_000_000_000_000; // 50 TeraGas
