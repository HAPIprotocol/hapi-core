use near_sdk::{collections::UnorderedSet, env, near_bindgen, require, AccountId};

use crate::{
    case::CaseId, reporter::Role, Category, Contract, ContractExt, RiskScore, StorageKey,
    ERROR_ADDRESS_ALREADY_EXISTS, ERROR_ALREADY_CONFIRMED, ERROR_CASE_NOT_FOUND,
    ERROR_INVALID_RISK_SCORE, ERROR_INVALID_ROLE, ERROR_REPORTER_IS_INACTIVE,
    ERROR_REPORT_CONFIRMATION,
};

use super::Address;

const MAX_RISK_SCORE: u8 = 10;

#[near_bindgen]
impl Contract {
    pub fn create_address(
        &mut self,
        address: AccountId,
        category: Category,
        risk_score: RiskScore,
        case_id: CaseId,
    ) {
        let reporter = self.get_reporter_by_account(env::predecessor_account_id());

        match reporter.role {
            Role::Tracer | Role::Publisher | Role::Authority => {
                require!(reporter.is_active(), ERROR_REPORTER_IS_INACTIVE);
            }
            _ => env::panic_str(ERROR_INVALID_ROLE),
        }

        require!(self.cases.get(&case_id).is_some(), ERROR_CASE_NOT_FOUND);
        require!(risk_score <= MAX_RISK_SCORE, ERROR_INVALID_RISK_SCORE);

        let address_entity = Address {
            address: address.clone(),
            category,
            risk_score,
            case_id,
            reporter_id: reporter.id,
            confirmations: UnorderedSet::new(StorageKey::Confirmations {
                address: address.clone(),
            }),
        };

        require!(
            self.addresses
                .insert(&address, &address_entity.into())
                .is_none(),
            ERROR_ADDRESS_ALREADY_EXISTS
        );
    }

    pub fn update_address(
        &mut self,
        address: AccountId,
        category: Category,
        risk_score: RiskScore,
        case_id: CaseId,
    ) {
        let address_entity = self.get_address_internal(&address);

        let reporter = self.get_reporter_by_account(env::predecessor_account_id());

        match reporter.role {
            Role::Publisher => {
                require!(
                    reporter.id == address_entity.reporter_id,
                    ERROR_INVALID_ROLE
                );
            }
            Role::Authority => {}
            _ => env::panic_str(ERROR_INVALID_ROLE),
        }

        require!(reporter.is_active(), ERROR_REPORTER_IS_INACTIVE);

        require!(self.cases.get(&case_id).is_some(), ERROR_CASE_NOT_FOUND);
        require!(risk_score <= MAX_RISK_SCORE, ERROR_INVALID_RISK_SCORE);

        let mut address_entity: Address = self.get_address_internal(&address);

        address_entity.category = category;
        address_entity.risk_score = risk_score;
        address_entity.case_id = case_id;

        self.addresses.insert(&address, &address_entity.into());
    }

    pub fn confirm_address(&mut self, address: AccountId) {
        let reporter = self.get_reporter_by_account(env::predecessor_account_id());

        match reporter.role {
            Role::Validator | Role::Publisher => {
                require!(reporter.is_active(), ERROR_REPORTER_IS_INACTIVE);
            }
            _ => env::panic_str(ERROR_INVALID_ROLE),
        }

        let mut address_entity: Address = self.get_address_internal(&address);

        require!(
            address_entity.reporter_id != reporter.id,
            ERROR_REPORT_CONFIRMATION
        );

        require!(
            address_entity.confirmations.insert(&reporter.id),
            ERROR_ALREADY_CONFIRMED
        );

        self.addresses.insert(&address, &address_entity.into());
    }
}
