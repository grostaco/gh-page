use serde::{Deserialize, Serialize};
use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum SpotifyError {
    #[error("Unauthorized")]
    Unauthorized(Unauthorized),
    #[error("Forbidden request")]
    Forbidden(Forbidden),
    #[error("Ratelimited")]
    RateLimited(RateLimited),
    #[error("reqwest error {0}")]
    Reqwest(#[from] reqwest::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Unauthorized {
    error: Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Forbidden {
    error: Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RateLimited {
    error: Error,
}

#[derive(Serialize, Deserialize, Debug)]
struct Error {
    status: u64,
    message: String,
}
