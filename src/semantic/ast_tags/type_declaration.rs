use crate::semantic::symbol_table::ScopeId;

#[derive(Debug)]
pub struct TypeDeclarationAttributes {
    pub scope: ScopeId,
}
