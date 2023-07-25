use ethers_contract::ContractError;

use crate::client::result::ClientError;

pub(super) fn map_ethers_error<M: ethers_providers::Middleware>(
    caller: &str,
    e: ContractError<M>,
) -> ClientError {
    match e {
        // TODO: get rid of black magic parsing
        ContractError::Revert(e) => {
            let e = if e.len() > 64 { &e[64..] } else { &e };

            ClientError::Ethers(format!(
                "`{caller}` reverted with: {}",
                String::from_utf8_lossy(e)
                    .chars()
                    .filter(|c| !c.is_control())
                    .collect::<String>()
            ))
        }
        _ => ClientError::Ethers(format!("`{caller}` failed: {e}")),
    }
}
