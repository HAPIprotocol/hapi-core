use hapi_explorer::{
    application::Application, configuration::Configuration, observability::setup_tracing,
};

use {
    std::env,
    tokio::{
        spawn,
        time::{sleep, Duration},
    },
};

const WAITING_TIMESTAMP: u64 = 100;
const TRACING_ENV_VAR: &str = "ENABLE_TRACING";

pub struct TestApp {
    pub server_addr: String,
    pub reqwest_client: reqwest::Client,
}

impl TestApp {
    pub async fn start() -> Self {
        if env::var(TRACING_ENV_VAR).unwrap_or_default().eq("1") {
            if let Err(e) = setup_tracing("debug", false) {
                println!("Failed to setup tracing: {}", e);
            }
        }

        let configuration = Self::generate_configuration();

        Self::from_configuration(configuration).await
    }

    pub async fn from_configuration(settings: Configuration) -> Self {
        let application =
            Application::from_configuration(settings).expect("Failed to build application");
        let port = application.port();

        spawn(application.run());
        sleep(Duration::from_millis(WAITING_TIMESTAMP)).await;

        TestApp {
            server_addr: format!("http://127.0.0.1:{port}"),
            reqwest_client: reqwest::Client::new(),
        }
    }

    pub fn generate_configuration() -> Configuration {
        let mut configuration = Configuration::default();
        configuration.listener = "127.0.0.1:0".parse().expect("Failed to parse address");

        configuration
    }
}
