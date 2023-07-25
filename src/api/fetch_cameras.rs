use crate::{UnifiProtectCamera, UnifiProtectServer};
use reqwest::Client;

impl UnifiProtectServer {
    pub async fn fetch_cameras(&mut self) -> Result<(), String> {
        let response = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
            .get(&(self.uri.to_string() + "/proxy/protect/api/cameras"))
            .headers(self.headers.clone())
            .send()
            .await
            .expect("Failed to make fetch cameras request");

        // Something went wrong with the login call, possibly a controller reboot or failure.
        if !response.status().is_success() {
            println!("Failed to fetch cameras: {}", response.status());
            return Err(String::from("Failed to make cameras request!"));
        }

        let parsed_cameras_result = response.json::<Vec<UnifiProtectCamera>>().await;
        if parsed_cameras_result.is_err() {
            return Err(format!("Failed to parse camera-data: {}", parsed_cameras_result.err().unwrap().to_string()));
        }
        self.cameras = parsed_cameras_result.unwrap();

        Ok(())
    }
}
