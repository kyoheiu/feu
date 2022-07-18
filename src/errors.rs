use thiserror::Error;

#[derive(Error, Debug)]
pub enum FeuError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Miniserde(#[from] miniserde::Error),
}
