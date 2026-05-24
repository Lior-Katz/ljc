use crate::ast::{MethodBody, MethodDeclaration, Modifiers};
use crate::semantic::error::{Coalesce, SemanticResult};
use crate::semantic::statements::statement;

#[allow(unused_variables)]
pub fn method(method: &MethodDeclaration, modifiers: &Modifiers) -> SemanticResult {
    method_body(&method.body)
}

pub fn method_body(body: &MethodBody) -> SemanticResult {
    match body {
        MethodBody::Block(statements) => statements.coalesce(|s| statement(s)),
        _ => todo!("Only block method bodies are supported for now"),
    }
}
