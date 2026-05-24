use crate::ast::{MethodBody, MethodDeclaration, Modifiers};
use crate::error::Diagnose;
use crate::semantic::error::{Coalesce, SemanticResult, UnimplementedFeature};
use crate::semantic::statements::statement;

#[allow(unused_variables)]
pub fn method(method: &MethodDeclaration, modifiers: &Modifiers) -> SemanticResult {
    method_body(&method.body)
}

pub fn method_body(body: &MethodBody) -> SemanticResult {
    match body {
        MethodBody::Block(statements) => statements.coalesce(|s| statement(s)),
        MethodBody::Semicolon(s) => Err(UnimplementedFeature::NoBodyMethod.at(s.clone()).into()),
    }
}
