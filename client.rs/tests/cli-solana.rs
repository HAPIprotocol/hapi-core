use anchor_client::solana_sdk::signature::Signer;
use serde_json::json;

mod assert;
mod cmd_utils;
mod solana;

use solana::setup::Setup;
use spl_token::solana_program::pubkey::Pubkey;

// #[tokio::test]
#[tokio::test(flavor = "multi_thread")]
async fn solana_cli_works() {
    println!("Running solana-cli tests");
    let t = Setup::new().await;

    let unlock_duration = 3;
    let validator_stake = 10;
    let tracer_stake = 11;
    let publisher_stake = 12;
    let authority_stake = 13;
    let address_confirmation_reward = 5;
    let address_tracer_reward = 6;
    let asset_confirmation_reward = 7;
    let asset_tracer_reward = 8;

    let admin_secret = t.data.get_wallet("admin").keypair.to_base58_string();
    let authority_secret = t.data.get_wallet("authority").keypair.to_base58_string();

    let admin_pubkey = t.data.get_wallet("admin").keypair.pubkey().to_string();
    let authority_pubkey = t.data.get_wallet("authority").keypair.pubkey().to_string();
    let mint = t.data.get_wallet("mint").keypair.pubkey().to_string();

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

    t.print("Check that initial stake configuration is empty");
    assert_json_output!(
        t.exec(["configuration", "get-stake"]),
        json!({
            "configuration": {
                "token": Pubkey::default().to_string(),
                "unlock_duration": 0,
                "validator_stake": 0.to_string(),
                "tracer_stake": 0.to_string(),
                "publisher_stake": 0.to_string(),
                "authority_stake": 0.to_string()
            }
        })
    );

    t.print("Update stake configuration");
    assert_tx_output!(t.exec([
        "configuration",
        "update-stake",
        &mint,
        &unlock_duration.to_string(),
        &validator_stake.to_string(),
        &tracer_stake.to_string(),
        &publisher_stake.to_string(),
        &authority_stake.to_string(),
    ]));

    t.print("Make sure that the new stake configuration is applied");
    assert_json_output!(
        t.exec(["configuration", "get-stake"]),
        json!({
            "configuration": {
                "token": mint,
                "unlock_duration": unlock_duration,
                "validator_stake": validator_stake.to_string(),
                "tracer_stake": tracer_stake.to_string(),
                "publisher_stake": publisher_stake.to_string(),
                "authority_stake": authority_stake.to_string()
            }
        })
    );

    t.print("Check that initial reward configuration is empty");
    assert_json_output!(
        t.exec(["configuration", "get-reward"]),
        json!({
            "configuration": {
                "token": Pubkey::default().to_string(),
                "address_confirmation_reward": 0.to_string(),
                "address_tracer_reward": 0.to_string(),
                "asset_confirmation_reward": 0.to_string(),
                "asset_tracer_reward": 0.to_string()
            }
        })
    );

    t.print("Update reward configuration");
    assert_tx_output!(t.exec([
        "configuration",
        "update-reward",
        &mint,
        &address_confirmation_reward.to_string(),
        &address_tracer_reward.to_string(),
        &asset_confirmation_reward.to_string(),
        &asset_tracer_reward.to_string(),
    ]));

    t.print("Make sure that the new reward configuration is applied");
    assert_json_output!(
        t.exec(["configuration", "get-reward"]),
        json!({
            "configuration": {
                "token": mint,
                "address_confirmation_reward": address_confirmation_reward.to_string(),
                "address_tracer_reward": address_tracer_reward.to_string(),
                "asset_confirmation_reward": asset_confirmation_reward.to_string(),
                "asset_tracer_reward": asset_tracer_reward.to_string()
            }
        })
    );
}
