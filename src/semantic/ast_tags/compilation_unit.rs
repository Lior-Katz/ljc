use crate::semantic::symbol_table::ScopeId;

#[derive(Debug)]
pub struct CompilationUnitAttributes {
    #[expect(dead_code)]
    pub scope: ScopeId,
}
