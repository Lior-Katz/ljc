use crate::ast::declarations::TypeDeclaration;
use crate::ast::Modified;

#[derive(Debug)]
pub enum CompilationUnit {
    Ordinary(Vec<Modified<TypeDeclaration>>),
}
