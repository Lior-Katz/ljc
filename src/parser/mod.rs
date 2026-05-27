mod parser;
pub use parser::Parser;
mod error;
pub use error::Error as ParserError;

pub type Diagnostic = crate::error::Diagnostic<ParserError>;
