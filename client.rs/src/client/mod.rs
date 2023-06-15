use anyhow::anyhow;
use serde::Serialize;
use std::str::FromStr;

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
mod uuid;

#[derive(Default, Clone, PartialEq, Debug, Serialize)]
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

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "none",
                Self::WalletService => "wallet_service",
                Self::MerchantService => "merchant_service",
                Self::MiningPool => "mining_pool",
                Self::Exchange => "exchange",
                Self::DeFi => "defi",
                Self::OTCBroker => "otc_broker",
                Self::ATM => "atm",
                Self::Gambling => "gambling",
                Self::IllicitOrganization => "illicit_organization",
                Self::Mixer => "mixer",
                Self::DarknetService => "darknet_service",
                Self::Scam => "scam",
                Self::Ransomware => "ransomware",
                Self::Theft => "theft",
                Self::Counterfeit => "counterfeit",
                Self::TerroristFinancing => "terrorist_financing",
                Self::Sanctions => "sanctions",
                Self::ChildAbuse => "child_abuse",
                Self::Hacker => "hacker",
                Self::HighRiskJurisdiction => "high_risk_jurisdiction",
            }
        )
    }
}

impl FromStr for Category {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "wallet_service" => Ok(Self::WalletService),
            "merchant_service" => Ok(Self::MerchantService),
            "mining_pool" => Ok(Self::MiningPool),
            "exchange" => Ok(Self::Exchange),
            "defi" => Ok(Self::DeFi),
            "otc_broker" => Ok(Self::OTCBroker),
            "atm" => Ok(Self::ATM),
            "gambling" => Ok(Self::Gambling),
            "illicit_organization" => Ok(Self::IllicitOrganization),
            "mixer" => Ok(Self::Mixer),
            "darknet_service" => Ok(Self::DarknetService),
            "scam" => Ok(Self::Scam),
            "ransomware" => Ok(Self::Ransomware),
            "theft" => Ok(Self::Theft),
            "counterfeit" => Ok(Self::Counterfeit),
            "terrorist_financing" => Ok(Self::TerroristFinancing),
            "sanctions" => Ok(Self::Sanctions),
            "child_abuse" => Ok(Self::ChildAbuse),
            "hacker" => Ok(Self::Hacker),
            "high_risk_jurisdiction" => Ok(Self::HighRiskJurisdiction),
            _ => Err(anyhow!("invalid category")),
        }
    }
}
