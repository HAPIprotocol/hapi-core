use crate::{
    context::TestContext,
    reporter::Role,
    utils::{CallExecutionDetailsExtension, ViewResultDetailsExtension},
    ERROR_CASE_ALREADY_EXISTS, ERROR_INVALID_ROLE,
};
mod helpers;
pub use helpers::CaseId;
use helpers::{Case, CaseStatus};
use near_sdk::{json_types::U128, serde_json::json};
use uuid::Uuid;

#[tokio::test]
async fn test_case() {
    let context = TestContext::new().await;

    let publisher_id = U128(Uuid::new_v4().as_u128());
    let tracer_id = U128(Uuid::new_v4().as_u128());
    let case_id = U128(Uuid::new_v4().as_u128());
    let case_id_2 = U128(Uuid::new_v4().as_u128());

    // prepare reporter(Publisher)
    context
        .authority
        .call(&context.contract.id(), "update_stake_configuration")
        .args_json(json!({"stake_configuration":context.get_stake_configuration().await}))
        .transact()
        .await
        .assert_success("update stake configuration");

    context
        .prepare_reporter(publisher_id, &context.user_1, Role::Publisher)
        .await;

    // create case
    context
        .user_1
        .call(&context.contract.id(), "create_case")
        .args_json(json!({"id": case_id, "name": "case", "url": "case.com"}))
        .transact()
        .await
        .assert_success("create case");

    // check case
    let case: Case = context
        .user_1
        .view(&context.contract.id(), "get_case")
        .args_json(json!({ "id": case_id }))
        .await
        .parse("get case");

    assert_eq!(case.status, CaseStatus::Open, "wrong status");
    assert_eq!(case.reporter_id, publisher_id, "wrong reporter id");

    // create case with the same id
    context
        .user_1
        .call(&context.contract.id(), "create_case")
        .args_json(json!({"id": case_id, "name": "case", "url": "case.com"}))
        .transact()
        .await
        .assert_failure("create case", ERROR_CASE_ALREADY_EXISTS);

    // update case
    context
        .user_1
        .call(&context.contract.id(), "update_case")
        .args_json(json!({"id": case_id, "name": "case", "status": "Closed", "url": "case.com"}))
        .transact()
        .await
        .assert_success("update case");

    // check case
    let case: Case = context
        .user_1
        .view(&context.contract.id(), "get_case")
        .args_json(json!({ "id": case_id }))
        .await
        .parse("get case");

    assert_eq!(case.status, CaseStatus::Closed, "wrong status");

    // create second case
    context
        .user_1
        .call(&context.contract.id(), "create_case")
        .args_json(json!({"id": case_id_2, "name": "case", "url": "case.com"}))
        .transact()
        .await
        .assert_success("create case");

    // check number of cases
    let number_of_cases: u64 = context
        .user_1
        .view(&context.contract.id(), "get_case_count")
        .args_json(json!({}))
        .await
        .parse("get cases count");

    assert_eq!(number_of_cases, 2, "wrong number of cases");

    // prepare reporter(Tracer)
    context
        .authority
        .call(&context.contract.id(), "update_stake_configuration")
        .args_json(json!({"stake_configuration":context.get_stake_configuration().await}))
        .transact()
        .await
        .assert_success("update stake configuration");

    context
        .prepare_reporter(tracer_id, &context.user_2, Role::Tracer)
        .await;

    // create case
    context
        .user_2
        .call(&context.contract.id(), "create_case")
        .args_json(json!({"id": case_id, "name": "case", "url": "case.com"}))
        .transact()
        .await
        .assert_failure("create case", ERROR_INVALID_ROLE);

    // get cases
    let cases: Vec<Case> = context
        .user_2
        .view(&context.contract.id(), "get_cases")
        .args_json(json!({"skip": 0, "take": 10}))
        .await
        .parse("get cases");

    assert_eq!(cases.len(), 2, "wrong number of cases");
    assert_eq!(cases[1].status, CaseStatus::Open, "wrong status");
}
