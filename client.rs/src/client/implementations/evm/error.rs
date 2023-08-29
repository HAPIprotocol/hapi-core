use ethers_contract::ContractError;

use crate::client::result::ClientError;

pub(super) fn map_ethers_error<M: ethers_providers::Middleware>(
    caller: &str,
    e: ContractError<M>,
) -> ClientError {
    match e {
        ContractError::Revert(e) => match e {
            _ if e.is_empty() => {
                ClientError::Ethers(format!("`{caller}` reverted with empty message"))
            }
            // TODO: get rid of black magic parsing
            _ if e.len() > 64 => ClientError::Ethers(format!(
                "`{caller}` reverted with: {}",
                String::from_utf8_lossy(&e[64..])
                    .chars()
                    .filter(|c| !c.is_control())
                    .collect::<String>()
            )),
            e => ClientError::Ethers(format!(
                "`{caller}` reverted with: {}",
                String::from_utf8_lossy(&e)
                    .chars()
                    .filter(|c| !c.is_control())
                    .collect::<String>()
            )),
        },
        _ => ClientError::Ethers(format!("`{caller}` failed: {e}")),
    }
}
