pub mod client;

pub use client::{
    amount::Amount,
    entities::network::HapiCoreNetwork,
    implementations::{
        HapiCoreEvm, HapiCoreNear, HapiCoreSolana, TokenContractEvm, TokenContractNear,
    },
    interface::{HapiCore, HapiCoreOptions},
};

#[cfg(test)]
pub use client::implementations::solana::get_network_account;
