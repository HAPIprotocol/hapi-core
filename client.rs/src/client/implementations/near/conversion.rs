use hapi_core_near::{
    AddressView as NearAddress, AssetView as NearAsset, Case as NearCase,
    CaseStatus as NearCaseStatus, Category as NearCategory, Reporter as NearReporter,
    ReporterStatus as NearReporterStatus, Role as NearReporterRole,
};
use near_sdk::{json_types::U64, AccountId};
use uuid::Uuid;

use crate::client::{
    entities::{
        address::Address,
        asset::Asset,
        case::{Case, CaseStatus},
        category::Category,
        reporter::{Reporter, ReporterRole, ReporterStatus},
    },
    result::{ClientError, Result},
};

impl From<ReporterRole> for NearReporterRole {
    fn from(value: ReporterRole) -> Self {
        match value {
            ReporterRole::Validator => NearReporterRole::Validator,
            ReporterRole::Tracer => NearReporterRole::Tracer,
            ReporterRole::Publisher => NearReporterRole::Publisher,
            ReporterRole::Authority => NearReporterRole::Authority,
        }
    }
}

impl From<ReporterStatus> for NearReporterStatus {
    fn from(value: ReporterStatus) -> Self {
        match value {
            ReporterStatus::Active => NearReporterStatus::Active,
            ReporterStatus::Inactive => NearReporterStatus::Inactive,
            ReporterStatus::Unstaking => NearReporterStatus::Unstaking,
        }
    }
}

impl From<CaseStatus> for NearCaseStatus {
    fn from(value: CaseStatus) -> Self {
        match value {
            CaseStatus::Closed => NearCaseStatus::Closed,
            CaseStatus::Open => NearCaseStatus::Open,
        }
    }
}

impl From<Category> for NearCategory {
    fn from(category: Category) -> Self {
        match category {
            Category::None => NearCategory::None,
            Category::WalletService => NearCategory::WalletService,
            Category::MerchantService => NearCategory::MerchantService,
            Category::MiningPool => NearCategory::MiningPool,
            Category::Exchange => NearCategory::Exchange,
            Category::DeFi => NearCategory::DeFi,
            Category::OTCBroker => NearCategory::OTCBroker,
            Category::ATM => NearCategory::ATM,
            Category::Gambling => NearCategory::Gambling,
            Category::IllicitOrganization => NearCategory::IllicitOrganization,
            Category::Mixer => NearCategory::Mixer,
            Category::DarknetService => NearCategory::DarknetService,
            Category::Scam => NearCategory::Scam,
            Category::Ransomware => NearCategory::Ransomware,
            Category::Theft => NearCategory::Theft,
            Category::Counterfeit => NearCategory::Counterfeit,
            Category::TerroristFinancing => NearCategory::TerroristFinancing,
            Category::Sanctions => NearCategory::Sanctions,
            Category::ChildAbuse => NearCategory::ChildAbuse,
            Category::Hacker => NearCategory::Hacker,
            Category::HighRiskJurisdiction => NearCategory::HighRiskJurisdiction,
        }
    }
}

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

impl TryFrom<Reporter> for NearReporter {
    type Error = ClientError;

    fn try_from(reporter: Reporter) -> Result<Self> {
        Ok(NearReporter {
            id: reporter.id.as_u128().into(),
            account_id: AccountId::try_from(reporter.account)?,
            role: reporter.role.into(),
            status: reporter.status.into(),
            name: reporter.name,
            url: reporter.url,
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

impl TryFrom<Case> for NearCase {
    type Error = ClientError;

    fn try_from(case: Case) -> Result<Self> {
        Ok(NearCase {
            id: case.id.as_u128().into(),
            status: case.status.into(),
            name: case.name,
            url: case.url,
            reporter_id: case.reporter_id.as_u128().into(),
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
            confirmations: address.confirmations_count,
        })
    }
}

impl TryFrom<Address> for NearAddress {
    type Error = ClientError;

    fn try_from(address: Address) -> Result<Self> {
        Ok(NearAddress {
            address: AccountId::try_from(address.address)?,
            category: address.category.into(),
            risk_score: address.risk,
            case_id: address.case_id.as_u128().into(),
            reporter_id: address.reporter_id.as_u128().into(),
            confirmations_count: 0, // TODO: add confirmations count
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
            confirmations: asset.confirmations_count,
        })
    }
}

impl TryFrom<Asset> for NearAsset {
    type Error = ClientError;

    fn try_from(asset: Asset) -> Result<Self> {
        Ok(NearAsset {
            address: AccountId::try_from(asset.address)?,
            id: U64(asset.asset_id.into()),
            category: asset.category.into(),
            risk_score: asset.risk,
            case_id: asset.case_id.as_u128().into(),
            reporter_id: asset.reporter_id.as_u128().into(),
            confirmations_count: 0, // TODO: add confirmations count
        })
    }
}
