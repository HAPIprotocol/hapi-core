use crate::utils::DISCRIMINATOR_LENGTH;
use anchor_lang::prelude::*;

#[account]
pub struct Address {
    /// Account version
    pub version: u16,

    /// Community account, which this address belongs to
    pub community: Pubkey,

    /// Network account, which this address belongs to
    pub network: Pubkey,

    /// Actual address public key
    pub address: [u8; 64],

    /// Seed bump for PDA
    pub bump: u8,

    /// ID of the associated case
    pub case_id: u64,

    /// Reporter account public key
    pub reporter: Pubkey,

    /// Category of illicit activity identified with this address
    pub category: Category,

    /// Address risk score 0..10 (0 is safe, 10 is maximum risk)
    pub risk: u8,

    /// Confirmation count for this address
    pub confirmations: u8,

    /// Accumulated payment amount for report
    pub replication_bounty: u64,
}

impl Address {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + (2 + 32 + 32 + 64 + 1 + 8 + 32 + 1 + 1 + 1 + 8);
    pub const VERSION: u16 = 1;
}

#[derive(Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum Category {
    // Tier 0
    /// None
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
}

impl Default for Category {
    fn default() -> Self {
        Category::None
    }
}
