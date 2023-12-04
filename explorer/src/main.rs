use hapi_explorer::{
    application::Application, configuration::get_configuration, observability::setup_tracing,
};

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration.");
    setup_tracing(&configuration.log_level, configuration.is_json_logging)
        .expect("Failed to set up tracing");

    let app = Application::from_configuration(configuration)
        .await
        .expect("Failed to build application");

    app.run_server().await.expect("Failed to run server");
}
