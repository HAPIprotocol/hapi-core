use near_sdk::Timestamp;
use std::ops::Mul;
use workspaces::{network::Sandbox, Account, Contract, Worker};

use crate::INITIAL_NEAR_USER_BALANCE;

const NS: u64 = 1_000_000_000;
pub const ONE_TGAS: u64 = 1_000_000_000_000;

pub trait U128Extension {
    fn to_decimals(self, decimals: u8) -> u128;
}

impl U128Extension for u128 {
    fn to_decimals(self, decimals: u8) -> u128 {
        self.mul(10_u128.pow(decimals.into()))
    }
}

pub trait GasExtension {
    fn to_tgas(self) -> u64;
}

impl GasExtension for u64 {
    fn to_tgas(self) -> u64 {
        self * ONE_TGAS
    }
}

pub trait TimestampExtension {
    fn sec_to_ns(self) -> Timestamp;
    fn ns_to_sec(self) -> Timestamp;
    fn minutes_to_sec(self) -> Timestamp;
    fn add_minutes(self, minutes: u64) -> Timestamp;
}

impl TimestampExtension for Timestamp {
    fn sec_to_ns(self) -> Timestamp {
        self * NS
    }

    fn ns_to_sec(self) -> Timestamp {
        self / NS
    }

    fn minutes_to_sec(self) -> Timestamp {
        self.mul(60)
    }

    fn add_minutes(self, minutes: u64) -> Timestamp {
        std::ops::Add::add(self, minutes.minutes_to_sec().sec_to_ns())
    }
}

pub async fn create_account(worker: &Worker<Sandbox>, account_prefix: &str) -> Account {
    worker
        .root_account()
        .unwrap()
        .create_subaccount(account_prefix)
        .initial_balance(INITIAL_NEAR_USER_BALANCE)
        .transact()
        .await
        .unwrap()
        .result
}

pub async fn deploy_contract(
    worker: &Worker<Sandbox>,
    account_prefix: &str,
    wasm: &[u8],
) -> Contract {
    create_account(worker, account_prefix)
        .await
        .deploy(wasm)
        .await
        .unwrap()
        .result
}
