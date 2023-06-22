use super::DISCRIMINATOR_LENGTH;
use anchor_lang::prelude::*;

#[account]
pub struct Address {
    /// Account version
    pub version: u16,

    /// Seed bump for PDA
    pub bump: u8,

    /// Network account
    pub network: Pubkey,

    /// Actual address public key
    pub address: [u8; 64],

    /// Primary category of activity detected on the address
    pub category: Category,

    /// Estimated risk score on a scale from 0 to 10 (where 0 is safe and 10 is maximum risk)
    pub risk_score: u8,

    /// Case UUID
    pub case_id: u128,

    /// Case UUID
    pub reporter_id: u128,

    /// Confirmation count for this address
    pub confirmations: u8,
}

impl Address {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + (2 + 1 + 32 + 64 + 1 + 1 + 16 + 16 + 1);
    pub const VERSION: u16 = 1;
}

#[derive(Default, Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
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
