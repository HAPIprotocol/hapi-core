use std::str::FromStr;

#[derive(Clone)]
pub enum HapiCoreNetwork {
    Ethereum,
    Bsc,
    Solana,
    Bitcoin,
    Near,
}

impl FromStr for HapiCoreNetwork {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ethereum" => Ok(Self::Ethereum),
            "bsc" => Ok(Self::Bsc),
            "solana" => Ok(Self::Solana),
            "bitcoin" => Ok(Self::Bitcoin),
            "near" => Ok(Self::Near),
            _ => Err(()),
        }
    }
}

impl std::fmt::Debug for HapiCoreNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ethereum => write!(f, "ethereum"),
            Self::Bsc => write!(f, "bsc"),
            Self::Solana => write!(f, "solana"),
            Self::Bitcoin => write!(f, "bitcoin"),
            Self::Near => write!(f, "near"),
        }
    }
}
