mod cli;
mod configuration;

use cli::HapiCli;
use configuration::{HapiCfg, MigrateAccount};

use anyhow::Result;
use colored::*;

fn migrate(hapi_cli: &HapiCli, cfg: HapiCfg) -> Result<()> {
    for acc in &cfg.migrate_accounts {
        match acc {
            MigrateAccount::All => {
                hapi_cli.migrate_communities(&cfg.communities)?;
                hapi_cli.migrate_networks()?;
                hapi_cli.migrate_reporters()?;
                hapi_cli.migrate_reporter_rewards()?;
                hapi_cli.migrate_cases()?;
                hapi_cli.migrate_addresses()?;
                hapi_cli.migrate_assets()?;
                break;
            }
            MigrateAccount::Community => {
                hapi_cli.migrate_communities(&cfg.communities)?;
            }
            MigrateAccount::Network => {
                hapi_cli.migrate_networks()?;
            }
            MigrateAccount::Reporter => {
                hapi_cli.migrate_reporters()?;
            }
            MigrateAccount::ReporterReward => {
                hapi_cli.migrate_reporter_rewards()?;
            }
            MigrateAccount::Case => {
                hapi_cli.migrate_cases()?;
            }
            MigrateAccount::Address => {
                hapi_cli.migrate_addresses()?;
            }
            MigrateAccount::Asset => {
                hapi_cli.migrate_assets()?;
            }
        }
    }

    Ok(())
}

fn main() {
    let cfg = HapiCfg::build().expect("Unable to configure");
    let hapi_cli = HapiCli::new(&cfg);

    let exit_code = match hapi_cli {
        Ok(cli) => match migrate(&cli, cfg) {
            Ok(()) => {
                println!("{}", "Migration successfully completed".green());
                0
            }
            Err(err) => {
                println!("{}: {}", "Migration failed".red(), err);
                1
            }
        },
        Err(err) => {
            println!("{}: {}", "Failed to initialize client".red(), err);
            1
        }
    };

    std::process::exit(exit_code)
}
