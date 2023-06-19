pub struct HapiCoreEvmOptions {
    pub provider_url: String,
    pub contract_address: String,
    pub private_key: Option<String>,
    pub chain_id: Option<u64>,
}
