use mockito::{Matcher, Mock, Server, ServerGuard};

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
            let mock = self
                .server
                .mock("POST", "/")
                .match_body(Matcher::PartialJsonString(
                    serde_json::to_string(event).expect("Failed to serialize payload"),
                ))
                .expect(1)
                .create();

            self.mocks.push(mock);
        }
    }

    pub fn check_mocks(&self) {
        for mock in &self.mocks {
            mock.assert();
        }
    }
}
