use anchor_client::solana_sdk::signature::Signer;
use serde_json::json;

mod assert;
mod cmd_utils;
mod solana;

use solana::setup::Setup;

// #[tokio::test]
#[tokio::test(flavor = "multi_thread")]
async fn solana_cli_works() {
    println!("Running solana-cli tests");
    let t = Setup::new().await;

    let admin_secret = t.data.get_wallet("admin").keypair.to_base58_string();
    let authority_secret = t.data.get_wallet("authority").keypair.to_base58_string();

    let admin_pubkey = t.data.get_wallet("admin").keypair.pubkey().to_string();
    let authority_pubkey = t.data.get_wallet("authority").keypair.pubkey().to_string();

    t.print("Check that initial authority matches the key of contract deployer");
    assert_json_output!(
        t.exec(["authority", "get"]),
        json!({ "authority": &admin_pubkey})
    );

    t.print("Assign authority to a new address");
    std::env::set_var("PRIVATE_KEY", &admin_secret);
    assert_tx_output!(t.exec(["authority", "set", &authority_pubkey]));

    t.print("Make sure that authority has changed");
    assert_json_output!(
        t.exec(["authority", "get"]),
        json!({ "authority": &authority_pubkey })
    );

    t.print("Use the private key of the new authority to change the authority back");
    assert_tx_output!(t.exec([
        "authority",
        "set",
        &admin_pubkey,
        "--private-key",
        &authority_secret,
    ]));

    t.print("Make sure that authority has changed back");
    assert_json_output!(
        t.exec(["authority", "get"]),
        json!({ "authority": &admin_pubkey })
    );
}
