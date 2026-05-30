use crate::ast::{Program, WithModifiers};
use crate::semantic::error::{CoalesceIter, SemanticResult};

mod classes;
mod error;
pub use error::Error as SemanticError;
mod expressions;
mod statements;
mod types;

pub type Diagnostic = crate::error::Diagnostic<SemanticError>;

#[allow(dead_code)]
pub struct SemanticAnalyzer {}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn analyze(&mut self, program: &Program) -> SemanticResult {
        match program {
            Program::Ordinary(top_level_declarations) => {
                top_level_declarations.coalesce(|WithModifiers { item: declaration, modifiers }| {
                    self.top_level_class_or_interface_declaration(declaration, modifiers)
                })
            }
        }
    }
}
