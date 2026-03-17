use crate::lexer::tokens::Token;

#[derive(Debug)]
pub enum LexError {
    IoError(String),
    InvalidSequence(ErrorDescription),
}

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

pub fn invalid_sequence(line: usize, column: usize, cause: &str) -> Result<Option<Token>, LexError> {
    Err(LexError::InvalidSequence(ErrorDescription::new(line, column, String::from(cause))))
}