use anyhow::{bail, Result};

use crate::{Indexer, IndexingCursor};

impl Indexer {
    pub(crate) async fn send_heartbeat(&self, cursor: &IndexingCursor) -> Result<()> {
        let url = format!(
            "{}/indexer/{}/heartbeat",
            self.webhook_url,
            self.client.get_id()
        );

        let response = self
            .web_client
            .put(&url)
            .bearer_auth(self.jwt_token.as_str())
            .json(&cursor)
            .send()
            .await?;

        if !response.status().is_success() {
            bail!("Heartbeat request failed: {:?}", response.text().await?);
        }

        Ok(())
    }
}
