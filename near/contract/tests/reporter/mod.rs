use crate::{
    context::TestContext,
    errors::ERROR_REPORTER_NOT_FOUND,
    utils::{CallExecutionDetailsExtension, GasExtension, ViewResultDetailsExtension},
    U128Extension, ERROR_REPORTER_IS_ACTIVE, ERROR_REPORTER_IS_INACTIVE,
    ERROR_UNLOCK_DURATION_NOT_PASSED, INITIAL_USER_BALANCE, STAKE_AMOUNTS, UNLOCK_DURATION,
};
use near_sdk::{json_types::U128, serde_json::json};

mod helpers;
pub use helpers::{Reporter, ReporterId, ReporterStatus, Role};
use uuid::Uuid;

#[tokio::test]
async fn test_reporter() {
    let context = TestContext::new().await;

    let id = U128(Uuid::new_v4().as_u128());

    context
        .authority
        .call(&context.contract.id(), "update_stake_configuration")
        .args_json(json!({"stake_configuration":context.get_stake_configuration().await}))
        .transact()
        .await
        .assert_success("update stake configuration");

    // get_reporter(fail)
    context
        .authority
        .call(&context.contract.id(), "get_reporter")
        .args_json(json!({ "id": id }))
        .transact()
        .await
        .assert_failure("get reporter", ERROR_REPORTER_NOT_FOUND);

    // create_reporter
    context
        .authority
        .call(&context.contract.id(), "create_reporter")
        .args_json(json!({"id": id, "account_id": context.user_1.id(), "name": "reporter", "role": "Publisher", "url": "reporter.com"}))
        .transact()
        .await
        .assert_success("create reporter");

    // get_reporter
    let reporter: Reporter = context
        .authority
        .view(&context.contract.id(), "get_reporter")
        .args_json(json!({ "id": id }))
        .await
        .parse("get_reporter");

    assert_eq!(reporter.role, Role::Publisher, "wrong role");
    assert_eq!(reporter.status, ReporterStatus::Inactive, "wrong status");

    // activate reporter
    context
        .ft_transfer_call(
            &context.user_1,
            &context.stake_token,
            STAKE_AMOUNTS.publisher.0,
        )
        .await
        .assert_success("activate reporter");

    // check reporter activated
    let reporter: Reporter = context
        .authority
        .view(&context.contract.id(), "get_reporter")
        .args_json(json!({ "id": id }))
        .await
        .parse("get_reporter");

    assert_eq!(reporter.status, ReporterStatus::Active, "wrong status");

    // update reporter
    context
        .authority
        .call(&context.contract.id(), "update_reporter")
        .args_json(json!({"id": id, "account_id": context.user_1.id(), "name": "reporter", "role": "Publisher", "url": "reporter.hapi"}))
        .transact()
        .await
        .assert_success("update reporter");

    // check reporter updated
    let reporter: Reporter = context
        .authority
        .view(&context.contract.id(), "get_reporter")
        .args_json(json!({ "id": id }))
        .await
        .parse("get_reporter");

    assert_eq!(reporter.url, "reporter.hapi", "wrong url");

    // try to unstake
    context
        .user_1
        .call(&context.contract.id(), "unstake")
        .gas(60.to_tgas())
        .transact()
        .await
        .assert_failure("unstake", ERROR_REPORTER_IS_ACTIVE);

    // deactivate reporter
    context
        .user_1
        .call(&context.contract.id(), "deactivate_reporter")
        .transact()
        .await
        .assert_success("deactivate reporter");

    let reporter: Reporter = context
        .authority
        .view(&context.contract.id(), "get_reporter")
        .args_json(json!({ "id": id }))
        .await
        .parse("get_reporter");

    assert_eq!(reporter.status, ReporterStatus::Unstaking, "wrong status");

    // try to unstake
    context
        .user_1
        .call(&context.contract.id(), "unstake")
        .gas(60.to_tgas())
        .transact()
        .await
        .assert_failure("unstake", ERROR_UNLOCK_DURATION_NOT_PASSED);

    // wait for unlock duration
    context.fast_forward(UNLOCK_DURATION).await;

    // unstake
    context
        .user_1
        .call(&context.contract.id(), "unstake")
        .gas(60.to_tgas())
        .transact()
        .await
        .assert_success("unstake");

    // check reporter unstaked
    let reporter: Reporter = context
        .authority
        .view(&context.contract.id(), "get_reporter")
        .args_json(json!({ "id": id }))
        .await
        .parse("get_reporter");

    assert_eq!(reporter.status, ReporterStatus::Inactive, "wrong status");
    assert_eq!(reporter.stake.0, 0, "wrong stake");

    let balance = context
        .ft_balance_of(&context.stake_token, context.user_1.id())
        .await;
    assert_eq!(
        balance.0,
        INITIAL_USER_BALANCE.to_decimals(context.stake_token.decimals),
        "wrong balance"
    );

    // try to unstake again
    context
        .user_1
        .call(&context.contract.id(), "unstake")
        .gas(60.to_tgas())
        .transact()
        .await
        .assert_failure("unstake", ERROR_REPORTER_IS_INACTIVE);
}
