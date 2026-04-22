use crate::UnifiProtectServer;
use crate::error::Error;
use reqwest::Client;
use tokio::io::AsyncWriteExt;

impl UnifiProtectServer {
    // --- Thumbnail Downloading ---

    pub async fn download_event_thumbnail(
        &self,
        event_id: &str,
        output_path: &str,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<(), Error> {
        // Collect query parameters for width and height if provided
        let mut query_params = Vec::new();
        if let Some(width) = width {
            query_params.push(format!("w={}", width));
        }
        if let Some(height) = height {
            query_params.push(format!("h={}", height));
        }

        // Build the query string
        let query_string = if query_params.is_empty() {
            "".to_string()
        } else {
            format!("?{}", query_params.join("&"))
        };

        // Construct the full URL for the thumbnail request
        let url = format!(
            "{}/proxy/protect/api/events/{}/thumbnail{}",
            self.uri, event_id, query_string
        );

        // Build and send the HTTP GET request
        let mut response = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?
            .get(&url)
            .headers(self.headers.clone())
            .send()
            .await?;

        // Handle failure by reading the response text
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_else(|_| "Could not read response text".to_string());
            return Err(Error::ThumbnailDownloadFailed(format!("Status: {}, Body: {}", status, text)));
        }

        // Create the output file and stream the response content into it
        let mut file = tokio::fs::File::create(output_path).await?;

        while let Some(chunk) = response.chunk().await? {
            file.write_all(&chunk).await?;
        }

        Ok(())
    }
}
