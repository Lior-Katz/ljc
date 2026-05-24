mod diagnostic;
pub(crate) use diagnostic::Diagnose;
pub use diagnostic::{Diagnostic, SourceWithDiagnostic};

pub use crate::parser::{Diagnostic as ParserDiagnostic, ParserError};
pub use crate::semantic::{Diagnostic as SemanticDiagnostic, SemanticError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ParseError(#[from] ParserError),

    #[error(transparent)]
    SemanticError(#[from] SemanticError),
}

impl From<ParserDiagnostic> for Diagnostic<Error> {
    fn from(value: ParserDiagnostic) -> Self {
        Error::from(value.message).at(value.span)
    }
}

impl From<SemanticDiagnostic> for Diagnostic<Error> {
    fn from(value: SemanticDiagnostic) -> Self {
        Error::from(value.message).at(value.span)
    }
}
