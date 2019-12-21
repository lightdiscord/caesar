use std::num::ParseIntError;
use std::io;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    InvalidNumber
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Error::ParseIntError(error)
    }
}