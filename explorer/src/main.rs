use {
    anyhow::{anyhow, Result},
    clap::{command, Arg, ArgMatches, Command},
    hapi_explorer::{
        application::Application, configuration::get_configuration, observability::setup_tracing,
    },
};

fn command_matcher() -> Result<ArgMatches> {
    Command::new(env!("CARGO_PKG_NAME"))
        .subcommand_required(true)
        .subcommand(
            command!("network")
                .about("Start the indexer worker")
                .subcommand_required(true)
                .subcommand(
                    Command::new("create")
                        .about("Create a new network")
                        .arg(
                            Arg::new("id")
                                .value_name("NETWORK_ID")
                                .index(1)
                                .required(true)
                                .help("Network ID"),
                        )
                        .arg(
                            Arg::new("name")
                                .value_name("NAME")
                                .index(2)
                                .required(true)
                                .help("Network name"),
                        )
                        .arg(
                            Arg::new("backend")
                                .value_name("BACKEND")
                                .index(3)
                                .required(true)
                                .help("Network backend"),
                        )
                        .arg(
                            Arg::new("chain-id")
                                .value_name("CHAIN_ID")
                                .index(4)
                                .required(false)
                                .help("Network chain ID (optional)"),
                        )
                        .arg(
                            Arg::new("stake-token")
                                .value_name("STAKE_TOKEN")
                                .index(5)
                                .required(true)
                                .help("Network stake token"),
                        )
                        .arg(
                            Arg::new("authority")
                                .value_name("AUTHORITY")
                                .index(5)
                                .required(true)
                                .help("Network authority"),
                        ),
                )
                .subcommand(
                    Command::new("update")
                        .about("Update a network")
                        .arg(
                            Arg::new("id")
                                .value_name("NETWORK_ID")
                                .index(1)
                                .required(true)
                                .help("Network ID"),
                        )
                        .arg(
                            Arg::new("name")
                                .value_name("NAME")
                                .index(2)
                                .required(false)
                                .help("Network name"),
                        )
                        .arg(
                            Arg::new("stake-token")
                                .value_name("STAKE_TOKEN")
                                .index(5)
                                .required(false)
                                .help("Network stake token"),
                        )
                        .arg(
                            Arg::new("authority")
                                .value_name("AUTHORITY")
                                .index(5)
                                .required(false)
                                .help("Network authority"),
                        ),
                ),
        )
        .subcommand(command!("server").about("Start explorer server"))
        .try_get_matches()
        .map_err(|e| anyhow!("Failed to parse command: {}", e))
}

fn parse_arg<T: Clone + Send + Sync + 'static>(args: &ArgMatches, arg_name: &str) -> Result<T> {
    Ok(args
        .get_one::<T>(arg_name)
        .ok_or(anyhow!("`{arg_name}` is required"))?
        .clone())
}

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = get_configuration()?;
    setup_tracing(&configuration.log_level, configuration.is_json_logging)?;

    let app = Application::from_configuration(configuration).await?;

    match command_matcher()?.subcommand() {
        Some(("server", _)) => app.run_server().await,
        Some(("network", matches)) => match matches.subcommand() {
            Some(("create", args)) => {
                let id = parse_arg(args, "id")?;
                let name = parse_arg(args, "name")?;
                let backend = parse_arg(args, "backend")?;
                let chain_id = parse_arg(args, "chain-id")?;
                let stake_token = parse_arg(args, "stake-token")?;
                let authority = parse_arg(args, "authority")?;

                app.create_network(id, name, backend, chain_id, authority, stake_token)
                    .await
            }
            Some(("update", args)) => {
                let id = parse_arg(args, "id")?;
                let name = parse_arg(args, "name").unwrap_or_default();
                let stake_token = parse_arg(args, "stake-token").unwrap_or_default();
                let authority = parse_arg(args, "authority").unwrap_or_default();

                app.update_network(id, name, authority, stake_token).await
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
