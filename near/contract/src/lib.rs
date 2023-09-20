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
pub mod token_transferer;
pub mod utils;

pub use address::{AddressView, VAddress};
pub use assets::{AssetId, AssetView, VAsset};
pub use case::{Case, CaseId, VCase};
pub use errors::*;
pub use reporter::{Reporter, ReporterId, VReporter};
pub use reward::RewardConfiguration;
pub use stake::StakeConfiguration;
pub use utils::TimestampExtension;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
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

    /// The address belongs to a hacker or a group of hackers
    Hacker,

    /// Address belongs to a person or an organization from a high risk jurisdiction
    HighRiskJurisdiction,
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
    assets: UnorderedMap<AssetId, VAsset>,
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
