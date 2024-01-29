use crate::helpers::{RequestSender, TestApp, METRICS_ENV_VAR, WAITING_INTERVAL};
use {
    hapi_core::client::{entities::address::Address, events::EventName},
    tokio::time::{sleep, Duration},
};

#[tokio::test]
async fn metrics_test() {
    std::env::set_var(METRICS_ENV_VAR, "1");
    let test_app = TestApp::start().await;

    let sender = RequestSender::new(test_app.server_addr.clone());
    test_app
        .global_setup::<Address>(&sender, EventName::UpdateAddress)
        .await;

    sleep(Duration::from_millis(WAITING_INTERVAL)).await;

    let metrics = sender
        .web_client
        .get(format!("{}/metrics", test_app.server_addr.clone()))
        .send()
        .await
        .expect("Failed to get metrics");

    assert!(metrics.status().is_success());

    let payload = metrics
        .text()
        .await
        .expect("Failed to get payload")
        .lines()
        .map(Into::into)
        .collect();

    let entity_count = test_app.networks.len();

    check_metric(
        &payload,
        "address",
        &[r#"category="DeFi""#, r#"risk="6""#],
        &[r#"category="Sanctions""#, r#"risk="10""#],
        entity_count,
    );

    check_metric(
        &payload,
        "asset",
        &[r#"category="Gambling""#, r#"risk="8""#],
        &[r#"category="Scam""#, r#"risk="9""#],
        entity_count,
    );

    check_metric(
        &payload,
        "case",
        &[r#"status="open""#],
        &[r#"status="closed""#],
        entity_count,
    );

    std::env::set_var(METRICS_ENV_VAR, "0");
}

fn split_string(s: &str, name: &str) -> Option<(String, String, usize)> {
    if s.contains("{") {
        let parts: Vec<&str> = s.splitn(2, '{').collect();

        let first_part = parts[0].to_string();

        if first_part.eq(name) {
            let second_parts: Vec<&str> = parts[1].splitn(2, '}').collect();
            let second_part = second_parts[0].to_string();

            let third_part: usize = second_parts[1]
                .trim()
                .parse::<usize>()
                .expect("Failed to get count");

            return Some((first_part, second_part, third_part));
        }
    }

    None
}

fn check_metric(
    payload: &Vec<String>,
    name: &str,
    old_lables: &[&str],
    new_lables: &[&str],
    entity_count: usize,
) {
    let mut metrics: Vec<(String, String, usize)> = payload
        .iter()
        .filter_map(|s| split_string(s, name))
        .collect();

    assert_eq!(metrics.len(), 2);

    metrics.sort_by_key(|&(_, _, after)| after);

    let (_, old, old_count) = metrics.first().unwrap();
    assert!(old_count.eq(&0));

    for lable in old_lables {
        assert!(old.contains(lable));
    }

    let (_, new, new_count) = metrics.last().unwrap();
    assert!(new_count.eq(&entity_count));

    for lable in new_lables {
        assert!(new.contains(lable));
    }
}
