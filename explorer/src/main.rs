use hapi_core::HapiCoreNetwork;
use hapi_explorer::{
    application::Application, configuration::get_configuration, observability::setup_tracing,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_configuration()?;
    setup_tracing(&configuration.log_level, configuration.is_json_logging)?;

    let app = Application::from_configuration(configuration).await?;

    let cmd = clap::Command::new(env!("CARGO_PKG_NAME"))
        .subcommand_required(true)
        .subcommand(
            clap::command!("create-indexer")
                .about("Create a new indexer and return jwt token")
                .arg(
                    clap::arg!(-c --"network" <NETWORK> "Network to use")
                        .value_parser(|network: &str| network.parse::<HapiCoreNetwork>()),
                ),
        )
        .subcommand(clap::command!("server").about("Run explorer server"));

    match cmd.get_matches().subcommand() {
        Some(("create-indexer", args)) => {
            let network = args
                .get_one::<HapiCoreNetwork>("network")
                .expect("Network is required");
            Ok(app.create_indexer(network.clone()))
        }
        Some(("server", _)) => Ok(app.run_server().await?),
        _ => unreachable!(),
    }
}
