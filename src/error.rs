use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid header value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    #[error("login failed: {0}")]
    LoginFailed(String),

    #[error("CSRF token missing")]
    CsrfTokenMissing,

    #[error("API error: {0}")]
    Api(String),

    #[error("failed to fetch cameras: {0}")]
    CameraFetchFailed(String),

    #[error("failed to download footage: {0}")]
    DownloadFailed(String),

    #[error("other error: {0}")]
    Other(String),
}
