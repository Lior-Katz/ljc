use crate::ast::{MethodBody, MethodDeclaration, Modifiers};
use crate::error::Diagnose;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::error::{CoalesceIter, SemanticResult, UnimplementedFeature};
use crate::semantic::symbol_table::ScopeId;

impl<'a> SemanticAnalyzer<'a> {
    #[allow(unused_variables)]
    pub(super) fn method(
        &mut self,
        method: &'a MethodDeclaration,
        modifiers: &Modifiers,
    ) -> SemanticResult {
        let body_scope = self.symbol_table.new_scope();
        self.method_body(&method.body, body_scope)
    }

    fn method_body(&mut self, body: &'a MethodBody, scope: ScopeId) -> SemanticResult {
        match body {
            MethodBody::Block(statements) => statements.coalesce(|s| self.statement(s, scope)),
            MethodBody::Semicolon(s) => {
                Err(UnimplementedFeature::NoBodyMethod.at(s.clone()).into())
            }
        }
    }
}
