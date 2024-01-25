use crate::client::{
    configuration::{RewardConfiguration, StakeConfiguration},
    entities::{
        address::Address,
        asset::{Asset, AssetId},
        case::{Case, CaseStatus},
        category::Category,
        reporter::{Reporter, ReporterRole, ReporterStatus},
    },
    result::{ClientError, Result},
};

use {
    hapi_core_solana::{
        bytes_to_string, Address as SolanaAddress, Asset as SolanaAsset, Case as SolanaCase,
        CaseStatus as SolanaCaseStatus, Category as SolanaCategory, Network as SolanaNetwork,
        Reporter as SolanaReporter, ReporterRole as SolanaReporterRole,
        ReporterStatus as SolanaReporterStatus, RewardConfiguration as SolanaRewardConfiguration,
        StakeConfiguration as SolanaStakeConfiguration,
    },
    std::str::FromStr,
    uuid::Uuid,
};

impl From<StakeConfiguration> for SolanaStakeConfiguration {
    fn from(configuration: StakeConfiguration) -> Self {
        Self {
            unlock_duration: configuration.unlock_duration,
            validator_stake: configuration.validator_stake.into(),
            tracer_stake: configuration.tracer_stake.into(),
            publisher_stake: configuration.publisher_stake.into(),
            authority_stake: configuration.authority_stake.into(),
            // TODO: add appraiser stake
            appraiser_stake: 0u64,
        }
    }
}

impl From<RewardConfiguration> for SolanaRewardConfiguration {
    fn from(configuration: RewardConfiguration) -> Self {
        Self {
            address_confirmation_reward: configuration.address_confirmation_reward.into(),
            address_tracer_reward: configuration.address_tracer_reward.into(),
            asset_confirmation_reward: configuration.asset_confirmation_reward.into(),
            asset_tracer_reward: configuration.asset_tracer_reward.into(),
        }
    }
}

impl From<ReporterRole> for SolanaReporterRole {
    fn from(value: ReporterRole) -> Self {
        match value {
            ReporterRole::Validator => SolanaReporterRole::Validator,
            ReporterRole::Tracer => SolanaReporterRole::Tracer,
            ReporterRole::Publisher => SolanaReporterRole::Publisher,
            ReporterRole::Authority => SolanaReporterRole::Authority,
        }
    }
}

impl From<ReporterStatus> for SolanaReporterStatus {
    fn from(value: ReporterStatus) -> Self {
        match value {
            ReporterStatus::Inactive => SolanaReporterStatus::Inactive,
            ReporterStatus::Active => SolanaReporterStatus::Active,
            ReporterStatus::Unstaking => SolanaReporterStatus::Unstaking,
        }
    }
}

impl From<CaseStatus> for SolanaCaseStatus {
    fn from(value: CaseStatus) -> Self {
        match value {
            CaseStatus::Closed => SolanaCaseStatus::Closed,
            CaseStatus::Open => SolanaCaseStatus::Open,
        }
    }
}

impl From<Category> for SolanaCategory {
    fn from(value: Category) -> Self {
        match value {
            Category::None => SolanaCategory::None,
            Category::WalletService => SolanaCategory::WalletService,
            Category::MerchantService => SolanaCategory::MerchantService,
            Category::MiningPool => SolanaCategory::MiningPool,
            Category::Exchange => SolanaCategory::Exchange,
            Category::DeFi => SolanaCategory::DeFi,
            Category::OTCBroker => SolanaCategory::OTCBroker,
            Category::ATM => SolanaCategory::ATM,
            Category::Gambling => SolanaCategory::Gambling,
            Category::IllicitOrganization => SolanaCategory::IllicitOrganization,
            Category::Mixer => SolanaCategory::Mixer,
            Category::DarknetService => SolanaCategory::DarknetService,
            Category::Scam => SolanaCategory::Scam,
            Category::Ransomware => SolanaCategory::Ransomware,
            Category::Theft => SolanaCategory::Theft,
            Category::Counterfeit => SolanaCategory::Counterfeit,
            Category::TerroristFinancing => SolanaCategory::TerroristFinancing,
            Category::Sanctions => SolanaCategory::Sanctions,
            Category::ChildAbuse => SolanaCategory::ChildAbuse,
            Category::Hacker => SolanaCategory::Hacker,
            Category::HighRiskJurisdiction => SolanaCategory::HighRiskJurisdiction,
        }
    }
}

