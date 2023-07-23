pub mod camera;
pub mod auth;
pub mod api;

use camera::*;
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
