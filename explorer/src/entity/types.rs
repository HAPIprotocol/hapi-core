use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "case_status")]
pub enum CaseStatus {
    #[sea_orm(string_value = "closed")]
    Closed,
    #[sea_orm(string_value = "open")]
    Open,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "category")]
pub enum Category {
    #[sea_orm(string_value = "atm")]
    Atm,
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
    OtcBroker,
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
