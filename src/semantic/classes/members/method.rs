use crate::ast::{MethodBody, MethodDeclaration, Modifiers};
use crate::error::Diagnose;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::error::{CoalesceIter, SemanticResult, UnimplementedFeature};

impl SemanticAnalyzer<'_> {
    #[allow(unused_variables)]
    pub(super) fn method(
        &mut self,
        method: &MethodDeclaration,
        modifiers: &Modifiers,
    ) -> SemanticResult {
        self.method_body(&method.body)
    }

    fn method_body(&mut self, body: &MethodBody) -> SemanticResult {
        match body {
            MethodBody::Block(statements) => statements.coalesce(|s| self.statement(s)),
            MethodBody::Semicolon(s) => {
                Err(UnimplementedFeature::NoBodyMethod.at(s.clone()).into())
            }
        }
    }
}
