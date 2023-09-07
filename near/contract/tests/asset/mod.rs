use near_sdk::{json_types::U128, serde_json::json};
use uuid::Uuid;

use crate::{
    context::TestContext,
    reporter::Role,
    utils::{CallExecutionDetailsExtension, ViewResultDetailsExtension},
    ERROR_CASE_NOT_FOUND,
};

mod helpers;
use crate::address::Category;
use helpers::Asset;

#[tokio::test]
async fn test_asset() {
    let context = TestContext::new().await;

    let validator_id = U128(Uuid::new_v4().as_u128());
    let tracer_id = U128(Uuid::new_v4().as_u128());
    let authority_id = U128(Uuid::new_v4().as_u128());
    let case_id = U128(Uuid::new_v4().as_u128());

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

    // create asset without existed case
    context
        .user_1
        .call(&context.contract.id(), "create_asset")
        .args_json(json!({"address": "test.near","id": "10",  "category": "Scam", "risk_score": 1, "case_id": case_id, "reporter_id": tracer_id}))
        .transact()
        .await
        .assert_failure("create asset", ERROR_CASE_NOT_FOUND);

    context
        .prepare_reporter(authority_id, &context.authority, Role::Authority)
        .await;

    // create case
    context
        .authority
        .call(&context.contract.id(), "create_case")
        .args_json(json!({"id": case_id, "name": "case", "url": "case.com"}))
        .transact()
        .await
        .assert_success("create case");

    // create asset
    context
        .user_1
        .call(&context.contract.id(), "create_asset")
        .args_json(json!({"address": "test.near","id": "10", "category": "TerroristFinancing", "risk_score": 1, "case_id": case_id, "reporter_id": tracer_id}))
        .transact()
        .await
        .assert_success("create asset");

    // prepare validator
    context
        .prepare_reporter(validator_id, &context.user_2, Role::Validator)
        .await;

    // confirm asset
    context
        .user_2
        .call(&context.contract.id(), "confirm_asset")
        .args_json(json!({"address": "test.near","id": "10"}))
        .transact()
        .await
        .assert_success("confirm asset");

    // check asset
    let asset: Asset = context
        .user_1
        .view(&context.contract.id(), "get_asset")
        .args_json(json!({"address": "test.near","id": "10"}))
        .await
        .parse("get_asset");

    assert_eq!(asset.confirmations_count, 1);

    // update asset
    context
        .authority
        .call(&context.contract.id(), "update_asset")
        .args_json(json!({"address": "test.near","id": "10", "category": "Scam", "risk_score": 5, "case_id": case_id}))
        .transact()
        .await
        .assert_success("update asset");

    // check asset
    let asset: Asset = context
        .user_1
        .view(&context.contract.id(), "get_asset")
        .args_json(json!({"address": "test.near","id": "10"}))
        .await
        .parse("get_asset");

    assert_eq!(asset.risk_score, 5);
    assert_eq!(asset.category, Category::Scam);
}
