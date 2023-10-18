use near_sdk::{json_types::U128, Timestamp};

pub type UUID = U128;

const NS: u64 = 1_000_000_000;

pub const MAX_NAME_LENGTH: usize = 64;

pub trait TimestampExtension {
    fn to_sec(&self) -> u64;
}

impl TimestampExtension for Timestamp {
    fn to_sec(&self) -> u64 {
        self / NS
    }
}
