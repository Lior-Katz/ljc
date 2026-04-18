use crate::ast::declarations::TopLevelClassOrInterfaceDeclaration;

#[derive(Debug)]
pub enum CompilationUnit {
    Ordinary(Vec<TopLevelClassOrInterfaceDeclaration>),
}
