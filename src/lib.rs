mod camera;
mod auth;
mod api;

pub use camera::UnifiProtectCamera;
use reqwest::header::HeaderMap;

pub struct UnifiProtectServer {
    pub uri: String,
    pub cameras: Vec<UnifiProtectCamera>,
    headers: HeaderMap,
}

impl UnifiProtectServer {
    pub fn new(uri: &str) -> UnifiProtectServer {
        UnifiProtectServer {
            uri: uri.to_string(),
            cameras: Vec::new(),
            headers: Default::default(),
        }
    }
}
