use near_sdk::{collections::UnorderedSet, env, json_types::U64, near_bindgen, require, AccountId};

use crate::{
    reporter::Role, CaseId, Category, Contract, ContractExt, RiskScore, ERROR_ALREADY_CONFIRMED,
    ERROR_ASSET_ALREADY_EXISTS, ERROR_ASSET_NOT_FOUND, ERROR_CASE_NOT_FOUND, ERROR_INVALID_ROLE,
    ERROR_REPORTER_IS_INACTIVE, ERROR_REPORT_CONFIRMATION,
};

use super::Asset;

#[near_bindgen]
impl Contract {
    pub fn create_asset(
        &mut self,
        address: AccountId,
        id: U64,
        category: Category,
        risk_score: RiskScore,
        case_id: CaseId,
    ) {
        let reporter = self.get_reporter_by_account(env::predecessor_account_id());

        match reporter.role {
            Role::Publisher | Role::Authority | Role::Tracer => {
                require!(reporter.is_active(), ERROR_REPORTER_IS_INACTIVE);
            }
            _ => {
                env::panic_str(ERROR_INVALID_ROLE);
            }
        }

        require!(self.cases.get(&case_id).is_some(), ERROR_CASE_NOT_FOUND);

        let asset_id = get_asset_id(&address, &id);

        let asset = Asset {
            address,
            id,
            category,
            risk_score,
            case_id,
            reporter_id: reporter.id,
            confirmations: UnorderedSet::new(asset_id.clone().into_bytes()),
        };

        require!(
            self.assets.insert(&asset_id, &asset.into()).is_none(),
            ERROR_ASSET_ALREADY_EXISTS
        );
    }

    pub fn update_asset(
        &mut self,
        address: AccountId,
        id: U64,
        category: Category,
        risk_score: RiskScore,
        case_id: CaseId,
    ) {
        let reporter = self.get_reporter_by_account(env::predecessor_account_id());

        let asset_id = get_asset_id(&address, &id);

        let mut asset: Asset = self
            .assets
            .get(&asset_id)
            .expect(ERROR_ASSET_NOT_FOUND)
            .into();

        match reporter.role {
            Role::Publisher => {
                require!(reporter.id == asset.reporter_id, ERROR_INVALID_ROLE);
            }
            Role::Authority => {}
            _ => env::panic_str(ERROR_INVALID_ROLE),
        }

        require!(reporter.is_active(), ERROR_REPORTER_IS_INACTIVE);

        require!(self.cases.get(&case_id).is_some(), ERROR_CASE_NOT_FOUND);

        asset.category = category;
        asset.risk_score = risk_score;
        asset.case_id = case_id;

        self.assets.insert(&asset_id, &asset.into());
    }

    pub fn confirm_asset(&mut self, address: AccountId, id: U64) {
        let reporter = self.get_reporter_by_account(env::predecessor_account_id());

        match reporter.role {
            Role::Validator | Role::Publisher => {
                require!(reporter.is_active(), ERROR_REPORTER_IS_INACTIVE);
            }
            _ => env::panic_str(ERROR_INVALID_ROLE),
        }

        let asset_id = get_asset_id(&address, &id);

        let mut asset: Asset = self
            .assets
            .get(&asset_id)
            .expect(ERROR_ASSET_NOT_FOUND)
            .into();

        require!(asset.reporter_id != reporter.id, ERROR_REPORT_CONFIRMATION);

        require!(
            asset.confirmations.insert(&reporter.id),
            ERROR_ALREADY_CONFIRMED
        );

        self.assets.insert(&asset_id, &asset.into());
    }
}

pub(crate) fn get_asset_id(address: &AccountId, id: &U64) -> String {
    format!("{}:{}", address, id.0)
}
