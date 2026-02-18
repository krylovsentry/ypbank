use std::fmt;
use std::fmt::{write, Formatter};
use std::io::Error;
use std::num::ParseIntError;
use std::str::FromStr;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum ParserError {
    Io(std::io::Error),
    FormatParseError(String),
    InvalidTransactionStatus(String),
    InvalidTransactionType(String),
    MissingRequiredFields(String),
    ParseIntError(ParseIntError),
    InvalidTransactionLineFormat{
        line: String,
    },
    DuplicatedFieldInRecord{
        key: String,
    },
    Utf8Error(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::Io(err) => write!(f, "IO error: {}", err),
            ParserError::FormatParseError(err) => write!(f, "Format parsing error: {}", err),
            ParserError::InvalidTransactionStatus(err) => {
                write!(f, "Invalid transaction state: {}", err)
            }
            ParserError::InvalidTransactionType(err) => {
                write!(f, "Invalid transaction type: {}", err)
            }
            ParserError::MissingRequiredFields(err) => {
                write!(f, "Missing required transaction fields: {}", err)
            },
            ParserError::ParseIntError(err) => {
                write!(f, "Error parsing integer: {}", err)
            },
            ParserError::InvalidTransactionLineFormat { line } => {
                write!(f, "Invalid transaction line format: {}", line)
            },
            ParserError::DuplicatedFieldInRecord{ key} => {
                write!(f, "Duplicate field '{}' in record", key)
            },
            ParserError::Utf8Error(err) => {
                write!(f, "Error parsing UTF-8: {}", err)
            }
        }
    }
}

impl From<std::io::Error> for ParserError {
    fn from(value: Error) -> Self {
        Self::Io(value)
    }
}

impl From<std::num::ParseIntError> for ParserError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl From<std::string::FromUtf8Error> for ParserError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        ParserError::Utf8Error(err.to_string())
    }
}