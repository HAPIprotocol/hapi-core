use crate::{case::CaseId, reporter::ReporterId};
use near_sdk::{
    serde::{Deserialize, Serialize},
    AccountId,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Address {
    pub address: AccountId,
    pub category: Category,
    pub risk_score: RiskScore,
    pub case_id: CaseId,
    pub reporter_id: ReporterId,
    pub confirmations_count: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Category {
    // HAPI returns 'None' when address wasn't find in database
    None,
    // Wallet service - custodial or mixed wallets
    WalletService,
    // Merchant service
    MerchantService,
    // Mining pool
    MiningPool,
    // Exchange
    Exchange,
    // DeFi application
    DeFi,
    // OTC Broker
    OTCBroker,
    // Cryptocurrency ATM
    ATM,
    // Gambling
    Gambling,
    // Illicit organization
    IllicitOrganization,
    // Mixer
    Mixer,
    // Darknet market or service
    DarknetService,
    // Scam
    Scam,
    // Ransomware
    Ransomware,
    // Theft - stolen funds
    Theft,
    // Counterfeit - fake assets
    Counterfeit,
    // Terrorist financing
    TerroristFinancing,
    // Sanctions
    Sanctions,
    // Child abuse and porn materials
    ChildAbuse,
}

pub type RiskScore = u8;
