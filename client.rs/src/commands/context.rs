use clap::ArgMatches;
use std::str::FromStr;

use hapi_core::{
    client::{implementations::TokenContractSolana, token::TokenContract},
    HapiCore, HapiCoreEvm, HapiCoreNear, HapiCoreNetwork, HapiCoreOptions, HapiCoreSolana,
    TokenContractEvm, TokenContractNear,
};

#[derive(Default)]
pub enum CommandOutput {
    #[default]
    Plain,
    Json,
}

impl FromStr for CommandOutput {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plain" => Ok(CommandOutput::Plain),
            "json" => Ok(CommandOutput::Json),
            _ => Err(anyhow::anyhow!("Unknown command output")),
        }
    }
}

pub(crate) struct HapiCoreCommandContext {
    pub hapi_core: Box<dyn HapiCore>,
    pub output: CommandOutput,
}

pub(crate) struct TokenCommandContext {
    pub token: Box<dyn TokenContract>,
    pub output: CommandOutput,
}

impl TryFrom<&ArgMatches> for TokenCommandContext {
    type Error = anyhow::Error;

    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        let network: HapiCoreNetwork = matches
            .get_one::<String>("network")
            .ok_or(anyhow::anyhow!("`network` is required"))?
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse `network`: {:?}", e))?;

        let provider_url = matches
            .get_one::<String>("provider-url")
            .ok_or(anyhow::anyhow!("`provider-url` is required"))?
            .to_owned();

        let contract_address = matches
            .get_one::<String>("token-contract")
            .ok_or(anyhow::anyhow!("`token-contract` is required"))?
            .to_owned();

        let private_key: Option<String> = matches.get_one::<String>("private-key").cloned();

        let output: CommandOutput = matches
            .get_one::<String>("output")
            .unwrap_or(&"plain".to_string())
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse `output`: {:?}", e))?;

        let token: Box<dyn TokenContract> = match network {
            HapiCoreNetwork::Ethereum => Box::new(TokenContractEvm::new(HapiCoreOptions {
                provider_url,
                contract_address,
                private_key,
                chain_id: None,
            })?),
            HapiCoreNetwork::Bsc => Box::new(TokenContractEvm::new(HapiCoreOptions {
                provider_url,
                contract_address,
                private_key,
                chain_id: None,
            })?),
            HapiCoreNetwork::Solana => Box::new(TokenContractSolana::new()?),
            HapiCoreNetwork::Bitcoin => Box::new(TokenContractSolana::new()?),
            HapiCoreNetwork::Near => Box::new(TokenContractNear::new()?),
        };

        Ok(Self { token, output })
    }
}

impl TryFrom<&ArgMatches> for HapiCoreCommandContext {
    type Error = anyhow::Error;

    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        let network: HapiCoreNetwork = matches
            .get_one::<String>("network")
            .ok_or(anyhow::anyhow!("`network` is required"))?
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse `network`: {:?}", e))?;

        let provider_url = matches
            .get_one::<String>("provider-url")
            .ok_or(anyhow::anyhow!("`provider-url` is required"))?
            .to_owned();

        let contract_address = matches
            .get_one::<String>("contract-address")
            .ok_or(anyhow::anyhow!("`contract-address` is required"))?
            .to_owned();

        let private_key: Option<String> = matches.get_one::<String>("private-key").cloned();

        let chain_id = matches
            .get_one::<String>("chain-id")
            .map(|s| {
                s.parse::<u64>()
                    .map_err(|e| anyhow::anyhow!("`chain-id`: {e}"))
            })
            .transpose()?;

        let output: CommandOutput = matches
            .get_one::<String>("output")
            .unwrap_or(&"plain".to_string())
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse `output`: {:?}", e))?;

        let options = HapiCoreOptions {
            provider_url,
            contract_address,
            private_key,
            chain_id,
        };

        let hapi_core: Box<dyn HapiCore> = match network {
            HapiCoreNetwork::Ethereum => Box::new(HapiCoreEvm::new(options)?),
            HapiCoreNetwork::Bsc => Box::new(HapiCoreEvm::new(options)?),
            HapiCoreNetwork::Solana => Box::new(HapiCoreSolana::new()?),
            HapiCoreNetwork::Bitcoin => Box::new(HapiCoreSolana::new()?),
            HapiCoreNetwork::Near => Box::new(HapiCoreNear::new()?),
        };

        Ok(Self { hapi_core, output })
    }
}
