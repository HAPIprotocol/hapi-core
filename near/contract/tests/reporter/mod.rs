use crate::*;

mod helpers;
use helpers::{Reporter, Role};
use uuid::Uuid;

#[tokio::test]
async fn view_reporter() -> anyhow::Result<()> {
    let context = TestContext::new().await;

    let id = Uuid::new_v4();

    // get_reporter(fail)
    context
        .authority
        .call(&context.contract.id(), "get_reporter")
        .args_json(json!({"id": id.to_string()}))
        .transact()
        .await
        .assert_failure("get reporter", REPORTER_NOT_FOUND);

    // create_reporter
    context
        .authority
        .call(&context.contract.id(), "create_reporter")
        .args_json(json!({"id": id.to_string(), "account_id": context.user_1.id(), "name": "reporter", "role": "Publisher", "url": "reporter.com"}))
        .transact()
        .await
        .assert_success("create reporter");

    // get_reporter
    let reporter: Reporter = context
        .authority
        .view(&context.contract.id(), "get_reporter")
        .args_json(json!({"id": id.to_string()}))
        .await
        .get_result("get_reporter");

    assert_eq!(reporter.role, Role::Publisher, "wrong role");

    Ok(())
}
