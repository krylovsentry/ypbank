use std::fmt;
use std::fmt::Formatter;
use std::io::Error;

#[derive(Debug)]
pub enum ParserError {
    Io(std::io::Error),

}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::Io(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl From<std::io::Error> for ParserError {
    fn from(value: Error) -> Self {
        Self::Io(value)
    }
}
