#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
}

pub type Result<T> = std::result::Result<T, Error>;
