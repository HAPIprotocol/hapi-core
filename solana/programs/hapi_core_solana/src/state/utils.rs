use crate::error::ErrorCode;
use anchor_lang::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum Category {
    // Tier 0
    /// None
    #[default]
    None = 0,

    // Tier 1 - Low risk
    /// Wallet service - custodial or mixed wallets
    WalletService,

    /// Merchant service
    MerchantService,

    /// Mining pool
    MiningPool,

    // Tier 2 - Medium risk
    /// Exchange
    Exchange,

    /// DeFi application
    DeFi,

    /// OTC Broker
    OTCBroker,

    /// Cryptocurrency ATM
    ATM,

    /// Gambling
    Gambling,

    // Tier 3 - High risk
    /// Illicit organization
    IllicitOrganization,

    /// Mixer
    Mixer,

    /// Darknet market or service
    DarknetService,

    /// Scam
    Scam,

    /// Ransomware
    Ransomware,

    /// Theft - stolen funds
    Theft,

    /// Counterfeit - fake assets
    Counterfeit,

    // Tier 4 - Severe risk
    /// Terrorist financing
    TerroristFinancing,

    /// Sanctions
    Sanctions,

    /// Child abuse and porn materials
    ChildAbuse,

    /// The address belongs to a hacker or a group of hackers
    Hacker,

    /// Address belongs to a person or an organization from a high risk jurisdiction
    HighRiskJurisdiction,
}

pub fn bytes_to_string(arr: &[u8]) -> Result<String> {
    let null_index = arr.iter().position(|&ch| ch == b'\0').unwrap_or(arr.len());

    String::from_utf8(arr[0..null_index].to_vec()).map_err(|_| ErrorCode::InvalidData.into())
}
