pub mod address;
pub mod asset;
pub mod case;
pub mod network;
pub mod reporter;
pub mod types;

pub trait FromPayload<T>: Sized {
    fn from(network_id: uuid::Uuid, value: T) -> Self;
}
