use {
    anyhow::{bail, Result},
    hapi_core::{
        client::{
            entities::{address::Address, asset::Asset, case::Case, reporter::Reporter},
            events::EventName,
        },
        HapiCoreNetwork,
    },
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

use super::Indexer;

/// Webhook payload
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PushPayload {
    pub id: Uuid,
    pub network: HapiCoreNetwork,
    pub event: PushEvent,
    pub data: PushData,
}

/// Event data
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PushEvent {
    /// Event name
    pub name: EventName,
    /// Hash of the transaction
    pub tx_hash: String,
    /// Index of the event in a transaction (for multi-instruction transactions, i.e. Solana)
    pub tx_index: u64,
    /// Timestamp of the transaction block
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum PushData {
    Address(Address),
    Asset(Asset),
    Case(Case),
    Reporter(Reporter),
}

impl From<Address> for PushData {
    fn from(address: Address) -> Self {
        Self::Address(address)
    }
}

impl From<Asset> for PushData {
    fn from(asset: Asset) -> Self {
        Self::Asset(asset)
    }
}

impl From<Case> for PushData {
    fn from(case: Case) -> Self {
        Self::Case(case)
    }
}

impl From<Reporter> for PushData {
    fn from(reporter: Reporter) -> Self {
        Self::Reporter(reporter)
    }
}

impl Indexer {
    pub(crate) async fn send_webhook(&self, payload: &PushPayload) -> Result<()> {
        let url = format!(
            "{}/events",
            self.webhook_url,
        );

        let response = self
            .web_client
            .post(url)
            .bearer_auth(self.jwt_token.as_str())
            .json(payload)
            .send()
            .await?;

        if !response.status().is_success() {
            bail!("Webhook request failed: {:?}", response.text().await?);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use hapi_core::client::entities::{address::Address, category::Category};

    use super::*;

    #[test]
    fn test_push_payload_serialization() {
        // Create a sample PushPayload
        let payload = PushPayload {
            id: uuid::uuid!("f6b9e9a0-9b7a-4e1a-8b0a-9e2a5e8e4b5e"),
            network: HapiCoreNetwork::Ethereum,
            event: PushEvent {
                name: EventName::CreateAddress,
                tx_hash: "acf0734ab380f3964e1f23b1fd4f5a5125250208ec17ff11c9999451c138949f"
                    .to_string(),
                tx_index: 0,
                timestamp: 1690888679,
            },
            data: PushData::Address(Address {
                address: "0x922ffdfcb57de5dd6f641f275e98b684ce5576a3".to_string(),
                case_id: uuid::uuid!("de1659f2-b802-49ee-98dd-6e4ce0453067"),
                reporter_id: uuid::uuid!("1466cf4f-1d71-4153-b9ad-4a9c1b48101e"),
                category: Category::None,
                risk: 0,
                confirmations: 3,
            }),
        };

        // Serialize the PushPayload to JSON
        let json = serde_json::to_string(&payload).unwrap();

        assert_eq!(
            json,
            r#"{"id":"f6b9e9a0-9b7a-4e1a-8b0a-9e2a5e8e4b5e","network":"Ethereum","event":{"name":"create_address","tx_hash":"acf0734ab380f3964e1f23b1fd4f5a5125250208ec17ff11c9999451c138949f","tx_index":0,"timestamp":1690888679},"data":{"Address":{"address":"0x922ffdfcb57de5dd6f641f275e98b684ce5576a3","case_id":"de1659f2-b802-49ee-98dd-6e4ce0453067","reporter_id":"1466cf4f-1d71-4153-b9ad-4a9c1b48101e","risk":0,"category":"None","confirmations":3}}}"#
        );

        // Deserialize the JSON back into a PushPayload
        let deserialized_payload: PushPayload = serde_json::from_str(&json).unwrap();

        // Ensure that the deserialized PushPayload matches the original
        assert_eq!(payload, deserialized_payload);
    }
}
