mod helpers;
use crate::{
    context::TestContext,
    reporter::Role,
    utils::{CallExecutionDetailsExtension, ViewResultDetailsExtension},
    ERROR_CASE_NOT_FOUND,
};
use helpers::{Address, Category};
use near_sdk::serde_json::json;

use uuid::Uuid;

#[tokio::test]
async fn test_address() {
    let context = TestContext::new().await;

    let validator_id = Uuid::new_v4();
    let tracer_id = Uuid::new_v4();
    let authority_id = Uuid::new_v4();
    let case_id = Uuid::new_v4();

    // prepare reporter(Publisher)
    context
        .authority
        .call(&context.contract.id(), "update_stake_configuration")
        .args_json(json!({"stake_configuration":context.get_stake_configuration().await}))
        .transact()
        .await
        .assert_success("update stake configuration");

    context
        .prepare_reporter(tracer_id, &context.user_1, Role::Tracer)
        .await;

    // create address without existed case
    context
        .user_1
        .call(&context.contract.id(), "create_address")
        .args_json(json!({"address": "test.near", "category": "Scam", "risk_score": 1, "case_id": case_id.to_string(), "reporter_id": tracer_id.to_string()}))
        .transact()
        .await
        .assert_failure("create address", ERROR_CASE_NOT_FOUND);

    context
        .prepare_reporter(authority_id, &context.authority, Role::Authority)
        .await;

    // create case
    context
        .authority
        .call(&context.contract.id(), "create_case")
        .args_json(json!({"id": case_id.to_string(), "name": "case", "url": "case.com"}))
        .transact()
        .await
        .assert_success("create case");

    // create address
    context
        .user_1
        .call(&context.contract.id(), "create_address")
        .args_json(json!({"address": "test.near", "category": "TerroristFinancing", "risk_score": 1, "case_id": case_id.to_string(), "reporter_id": tracer_id.to_string()}))
        .transact()
        .await
        .assert_success("create address");

    // prepare validator
    context
        .prepare_reporter(validator_id, &context.user_2, Role::Validator)
        .await;

    // confirm address
    context
        .user_2
        .call(&context.contract.id(), "confirm_address")
        .args_json(json!({"address": "test.near"}))
        .transact()
        .await
        .assert_success("confirm address");

    // check address
    let address: Address = context
        .user_1
        .view(&context.contract.id(), "get_address")
        .args_json(json!({"address": "test.near"}))
        .await
        .parse("get_address");

    assert_eq!(address.confirmations_count, 1);

    // update address
    context
        .authority
        .call(&context.contract.id(), "update_address")
        .args_json(json!({"address": "test.near", "category": "Scam", "risk_score": 5, "case_id": case_id.to_string()}))
        .transact()
        .await
        .assert_success("update address");

    // check address
    let address: Address = context
        .user_1
        .view(&context.contract.id(), "get_address")
        .args_json(json!({"address": "test.near"}))
        .await
        .parse("get_address");

    assert_eq!(address.risk_score, 5);
    assert_eq!(address.category, Category::Scam);
}
