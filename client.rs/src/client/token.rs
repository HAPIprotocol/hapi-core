use async_trait::async_trait;

use super::{
    amount::Amount,
    result::{Result, Tx},
};

#[async_trait]
pub trait TokenContract {
    /// Whether the HAPI Core contract requires approval before token transfer
    fn is_approve_needed(&self) -> bool;

    /// Transfer tokens to another address
    async fn transfer(&self, to: &str, amount: Amount) -> Result<Tx>;

    /// Approve another address to spend tokens
    async fn approve(&self, spender: &str, amount: Amount) -> Result<Tx>;

    /// Get the amount of tokens on this address
    async fn balance(&self, addr: &str) -> Result<Amount>;
}
