use {
    hapi_indexer::{PushEvent, PushPayload},
    mockito::{Matcher, Mock, Server, ServerGuard},
};

use super::TestBatch;

pub struct WebhookServiceMock {
    mocks: Vec<Mock>,
    // TODO: maybe this is redundant
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
                let payload = PushPayload {
                    event: PushEvent {
                        name: event.name.clone(),
                        tx_hash: event.hash.clone(),
                        tx_index: 0,
                        timestamp: 123,
                    },
                    data: data.clone(),
                };

                let mock = self
                    .server
                    .mock("POST", "/")
                    .match_body(Matcher::PartialJsonString(
                        serde_json::to_string(&payload).expect("Failed to serialize payload"),
                    ))
                    .expect(1)
                    .create();

                self.mocks.push(mock);
            }
        }
    }

    pub fn check_mocks(&self) {
        for mock in &self.mocks {
            mock.assert();
        }
    }
}
