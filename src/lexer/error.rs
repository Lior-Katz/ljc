use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum LexError {
    IoError(String),
    InvalidSequence(ErrorDescription),
    NumericLiteralError(ErrorDescription),
    InvalidEscape(ErrorDescription),
}

impl Display for LexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LexError::IoError(e) => write!(f, "IO error: {}", e),
            | LexError::InvalidSequence(desc)
            | LexError::NumericLiteralError(desc)
            | LexError::InvalidEscape(desc) => {
                write!(f, "{}:{}\t{}", desc.line, desc.column, desc.cause)
            }
        }
    }
}

impl Error for LexError {}

#[derive(Debug)]
pub struct ErrorDescription {
    pub line: usize,
    pub column: usize,
    pub cause: String,
}

impl ErrorDescription {
    pub fn new(line: usize, column: usize, cause: String) -> Self {
        Self {
            line,
            column,
            cause,
        }
    }
}

pub fn invalid_sequence<_T>(line: usize, column: usize, cause: &str) -> Result<_T, LexError> {
    Err(LexError::InvalidSequence(ErrorDescription::new(
        line,
        column,
        String::from(cause),
    )))
}

pub fn numeric_literal_error<_T>(line: usize, column: usize, cause: &str) -> Result<_T, LexError> {
    Err(LexError::NumericLiteralError(ErrorDescription::new(
        line,
        column,
        String::from(cause),
    )))
}

pub fn invalid_escape<_T>(line: usize, column: usize) -> Result<_T, LexError> {
    Err(LexError::InvalidEscape(ErrorDescription::new(
        line,
        column,
        String::from("Invalid escape character"),
    )))
}
