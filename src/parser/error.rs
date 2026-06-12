use crate::LexicalError;
use crate::collections::bitflag_combination;
use crate::error::Diagnose;
use crate::file::Span;
use crate::lexer::Symbol;
use crate::parser::Diagnostic;

use bitflags::{bitflags, bitflags_match};
use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;

pub type ParseResult<T> = Result<T, Failure>;

pub enum Failure {
    NoProduction,
    Error(Diagnostic),
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Lexical(LexicalError),
    #[error("Missing declaration after modifiers.\nHint: expected {0}")]
    DanglingModifiers(ExpectedDeclarationType),
    #[error("Trailing {0} not allowed here")]
    MissingElementAfterDelimiter(Symbol),
    #[error("Symbol expected: {0}")]
    SymbolExpected(Symbol),
    #[error("Symbol expected: {0} or {1}")]
    SymbolExpected2(Symbol, Symbol),
    #[error("Expected {0}")]
    SyntaxExpected(SyntaxKind),
    #[error("Expected {0} after {1}")]
    SyntaxExpectedAfter(SyntaxKind, Symbol),
    #[error("Invalid type name")]
    RestrictedTypeName,
    #[error("Expected identifier")]
    IdentifierExpected,
    #[error("Expected block after 'try'\nnote: a block must start with '{{'")]
    MissingTryBlock,
    #[error("Missing class body\nhint: expected '{{'")]
    MissingClassBody,
    #[error("Only field access, array access, or simple name expressions can be used as the left-hand-side of an assignment")]
    NotLValue,
}

impl From<LexicalError> for Error {
    fn from(value: LexicalError) -> Self {
        Error::Lexical(value)
    }
}

impl From<Diagnostic> for Failure {
    fn from(value: Diagnostic) -> Self {
        Failure::Error(value)
    }
}

impl<T> From<(T, Span)> for Diagnostic
where
    T: Into<Error>,
{
    fn from(value: (T, Span)) -> Self {
        value.0.into().at(value.1)
    }
}

pub trait AssertResult {
    fn assert(self, error: Diagnostic) -> Self;
    fn assert_if(self, cond: bool, error: Diagnostic) -> Self;
}

impl<T> AssertResult for ParseResult<T> {
    fn assert(self, error: Diagnostic) -> Self {
        match self {
            Ok(v) => Ok(v),
            Err(Failure::NoProduction) => Err(Failure::Error(error)),
            Err(Failure::Error(cause)) => Err(Failure::Error(cause)),
        }
    }

    fn assert_if(self, cond: bool, error: Diagnostic) -> Self {
        if cond { self.assert(error) } else { self }
    }
}

#[derive(Debug)]
pub enum SyntaxKind {
    Expression,
    Statement,
    Type,
}

impl Display for SyntaxKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxKind::Expression => write!(f, "expression"),
            SyntaxKind::Statement => write!(f, "statement"),
            SyntaxKind::Type => write!(f, "type"),
        }
    }
}

bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone)]
    struct DeclarationTypeFlags: u16 {
        const CLASS                 = 1 << 0;
        const RECORD                = 1 << 1;
        const ENUM                  = 1 << 2;
        const INTERFACE             = 1 << 3;
        const ANNOTATION_INTERFACE  = 1 << 4;
        const CONSTRUCTOR           = 1 << 5;
        const METHOD                = 1 << 6;
        const FIELD                 = 1 << 7;
        const PARAMETER             = 1 << 8;
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExpectedDeclarationType(DeclarationTypeFlags);

impl ExpectedDeclarationType {
    pub const CLASS: Self = Self(DeclarationTypeFlags::CLASS);

    pub const RECORD: Self = Self(DeclarationTypeFlags::RECORD);

    pub const ENUM: Self = Self(DeclarationTypeFlags::ENUM);

    pub const INTERFACE: Self = Self(DeclarationTypeFlags::INTERFACE);

    pub const ANNOTATION_INTERFACE: Self = Self(DeclarationTypeFlags::ANNOTATION_INTERFACE);

    pub const CONSTRUCTOR: Self = Self(DeclarationTypeFlags::CONSTRUCTOR);

    pub const METHOD: Self = Self(DeclarationTypeFlags::METHOD);

    pub const FIELD: Self = Self(DeclarationTypeFlags::FIELD);

    pub const PARAMETER: Self = Self(DeclarationTypeFlags::PARAMETER);

    pub const TOP_LEVEL: Self = Self(bitflag_combination!(
        DeclarationTypeFlags,
        CLASS | RECORD | ENUM | INTERFACE | ANNOTATION_INTERFACE,
    ));

    pub const CLASS_MEMBER: Self = Self(bitflag_combination!(
        DeclarationTypeFlags,
        CLASS | RECORD | ENUM | INTERFACE | ANNOTATION_INTERFACE | CONSTRUCTOR | METHOD | FIELD,
    ));

    pub fn contains(self, other: Self) -> bool {
        self.0.contains(other.0)
    }

    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl Display for ExpectedDeclarationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let items: Vec<&'static str> = self
            .0
            .iter()
            .map(|flag| {
                bitflags_match!(flag, {
                    DeclarationTypeFlags::CLASS => "class",
                    DeclarationTypeFlags::RECORD => "record",
                    DeclarationTypeFlags::ENUM => "enum",
                    DeclarationTypeFlags::INTERFACE => "interface",
                    DeclarationTypeFlags::ANNOTATION_INTERFACE => "annotation interface",
                    DeclarationTypeFlags::CONSTRUCTOR => "constructor",
                    DeclarationTypeFlags::METHOD => "method",
                    DeclarationTypeFlags::FIELD => "field",
                    DeclarationTypeFlags::PARAMETER => "parameter",
                    _ => unreachable!(),
                })
            })
            .collect();
        match items.as_slice() {
            [] => Ok(()),
            [x] => write!(f, "{x} declaration"),
            [a, b] => write!(f, "{a} or {b} declaration"),
            _ => {
                let head = items[..items.len() - 1].join(", ");
                let tail = &items[items.len() - 1];
                write!(f, "{head}, or {tail} declaration")
            }
        }
    }
}
