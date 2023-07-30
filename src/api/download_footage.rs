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
        for channel in 0..4 {
            let endpoint = format!(
                "{}/proxy/protect/api/video/export?camera={}\
                {}\
                &channel={}\
                &filename={}.mp4\
                &lens=0\
                &start={}\
                &end={}\
                &type={}",
                self.uri, camera.id,
                (if recording_type == "timelapse" {
                    "&fps=4"
                } else {
                    ""
                }),
                channel, camera.mac, start_unix, end_unix, recording_type
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
                let error_msg = response
                    .json::<ErrorResponse>()
                    .await
                    .expect("Failed to parse response json")
                    .error;
                if error_msg
                    .contains("o files found") || error_msg.contains("track information is not valid"){
                    continue;
                }
                return Err(format!("Failed to download video."));
            }
            let mut file = std::fs::File::create(output_path).expect("Failed to create file");
            let mut content = Cursor::new(response.bytes().await.expect("Failed to get bytes"));
            std::io::copy(&mut content, &mut file).expect("Failed to copy content to file");

            return Ok(true)
        }
        return Ok(false)
    }
}
