use hapi_explorer::{
    application::Application, configuration::get_configuration, observability::setup_tracing,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_configuration()?;
    setup_tracing(&configuration.log_level, configuration.is_json_logging)?;

    let app = Application::from_configuration(configuration).await?;

    app.run_server().await
}
