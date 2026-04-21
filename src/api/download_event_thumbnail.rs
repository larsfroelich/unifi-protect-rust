use crate::UnifiProtectServer;
use crate::error::Error;
use reqwest::Client;
use tokio::io::AsyncWriteExt;

impl UnifiProtectServer {
    pub async fn download_event_thumbnail(
        &self,
        event_id: &str,
        output_path: &str,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<(), Error> {
        let mut query_params = Vec::new();
        if let Some(w) = width {
            query_params.push(format!("w={}", w));
        }
        if let Some(h) = height {
            query_params.push(format!("h={}", h));
        }

        let query_string = if query_params.is_empty() {
            "".to_string()
        } else {
            format!("?{}", query_params.join("&"))
        };

        let endpoint = format!(
            "{}/proxy/protect/api/events/{}/thumbnail{}",
            self.uri, event_id, query_string
        );

        let mut response = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?
            .get(&endpoint)
            .headers(self.headers.clone())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::ThumbnailDownloadFailed(format!("Status: {}", response.status())));
        }

        let mut file = tokio::fs::File::create(output_path).await?;

        while let Some(chunk) = response.chunk().await? {
            file.write_all(&chunk).await?;
        }

        Ok(())
    }
}
