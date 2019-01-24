use std::io;
use std::num;
use std::result;

#[derive(Debug)]
pub enum ParseError {
    Integer(num::ParseIntError),
    Float(num::ParseFloatError),
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

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Self {
        Error::Parse(ParseError::Integer(err))
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(err: num::ParseFloatError) -> Self {
        Error::Parse(ParseError::Float(err))
    }
}

pub type Result<T> = result::Result<T, Error>;
