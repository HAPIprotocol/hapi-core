use anyhow::anyhow;
use serde::{de, Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::client::result::ClientError;

#[derive(Default, Clone, PartialEq, Debug)]
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

impl Serialize for Category {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Category {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(de::Error::custom)
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "None",
                Self::WalletService => "WalletService",
                Self::MerchantService => "MerchantService",
                Self::MiningPool => "MiningPool",
                Self::Exchange => "Exchange",
                Self::DeFi => "DeFi",
                Self::OTCBroker => "OtcBroker",
                Self::ATM => "Atm",
                Self::Gambling => "Gambling",
                Self::IllicitOrganization => "IllicitOrganization",
                Self::Mixer => "Mixer",
                Self::DarknetService => "DarknetService",
                Self::Scam => "Scam",
                Self::Ransomware => "Ransomware",
                Self::Theft => "Theft",
                Self::Counterfeit => "Counterfeit",
                Self::TerroristFinancing => "TerroristFinancing",
                Self::Sanctions => "Sanctions",
                Self::ChildAbuse => "ChildAbuse",
                Self::Hacker => "Hacker",
                Self::HighRiskJurisdiction => "HighRiskJurisdiction",
            }
        )
    }
}

impl FromStr for Category {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(Self::None),
            "WalletService" => Ok(Self::WalletService),
            "MerchantService" => Ok(Self::MerchantService),
            "MiningPool" => Ok(Self::MiningPool),
            "Exchange" => Ok(Self::Exchange),
            "DeFi" => Ok(Self::DeFi),
            "OTCBroker" => Ok(Self::OTCBroker),
            "ATM" => Ok(Self::ATM),
            "Gambling" => Ok(Self::Gambling),
            "IllicitOrganization" => Ok(Self::IllicitOrganization),
            "Mixer" => Ok(Self::Mixer),
            "DarknetService" => Ok(Self::DarknetService),
            "Scam" => Ok(Self::Scam),
            "Ransomware" => Ok(Self::Ransomware),
            "Theft" => Ok(Self::Theft),
            "Counterfeit" => Ok(Self::Counterfeit),
            "TerroristFinancing" => Ok(Self::TerroristFinancing),
            "Sanctions" => Ok(Self::Sanctions),
            "ChildAbuse" => Ok(Self::ChildAbuse),
            "Hacker" => Ok(Self::Hacker),
            "HighRiskJurisdiction" => Ok(Self::HighRiskJurisdiction),
            _ => Err(anyhow!("invalid category")),
        }
    }
}

impl TryFrom<u8> for Category {
    type Error = ClientError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::WalletService),
            2 => Ok(Self::MerchantService),
            3 => Ok(Self::MiningPool),
            4 => Ok(Self::Exchange),
            5 => Ok(Self::DeFi),
            6 => Ok(Self::OTCBroker),
            7 => Ok(Self::ATM),
            8 => Ok(Self::Gambling),
            9 => Ok(Self::IllicitOrganization),
            10 => Ok(Self::Mixer),
            11 => Ok(Self::DarknetService),
            12 => Ok(Self::Scam),
            13 => Ok(Self::Ransomware),
            14 => Ok(Self::Theft),
            15 => Ok(Self::Counterfeit),
            16 => Ok(Self::TerroristFinancing),
            17 => Ok(Self::Sanctions),
            18 => Ok(Self::ChildAbuse),
            19 => Ok(Self::Hacker),
            20 => Ok(Self::HighRiskJurisdiction),
            _ => Err(ClientError::ContractData(format!(
                "invalid case status: {value}",
            ))),
        }
    }
}
