use {
    hapi_core::client::events::EventName,
    hapi_indexer::{PushEvent, PushPayload},
    mockito::{Matcher, Mock, Server, ServerGuard},
};

use super::TestBatch;

pub struct WebhookServiceMock {
    mocks: Vec<Mock>,
    pub server: ServerGuard,
}

impl WebhookServiceMock {
    pub fn new() -> Self {
        Self {
            mocks: vec![],
            server: Server::new(),
        }
    }
    pub fn set_mocks(&mut self, batch: &TestBatch) {
        for event in batch {
            if let Some(data) = &event.data {
                if event.name != EventName::ConfirmAddress && event.name != EventName::ConfirmAsset
                {
                    let payload = PushPayload {
                        id: uuid::Uuid::default(),
                        network: event.network.clone(),
                        event: PushEvent {
                            name: event.name.clone(),
                            tx_hash: event.hash.clone(),
                            tx_index: 0,
                            timestamp: 123,
                        },
                        data: data.clone(),
                    };

                    let mut payload_json =
                        serde_json::to_value(&payload).expect("Failed to serialize payload");

                    // delete id field from request because it generates randomly and we can't predict it
                    payload_json.as_object_mut().unwrap().remove("id");

                    let mock = self
                        .server
                        .mock("POST", "/")
                        .match_body(Matcher::PartialJson(payload_json))
                        .expect(1)
                        .create();

                    self.mocks.push(mock);
                }
            }
        }
    }

    pub fn check_mocks(&self) {
        for mock in &self.mocks {
            mock.assert();
        }
    }
}
