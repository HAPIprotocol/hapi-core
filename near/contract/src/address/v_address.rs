
#[derive(BorshDeserialize, BorshSerialize)]
pub enum VAddress{
    Current(Address),
}

impl From<Address> for VAddress {
    fn from(address: Address) -> Self {
        VAddress::Current(address)
    }
}

impl From<VAddress> for Address {
    fn from(v_address: VAddress) -> Self {
        match v_address {
            VAddress::Current(address) => address,
        }
    }
}


