use crate::ast::declarations::TypeDeclaration;
use crate::ast::Modified;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum CompilationUnit {
    Ordinary(Vec<Modified<TypeDeclaration>>),
}
