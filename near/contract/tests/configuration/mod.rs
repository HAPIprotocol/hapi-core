use crate::{
    context::TestContext,
    errors::ERROR_ONLY_AUTHORITY,
    utils::{CallExecutionDetailsExtension, ViewResultDetailsExtension},
};
use near_sdk::serde_json::json;

mod helpers;
pub use helpers::*;

#[tokio::test]
async fn test_configuration() {
    let context = TestContext::new().await;

    context
        .authority
        .call(&context.contract.id(), "update_stake_configuration")
        .args_json(json!({"stake_configuration":context.get_stake_configuration().await}))
        .transact()
        .await
        .assert_success("update stake configuration");

    context
        .authority
        .call(&context.contract.id(), "update_reward_configuration")
        .args_json(json!({"reward_configuration":context.get_reward_configuration().await}))
        .transact()
        .await
        .assert_success("update reward configuration");

    // view configuration
    let stake_configuration:StakeConfiguration =
        context
            .authority
            .view(&context.contract.id(), "get_stake_configuration")
            .await
            .parse("get_stake_configuration");

    let reward_configuration:RewardConfiguration =
        context
            .authority
            .view(&context.contract.id(), "get_reward_configuration")
            .await
            .parse("get_reward_configuration");


    assert_eq!(
        stake_configuration.token, context.stake_token.id,
        "wrong stake token"
    );
    assert_eq!(
        stake_configuration.stake_amounts.validator,
        context
            .get_stake_configuration()
            .await
            .stake_amounts
            .validator,
        "wrong validator stake amount"
    );
    assert_eq!(reward_configuration.token, context.reward_token.id);
    assert_eq!(
        reward_configuration.reward_amounts.address_confirmation,
        context
            .get_reward_configuration()
            .await
            .reward_amounts
            .address_confirmation,
        "wrong address confirmation reward amount"
    );

    context
        .authority
        .call(&context.contract.id(), "set_authority")
        .args_json(json!({"authority": context.user_1.id()}))
        .transact()
        .await
        .assert_success("set authority");

    // check authority
    let authority: String = context
        .authority
        .view(&context.contract.id(), "get_authority")
        .await
        .parse("get_authority");

    assert_eq!(
        authority,
        context.user_1.id().to_string(),
        "wrong authority"
    );
}

// All methods must be called not from authority.

#[tokio::test]
async fn authority_methods() {
    let context = TestContext::new().await;

    //  update stake configuration(fail)
    context
        .user_1
        .call(&context.contract.id(), "update_stake_configuration")
        .args_json(json!({"stake_configuration":context.get_stake_configuration().await}))
        .transact()
        .await
        .assert_failure("update stake configuration", ERROR_ONLY_AUTHORITY);

    //  update reward configuration(fail)
    context
        .user_1
        .call(&context.contract.id(), "update_reward_configuration")
        .args_json(json!({"reward_configuration":context.get_reward_configuration().await}))
        .transact()
        .await
        .assert_failure("update reward configuration", ERROR_ONLY_AUTHORITY);

    // set authority(fail)
    context
        .user_1
        .call(&context.contract.id(), "set_authority")
        .args_json(json!({"authority": context.user_1.id()}))
        .transact()
        .await
        .assert_failure("set authority", ERROR_ONLY_AUTHORITY);
}
