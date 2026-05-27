mod diagnostic;
pub(crate) use diagnostic::Diagnose;
pub use diagnostic::{Diagnostic, SourceWithDiagnostic};

pub use crate::parser::{Diagnostic as ParserDiagnostic, ParserError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ParseError(#[from] ParserError),
}

impl From<ParserDiagnostic> for Diagnostic<Error> {
    fn from(value: ParserDiagnostic) -> Self {
        Error::from(value.message).at(value.span)
    }
}
