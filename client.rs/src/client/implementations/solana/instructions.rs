use {
    anchor_client::anchor_lang::{AnchorDeserialize, AnchorSerialize},
    hapi_core_solana::{
        CaseStatus, Category, ReporterRole, RewardConfiguration, StakeConfiguration,
    },
    spl_token::solana_program::pubkey::Pubkey,
};

/// Byte index of bump in account data
pub const DISCRIMINATOR_SIZE: usize = 8;

/// Hapi core instructions
#[repr(usize)]
pub enum HapiInstruction {
    CreateNetwork = 0,
    UpdateStakeConfiguration,
    UpdateRewardConfiguration,
    SetAuthority,
    CreateReporter,
    UpdateReporter,
    ActivateReporter,
    DeactivateReporter,
    Unstake,
    CreateCase,
    UpdateCase,
    CreateAddress,
    UpdateAddress,
    ConfirmAddress,
    CreateAsset,
    UpdateAsset,
    ConfirmAsset,
}

/// Hapi core instruction data
#[derive(PartialEq, Debug)]
pub enum InstructionData {
    CreateNetwork(CreateNetworkData),
    UpdateStakeConfiguration(StakeConfiguration),
    UpdateRewardConfiguration(RewardConfiguration),
    SetAuthority(),
    CreateReporter(CreateReporterData),
    UpdateReporter(UpdateReporterData),
    ActivateReporter(),
    DeactivateReporter(),
    Unstake(),
    CreateCase(CreateCaseData),
    UpdateCase(UpdateCaseData),
    CreateAddress(CreateAddressData),
    UpdateAddress(UpdateAddressData),
    ConfirmAddress(u8),
    CreateAsset(CreateAssetData),
    UpdateAsset(UpdateAssetData),
    ConfirmAsset(u8),
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Default, Debug)]
pub struct CreateNetworkData {
    pub name: [u8; 32],
    pub stake_info: StakeConfiguration,
    pub reward_info: RewardConfiguration,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Default, Debug)]
pub struct CreateReporterData {
    pub reporter_id: u128,
    pub account: Pubkey,
    pub name: String,
    pub role: ReporterRole,
    pub url: String,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Default, Debug)]
pub struct UpdateReporterData {
    pub account: Pubkey,
    pub name: String,
    pub role: ReporterRole,
    pub url: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Default, Debug)]
pub struct CreateCaseData {
    pub case_id: u128,
    pub name: String,
    pub url: String,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Default, Debug)]
pub struct UpdateCaseData {
    pub name: String,
    pub url: String,
    pub status: CaseStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug)]
pub struct CreateAddressData {
    pub address: [u8; 64],
    pub category: Category,
    pub risk: u8,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Default, Debug)]
pub struct UpdateAddressData {
    pub category: Category,
    pub risk: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug)]
pub struct CreateAssetData {
    pub addr: [u8; 64],
    pub asset_id: [u8; 64],
    pub category: Category,
    pub risk_score: u8,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Default, Debug)]
pub struct UpdateAssetData {
    pub category: Category,
    pub risk_score: u8,
}

/// Hashes instruction names to bytearray
// #[cfg(feature = "decode")]
pub fn get_hapi_sighashes() -> Vec<[u8; 8]> {
    let names = [
        "create_network",
        "update_stake_configuration",
        "update_reward_configuration",
        "set_authority",
        "create_reporter",
        "update_reporter",
        "activate_reporter",
        "deactivate_reporter",
        "unstake",
        "create_case",
        "update_case",
        "create_address",
        "update_address",
        "confirm_address",
        "create_asset",
        "update_asset",
        "confirm_asset",
    ];

    names
        .iter()
        .map(|n| get_instruction_sighash(*n))
        .collect::<Vec<_>>()
}

/// Hashes instruction names to bytearray
// #[cfg(feature = "decode")]
pub(crate) fn get_instruction_sighash(name: &str) -> [u8; 8] {
    use sha2::Digest;

    let mut hasher = sha2::Sha256::new();
    hasher.update(format!("global:{}", name).as_bytes());

    let mut sighash = [0u8; DISCRIMINATOR_SIZE];
    sighash.copy_from_slice(&hasher.finalize()[..DISCRIMINATOR_SIZE]);
    sighash
}
