use super::{
    CallExecutionDetailsExtension, GasExtension, U128Extension, ViewResultDetailsExtension,
};
use near_contract_standards::storage_management::StorageBalanceBounds;
use near_sdk::{json_types::U128, serde_json::json, ONE_YOCTO};
use workspaces::{result::ExecutionFinalResult, AccountId, Contract};

use crate::context::TestContext;

pub const TOKEN: &[u8] = include_bytes!("../../../res/fungible_token.wasm");

pub const STAKE_TOKEN_DECIMAL: u8 = 6;
pub const REWARD_TOKEN_DECIMAL: u8 = 18;

pub type Decimals = u8;

#[derive(Clone)]
pub struct Token {
    pub id: AccountId,
    pub contract: Contract,
    pub decimals: Decimals,
}

/*
 * TOKEN contract functions
 */
impl TestContext {
    pub async fn init_token(&self, token: &Token) {
        token
            .contract
            .call("new")
            .args_json(json!({
                "owner_id": token.id,
                "total_supply": 10000_u128.to_decimals(token.decimals).to_string(),
                "metadata": {
                    "spec": "ft-1.0.0",
                    "name": format!("Example Token Name {}", token.decimals),
                    "symbol": format!("EXAMPLE {}", token.decimals),
                    "decimals": token.decimals
                }
            }
            ))
            .transact()
            .await
            .assert_success(format!("init token with decimals {}", token.decimals).as_str());
    }

    pub async fn storage_deposit(&self, token: &Token, account_id: &workspaces::AccountId) {
        let storage_deposit = self.storage_balance_bounds(token.clone()).await;
        token
            .contract
            .call("storage_deposit")
            .args_json(json!({ "account_id": account_id }))
            .deposit(storage_deposit.min.0)
            .transact()
            .await
            .assert_success("storage deposit");
    }

    pub async fn storage_balance_bounds(&self, token: Token) -> StorageBalanceBounds {
        token
            .contract
            .view("storage_balance_bounds")
            .await
            .parse("storage balance bounds")
    }

    pub async fn ft_transfer(
        &self,
        token: &Token,
        receiver_id: &workspaces::AccountId,
        amount: u128,
    ) {
        token
            .contract
            .call("ft_transfer")
            .args_json(json!({
                "receiver_id": receiver_id,
                "amount": amount.to_decimals(token.decimals).to_string()
            }))
            .deposit(ONE_YOCTO)
            .transact()
            .await
            .assert_success("transfer token");
    }

    pub async fn ft_transfer_call(
        &self,
        sender: &workspaces::Account,
        token: &Token,
        amount: u128,
    ) -> Result<ExecutionFinalResult, workspaces::error::Error> {
        sender
            .call(&token.id, "ft_transfer_call")
            .args_json(json!({
                "receiver_id": self.contract.id(),
                "amount": amount.to_decimals(token.decimals).to_string(),
                "msg": format!("")
            }))
            .deposit(ONE_YOCTO)
            .gas(60.to_tgas())
            .transact()
            .await
    }

    pub async fn ft_balance_of(&self, token: &Token, account_id: &workspaces::AccountId) -> U128 {
        token
            .contract
            .view("ft_balance_of")
            .args_json(json!({ "account_id": account_id }))
            .await
            .parse("get token balance")
    }
}
