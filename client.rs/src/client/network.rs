use {
    serde::{
        de::{self, Visitor},
        Deserializer,
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

impl<'de> serde::Deserialize<'de> for HapiCoreNetwork {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(HapiCoreNetworkVisitor)
    }
}
