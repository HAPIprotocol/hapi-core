pub mod address;
pub mod amount;
pub mod asset;
pub mod case;
pub mod configuration;
pub mod implementations;
pub mod interface;
pub mod network;
pub mod reporter;
pub mod result;

pub type Uuid = u128;

#[derive(Default, Clone, PartialEq)]
pub enum Category {
    #[default]
    None = 0,
    WalletService = 1,
    MerchantService = 2,
    MiningPool = 3,
    Exchange = 4,
    DeFi = 5,
    OTCBroker = 6,
    ATM = 7,
    Gambling = 8,
    IllicitOrganization = 9,
    Mixer = 10,
    DarknetService = 11,
    Scam = 12,
    Ransomware = 13,
    Theft = 14,
    Counterfeit = 15,
    TerroristFinancing = 16,
    Sanctions = 17,
    ChildAbuse = 18,
    Hacker = 19,
    HighRiskJurisdiction = 20,
}
