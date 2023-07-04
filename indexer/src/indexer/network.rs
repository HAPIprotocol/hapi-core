use std::str::FromStr;

#[derive(Debug, Clone)]
pub(crate) enum Network {
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
            "ethereum" => Ok(Network::Ethereum),
            "bsc" => Ok(Network::Bsc),
            "solana" => Ok(Network::Solana),
            "bitcoin" => Ok(Network::Bitcoin),
            "near" => Ok(Network::Near),
            _ => Err(anyhow::anyhow!("Invalid network: {}", s)),
        }
    }
}
