use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum Error {
    #[error("Reqwest error")]
    ReqestError,
}
