use near_sdk::Timestamp;

const NS: u64 = 1_000_000_000;

pub trait TimestampExtension {
    fn to_sec(&self) -> u64;
}

impl TimestampExtension for Timestamp {
    fn to_sec(&self) -> u64 {
        self / NS
    }
}
