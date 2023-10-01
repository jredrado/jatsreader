
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Other type of error")]
    Other
}