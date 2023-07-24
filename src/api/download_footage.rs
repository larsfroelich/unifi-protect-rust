use crate::camera::UnifiProtectCamera;
use crate::{ErrorResponse, UnifiProtectServer};
use reqwest::Client;
use std::io::Cursor;

impl UnifiProtectServer {
    pub async fn download_footage(
        &self,
        camera: &UnifiProtectCamera,
        output_path: &str,
        recording_type: &str,
        start_unix: i64,
        end_unix: i64,
    ) -> Result<bool, String> {
        let endpoint = format!(
            "{}/proxy/protect/api/video/export?camera={}\
            &channel=0\
            &filename={}.mp4\
            &lens=0\
            &start={}\
            &end={}\
            &type={}",
            self.uri, camera.id, camera.mac, start_unix, end_unix, recording_type
        );

        let response = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
            .get(&endpoint)
            .headers(self.headers.clone())
            .send()
            .await
            .expect("Failed to send request");
        if !response.status().is_success() {
            if response
                .json::<ErrorResponse>()
                .await
                .expect("Failed to parse response json")
                .error
                .contains("o files found")
            {
                return Ok(false);
            }
            return Err(format!("Failed to download video."));
        }
        let mut file = std::fs::File::create(output_path).expect("Failed to create file");
        let mut content = Cursor::new(response.bytes().await.expect("Failed to get bytes"));
        std::io::copy(&mut content, &mut file).expect("Failed to copy content to file");

        Ok(true)
    }
}
