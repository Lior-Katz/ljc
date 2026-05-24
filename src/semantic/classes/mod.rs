mod class;
use crate::ast::{ClassDeclaration, Modifiers, TopLevelClassOrInterfaceDeclaration};
use crate::semantic::error::SemanticResult;
pub use class::class_declaration;

mod members;

pub fn top_level_class_or_interface_declaration(
    declaration: &TopLevelClassOrInterfaceDeclaration,
    modifiers: &Modifiers,
) -> SemanticResult {
    match declaration {
        TopLevelClassOrInterfaceDeclaration::Class(ClassDeclaration::NormalClass(c)) => {
            class_declaration(c, modifiers)
        }
        _ => todo!("Only normal class declarations are supported for now"),
    }
}
