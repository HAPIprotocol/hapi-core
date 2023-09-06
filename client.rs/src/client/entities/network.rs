use {
    serde::{
        de::{self, Visitor},
        Deserialize,
    },
    std::{fmt, str::FromStr},
};

#[derive(Default, Debug, Clone)]
pub enum HapiCoreNetwork {
    #[default]
    Sepolia,
    Ethereum,
    Bsc,
    Solana,
    Bitcoin,
    Near,
}

impl FromStr for HapiCoreNetwork {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sepolia" => Ok(Self::Sepolia),
            "ethereum" => Ok(Self::Ethereum),
            "bsc" => Ok(Self::Bsc),
            "solana" => Ok(Self::Solana),
            "bitcoin" => Ok(Self::Bitcoin),
            "near" => Ok(Self::Near),
            _ => Err(anyhow::anyhow!("Invalid network: {}", s)),
        }
    }
}

// Solana network naming is related to this
impl fmt::Display for HapiCoreNetwork {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HapiCoreNetwork::Sepolia => write!(f, "sepolia"),
            HapiCoreNetwork::Ethereum => write!(f, "ethereum"),
            HapiCoreNetwork::Bsc => write!(f, "bsc"),
            HapiCoreNetwork::Solana => write!(f, "solana"),
            HapiCoreNetwork::Bitcoin => write!(f, "bitcoin"),
            HapiCoreNetwork::Near => write!(f, "near"),
        }
    }
}
struct HapiCoreNetworkVisitor;

impl<'de> Visitor<'de> for HapiCoreNetworkVisitor {
    type Value = HapiCoreNetwork;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid string for HapiCoreNetwork")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<HapiCoreNetwork, E> {
        HapiCoreNetwork::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for HapiCoreNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(HapiCoreNetworkVisitor)
    }
}
