use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedMap,
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault,
};

pub mod address;
pub mod assets;
pub mod case;
pub mod configuration;
pub mod errors;
pub mod reporter;
pub mod reward;
pub mod stake;

pub use address::Address;
pub use assets::{Asset, AssetID};
pub use case::{Case, CaseId};
pub use errors::*;
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

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Addresses,
    Assets,
    Cases,
    Reporters,
}

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

// init Contract
#[near_bindgen]
impl Contract {
    #[init]
    pub fn initialize() -> Self {
        Self {
            authority: env::predecessor_account_id(),
            stake_configuration: StakeConfiguration::default(),
            reward_configuration: RewardConfiguration::default(),
            reporters: UnorderedMap::new(StorageKey::Reporters),
            cases: UnorderedMap::new(StorageKey::Cases),
            addresses: UnorderedMap::new(StorageKey::Addresses),
            assets: UnorderedMap::new(StorageKey::Assets),
        }
    }
}
