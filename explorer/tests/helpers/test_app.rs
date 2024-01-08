use super::{get_test_data, RequestSender};

use {
    hapi_core::{client::events::EventName, HapiCoreNetwork},
    hapi_explorer::{
        application::Application,
        configuration::Configuration,
        entity::{address, asset, case, reporter},
        observability::setup_tracing,
    },
    hapi_indexer::PushData,
    migration::{Migrator, MigratorTrait},
    sea_orm::{Database, DatabaseConnection, EntityTrait},
    {
        std::env,
        tokio::{
            spawn,
            time::{sleep, Duration},
        },
    },
};

pub const WAITING_TIMESTAMP: u64 = 100;
const TRACING_ENV_VAR: &str = "ENABLE_TRACING";

pub struct TestApp {
    pub server_addr: String,
    pub db_connection: DatabaseConnection,
    pub networks: Vec<HapiCoreNetwork>,
}

impl TestApp {
    pub async fn start() -> Self {
        if env::var(TRACING_ENV_VAR).unwrap_or_default().eq("1") {
            if let Err(e) = setup_tracing("debug", false) {
                println!("Failed to setup tracing: {}", e);
            }
        }

        let configuration = generate_configuration();

        Self::from_configuration(configuration).await
    }

    async fn prepare_networks(app: &Application) -> Vec<HapiCoreNetwork> {
        let networks = vec![
            HapiCoreNetwork::Ethereum,
            HapiCoreNetwork::Solana,
            HapiCoreNetwork::Near,
            HapiCoreNetwork::Sepolia,
        ];

        for network in &networks {
            app.create_network(
                network.to_string(),
                "name".to_string(),
                network.clone().into(),
                None,
                "authority".to_string(),
                "stake_token".to_string(),
            )
            .await
            .expect("Failed to create network")
        }

        networks
    }

    pub async fn from_configuration(configuration: Configuration) -> Self {
        let db_connection = Database::connect(configuration.database_url.as_str())
            .await
            .expect("Failed to connect to database");

        Migrator::down(&db_connection, None)
            .await
            .expect("Failed to migrate down");

        let application = Application::from_configuration(configuration)
            .await
            .expect("Failed to build application");
        let port = application.port();
        let networks = Self::prepare_networks(&application).await;

        spawn(application.run_server());
        sleep(Duration::from_millis(WAITING_TIMESTAMP)).await;

        TestApp {
            server_addr: format!("http://127.0.0.1:{port}"),
            db_connection,
            networks,
        }
    }

    pub async fn check_entity(&self, data: PushData, network: HapiCoreNetwork) {
        let network = network.into();
        match data {
            PushData::Address(address) => {
                let result = address::Entity::find_by_id((network, address.address.clone()))
                    .all(&self.db_connection)
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
                .all(&self.db_connection)
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
                    .all(&self.db_connection)
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
                    .all(&self.db_connection)
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

    pub async fn setup_entities(
        &self,
        sender: &RequestSender,
        event: EventName,
    ) -> Vec<(PushData, &HapiCoreNetwork)> {
        let mut data = vec![];

        for network in &self.networks {
            let test_data = get_test_data(network.to_owned());

            for payload in test_data {
                sender.send("events", &payload).await;
                sleep(Duration::from_millis(WAITING_TIMESTAMP)).await;

                if event == payload.event.name {
                    // if let PushData::Address(addr_payload) = payload.data {
                    //     data.push(payload.data);
                    // } else {
                    //     panic!("Wrong payload type");
                    // }

                    data.push((payload.data, network));
                }
            }
        }

        data
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
