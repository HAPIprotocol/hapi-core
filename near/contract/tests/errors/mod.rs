pub const ONLY_AUTHORITY: &str = "Only authority can call this method";

use crate::*;

// All methods must be called not from owner.

#[tokio::test]
async fn owner_methods() -> anyhow::Result<()> {
    let context = TestContext::new().await;

    //  update stake configuration(fail)
    context
        .user_1
        .call(&context.contract.id(), "update_stake_configuration")
        .args_json(json!({"stake_configuration":context.get_stake_configuration().await}))
        .transact()
        .await
        .assert_failure("update stake configuration", ONLY_AUTHORITY);

    //  update reward configuration(fail)
    context
        .user_1
        .call(&context.contract.id(), "update_reward_configuration")
        .args_json(json!({"reward_configuration":context.get_reward_configuration().await}))
        .transact()
        .await
        .assert_failure("update reward configuration", ONLY_AUTHORITY);

    // set authority(fail)
    context
        .user_1
        .call(&context.contract.id(), "set_authority")
        .args_json(json!({"authority": context.user_1.id()}))
        .transact()
        .await
        .assert_failure("set authority", ONLY_AUTHORITY);

    Ok(())
}
