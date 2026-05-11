mod ast;
#[macro_use]
pub mod lexer;
pub use lexer::Error as LexicalError;
pub mod parser;
mod collections;