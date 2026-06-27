use crate::ast::FieldDeclaration;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::ast_tags::FieldDeclarationAttributes;
use crate::semantic::error::SemanticResult;
use crate::semantic::symbol_table::ScopeId;
use crate::semantic::types::Type;

impl<'a> SemanticAnalyzer<'a> {
    pub fn field_type(
        &mut self,
        field_declaration: &'a FieldDeclaration,
        scope: ScopeId,
    ) -> SemanticResult<Type> {
        if let Some(attributes) = self.attributes.get_mut(field_declaration) {
            Ok(attributes.ty.clone())
        } else {
            let ty = self.resolve(&field_declaration.variable_type, scope)?;
            self.attributes
                .insert(field_declaration, FieldDeclarationAttributes { ty: ty.clone() });
            Ok(ty)
        }
    }
}
