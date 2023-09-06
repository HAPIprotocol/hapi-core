use crate::{
    address::{Address, VAddress},
    CaseId, Category, Contract, ContractExt, ReporterId, RiskScore, ERROR_ADDRESS_NOT_FOUND,
};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen,
    serde::{Serialize, Deserialize},
    AccountId,
};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AddressView {
    pub address: AccountId,
    pub category: Category,
    pub risk_score: RiskScore,
    pub case_id: CaseId,
    pub reporter_id: ReporterId,
    pub confirmations_count: u64,
}

impl From<VAddress> for AddressView {
    fn from(v_address: VAddress) -> Self {
        let address: Address = v_address.into();
        Self {
            address: address.address,
            category: address.category,
            risk_score: address.risk_score,
            case_id: address.case_id,
            reporter_id: address.reporter_id,
            confirmations_count: address.confirmations.len(),
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn get_address(&self, address: &AccountId) -> AddressView {
        self.addresses
            .get(address)
            .expect(ERROR_ADDRESS_NOT_FOUND)
            .into()
    }

    pub fn get_addresses(&self, take: u64, skip: u64) -> Vec<AddressView> {
        self.addresses
            .iter()
            .skip(skip as _)
            .take(take as _)
            .map(|(_, address)| address.into())
            .collect()
    }

    pub fn get_address_count(&self) -> u64 {
        self.addresses.len()
    }
}

impl Contract {
    pub fn get_address_internal(&self, address: &AccountId) -> Address {
        self.addresses
            .get(address)
            .expect(ERROR_ADDRESS_NOT_FOUND)
            .into()
    }
}
