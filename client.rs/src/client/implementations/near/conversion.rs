use hapi_core_near::{
    AddressView as NearAddress, AssetView as NearAsset, Case as NearCase, Reporter as NearReporter,
};
use uuid::Uuid;

use crate::client::{
    entities::{address::Address, asset::Asset, case::Case, reporter::Reporter},
    result::{ClientError, Result},
};

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

impl TryFrom<NearCase> for Case {
    type Error = ClientError;

    fn try_from(case: NearCase) -> Result<Self> {
        Ok(Case {
            id: Uuid::from_u128(case.id.0),
            status: (case.status as u8).try_into()?,
            name: case.name.to_string(),
            url: case.url.to_string(),
            reporter_id: Uuid::from_u128(case.reporter_id.0),
        })
    }
}

impl TryFrom<NearAddress> for Address {
    type Error = ClientError;

    fn try_from(address: NearAddress) -> Result<Self> {
        Ok(Address {
            address: address.address.to_string(),
            category: (address.category as u8).try_into()?,
            risk: address.risk_score,
            case_id: Uuid::from_u128(address.case_id.0),
            reporter_id: Uuid::from_u128(address.reporter_id.0),
        })
    }
}

impl TryFrom<NearAsset> for Asset {
    type Error = ClientError;

    fn try_from(asset: NearAsset) -> Result<Self> {
        Ok(Asset {
            address: asset.address.to_string(),
            asset_id: asset.id.0.into(),
            category: (asset.category as u8).try_into()?,
            risk: asset.risk_score,
            case_id: Uuid::from_u128(asset.case_id.0),
            reporter_id: Uuid::from_u128(asset.reporter_id.0),
        })
    }
}
