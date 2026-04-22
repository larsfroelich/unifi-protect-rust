use crate::{UnifiProtectEvent, UnifiProtectServer};
use crate::error::Error;
use reqwest::Client;

impl UnifiProtectServer {
    // --- Event Fetching ---

    pub async fn fetch_events(
        &self,
        camera_id: &str,
        start_unix_ms: i64,
        end_unix_ms: i64,
        types: Option<Vec<&str>>,
    ) -> Result<Vec<UnifiProtectEvent>, Error> {
        // Prepare the event types query parameter
        let types_str = match types {
            Some(t) => format!("&types={}", t.join(",")),
            None => "".to_string(),
        };

        // Construct the full API endpoint URL
        let endpoint = format!(
            "{}/proxy/protect/api/events?cameras={}            &start={}            &end={}            {}            &withoutDescriptions=true",
            self.uri, camera_id, start_unix_ms, end_unix_ms, types_str
        );

        // Build and send the HTTP request
        let response = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?
            .get(&endpoint)
            .headers(self.headers.clone())
            .send()
            .await?;

        // Check for API errors
        if !response.status().is_success() {
            return Err(Error::EventFetchFailed(format!("Status: {}", response.status())));
        }

        // Parse and return the event list
        Ok(response.json().await?)
    }
}
