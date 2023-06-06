use clap::{Arg, Command};

use hapi_core::{HapiCore, HapiCoreEvm, HapiCoreEvmOptions, HapiCoreNetwork};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = Command::new(env!("CARGO_CRATE_NAME"))
        .about("Client for HAPI Protocol contracts")
        .subcommand_required(true)
        .arg(
            Arg::new("network")
                .short('n')
                .long("network")
                .global(true)
                .value_parser(["ethereum", "bsc", "solana", "bitcoin", "near"])
                .help("Network to use"),
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
        .get_matches();

    let network: HapiCoreNetwork = matches
        .get_one::<String>("network")
        .expect("`network` is required")
        .parse()
        .map_err(|e| anyhow::anyhow!("Failed to parse network: {:?}", e))?;

    let hapi_core = match network {
        HapiCoreNetwork::Ethereum => HapiCoreEvm::new(HapiCoreEvmOptions {
            provider_url: "http://localhost:8545".to_string(),
            contract_address: "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0".to_string(),
            private_key: Some(
                "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string(),
            ),
        })?,
        HapiCoreNetwork::Bsc => HapiCoreEvm::new(HapiCoreEvmOptions {
            provider_url: "http://localhost:8545".to_string(),
            contract_address: "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0".to_string(),
            private_key: Some(
                "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string(),
            ),
        })?,
        HapiCoreNetwork::Solana => todo!(),
        HapiCoreNetwork::Bitcoin => todo!(),
        HapiCoreNetwork::Near => todo!(),
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
        _ => unreachable!(),
    }

    Ok(())
}
