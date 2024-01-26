use {
    anyhow::Result,
    async_graphql::{InputType, OutputType},
    axum::{extract::MatchedPath, http::Request, middleware::Next, response::IntoResponse},
    metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle},
    sea_orm::EntityTrait,
    std::time::Instant,
};

use crate::{
    application::Application,
    entity::{address, asset, case, network, pagination::EntityInput, reporter, EntityFilter},
    service::EntityQuery,
};

const REQUEST_DURATION_METRIC: &str = "http_requests_duration_seconds";
const REQUEST_DURATION_TOTAL: &str = "http_requests_total";

const REPORTER_METRIC: &str = "reporter";
const CASE_METRIC: &str = "case";
const ADDRESS_METRIC: &str = "address";
const ASSET_METRIC: &str = "asset";
const NETWORK_METRIC: &str = "network";

pub(crate) fn setup_metrics() -> PrometheusHandle {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full(REQUEST_DURATION_METRIC.to_string()),
            EXPONENTIAL_SECONDS,
        )
        .unwrap_or_else(|_| {
            panic!(
                "Could not initialize the bucket for '{}'",
                REQUEST_DURATION_METRIC
            )
        })
        .install_recorder()
        .expect("Could not install the Prometheus recorder")
}

pub(crate) async fn track_metrics<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();

    let response = next.run(req).await;

    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    metrics::increment_counter!(REQUEST_DURATION_TOTAL, &labels);
    metrics::histogram!(REQUEST_DURATION_METRIC, latency, &labels);

    response
}

impl Application {
    pub(crate) async fn setup_entity_metrics(&self) -> Result<()> {
        self.fetch_metrics::<reporter::Entity, _>(increament_reporter_metrics)
            .await?;
        self.fetch_metrics::<case::Entity, _>(increament_case_metrics)
            .await?;
        self.fetch_metrics::<address::Entity, _>(increament_address_metrics)
            .await?;
        self.fetch_metrics::<asset::Entity, _>(increament_asset_metrics)
            .await?;
        self.fetch_metrics::<network::Entity, _>(increament_network_metrics)
            .await?;

        Ok(())
    }

    async fn fetch_metrics<M, F>(&self, metric_fn: F) -> Result<()>
    where
        M: EntityTrait + EntityFilter,
        <M as EntityFilter>::Filter: InputType + Default,
        <M as EntityFilter>::Condition: InputType + Default,
        M::Model: OutputType,
        M::Column: From<<M as EntityFilter>::Condition>,
        F: Fn(M::Model),
    {
        let page =
            EntityQuery::find_many::<M>(&self.state.database_conn, EntityInput::default()).await?;

        for entity in page.data {
            metric_fn(entity);
        }

        Ok(())
    }
}

pub fn increament_reporter_metrics(model: reporter::Model) {
    let labels = [
        ("status", model.status.to_string()),
        ("role", model.role.to_string()),
    ];

    metrics::increment_counter!(REPORTER_METRIC, &labels);
}

pub fn increament_case_metrics(model: case::Model) {
    let labels = [
        ("reporter", model.reporter_id.to_string()),
        ("status", model.status.to_string()),
    ];

    metrics::increment_counter!(CASE_METRIC, &labels);
}

pub fn increament_address_metrics(model: address::Model) {
    let labels = [
        ("reporter", model.reporter_id.to_string()),
        ("category", model.category.to_string()),
        ("risk", model.risk.to_string()),
    ];

    metrics::increment_counter!(ADDRESS_METRIC, &labels);
}

pub fn increament_asset_metrics(model: asset::Model) {
    let labels = [
        ("reporter", model.reporter_id.to_string()),
        ("category", model.category.to_string()),
        ("risk", model.risk.to_string()),
    ];

    metrics::increment_counter!(ASSET_METRIC, &labels);
}

pub fn increament_network_metrics(model: network::Model) {
    let labels = [
        ("backend", model.backend.to_string()),
        ("authority", model.authority),
    ];

    metrics::increment_counter!(NETWORK_METRIC, &labels);
}
