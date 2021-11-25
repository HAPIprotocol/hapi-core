use anchor_lang::prelude::*;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub enum Category {
    // Tier 0
    /// None
    None = 0,

    // Tier 1 - Low risk
    /// Wallet service - custodial or mixed wallets
    WalletService = 1,

    /// Merchant service
    MerchantService = 2,

    /// Mining pool
    MiningPool = 4,

    /// Exchange (Low Risk) - Exchange with high KYC standards
    LowRiskExchange = 8,

    // Tier 2 - Medium risk
    /// Exchange (Medium Risk)
    MediumRiskExchange = 16,

    /// DeFi application
    DeFi = 32,

    /// OTC Broker
    OTCBroker = 64,

    /// Cryptocurrency ATM
    ATM = 128,

    /// Gambling
    Gambling = 256,

    // Tier 3 - High risk
    /// Illicit organization
    IllicitOrganization = 512,

    /// Mixer
    Mixer = 1024,

    /// Darknet market or service
    DarknetService = 2048,

    /// Scam
    Scam = 4096,

    /// Ransomware
    Ransomware = 8192,

    /// Theft - stolen funds
    Theft = 16384,

    /// Counterfeit - fake assets
    Counterfeit = 32768,

    // Tier 4 - Severe risk
    /// Terrorist financing
    TerroristFinancing = 65536,

    /// Sanctions
    Sanctions = 131072,

    /// Child abuse and porn materials
    ChildAbuse = 262144,
}

impl Default for Category {
    fn default() -> Self {
        Category::None
    }
}
