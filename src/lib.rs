mod unifi_protect_camera;

use http::{Request, Response};
use crate::unifi_protect_camera::UnifiProtectCamera;


pub struct UnifiProtectServer {
    pub uri: String,
    pub username: String,
    pub password: String,
    pub cameras: Vec<UnifiProtectCamera>
}

impl UnifiProtectServer {
    pub fn new(uri: &str, username: &str, password: &str) -> UnifiProtectServer {
        UnifiProtectServer {
            uri: uri.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            cameras: Vec::new()
        }
    }

    pub fn fetch_cameras(&mut self) {



    }
}


#[cfg(test)]
mod tests {

}