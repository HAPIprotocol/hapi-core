use super::{create_jwt, get_test_data, RequestSender, TestData};

use {
    hapi_core::{client::events::EventName, HapiCoreNetwork},
    hapi_explorer::{
        application::Application,
        configuration::Configuration,
        entity::{address, asset, case, network::Model as NetworkModel, reporter},
        observability::setup_tracing,
    },
    hapi_indexer::{PushData, PushPayload},
    sea_orm::{DatabaseConnection, EntityTrait},
    std::{env, sync::Arc},
    tokio::{
        spawn,
        sync::Notify,
        task::JoinHandle,
        time::{sleep, Duration},
    },
};

pub const WAITING_INTERVAL: u64 = 100;
pub const MIGRATION_COUNT: u32 = 10;
const TRACING_ENV_VAR: &str = "ENABLE_TRACING";

pub struct TestNetwork {
    pub backend: HapiCoreNetwork,
    pub model: NetworkModel,
    pub token: String,
}

pub struct TestApp {
    pub server_addr: String,
    pub db_connection: DatabaseConnection,
    pub networks: Vec<TestNetwork>,
    pub server_handle: Option<JoinHandle<()>>,
    stop_signal: Arc<Notify>,
}

impl TestApp {
    pub async fn start() -> Self {
        if env::var(TRACING_ENV_VAR).unwrap_or_default().eq("1") {
            if let Err(e) = setup_tracing("debug", false) {
                println!("Failed to setup tracing: {}", e);
            }
        }

        let configuration = generate_configuration();

        let mut app = Application::from_configuration(configuration.clone())
            .await
            .expect("Failed to build app");

        let db_connection = TestApp::prepare_database(&app).await;
        let networks = Self::prepare_networks(&app).await;
        let port = app.port();

        let stop_signal = Arc::new(Notify::new());
        let receiver = stop_signal.clone();

        // Spawn a background task
        let server_handle = spawn(async move {
            app.run_server().await.expect("Failed to run server");
            receiver.notified().await;
            app.shutdown().await.expect("Failed to shutdown app");
        });

        sleep(Duration::from_millis(WAITING_INTERVAL)).await;

        TestApp {
            server_addr: format!("http://127.0.0.1:{}", port),
            db_connection,
            server_handle: Some(server_handle),
            networks,
            stop_signal,
        }
    }

    pub async fn shutdown(&mut self) {
        self.stop_signal.notify_one();

        if let Some(handle) = self.server_handle.take() {
            handle.await.expect("Failed to shutdown server");
        };
    }

    pub async fn prepare_database(app: &Application) -> DatabaseConnection {
        let db_connection = app.state.database_conn.clone();

        app.migrate(Some(sea_orm_cli::MigrateSubcommands::Down {
            num: MIGRATION_COUNT,
        }))
        .await
        .expect("Failed to migrate down");

        app.migrate(None).await.expect("Failed to migrate up");

        db_connection
    }

    async fn prepare_networks(app: &Application) -> Vec<TestNetwork> {
        let networks = vec![
            HapiCoreNetwork::Ethereum,
            HapiCoreNetwork::Solana,
            HapiCoreNetwork::Near,
            HapiCoreNetwork::Sepolia,
        ];

        let mut res = vec![];

        for network in &networks {
            let id = network.to_string();
            let name = "test_name".to_string();
            let backend = network.clone().into();
            let chain_id = Some("test_chain_id".to_string());
            let authority = "test_authority".to_string();
            let stake_token = "test_stake_token".to_string();

            app.create_network(
                id.clone(),
                name.clone(),
                backend,
                chain_id.clone(),
                authority.clone(),
                stake_token.clone(),
            )
            .await
            .expect("Failed to create network");

            let token = app
                .create_indexer(backend.to_owned().into())
                .await
                .expect("Failed to create indexer");

            let data = TestNetwork {
                backend: network.to_owned(),
                model: NetworkModel {
                    id,
                    name,
                    backend,
                    chain_id,
                    authority,
                    stake_token,
                    created_at: chrono::Utc::now().naive_utc(),
                    updated_at: chrono::Utc::now().naive_utc(),
                },
                token,
            };

            res.push(data);
        }

        res
    }

