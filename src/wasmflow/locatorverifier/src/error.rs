use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to verify")]
    NotVerified,
    #[error("Unable to get resource")]
    NotFound,
    #[error("Unable to decode")]
    DecodeError,
    #[error("No response")]
    ResponseError,
    #[error("Other kind of error")]
    Other,
}
