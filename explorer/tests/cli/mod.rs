use {
    hapi_explorer::{
        entity::{indexer, network, types::NetworkBackend},
        migrations::Migrator,
    },
    sea_orm::{Database, DatabaseConnection, EntityTrait},
    sea_orm_migration::MigratorTrait,
    std::{env, process::Command},
    tokio::time::{sleep, Duration},
};

use crate::helpers::{RequestSender, MIGRATION_COUNT, WAITING_INTERVAL};

async fn setup() -> DatabaseConnection {
    env::set_var("CONFIG_PATH", "./configuration.sample.toml");

    let db = Database::connect("postgres://postgres:postgres@localhost:5432/explorer")
        .await
        .expect("Failed to connect database");

    Migrator::down(&db, Some(MIGRATION_COUNT))
        .await
        .expect("Failed to migrate down");

    Migrator::up(&db, None).await.expect("Failed to migrate up");

    db
}

#[tokio::test]
async fn server_command_test() {
    setup().await;

    Command::new("./target/debug/hapi-explorer")
        .args(["server"])
        .spawn()
        .expect("Failed to start server");

    sleep(Duration::from_millis(WAITING_INTERVAL)).await;

    let client = RequestSender::new("http://0.0.0.0:3000".to_string());

    client
        .get("health")
        .await
        .expect("Failed to get health check");
}

#[tokio::test]
async fn migrate_command_test() {
    let db = setup().await;

    let output = Command::new("./target/debug/hapi-explorer")
        .args([
            "migrate",
            "down",
            "--num",
            MIGRATION_COUNT.to_string().as_str(),
        ])
        .output()
        .expect("Failed to create network");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    sleep(Duration::from_millis(WAITING_INTERVAL)).await;

    assert_eq!(
        Migrator::get_applied_migrations(&db)
            .await
            .expect("Failed to get applied migrations")
            .len(),
        1
    );
}

#[tokio::test]
async fn network_command_test() {
    let db = setup().await;

    let id = String::from("test_network");
    let mut name = String::from("Test Network");
    let backend = NetworkBackend::Solana;
    let mut authority = String::from("Test Authority");
    let mut stake_token = String::from("Test Stake Token");

    let output = Command::new("./target/debug/hapi-explorer")
        .args([
            "network",
            "create",
            "--id",
            &id,
            "--name",
            &name,
            "--backend",
            &backend.to_string(),
            "--authority",
            &authority,
            "--stake-token",
            &stake_token,
        ])
        .output()
        .expect("Failed to create network");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    sleep(Duration::from_millis(WAITING_INTERVAL)).await;

    let result = network::Entity::find_by_id(id.clone())
        .all(&db)
        .await
        .expect("Failed to find address by id");

    assert_eq!(result.len(), 1);
    let network = result.first().unwrap();

    assert_eq!(network.id, id);
    assert_eq!(network.name, name);
    assert_eq!(network.backend, backend);
    assert_eq!(network.chain_id, None);
    assert_eq!(network.authority, authority);
    assert_eq!(network.stake_token, stake_token);

    name = String::from("Test Network 2");
    authority = String::from("Test Authority 2");
    stake_token = String::from("Test Stake Token 2");

    let output = Command::new("./target/debug/hapi-explorer")
        .args([
            "network",
            "update",
            "--id",
            &id,
            "--name",
            &name,
            "--authority",
            &authority,
            "--stake-token",
            &stake_token,
        ])
        .output()
        .expect("Failed to update network");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    sleep(Duration::from_millis(WAITING_INTERVAL)).await;

    let result = network::Entity::find_by_id(id.clone())
        .all(&db)
        .await
        .expect("Failed to find address by id");

    assert_eq!(result.len(), 1);
    let network = result.first().unwrap();

    assert_eq!(network.id, id);
    assert_eq!(network.name, name);
    assert_eq!(network.backend, backend);
    assert_eq!(network.chain_id, None);
    assert_eq!(network.authority, authority);
    assert_eq!(network.stake_token, stake_token);
}

#[tokio::test]
async fn indexer_command_test() {
    let db = setup().await;

    let id = String::from("test_network");
    let name = String::from("Test Network");
    let backend = NetworkBackend::Solana;
    let authority = String::from("Test Authority");
    let stake_token = String::from("Test Stake Token");

    let output = Command::new("./target/debug/hapi-explorer")
        .args([
            "network",
            "create",
            "--id",
            &id,
            "--name",
            &name,
            "--backend",
            &backend.to_string(),
            "--authority",
            &authority,
            "--stake-token",
            &stake_token,
        ])
        .output()
        .expect("Failed to create network");

    sleep(Duration::from_millis(WAITING_INTERVAL)).await;

    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    let output = Command::new("./target/debug/hapi-explorer")
        .args(["create-indexer", "--backend", &backend.to_string()])
        .output()
        .expect("Failed to create network");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    sleep(Duration::from_millis(WAITING_INTERVAL)).await;

    let indexers = indexer::Entity::find()
        .all(&db)
        .await
        .expect("Failed to find indexer");

    assert_eq!(indexers.len(), 1);

    let indexer = indexers.first().unwrap();
    assert_eq!(indexer.network_id, id);
}