    pub async fn check_entity(&self, data: PushData, network: HapiCoreNetwork) {
        let network = network.into();
        let db = &self.db_connection;

        match data {
            PushData::Address(address) => {
                let result = address::Entity::find_by_id((network, address.address.clone()))
                    .all(db)
                    .await
                    .expect("Failed to find address by id");

                assert_eq!(result.len(), 1);

                let address_model = result.first().unwrap();
                assert_eq!(address_model.address, address.address);
                assert_eq!(address_model.network, network);
                assert_eq!(address_model.case_id, address.case_id);
                assert_eq!(address_model.reporter_id, address.reporter_id);
                assert_eq!(address_model.risk, address.risk as i16);
                assert_eq!(address_model.category, address.category.into());
                assert_eq!(
                    address_model.confirmations,
                    address.confirmations.to_string()
                );
            }
            PushData::Asset(asset) => {
                let result = asset::Entity::find_by_id((
                    network,
                    asset.address.clone(),
                    asset.asset_id.to_string(),
                ))
                .all(db)
                .await
                .expect("Failed to find asset by id");

                assert_eq!(result.len(), 1);

                let asset_model = result.first().unwrap();
                assert_eq!(asset_model.address, asset.address);
                assert_eq!(asset_model.asset_id, asset.asset_id.to_string());
                assert_eq!(asset_model.case_id, asset.case_id);
                assert_eq!(asset_model.reporter_id, asset.reporter_id);
                assert_eq!(asset_model.risk, asset.risk as i16);
                assert_eq!(asset_model.category, asset.category.into());
                assert_eq!(asset_model.confirmations, asset.confirmations.to_string());
            }
            PushData::Case(case) => {
                let result = case::Entity::find_by_id((network, case.id.clone()))
                    .all(db)
                    .await
                    .expect("Failed to find case by id");

                assert_eq!(result.len(), 1);

                let case_model = result.first().unwrap();
                assert_eq!(case_model.name, case.name);
                assert_eq!(case_model.url, case.url);
                assert_eq!(case_model.status, case.status.into());
                assert_eq!(case_model.reporter_id, case.reporter_id);
            }
            PushData::Reporter(reporter) => {
                let result = reporter::Entity::find_by_id((network, reporter.id.clone()))
                    .all(db)
                    .await
                    .expect("Failed to find reporter by id");

                assert_eq!(result.len(), 1);

                let reporter_model = result.first().unwrap();
                assert_eq!(reporter_model.account, reporter.account);
                assert_eq!(reporter_model.role, reporter.role.into());
                assert_eq!(reporter_model.status, reporter.status.into());
                assert_eq!(reporter_model.name, reporter.name);
                assert_eq!(reporter_model.url, reporter.url);
                assert_eq!(reporter_model.stake, reporter.stake.to_string());
                assert_eq!(
                    reporter_model.unlock_timestamp,
                    reporter.unlock_timestamp.to_string()
                );
            }
        }
    }

    // Sends provided data or default events for all networks
    pub async fn setup_entities<T>(
        &self,
        sender: &RequestSender,
        event: EventName,
        test_data: Option<Vec<PushPayload>>,
    ) -> Vec<TestData<T>>
    where
        TestData<T>: From<PushPayload>,
    {
        let mut data = vec![];

        let test_data = if let Some(data) = test_data {
            data
        } else {
            self.networks
                .iter()
                .map(|network| get_test_data(network.backend.to_owned()))
                .flatten()
                .collect()
        };

        let token = create_jwt("my_ultra_secure_secret");

        for payload in test_data {
            sender
                .send("events", &payload, &token)
                .await
                .expect("Failed to send event");
            sleep(Duration::from_millis(WAITING_INTERVAL)).await;

            if event == payload.event.name {
                data.push(payload.into());
            }
        }

        data
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        self.stop_signal.notify_one();
    }
}

pub fn generate_configuration() -> Configuration {
    let mut configuration = Configuration::default();
    configuration.listener = "127.0.0.1:0".parse().expect("Failed to parse address");

    // TODO: implement db docker setup script
    configuration.database_url = env::var("DATABASE_URL")
        .unwrap_or("postgresql://postgres:postgres@localhost:5432/explorer".to_string());

    configuration
}
