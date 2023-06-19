use clap::{Arg, ArgGroup, ArgMatches, Command};
use std::str::FromStr;

use hapi_core::{
    client::{implementations::TokenContractSolana, token::TokenContract},
    HapiCore, HapiCoreEvm, HapiCoreEvmOptions, HapiCoreNear, HapiCoreNetwork, HapiCoreSolana,
    TokenContractEvm, TokenContractNear,
};

mod commands;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = Command::new(env!("CARGO_CRATE_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("network")
                .global(true)
                .long("network")
                .value_name("NETWORK")
                .env("NETWORK")
                .value_parser(["ethereum", "bsc", "solana", "bitcoin", "near"])
                .help("Network to use"),
        )
        .arg(
            Arg::new("provider-url")
                .global(true)
                .long("provider-url")
                .value_name("PROVIDER_URL")
                .env("PROVIDER_URL")
                .help("Provider URL"),
        )
        .arg(
            Arg::new("contract-address")
                .global(true)
                .long("contract-address")
                .value_name("CONTRACT_ADDRESS")
                .env("CONTRACT_ADDRESS")
                .help("Contract address"),
        )
        .arg(
            Arg::new("private-key")
                .global(true)
                .long("private-key")
                .value_name("PRIVATE_KEY")
                .env("PRIVATE_KEY")
                .hide_env(true)
                .help("Private key to sign transactions"),
        )
        .arg(
            Arg::new("output")
                .global(true)
                .long("output")
                .value_name("OUTPUT")
                .env("OUTPUT")
                .value_parser(["json", "text"])
                .help("Output format"),
        )
        .subcommand_required(true)
        .subcommand(
            Command::new("authority")
                .about("Authority commands")
                .subcommand_required(true)
                .subcommand(Command::new("get").about("Get authority address"))
                .subcommand(
                    Command::new("set")
                        .arg(
                            Arg::new("authority")
                                .value_name("AUTHORITY")
                                .index(1)
                                .required(true)
                                .help("New authority address"),
                        )
                        .about("Set new authority address"),
                ),
        )
        .subcommand(
            Command::new("configuration")
                .alias("cfg")
                .about("Configuration commands")
                .subcommand_required(true)
                .subcommand(Command::new("get-stake").about("Get stake configuration"))
                .subcommand(
                    Command::new("update-stake")
                        .about("Update stake configuration")
                        .arg(
                            Arg::new("token")
                                .value_name("TOKEN")
                                .index(1)
                                .required(true)
                                .help("Token address"),
                        )
                        .arg(
                            Arg::new("unlock-duration")
                                .value_name("UNLOCK_DURATION")
                                .index(2)
                                .required(true)
                                .help("Unlock duration"),
                        )
                        .arg(
                            Arg::new("validator-stake")
                                .value_name("VALIDATOR_STAKE")
                                .index(3)
                                .required(true)
                                .help("Validator stake"),
                        )
                        .arg(
                            Arg::new("tracer-stake")
                                .value_name("TRACER_STAKE")
                                .index(4)
                                .required(true)
                                .help("Tracer stake"),
                        )
                        .arg(
                            Arg::new("publisher-stake")
                                .value_name("PUBLISHER_STAKE")
                                .index(5)
                                .required(true)
                                .help("Publisher stake"),
                        )
                        .arg(
                            Arg::new("authority-stake")
                                .value_name("AUTHORITY_STAKE")
                                .index(6)
                                .required(true)
                                .help("Authority stake"),
                        ),
                )
                .subcommand(Command::new("get-reward").about("Get reward configuration"))
                .subcommand(
                    Command::new("update-reward")
                        .about("Update reward configuration")
                        .arg(
                            Arg::new("token")
                                .value_name("TOKEN")
                                .index(1)
                                .required(true)
                                .help("Token address"),
                        )
                        .arg(
                            Arg::new("address-confirmation-reward")
                                .value_name("ADDRESS_CONFIRMATION_REWARD")
                                .index(2)
                                .required(true)
                                .help("Address confirmation reward"),
                        )
                        .arg(
                            Arg::new("tracer-reward")
                                .value_name("TRACER_REWARD")
                                .index(3)
                                .required(true)
                                .help("Tracer reward"),
                        ),
                ),
        )
        .subcommand(
            Command::new("reporter")
                .about("Reporter commands")
                .subcommand_required(true)
                .subcommand(
                    Command::new("create")
                        .about("Create reporter")
                        .arg(
                            Arg::new("id")
                                .value_name("ID")
                                .index(1)
                                .required(true)
                                .help("Reporter UUID"),
                        )
                        .arg(
                            Arg::new("account")
                                .value_name("ACCOUNT")
                                .index(2)
                                .required(true)
                                .help("Reporter account address"),
                        )
                        .arg(
                            Arg::new("role")
                                .value_name("ROLE")
                                .index(3)
                                .required(true)
                                .help("Reporter role")
                                .value_parser(["validator", "tracer", "publisher", "authority"]),
                        )
                        .arg(
                            Arg::new("name")
                                .value_name("NAME")
                                .index(4)
                                .required(true)
                                .help("Reporter display name"),
                        )
                        .arg(
                            Arg::new("url")
                                .value_name("URL")
                                .index(5)
                                .required(true)
                                .help("Reporter URL"),
                        ),
                )
                .subcommand(Command::new("update").about("Update reporter"))
                .subcommand(
                    Command::new("get").about("Get reporter").arg(
                        Arg::new("id")
                            .value_name("ID")
                            .index(1)
                            .required(true)
                            .help("Reporter UUID"),
                    ),
                )
                .subcommand(Command::new("count").about("Get reporter count"))
                .subcommand(
                    Command::new("list")
                        .about("Get reporter list")
                        .arg(
                            Arg::new("skip")
                                .group("pagination")
                                .long("skip")
                                .short('s')
                                .value_name("SKIP")
                                .default_value("0")
                                .help("Skip first N items"),
                        )
                        .arg(
                            Arg::new("take")
                                .group("pagination")
                                .long("take")
                                .short('t')
                                .value_name("TAKE")
                                .default_value("10")
                                .help("Return N items"),
                        ),
                )
                .subcommand(Command::new("activate").about("Activate reporter"))
                .subcommand(Command::new("deactivate").about("Deactivate reporter"))
                .subcommand(Command::new("unstake").about("Unstake reporter")),
        )
        .subcommand(
            Command::new("case")
                .about("Case commands")
                .subcommand_required(true)
                .subcommand(Command::new("create").about("Create case"))
                .subcommand(Command::new("update").about("Update case"))
                .subcommand(
                    Command::new("get").about("Get case").arg(
                        Arg::new("id")
                            .value_name("ID")
                            .index(1)
                            .required(true)
                            .help("Case UUID"),
                    ),
                )
                .subcommand(Command::new("count").about("Get case count"))
                .subcommand(
                    Command::new("list")
                        .about("Get case list")
                        .arg(
                            Arg::new("skip")
                                .group("pagination")
                                .long("skip")
                                .short('s')
                                .value_name("SKIP")
                                .default_value("0")
                                .help("Skip first N items"),
                        )
                        .arg(
                            Arg::new("take")
                                .group("pagination")
                                .long("take")
                                .short('t')
                                .value_name("TAKE")
                                .default_value("10")
                                .help("Return N items"),
                        ),
                ),
        )
        .subcommand(
            Command::new("address")
                .about("Address commands")
                .subcommand_required(true)
                .subcommand(Command::new("create").about("Create address"))
                .subcommand(Command::new("update").about("Update address"))
                .subcommand(
                    Command::new("get").about("Get address").arg(
                        Arg::new("address")
                            .value_name("ADDRESS")
                            .index(1)
                            .required(true)
                            .help("Address"),
                    ),
                )
                .subcommand(Command::new("count").about("Get address count"))
                .subcommand(
                    Command::new("list")
                        .about("Get address list")
                        .group(ArgGroup::new("pagination").args(["skip", "take"]))
                        .arg(
                            Arg::new("skip")
                                .group("pagination")
                                .long("skip")
                                .short('s')
                                .value_name("SKIP")
                                .default_value("0")
                                .help("Skip first N items"),
                        )
                        .arg(
                            Arg::new("take")
                                .group("pagination")
                                .long("take")
                                .short('t')
                                .value_name("TAKE")
                                .default_value("10")
                                .help("Return N items"),
                        ),
                ),
        )
        .subcommand(
            Command::new("asset")
                .about("Asset commands")
                .subcommand_required(true)
                .subcommand(Command::new("create").about("Create asset"))
                .subcommand(Command::new("update").about("Update asset"))
                .subcommand(
                    Command::new("get")
                        .about("Get asset")
                        .arg(
                            Arg::new("address")
                                .value_name("ADDRESS")
                                .index(1)
                                .required(true)
                                .help("Asset contract address"),
                        )
                        .arg(
                            Arg::new("id")
                                .value_name("ID")
                                .index(2)
                                .required(true)
                                .help("Asset ID"),
                        ),
                )
                .subcommand(Command::new("count").about("Get asset count"))
                .subcommand(
                    Command::new("list")
                        .about("Get asset list")
                        .arg(
                            Arg::new("skip")
                                .group("pagination")
                                .long("skip")
                                .short('s')
                                .value_name("SKIP")
                                .default_value("0")
                                .help("Skip first N items"),
                        )
                        .arg(
                            Arg::new("take")
                                .group("pagination")
                                .long("take")
                                .short('t')
                                .value_name("TAKE")
                                .default_value("10")
                                .help("Return N items"),
                        ),
                ),
        )
        .subcommand(
            Command::new("token")
                .about("Token operations")
                .subcommand_required(true)
                .subcommand(
                    Command::new("transfer")
                        .about("Transfer token")
                        .arg(
                            Arg::new("token-contract")
                                .value_name("TOKEN_CONTRACT")
                                .index(1)
                                .required(true)
                                .help("Token contract address"),
                        )
                        .arg(
                            Arg::new("to")
                                .value_name("TO")
                                .index(2)
                                .required(true)
                                .help("Receiver address"),
                        )
                        .arg(
                            Arg::new("amount")
                                .value_name("AMOUNT")
                                .index(3)
                                .required(true)
                                .help("Amount to transfer"),
                        ),
                )
                .subcommand(
                    Command::new("approve")
                        .about("Approve token allowance")
                        .arg(
                            Arg::new("token-contract")
                                .value_name("TOKEN_CONTRACT")
                                .index(1)
                                .required(true)
                                .help("Token contract address"),
                        )
                        .arg(
                            Arg::new("spender")
                                .value_name("SPENDER")
                                .index(2)
                                .required(true)
                                .help("Address that receives allowance"),
                        )
                        .arg(
                            Arg::new("amount")
                                .value_name("AMOUNT")
                                .index(3)
                                .required(true)
                                .help("Amount to approve"),
                        ),
                )
                .subcommand(
                    Command::new("balance")
                        .about("Get token balance")
                        .arg(
                            Arg::new("token-contract")
                                .value_name("TOKEN_CONTRACT")
                                .index(1)
                                .required(true)
                                .help("Token contract address"),
                        )
                        .arg(
                            Arg::new("address")
                                .value_name("ADDRESS")
                                .index(2)
                                .required(true)
                                .help("Address to get balance for"),
                        ),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("authority", matches)) => match matches.subcommand() {
            Some(("get", matches)) => commands::get_authority(matches).await?,
            Some(("set", matches)) => commands::set_authority(matches).await?,
            _ => unreachable!(),
        },
        Some(("configuration", matches)) => match matches.subcommand() {
            Some(("get-stake", matches)) => commands::get_stake_configuration(matches).await?,
            Some(("update-stake", matches)) => {
                commands::update_stake_configuration(matches).await?
            }
            Some(("get-reward", matches)) => commands::get_reward_configuration(matches).await?,
            Some(("update-reward", matches)) => {
                commands::update_reward_configuration(matches).await?
            }
            _ => unreachable!(),
        },
        Some(("reporter", matches)) => match matches.subcommand() {
            Some(("create", matches)) => commands::create_reporter(matches).await?,
            Some(("update", matches)) => commands::update_reporter(matches).await?,
            Some(("get", matches)) => commands::get_reporter(matches).await?,
            Some(("count", matches)) => commands::get_reporter_count(matches).await?,
            Some(("list", matches)) => commands::get_reporters(matches).await?,
            Some(("activate", matches)) => commands::activate_reporter(matches).await?,
            Some(("deactivate", matches)) => commands::deactivate_reporter(matches).await?,
            Some(("unstake", matches)) => commands::unstake_reporter(matches).await?,
            _ => unreachable!(),
        },
        Some(("case", matches)) => match matches.subcommand() {
            Some(("create", matches)) => commands::create_case(matches).await?,
            Some(("update", matches)) => commands::update_case(matches).await?,
            Some(("get", matches)) => commands::get_case(matches).await?,
            Some(("count", matches)) => commands::get_case_count(matches).await?,
            Some(("list", matches)) => commands::get_cases(matches).await?,
            _ => unreachable!(),
        },
        Some(("address", matches)) => match matches.subcommand() {
            Some(("create", matches)) => commands::create_address(matches).await?,
            Some(("update", matches)) => commands::update_address(matches).await?,
            Some(("get", matches)) => commands::get_address(matches).await?,
            Some(("count", matches)) => commands::get_address_count(matches).await?,
            Some(("list", matches)) => commands::get_addresses(matches).await?,
            _ => unreachable!(),
        },
        Some(("asset", matches)) => match matches.subcommand() {
            Some(("create", matches)) => commands::create_asset(matches).await?,
            Some(("update", matches)) => commands::update_asset(matches).await?,
            Some(("get", matches)) => commands::get_asset(matches).await?,
            Some(("count", matches)) => commands::get_asset_count(matches).await?,
            Some(("list", matches)) => commands::get_assets(matches).await?,
            _ => unreachable!(),
        },
        Some(("token", matches)) => match matches.subcommand() {
            Some(("transfer", matches)) => commands::transfer_token(matches).await?,
            Some(("approve", matches)) => commands::approve_token(matches).await?,
            Some(("balance", matches)) => commands::balance_token(matches).await?,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };

    Ok(())
}

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

struct HapiCoreCommandContext {
    pub hapi_core: Box<dyn HapiCore>,
    pub output: CommandOutput,
}

impl TryFrom<&ArgMatches> for HapiCoreCommandContext {
    type Error = anyhow::Error;

    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        let network: HapiCoreNetwork = matches
            .get_one::<String>("network")
            .expect("`network` is required")
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse `network`: {:?}", e))?;

        let provider_url = matches
            .get_one::<String>("provider-url")
            .expect("`provider-url` is required")
            .to_owned();

        let contract_address = matches
            .get_one::<String>("contract-address")
            .expect("`contract-address` is required")
            .to_owned();

        let private_key: Option<String> = matches.get_one::<String>("private-key").cloned();

        let output: CommandOutput = matches
            .get_one::<String>("output")
            .unwrap_or(&"plain".to_string())
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse `output`: {:?}", e))?;

        let hapi_core: Box<dyn HapiCore> = match network {
            HapiCoreNetwork::Ethereum => Box::new(HapiCoreEvm::new(HapiCoreEvmOptions {
                provider_url,
                contract_address,
                private_key,
                chain_id: None,
            })?),
            HapiCoreNetwork::Bsc => Box::new(HapiCoreEvm::new(HapiCoreEvmOptions {
                provider_url,
                contract_address,
                private_key,
                chain_id: None,
            })?),
            HapiCoreNetwork::Solana => Box::new(HapiCoreSolana::new()?),
            HapiCoreNetwork::Bitcoin => Box::new(HapiCoreSolana::new()?),
            HapiCoreNetwork::Near => Box::new(HapiCoreNear::new()?),
        };

        Ok(Self { hapi_core, output })
    }
}

struct TokenCommandContext {
    pub token: Box<dyn TokenContract>,
    pub output: CommandOutput,
}

impl TryFrom<&ArgMatches> for TokenCommandContext {
    type Error = anyhow::Error;

    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        let network: HapiCoreNetwork = matches
            .get_one::<String>("network")
            .expect("`network` is required")
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse `network`: {:?}", e))?;

        let provider_url = matches
            .get_one::<String>("provider-url")
            .expect("`provider-url` is required")
            .to_owned();

        let contract_address = matches
            .get_one::<String>("token-contract")
            .expect("`token-contract` is required")
            .to_owned();

        let private_key: Option<String> = matches.get_one::<String>("private-key").cloned();

        let output: CommandOutput = matches
            .get_one::<String>("output")
            .unwrap_or(&"plain".to_string())
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse `output`: {:?}", e))?;

        let token: Box<dyn TokenContract> = match network {
            HapiCoreNetwork::Ethereum => Box::new(TokenContractEvm::new(HapiCoreEvmOptions {
                provider_url,
                contract_address,
                private_key,
                chain_id: None,
            })?),
            HapiCoreNetwork::Bsc => Box::new(TokenContractEvm::new(HapiCoreEvmOptions {
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
