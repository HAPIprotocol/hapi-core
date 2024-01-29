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

/// Gauge metric operation
pub enum MetricOp {
    Increment,
    Decrement,
}

impl Application {
    pub(crate) async fn setup_entity_metrics(&self) -> Result<()> {
        self.fetch_metrics::<reporter::Entity, _>(update_reporter_metrics)
            .await?;
        self.fetch_metrics::<case::Entity, _>(update_case_metrics)
            .await?;
        self.fetch_metrics::<address::Entity, _>(update_address_metrics)
            .await?;
        self.fetch_metrics::<asset::Entity, _>(update_asset_metrics)
            .await?;
        self.fetch_metrics::<network::Entity, _>(update_network_metrics)
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
        F: Fn(M::Model, MetricOp),
    {
        let page =
            EntityQuery::find_many::<M>(&self.state.database_conn, EntityInput::default()).await?;

        for entity in page.data {
            metric_fn(entity, MetricOp::Increment);
        }

        Ok(())
    }
}

pub fn update_reporter_metrics(model: reporter::Model, op: MetricOp) {
    let labels = vec![
        ("status", model.status.to_string()),
        ("role", model.role.to_string()),
    ];

    process_metric_op(REPORTER_METRIC, op, labels);
}

pub fn update_case_metrics(model: case::Model, op: MetricOp) {
    let labels = vec![("status", model.status.to_string())];

    process_metric_op(CASE_METRIC, op, labels);
}

pub fn update_address_metrics(model: address::Model, op: MetricOp) {
    let labels = vec![
        ("category", model.category.to_string()),
        ("risk", model.risk.to_string()),
    ];

    process_metric_op(ADDRESS_METRIC, op, labels);
}

pub fn update_asset_metrics(model: asset::Model, op: MetricOp) {
    let labels = vec![
        ("category", model.category.to_string()),
        ("risk", model.risk.to_string()),
    ];

    process_metric_op(ASSET_METRIC, op, labels);
}

pub fn update_network_metrics(model: network::Model, op: MetricOp) {
    let labels = vec![
        ("backend", model.backend.to_string()),
        ("authority", model.authority),
    ];

    process_metric_op(NETWORK_METRIC, op, labels);
}

fn process_metric_op(metric_name: &'static str, op: MetricOp, labels: Vec<(&'static str, String)>) {
    match op {
        MetricOp::Increment => {
            metrics::increment_gauge!(metric_name, 1.0, &labels);
        }
        MetricOp::Decrement => {
            metrics::decrement_gauge!(metric_name, 1.0, &labels);
        }
    }
}