impl TryFrom<SolanaNetwork> for StakeConfiguration {
    type Error = ClientError;

    fn try_from(network: SolanaNetwork) -> Result<Self> {
        Ok(StakeConfiguration {
            token: network.stake_mint.to_string(),
            unlock_duration: network.stake_configuration.unlock_duration,
            validator_stake: network.stake_configuration.validator_stake.into(),
            tracer_stake: network.stake_configuration.tracer_stake.into(),
            publisher_stake: network.stake_configuration.publisher_stake.into(),
            authority_stake: network.stake_configuration.authority_stake.into(),
        })
    }
}

impl TryFrom<SolanaNetwork> for RewardConfiguration {
    type Error = ClientError;

    fn try_from(network: SolanaNetwork) -> Result<Self> {
        Ok(RewardConfiguration {
            token: network.reward_mint.to_string(),
            address_confirmation_reward: network
                .reward_configuration
                .address_confirmation_reward
                .into(),
            address_tracer_reward: network.reward_configuration.address_tracer_reward.into(),
            asset_confirmation_reward: network
                .reward_configuration
                .asset_confirmation_reward
                .into(),
            asset_tracer_reward: network.reward_configuration.asset_tracer_reward.into(),
        })
    }
}

impl TryFrom<SolanaReporter> for Reporter {
    type Error = ClientError;

    fn try_from(reporter: SolanaReporter) -> Result<Self> {
        Ok(Reporter {
            id: Uuid::from_u128(reporter.id),
            account: reporter.account.to_string(),
            role: (reporter.role as u8).try_into()?,
            status: (reporter.status as u8).try_into()?,
            name: reporter.name.to_string(),
            url: reporter.url.to_string(),
            stake: reporter.stake.into(),
            unlock_timestamp: reporter.unlock_timestamp,
        })
    }
}

impl TryFrom<SolanaCase> for Case {
    type Error = ClientError;

    fn try_from(case: SolanaCase) -> Result<Self> {
        Ok(Case {
            id: Uuid::from_u128(case.id),
            name: case.name.to_string(),
            url: case.url.to_string(),
            status: (case.status as u8).try_into()?,
            reporter_id: Uuid::from_u128(case.reporter_id),
        })
    }
}

impl TryFrom<SolanaAddress> for Address {
    type Error = ClientError;

    fn try_from(addr: SolanaAddress) -> Result<Self> {
        Ok(Address {
            address: remove_zeroes(&addr.address)?,
            case_id: Uuid::from_u128(addr.case_id),
            reporter_id: Uuid::from_u128(addr.reporter_id),
            risk: addr.risk_score,
            category: (addr.category as u8).try_into()?,
            confirmations: addr.confirmations,
        })
    }
}

impl TryFrom<SolanaAsset> for Asset {
    type Error = ClientError;

    fn try_from(asset: SolanaAsset) -> Result<Self> {
        let asset_id = AssetId::from_str(
            &bytes_to_string(&asset.id)
                .map_err(|e| ClientError::AssetIdParseError(format!("invalid-bytes {e}")))?,
        )
        .map_err(|e| ClientError::AssetIdParseError(format!("invalid-asset-id {e}")))?;

        Ok(Asset {
            address: remove_zeroes(&asset.address)?,
            asset_id,
            case_id: Uuid::from_u128(asset.case_id),
            reporter_id: Uuid::from_u128(asset.reporter_id),
            risk: asset.risk_score,
            category: (asset.category as u8).try_into()?,
            confirmations: asset.confirmations,
        })
    }
}

fn remove_zeroes(bytes: &[u8]) -> Result<String> {
    let null_index = bytes
        .iter()
        .position(|&ch| ch == b'\0')
        .unwrap_or(bytes.len());

    String::from_utf8(bytes[0..null_index].to_vec())
        .map_err(|e| ClientError::SolanaAddressParseError(e.to_string()))
}
