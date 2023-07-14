use near_contract_standards::fungible_token::{core::ext_ft_core, receiver::FungibleTokenReceiver};
use near_sdk::{
    env, ext_contract, is_promise_success, json_types::U128, near_bindgen, AccountId, Gas, Promise,
    PromiseOrValue, ONE_YOCTO,
};

const GAS_FOR_FT_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_AFTER_FT_TRANSFER: Gas = Gas(10_000_000_000_000);

use crate::{reporter, Contract, ContractExt};

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn after_transfer_stake(&mut self, reporter_account: AccountId, amount: U128);
}

#[near_bindgen]
impl ExtSelf for Contract {
    #[private]
    fn after_transfer_stake(&mut self, reporter_account: AccountId, amount: U128) {
        if !is_promise_success() {
            let mut reporter = self.get_reporter_by_account(reporter_account);
            reporter.stake = amount;
            reporter.status = reporter::ReporterStatus::Unstaking;
            self.reporters
                .insert(&reporter.id.clone(), &reporter.into());
        }
    }
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    #[allow(unused_variables)]
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        self.stake_configuration.assert_token_valid();

        self.activate_reporter(sender_id, amount);
        PromiseOrValue::Value(U128(0))
    }
}

// #[near_bindgen]
impl Contract {
    pub(crate) fn transfer_stake(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        token_account_id: AccountId,
    ) -> Promise {
        ext_ft_core::ext(token_account_id.clone())
            .with_static_gas(GAS_FOR_FT_TRANSFER)
            .with_attached_deposit(ONE_YOCTO)
            .ft_transfer(
                receiver_id.clone(),
                amount,
                Some(format!("Transfer {} of {token_account_id}", amount.0)),
            )
            .then(
                ext_self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_AFTER_FT_TRANSFER)
                    .after_transfer_stake(receiver_id, amount),
            )
    }
}
