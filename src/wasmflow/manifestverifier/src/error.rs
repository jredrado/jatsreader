use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to verify")]
    NotVerified,
}
