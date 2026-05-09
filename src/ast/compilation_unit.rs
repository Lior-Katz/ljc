use crate::ast::declarations::TopLevelClassOrInterfaceDeclaration;
use crate::ast::Modified;

#[derive(Debug)]
pub enum CompilationUnit {
    Ordinary(Vec<Modified<TopLevelClassOrInterfaceDeclaration>>),
}
