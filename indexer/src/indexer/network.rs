use serde::de::{self, Deserializer, Visitor};
use std::fmt;
use std::str::FromStr;

struct NetworkVisitor;

impl<'de> Visitor<'de> for NetworkVisitor {
    type Value = Network;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid string for Network")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Network, E> {
        Network::from_str(value).map_err(E::custom)
    }
}

impl<'de> serde::Deserialize<'de> for Network {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(NetworkVisitor)
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) enum Network {
    #[default]
    Sepolia,
    Ethereum,
    Bsc,
    Solana,
    Bitcoin,
    Near,
}

impl FromStr for Network {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sepolia" => Ok(Network::Sepolia),
            "ethereum" => Ok(Network::Ethereum),
            "bsc" => Ok(Network::Bsc),
            "solana" => Ok(Network::Solana),
            "bitcoin" => Ok(Network::Bitcoin),
            "near" => Ok(Network::Near),
            _ => Err(anyhow::anyhow!("Invalid network: {}", s)),
        }
    }
}
