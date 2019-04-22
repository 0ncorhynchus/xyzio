use std::io;
use std::num::{ParseFloatError, ParseIntError};
use std::result;

#[derive(Debug)]
pub enum ParseError {
    Integer(ParseIntError),
    Float(ParseFloatError),
}

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    IllegalState(String),
    Parse(ParseError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::Parse(ParseError::Integer(err))
    }
}

impl From<ParseFloatError> for Error {
    fn from(err: ParseFloatError) -> Self {
        Error::Parse(ParseError::Float(err))
    }
}

pub type Result<T> = result::Result<T, Error>;
