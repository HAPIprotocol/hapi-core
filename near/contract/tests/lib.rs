use near_sdk::serde_json::json;
use workspaces::{network::Sandbox, Account, AccountId, Contract, Worker};

mod configuration;
pub mod errors;
mod reporter;
mod utils;
pub use errors::*;
pub use utils::*;

pub const SHOW_LOGS: bool = false;
pub const SHOW_DEFAULT_OUTPUT: bool = false;

const CONTRACT: &[u8] = include_bytes!("../../res/hapi_core_near.wasm");

pub const INITIAL_USER_BALANCE: u128 = 1000;
pub const INITIAL_NEAR_USER_BALANCE: u128 = 10000000000000000000000000; // 10 near

pub struct TestContext {
    worker: Worker<Sandbox>,

    authority: Account,
    user_1: Account,
    user_2: Account,

    contract: Contract,

    stake_token: Token,
    reward_token: Token,
}

impl TestContext {
    pub async fn new() -> Self {
        let worker = workspaces::sandbox().await.unwrap();

        let stake_token = deploy_contract(&worker, "stake", TOKEN).await;
        let reward_token = deploy_contract(&worker, "reward", TOKEN).await;

        let this = Self {
            worker: worker.clone(),

            authority: create_account(&worker, "authority").await,
            user_1: create_account(&worker, "user_1").await,
            user_2: create_account(&worker, "user_2").await,

            contract: deploy_contract(&worker, "hapi", CONTRACT).await,

            stake_token: Token {
                id: stake_token.id().clone(),
                contract: stake_token,
                decimals: STAKE_TOKEN_DECIMAL,
            },
            reward_token: Token {
                id: reward_token.id().clone(),
                contract: reward_token,
                decimals: REWARD_TOKEN_DECIMAL,
            },
        };

        for token in [this.stake_token.clone(), this.reward_token.clone()] {
            this.init_token(&token).await;

            this.storage_deposit(&token, this.authority.id()).await;
            this.storage_deposit(&token, this.user_1.id()).await;
            this.storage_deposit(&token, this.user_2.id()).await;
            this.storage_deposit(&token, this.contract.id()).await;
        }

        this.ft_transfer(&this.stake_token, this.authority.id(), 100)
            .await;

        this.ft_transfer(&this.stake_token, this.user_1.id(), INITIAL_USER_BALANCE)
            .await;
        this.ft_transfer(&this.stake_token, this.user_2.id(), INITIAL_USER_BALANCE)
            .await;

        this.ft_transfer(&this.reward_token, this.contract.id(), 1000)
            .await;

        this.authority
            .call(this.contract.id(), "initialize")
            .args_json(json!({}))
            .transact()
            .await
            .assert_success("init contract");

        this
    }
}

async fn create_account(worker: &Worker<Sandbox>, account_prefix: &str) -> Account {
    worker
        .root_account()
        .unwrap()
        .create_subaccount(account_prefix)
        .initial_balance(INITIAL_NEAR_USER_BALANCE)
        .transact()
        .await
        .unwrap()
        .result
}

async fn deploy_contract(worker: &Worker<Sandbox>, account_prefix: &str, wasm: &[u8]) -> Contract {
    create_account(worker, account_prefix)
        .await
        .deploy(wasm)
        .await
        .unwrap()
        .result
}
