use hapi_core::{HapiCore, HapiCoreEvm, HapiCoreEvmOptions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let hapi_core = HapiCoreEvm::new(HapiCoreEvmOptions {
        provider_url: "http://localhost:8545".to_string(),
        contract_address: "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0".to_string(),
        private_key: Some(
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string(),
        ),
    })?;

    let authority = hapi_core.get_authority().await?;

    println!("Authority: {authority}");

    let tx = hapi_core
        .set_authority("0x70997970C51812dc3A010C7d01b50e0d17dc79C8")
        .await?;

    println!("Tx: {}", tx.hash);

    Ok(())
}
