use crate::camera::UnifiProtectCameraSimple;
use crate::event::MotionEvent;
use crate::{ErrorResponse, UnifiProtectServer};
use crate::error::Error;
use reqwest::Client;
use tokio::io::AsyncWriteExt;

impl UnifiProtectServer {
    // --- Footage Downloading ---

    pub async fn download_footage(
        &self,
        camera: &UnifiProtectCameraSimple,
        output_path: &str,
        recording_type: &str,
        start_unix: i64,
        end_unix: i64,
    ) -> Result<bool, Error> {
        // Construct the API endpoint with all necessary parameters
        for channel in 0..4 {
            let endpoint = format!(
                "{}/proxy/protect/api/video/export?camera={}                {}                &channel={}                &filename={}.mp4                &lens=0                &start={}                &end={}                &type={}",
                self.uri,
                camera.id,
                (if recording_type == "timelapse" {
                    "&fps=4"
                } else {
                    ""
                }),
                channel,
                camera.mac,
                start_unix,
                end_unix,
                recording_type
            );

            // Execute the request
            let mut response = Client::builder()
                .danger_accept_invalid_certs(true)
                .build()?
                .get(&endpoint)
                .headers(self.headers.clone())
                .send()
                .await?;

            // Handle non-success status codes
            if !response.status().is_success() {
                let status_code = response.status();
                let error_msg = response.json::<ErrorResponse>().await.ok().map(|x| x.error);

                if let Some(ref msg) = error_msg {
                    if msg.contains("o files found") || msg.contains("track information is not valid") {
                        continue;
                    }
                    return Err(Error::DownloadFailed(format!("Status: {}, Error: {}", status_code, msg)));
                } else {
                    return Err(Error::DownloadFailed(format!("Status: {}", status_code)));
                }
            }

            // Stream the response body to the specified file
            let mut file = tokio::fs::File::create(output_path).await?;

            while let Some(chunk) = response.chunk().await? {
                file.write_all(&chunk).await?;
            }

            return Ok(true);
        }

        Ok(false)
    }

    // --- Event-specific Footage Downloading ---

    pub async fn download_event_footage(
        &self,
        camera: &UnifiProtectCameraSimple,
        event: &MotionEvent,
        output_path: &str,
        recording_type: &str,
        pre_padding_seconds: u64,
        post_padding_seconds: u64,
    ) -> Result<bool, Error> {
        // Calculate the adjusted start and end times with padding
        let start_unix = event.start / 1000 - pre_padding_seconds as i64;
        let end_unix = event.end / 1000 + post_padding_seconds as i64;

        // Delegate to the main download_footage method
        self.download_footage(camera, output_path, recording_type, start_unix, end_unix).await
    }
}
