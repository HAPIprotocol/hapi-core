use clap::{Arg, Command};

use hapi_core::{
    client::configuration::{RewardConfiguration, StakeConfiguration},
    HapiCore, HapiCoreEvm, HapiCoreEvmOptions, HapiCoreNear, HapiCoreNetwork, HapiCoreSolana,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = Command::new(env!("CARGO_CRATE_NAME"))
        .about("Client for HAPI Protocol contracts")
        .subcommand_required(true)
        .arg(
            Arg::new("network")
                .global(true)
                .value_name("NETWORK")
                .env("NETWORK")
                .value_parser(["ethereum", "bsc", "solana", "bitcoin", "near"])
                .help("Network to use"),
        )
        .arg(
            Arg::new("provider-url")
                .global(true)
                .value_name("PROVIDER_URL")
                .env("PROVIDER_URL")
                .help("Provider URL"),
        )
        .arg(
            Arg::new("contract-address")
                .global(true)
                .value_name("CONTRACT_ADDRESS")
                .env("CONTRACT_ADDRESS")
                .help("Contract address"),
        )
        .arg(
            Arg::new("private-key")
                .global(true)
                .value_name("PRIVATE_KEY")
                .env("PRIVATE_KEY")
                .help("Private key to sign transactions"),
        )
        .subcommand(Command::new("get-authority").about("Get authority address"))
        .subcommand(
            Command::new("set-authority")
                .arg(
                    Arg::new("authority")
                        .value_name("AUTHORITY")
                        .index(1)
                        .required(true)
                        .help("New authority address"),
                )
                .about("Set authority address"),
        )
        .subcommand(Command::new("get-stake-configuration").about("Get stake configuration"))
        .subcommand(
            Command::new("update-stake-configuration")
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
        .subcommand(Command::new("get-reward-configuration").about("Get reward configuration"))
        .subcommand(
            Command::new("update-reward-configuration")
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
        )
        .subcommand(Command::new("create-reporter").about("Create reporter"))
        .subcommand(Command::new("update-reporter").about("Update reporter"))
        .subcommand(Command::new("get-reporter").about("Get reporter"))
        .subcommand(Command::new("get-reporter-count").about("Get reporter count"))
        .subcommand(Command::new("get-reporters").about("Get reporters"))
        .subcommand(Command::new("activate-reporter").about("Activate reporter"))
        .subcommand(Command::new("deactivate-reporter").about("Deactivate reporter"))
        .subcommand(Command::new("unstake-reporter").about("Unstake reporter"))
        .subcommand(Command::new("create-case").about("Create case"))
        .subcommand(Command::new("update-case").about("Update case"))
        .subcommand(Command::new("get-case").about("Get case"))
        .subcommand(Command::new("get-case-count").about("Get case count"))
        .subcommand(Command::new("get-cases").about("Get cases"))
        .subcommand(Command::new("create-address").about("Create address"))
        .subcommand(Command::new("update-address").about("Update address"))
        .subcommand(Command::new("get-address").about("Get address"))
        .subcommand(Command::new("get-address-count").about("Get address count"))
        .subcommand(Command::new("get-addresses").about("Get addresses"))
        .subcommand(Command::new("create-asset").about("Create asset"))
        .subcommand(Command::new("update-asset").about("Update asset"))
        .subcommand(Command::new("get-asset").about("Get asset"))
        .subcommand(Command::new("get-asset-count").about("Get asset count"))
        .subcommand(Command::new("get-assets").about("Get assets"))
        .get_matches();

    let network: HapiCoreNetwork = matches
        .get_one::<String>("network")
        .expect("`network` is required")
        .parse()
        .map_err(|e| anyhow::anyhow!("Failed to parse network: {:?}", e))?;

    let provider_url = matches
        .get_one::<String>("provider-url")
        .expect("`provider-url` is required")
        .to_owned();

    let contract_address = matches
        .get_one::<String>("contract-address")
        .expect("`contract-address` is required")
        .to_owned();

    let private_key: Option<String> = matches.get_one::<String>("private-key").cloned();

    let hapi_core: Box<dyn HapiCore> = match network {
        HapiCoreNetwork::Ethereum => Box::new(HapiCoreEvm::new(HapiCoreEvmOptions {
            provider_url,
            contract_address,
            private_key,
        })?),
        HapiCoreNetwork::Bsc => Box::new(HapiCoreEvm::new(HapiCoreEvmOptions {
            provider_url,
            contract_address,
            private_key,
        })?),
        HapiCoreNetwork::Solana => Box::new(HapiCoreSolana::new()?),
        HapiCoreNetwork::Bitcoin => Box::new(HapiCoreSolana::new()?),
        HapiCoreNetwork::Near => Box::new(HapiCoreNear::new()?),
    };

    match matches.subcommand() {
        Some(("get-authority", _)) => {
            let authority = hapi_core.get_authority().await?;

            println!("Authority: {authority}");
        }
        Some(("set-authority", matches)) => {
            let authority = matches
                .get_one::<String>("authority")
                .expect("`authority` is required");

            let tx = hapi_core.set_authority(authority).await?;

            println!("Tx: {}", tx.hash);
        }
        Some(("update-stake-configuration", matches)) => {
            let cfg = StakeConfiguration {
                token: matches
                    .get_one::<String>("token")
                    .map(|v| v.parse().unwrap())
                    .expect("`token` is required"),
                unlock_duration: matches
                    .get_one::<String>("unlock-duration")
                    .map(|v| v.parse().unwrap())
                    .expect("`unlock-duration` is required"),
                validator_stake: matches
                    .get_one::<String>("validator-stake")
                    .map(|v| v.parse().unwrap())
                    .expect("`validator-stake` is required"),
                tracer_stake: matches
                    .get_one::<String>("tracer-stake")
                    .map(|v| v.parse().unwrap())
                    .expect("`tracer-stake` is required"),
                publisher_stake: matches
                    .get_one::<String>("publisher-stake")
                    .map(|v| v.parse().unwrap())
                    .expect("`publisher-stake` is required"),
                authority_stake: matches
                    .get_one::<String>("authority-stake")
                    .map(|v| v.parse().unwrap())
                    .expect("`authority-stake` is required"),
            };

            let tx = hapi_core.update_stake_configuration(cfg).await?;

            println!("Tx: {}", tx.hash);
        }
        Some(("get-stake-configuration", _)) => {
            let stake_configuration = hapi_core.get_stake_configuration().await?;

            println!("Stake configuration: {:?}", stake_configuration);
        }
        Some(("update-reward-configuration", matches)) => {
            let cfg = RewardConfiguration {
                token: matches
                    .get_one::<String>("token")
                    .map(|v| v.parse().unwrap())
                    .expect("`token` is required"),
                address_confirmation_reward: matches
                    .get_one::<String>("address-confirmation-reward")
                    .map(|v| v.parse().unwrap())
                    .expect("`address_confirmation_reward` is required"),
                tracer_reward: matches
                    .get_one::<String>("tracer-reward")
                    .map(|v| v.parse().unwrap())
                    .expect("`tracer_reward` is required"),
            };

            let tx = hapi_core.update_reward_configuration(cfg).await?;

            println!("Tx: {}", tx.hash);
        }
        Some(("get-reward-configuration", _)) => {
            let reward_configuration = hapi_core.get_reward_configuration().await?;

            println!("Reward configuration: {:?}", reward_configuration);
        }
        _ => unreachable!(),
    }

    Ok(())
}
