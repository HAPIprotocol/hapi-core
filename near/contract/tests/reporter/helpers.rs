use crate::{utils::CallExecutionDetailsExtension, STAKE_AMOUNTS};
use near_sdk::{
    json_types::U128,
    serde::{Deserialize, Serialize},
    serde_json::json,
    AccountId, Timestamp,
};
use workspaces::Account;

use crate::context::TestContext;
pub type ReporterId = U128;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum Role {
    Validator,
    Tracer,
    Publisher,
    Authority,
    Appraiser,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum ReporterStatus {
    Inactive,
    Active,
    Unstaking,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Reporter {
    pub id: ReporterId,
    pub account_id: AccountId,
    pub name: String,
    pub role: Role,
    pub status: ReporterStatus,
    pub stake: U128,
    pub url: String,
    pub unlock_timestamp: Timestamp,
}

impl TestContext {
    pub async fn prepare_reporter(&self, id: U128, account: &Account, role: Role) {
        let (role_str, amount) = match role {
            Role::Validator => ("Validator", STAKE_AMOUNTS.validator),
            Role::Tracer => ("Tracer", STAKE_AMOUNTS.tracer),
            Role::Publisher => ("Publisher", STAKE_AMOUNTS.publisher),
            Role::Authority => ("Authority", STAKE_AMOUNTS.authority),
            Role::Appraiser => ("Appraiser", U128(0)),
        };

        self.authority
            .call(&self.contract.id(), "create_reporter")
            .args_json(json!({"id": id, "account_id": account.id(), "name": role_str, "role": role_str, "url": role_str.to_lowercase() + ".com"}))
            .transact()
            .await
            .assert_success("create reporter");

        if amount.0 > 0 {
            self.ft_transfer_call(account, &self.stake_token, amount.0)
                .await
                .assert_success("activate reporter");
        }
    }
}
