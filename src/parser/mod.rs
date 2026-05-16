mod parser;
pub use parser::Parser;
mod error;

type Diagnostic = crate::error::Diagnostic<error::Error>;