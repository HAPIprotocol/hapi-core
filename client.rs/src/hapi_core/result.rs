pub enum ClientError {}

pub type Result<T> = std::result::Result<T, ClientError>;

#[derive(Default, Clone)]
pub struct Tx {
    pub hash: String,
}
