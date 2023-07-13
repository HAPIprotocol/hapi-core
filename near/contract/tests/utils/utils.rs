use near_sdk::Timestamp;
use std::ops::Mul;

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
