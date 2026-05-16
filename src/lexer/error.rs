use crate::file::Span;
use std::fmt::{Display, Formatter};

pub type LexResult<T> = Result<T, (Error, Span)>;

#[derive(Debug)]
pub enum Error {
    InvalidSequence(String),
    NumericLiteralError(String),
    InvalidEscape(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidSequence(cause)
            | Error::NumericLiteralError(cause)
            | Error::InvalidEscape(cause) => {
                write!(f, "{cause}")
            }
        }
    }
}

impl std::error::Error for Error {}

pub fn invalid_sequence<_T>(span: Span, cause: &str) -> LexResult<_T> {
    Err((Error::InvalidSequence(cause.to_string()), span))
}

pub fn numeric_literal_error<_T>(span: Span, cause: &str) -> LexResult<_T> {
    Err((Error::NumericLiteralError(cause.to_string()), span))
}

pub fn invalid_escape<_T>(span: Span) -> LexResult<_T> {
    Err((Error::InvalidEscape("Invalid escape character".to_string()), span))
}
