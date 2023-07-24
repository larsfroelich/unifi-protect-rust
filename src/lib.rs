pub mod api;
pub mod auth;
pub mod camera;

use camera::*;
use reqwest::header::HeaderMap;

pub struct UnifiProtectServer {
    pub uri: String,
    pub cameras: Vec<UnifiProtectCamera>,
    headers: HeaderMap,
}

#[derive(serde::Deserialize)]
struct ErrorResponse {
    error: String,
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
