use {
    anyhow::{bail, Result},
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

    pub(crate) async fn send_graphql(&self, query: &str, variables: Value) -> Result<Value> {
        let body = serde_json::to_string(&json!({ "query": query, "variables": variables }))
            .expect("Failed to serialize body");

        let response = RequestSender::check_response(
            self.web_client
                .post(format!("{}/{}", &self.address, "graphql"))
                .body(body)
                .send()
                .await?,
        )
        .await?;

        if let Some(errors) = response.get("errors") {
            bail!("GraphQL request failed: {:?}", errors);
        }

        Ok(response["data"].clone())
    }

    pub(crate) async fn send<T: Serialize + ?Sized>(
        &self,
        url: &str,
        body: &T,
        token: &str,
    ) -> Result<Value> {
        let response = self
            .web_client
            .post(format!("{}/{}", &self.address, url))
            .bearer_auth(token)
            .json(body)
            .send()
            .await?;

        RequestSender::check_response(response).await
    }

    pub(crate) async fn get(&self, url: &str) -> Result<Value> {
        let response = self
            .web_client
            .get(format!("{}/{}", &self.address, url))
            .send()
            .await?;

        RequestSender::check_response(response).await
    }

    async fn check_response(response: Response) -> Result<Value> {
        if !response.status().is_success() {
            bail!(
                "Failed to send request, status: {}, error: {}",
                response.status().as_str(),
                response.text().await.expect("Failed to get response text")
            );
        } else {
            Ok(response.json::<Value>().await.unwrap_or_default())
        }
    }
}
