use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("cannot deserialize response: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("unexpected status code: {0}")]
    UnexpectedStatus(u16),
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("missing response")]
    MissingResponse,
}
