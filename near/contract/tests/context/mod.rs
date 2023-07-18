use crate::{
    utils::{
        create_account, deploy_contract, CallExecutionDetailsExtension, Token,
        REWARD_TOKEN_DECIMAL, STAKE_TOKEN_DECIMAL, TOKEN,
    },
    CONTRACT, INITIAL_USER_BALANCE,
};
use workspaces::{network::Sandbox, Account, Contract, Worker};

pub struct TestContext {
    pub worker: Worker<Sandbox>,

    pub authority: Account,
    pub user_1: Account,
    pub user_2: Account,

    pub contract: Contract,

    pub stake_token: Token,
    pub reward_token: Token,
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
            .transact()
            .await
            .assert_success("init contract");

        this
    }
}
