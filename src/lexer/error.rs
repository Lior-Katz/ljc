use crate::file::Span;
use thiserror::Error;

pub type LexResult<T> = Result<T, (Error, Span)>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Empty char literal")]
    EmptyCharLiteral,
    #[error("Unclosed character literal")]
    UnclosedCharLiteral,
    #[error("Too many characters in character literal")]
    MultipleCharactersInCharLiteral,
    #[error("Unterminated string literal")]
    UnclosedStringLiteral,
    #[error("End of comment not found")]
    UnterminatedBlockComment,
    #[error("Illegal underscore")]
    NumericLiteralIllegalUnderscore,
    #[error("Numeric literal must contain at least one digit")]
    IncompleteNumericLiteral,
    #[error("Invalid escape sequence")]
    InvalidEscapeSequence,
}
