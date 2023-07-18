use crate::{
    utils::{CallExecutionDetailsExtension, ViewResultDetailsExtension},
    TestContext,
};
use near_sdk::serde_json::json;

mod helpers;
use helpers::*;

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
    let (stake_configuration, reward_configuration): (StakeConfiguration, RewardConfiguration) =
        context
            .authority
            .view(&context.contract.id(), "get_configuration")
            .await
            .parse("get_configuration");

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
}
