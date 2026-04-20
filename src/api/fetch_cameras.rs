use crate::{UnifiProtectCamera, UnifiProtectServer};
use crate::error::Error;
use reqwest::Client;
use crate::camera::UnifiProtectCameraSimple;

impl UnifiProtectServer {
    pub async fn fetch_cameras(&mut self, require_detailed_cameras : bool) -> Result<(), Error> {
        let response = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?
            .get(&(self.uri.to_string() + "/proxy/protect/api/cameras"))
            .headers(self.headers.clone())
            .send()
            .await?;

        // Something went wrong with the login call, possibly a controller reboot or failure.
        if !response.status().is_success() {
            return Err(Error::CameraFetchFailed(format!("Status: {}", response.status())));
        }

        // fetch the raw JSON text
        let cameras_raw_text = response.text().await?;

        // attempt to parse the most basic camera data
        let parsed_cameras_simple: Vec<UnifiProtectCameraSimple> = serde_json::from_str(&cameras_raw_text)?;
        self.cameras_simple = parsed_cameras_simple;

        // attempt to parse complete camera data
        match serde_json::from_str::<Vec<UnifiProtectCamera>>(&cameras_raw_text) {
            Ok(parsed_cameras) => {
                self.cameras = parsed_cameras;
            }
            Err(e) => {
                if require_detailed_cameras {
                    return Err(Error::Json(e));
                } else {
                    println!("Warning: Unable to parse complete set of camera data - data formats dont match: {}", e);
                }
            }
        }

        Ok(())
    }
}
