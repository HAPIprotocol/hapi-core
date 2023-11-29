use {
    hapi_core::client::entities::{
        case::CaseStatus as CaseStatusPayload,
        category::Category as CategoryPayload,
        reporter::{ReporterRole as ReporterRolePayload, ReporterStatus as ReporterStatusPayload},
    },
    sea_orm::entity::prelude::*,
};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "category")]
pub enum Category {
    #[sea_orm(string_value = "atm")]
    ATM,
    #[sea_orm(string_value = "child_abuse")]
    ChildAbuse,
    #[sea_orm(string_value = "counterfeit")]
    Counterfeit,
    #[sea_orm(string_value = "darknet_service")]
    DarknetService,
    #[sea_orm(string_value = "de_fi")]
    DeFi,
    #[sea_orm(string_value = "exchange")]
    Exchange,
    #[sea_orm(string_value = "gambling")]
    Gambling,
    #[sea_orm(string_value = "hacker")]
    Hacker,
    #[sea_orm(string_value = "high_risk_jurisdiction")]
    HighRiskJurisdiction,
    #[sea_orm(string_value = "illicit_organization")]
    IllicitOrganization,
    #[sea_orm(string_value = "merchant_service")]
    MerchantService,
    #[sea_orm(string_value = "mining_pool")]
    MiningPool,
    #[sea_orm(string_value = "mixer")]
    Mixer,
    #[sea_orm(string_value = "none")]
    None,
    #[sea_orm(string_value = "otc_broker")]
    OTCBroker,
    #[sea_orm(string_value = "ransomware")]
    Ransomware,
    #[sea_orm(string_value = "sanctions")]
    Sanctions,
    #[sea_orm(string_value = "scam")]
    Scam,
    #[sea_orm(string_value = "terrorist_financing")]
    TerroristFinancing,
    #[sea_orm(string_value = "theft")]
    Theft,
    #[sea_orm(string_value = "wallet_service")]
    WalletService,
}

impl From<CategoryPayload> for Category {
    fn from(payload: CategoryPayload) -> Self {
        match payload {
            CategoryPayload::None => Category::None,
            CategoryPayload::WalletService => Category::WalletService,
            CategoryPayload::MerchantService => Category::MerchantService,
            CategoryPayload::MiningPool => Category::MiningPool,
            CategoryPayload::Exchange => Category::Exchange,
            CategoryPayload::DeFi => Category::DeFi,
            CategoryPayload::OTCBroker => Category::OTCBroker,
            CategoryPayload::ATM => Category::ATM,
            CategoryPayload::Gambling => Category::Gambling,
            CategoryPayload::IllicitOrganization => Category::IllicitOrganization,
            CategoryPayload::Mixer => Category::Mixer,
            CategoryPayload::DarknetService => Category::DarknetService,
            CategoryPayload::Scam => Category::Scam,
            CategoryPayload::Ransomware => Category::Ransomware,
            CategoryPayload::Theft => Category::Theft,
            CategoryPayload::Counterfeit => Category::Counterfeit,
            CategoryPayload::TerroristFinancing => Category::TerroristFinancing,
            CategoryPayload::Sanctions => Category::Sanctions,
            CategoryPayload::ChildAbuse => Category::ChildAbuse,
            CategoryPayload::Hacker => Category::Hacker,
            CategoryPayload::HighRiskJurisdiction => Category::HighRiskJurisdiction,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "reporter_role")]
pub enum ReporterRole {
    #[sea_orm(string_value = "authority")]
    Authority,
    #[sea_orm(string_value = "publisher")]
    Publisher,
    #[sea_orm(string_value = "tracer")]
    Tracer,
    #[sea_orm(string_value = "validator")]
    Validator,
}

impl From<ReporterRolePayload> for ReporterRole {
    fn from(payload: ReporterRolePayload) -> Self {
        match payload {
            ReporterRolePayload::Validator => ReporterRole::Validator,
            ReporterRolePayload::Tracer => ReporterRole::Tracer,
            ReporterRolePayload::Publisher => ReporterRole::Publisher,
            ReporterRolePayload::Authority => ReporterRole::Authority,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "reporter_status")]
pub enum ReporterStatus {
    #[sea_orm(string_value = "active")]
    Active,
    #[sea_orm(string_value = "inactive")]
    Inactive,
    #[sea_orm(string_value = "unstaking")]
    Unstaking,
}

impl From<ReporterStatusPayload> for ReporterStatus {
    fn from(payload: ReporterStatusPayload) -> Self {
        match payload {
            ReporterStatusPayload::Inactive => ReporterStatus::Inactive,
            ReporterStatusPayload::Active => ReporterStatus::Active,
            ReporterStatusPayload::Unstaking => ReporterStatus::Unstaking,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "case_status")]
pub enum CaseStatus {
    #[sea_orm(string_value = "closed")]
    Closed,
    #[sea_orm(string_value = "open")]
    Open,
}

impl From<CaseStatusPayload> for CaseStatus {
    fn from(payload: CaseStatusPayload) -> Self {
        match payload {
            CaseStatusPayload::Closed => CaseStatus::Closed,
            CaseStatusPayload::Open => CaseStatus::Open,
        }
    }
}
