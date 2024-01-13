use {
    anyhow::Result,
    clap::{command, Parser, Subcommand},
    hapi_explorer::{
        application::Application, configuration::get_configuration, entity::types::NetworkBackend,
        observability::setup_tracing,
    },
    sea_orm_cli::MigrateSubcommands,
};

#[derive(Subcommand, PartialEq, Eq, Debug, Clone)]
pub enum NetworkSubcommands {
    #[command(about = "Create new network")]
    Create {
        #[arg(long, help = "Network string identifier ")]
        id: String,

        #[arg(long, help = "Network display name")]
        name: String,

        #[arg(long, help = "Network backend type")]
        backend: NetworkBackend,

        #[arg(long, default_value = None, help = "Network chain id (optional)")]
        chain_id: Option<String>,

        #[arg(long, help = "Network authority address")]
        authority: String,

        #[arg(long, help = "Stake token contract address")]
        stake_token: String,
    },
    #[command(about = "Update existing network")]
    Update {
        #[arg(long, help = "Network string identifier ")]
        id: String,

        #[arg(long, default_value = None, help = "Network display name")]
        name: Option<String>,

        #[arg(long, default_value = None, help = "Stake token contract address")]
        stake_token: Option<String>,

        #[arg(long, default_value = None, help = "Network authority address")]
        authority: Option<String>,
    },
}

#[derive(Parser)]
enum ExplorerCli {
    #[command(about = "Run explorer server")]
    Server,
    #[command(about = "Run commands related to networks")]
    Network {
        #[command(subcommand)]
        subcommand: NetworkSubcommands,
    },
    #[command(about = "Run migrations")]
    Migrate {
        #[command(subcommand)]
        subcommand: Option<MigrateSubcommands>,
    },
    CreateIndexer {
        #[arg(long, help = "Network backend type")]
        backend: NetworkBackend,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = get_configuration()?;
    setup_tracing(&configuration.log_level, configuration.is_json_logging)?;

    let app = Application::from_configuration(configuration).await?;

    match ExplorerCli::parse() {
        ExplorerCli::Server => app.run_server().await,
        ExplorerCli::Migrate { subcommand } => app.migrate(subcommand).await,
        ExplorerCli::Network { subcommand } => match subcommand {
            NetworkSubcommands::Create {
                id,
                name,
                backend,
                chain_id,
                authority,
                stake_token,
            } => {
                app.create_network(id, name, backend, chain_id, authority, stake_token)
                    .await
            }
            NetworkSubcommands::Update {
                id,
                name,
                stake_token,
                authority,
            } => app.update_network(id, name, authority, stake_token).await,
        },
        ExplorerCli::CreateIndexer { backend } => {
            app.create_indexer(backend).await?;

            Ok(())
        }
    }
}
