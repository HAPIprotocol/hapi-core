use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{collections::UnorderedMap, near_bindgen, AccountId, PanicOnDefault};

pub mod address;
pub mod assets;
pub mod case;
pub mod reporter;
pub mod reward;
pub mod stake;

pub use address::Address;
pub use assets::{Asset, AssetID};
pub use case::{Case, CaseId};
pub use reporter::{Reporter, ReporterId};
pub use reward::RewardConfiguration;
pub use stake::StakeConfiguration;

#[derive(BorshDeserialize, BorshSerialize)]
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

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    authority: AccountId,
    stake_configuration: StakeConfiguration,
    reward_configuration: RewardConfiguration,
    reporters: UnorderedMap<ReporterId, Reporter>,
    cases: UnorderedMap<CaseId, Case>,
    addresses: UnorderedMap<AccountId, Address>,
    assets: UnorderedMap<AssetID, Asset>,
}
