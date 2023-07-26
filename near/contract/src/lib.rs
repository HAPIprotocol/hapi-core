use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{LookupMap, UnorderedMap},
    env, near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId, BorshStorageKey, PanicOnDefault,
};

pub mod address;
pub mod assets;
pub mod case;
pub mod configuration;
pub mod errors;
pub mod reporter;
pub mod reward;
pub mod stake;
pub mod token_trasnferer;
pub mod utils;

pub use address::VAddress;
pub use assets::{AssetID, VAsset};
pub use case::{CaseId, VCase};
pub use errors::*;
pub use reporter::{ReporterId, VReporter};
pub use reward::RewardConfiguration;
pub use stake::StakeConfiguration;
pub use utils::TimestampExtension;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
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

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Addresses,
    Assets,
    Cases,
    Reporters,
    ReportersByAccount,
    Confirmations { address: AccountId },
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    authority: AccountId,
    stake_configuration: StakeConfiguration,
    reward_configuration: RewardConfiguration,
    reporters: UnorderedMap<ReporterId, VReporter>,
    cases: UnorderedMap<CaseId, VCase>,
    addresses: UnorderedMap<AccountId, VAddress>,
    assets: UnorderedMap<AssetID, VAsset>,
    reporters_by_account: LookupMap<AccountId, ReporterId>,
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
            reporters_by_account: LookupMap::new(StorageKey::ReportersByAccount),
        }
    }
}
