use crate::semantic::symbol_table::ScopeId;

#[derive(Debug)]
pub struct TypeDeclarationAttributes {
    #[expect(dead_code)]
    pub scope: ScopeId,
}
