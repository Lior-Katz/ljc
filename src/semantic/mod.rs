use crate::ast::{Program, WithModifiers};
use crate::semantic::classes::top_level_class_or_interface_declaration;
use crate::semantic::error::{Coalesce, SemanticResult};

mod classes;
mod error;
pub use error::Error as SemanticError;
mod expressions;
mod statements;

pub type Diagnostic = crate::error::Diagnostic<SemanticError>;

pub fn analyze(program: &Program) -> SemanticResult {
    match program {
        Program::Ordinary(top_level_declarations) => {
            top_level_declarations.coalesce(|WithModifiers { item: declaration, modifiers }| {
                top_level_class_or_interface_declaration(declaration, modifiers)
            })
        }
    }
}
