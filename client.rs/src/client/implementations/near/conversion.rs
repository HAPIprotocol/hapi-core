
use hapi_core_near::Reporter as NearReporter;
use crate::client::reporter::Reporter;
use crate::client::result::ClientError;
use crate::client::result::Result;

use uuid::Uuid;

impl TryFrom<NearReporter> for Reporter {
    type Error = ClientError;

    fn try_from(reporter: NearReporter) -> Result<Self> {
        Ok(Reporter {
            id: Uuid::from_u128(reporter.id.0),
            account: reporter.account_id.to_string(),
            role: (reporter.role as u8).try_into()?,
            status: (reporter.status as u8).try_into()?,
            name: reporter.name.to_string(),
            url: reporter.url.to_string(),
            stake: reporter.stake.into(),
            unlock_timestamp: reporter.unlock_timestamp,
        })
    }
}