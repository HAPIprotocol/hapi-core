pub mod address;
pub mod asset;
pub mod case;
pub mod indexer;
pub mod reporter;
pub mod types;

pub trait FromPayload<T>: Sized {
    fn from(network: &hapi_core::HapiCoreNetwork, value: T) -> Self;
}
