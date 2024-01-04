use {
    core::panic,
    reqwest::{Client, Response},
    serde::Serialize,
    serde_json::{json, Value},
};

pub struct RequestSender {
    pub web_client: Client,
    address: String,
}

impl RequestSender {
    pub(crate) fn new(address: String) -> Self {
        Self {
            web_client: Client::new(),
            address,
        }
    }

    pub(crate) async fn send_graphql(&self, query: &str, variables: Value) -> Value {
        let body = serde_json::to_string(&json!({ "query": query, "variables": variables }))
            .expect("Failed to serialize body");

        let response = self
            .web_client
            .post(format!("{}/{}", &self.address, "graphql"))
            .body(body)
            .send()
            .await
            .expect("Failed to send request");

        RequestSender::check_response(response).await
    }

    pub(crate) async fn send<T: Serialize + ?Sized>(&self, url: &str, body: &T) -> Value {
        let response = self
            .web_client
            .post(format!("{}/{}", &self.address, url))
            .json(body)
            .send()
            .await
            .expect("Failed to send request");

        RequestSender::check_response(response).await
    }

    pub(crate) async fn get(&self, url: &str) -> Value {
        let response = self
            .web_client
            .get(format!("{}/{}", &self.address, url))
            .send()
            .await
            .expect("Failed to execute request");

        RequestSender::check_response(response).await
    }

    async fn check_response(response: Response) -> Value {
        if !response.status().is_success() {
            panic!(
                "Failed to send request, status: {}, error: {}",
                response.status().as_str(),
                response.text().await.expect("Failed to get response text")
            );
        } else {
            response.json::<Value>().await.unwrap_or_default()
        }
    }
}
