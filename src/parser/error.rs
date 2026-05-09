use crate::lexer::{Symbol};
use thiserror::Error;
use crate::LexicalError;

pub type ParseResult<T> = Result<T, Failure>;

pub enum Failure {
    NoProduction,
    Error(Error),
}

impl From<Error> for Failure {
    fn from(value: Error) -> Self {
        Failure::Error(value)
    }
}

impl From<LexicalError> for Error {
    fn from(value: LexicalError) -> Self {
        Error::Lexical(value)
    }
}

impl From<LexicalError> for Failure {
    fn from(value: LexicalError) -> Self {
        Error::from(value).into()
    }
}

#[allow(dead_code)]
pub trait ResultExtension {
    fn assert(self, error: Error) -> Self;
}

impl<T> ResultExtension for ParseResult<T> {
    fn assert(self, error: Error) -> Self {
        match self {
            Ok(v) => Ok(v),
            Err(Failure::NoProduction) => Err(Failure::Error(error)),
            Err(Failure::Error(cause)) => Err(Failure::Error(cause)),
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Lexical(LexicalError),
    #[error("Symbol expected: {0}")]
    SymbolExpected(Symbol),
    #[error("Invalid type name")]
    RestrictedTypeName,
}
