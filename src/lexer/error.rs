use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    InvalidSequence(ErrorDescription),
    NumericLiteralError(ErrorDescription),
    InvalidEscape(ErrorDescription),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidSequence(desc)
            | Error::NumericLiteralError(desc)
            | Error::InvalidEscape(desc) => {
                write!(f, "{}:{}\t{}", desc.line, desc.column, desc.cause)
            }
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub struct ErrorDescription {
    pub line: usize,
    pub column: usize,
    pub cause: String,
}

impl ErrorDescription {
    pub fn new(line: usize, column: usize, cause: String) -> Self {
        Self { line, column, cause }
    }
}

pub fn invalid_sequence<_T>(line: usize, column: usize, cause: &str) -> Result<_T, Error> {
    Err(Error::InvalidSequence(ErrorDescription::new(
        line,
        column,
        String::from(cause),
    )))
}

pub fn numeric_literal_error<_T>(line: usize, column: usize, cause: &str) -> Result<_T, Error> {
    Err(Error::NumericLiteralError(ErrorDescription::new(
        line,
        column,
        String::from(cause),
    )))
}

pub fn invalid_escape<_T>(line: usize, column: usize) -> Result<_T, Error> {
    Err(Error::InvalidEscape(ErrorDescription::new(
        line,
        column,
        String::from("Invalid escape character"),
    )))
}
