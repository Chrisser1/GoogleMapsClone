use std::fmt;
use std::str::from_utf8;
use std::num::{ParseIntError, ParseFloatError};
use std::str::Utf8Error;
use std::error::Error as StdError;

/// Custom error type that can encapsulate different kinds of errors that might occur.
#[derive(Debug)]
pub enum ParseError {
    Utf8Error(Utf8Error),
    IntError(ParseIntError),
    FloatError(ParseFloatError),
    NoDataError,
}

impl From<Utf8Error> for ParseError {
    fn from(err: Utf8Error) -> Self {
        ParseError::Utf8Error(err)
    }
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError::IntError(err)
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(err: ParseFloatError) -> Self {
        ParseError::FloatError(err)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Utf8Error(e) => write!(f, "UTF-8 decoding error: {}", e),
            ParseError::IntError(e) => write!(f, "Integer parsing error: {}", e),
            ParseError::FloatError(e) => write!(f, "Floating point parsing error: {}", e),
            ParseError::NoDataError => write!(f, "No data available"),
        }
    }
}

impl StdError for ParseError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            ParseError::Utf8Error(e) => Some(e),
            ParseError::IntError(e) => Some(e),
            ParseError::FloatError(e) => Some(e),
            ParseError::NoDataError => None,
        }
    }
}

/// Converts bytes to a string, assuming UTF-8 encoding.
pub fn parse_string(bytes: Option<&[u8]>) -> Result<String, ParseError> {
    match bytes {
        Some(data) => from_utf8(data)
            .map(String::from)
            .map_err(ParseError::Utf8Error),  // Convert Utf8Error to ParseError
        None => Err(ParseError::NoDataError),
    }
}

/// Parses bytes as a 64-bit signed integer.
pub fn parse_i64(bytes: Option<&[u8]>) -> Result<i64, ParseError> {
    bytes
        .ok_or(ParseError::NoDataError)
        .and_then(|bytes| from_utf8(bytes).map_err(Into::into))
        .and_then(|str| str.parse::<i64>().map_err(Into::into))
}

/// Parses bytes as a 64-bit floating point number.
pub fn parse_f64(bytes: Option<&[u8]>) -> Result<f64, ParseError> {
    bytes
        .ok_or(ParseError::NoDataError)
        .and_then(|bytes| from_utf8(bytes).map_err(Into::into))
        .and_then(|str| str.parse::<f64>().map_err(Into::into))
}

/// Parses bytes as a 32-bit signed integer.
pub fn parse_i32(bytes: Option<&[u8]>) -> Result<i32, ParseError> {
    bytes
        .ok_or(ParseError::NoDataError)
        .and_then(|bytes| from_utf8(bytes).map_err(Into::into))
        .and_then(|str| str.parse::<i32>().map_err(Into::into))
}

use thiserror::Error;

/// Custom error type that can encapsulate both database errors and parsing errors
#[derive(Error, Debug)]
pub enum NodeQueryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] odbc_api::Error),
    #[error("Parsing error: {0}")]
    ParseError(#[from] ParseError),
}
