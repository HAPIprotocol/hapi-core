use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use crate::reporter::Reporter;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum VReporter {
    Current(Reporter),
}

impl From<VReporter> for Reporter {
    fn from(v: VReporter) -> Self {
        match v {
            VReporter::Current(reporter) => reporter,
        }
    }
}

impl From<Reporter> for VReporter {
    fn from(reporter: Reporter) -> Self {
        VReporter::Current(reporter)
    }
}
