use crate::ast::{Program, TypeDeclaration, WithModifiers};
use crate::semantic::ast_tags::{Attributes, CompilationUnitAttributes};
use crate::semantic::error::{CoalesceIter, SemanticResult};
use crate::semantic::symbol_table::{Entity, SymbolTable};

mod classes;
mod error;
pub use error::Error as SemanticError;
mod ast_tags;
mod expressions;
mod statements;
mod symbol_table;
mod types;

pub type Diagnostic = crate::error::Diagnostic<SemanticError>;

pub struct SemanticAnalyzer<'a> {
    attributes: Attributes<'a>,
    symbol_table: SymbolTable<'a>,
    types: Vec<&'a TypeDeclaration>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TypeId(usize);

impl<'a> SemanticAnalyzer<'a> {
    pub fn new() -> Self {
        Self {
            attributes: Attributes::new(),
            symbol_table: SymbolTable::new(),
            types: Vec::new(),
        }
    }

    pub fn analyze(mut self, program: &'a Program) -> SemanticResult {
        self.add_declaration_names(program);
        match program {
            Program::Ordinary(top_level_declarations) => {
                top_level_declarations.coalesce(|WithModifiers { item: declaration, modifiers }| {
                    self.top_level_class_or_interface_declaration(declaration, modifiers)
                })
            }
        }
    }

    fn add_declaration_names(&mut self, program: &'a Program) {
        let scope_id = self.symbol_table.new_scope();
        self.attributes
            .insert(program, CompilationUnitAttributes { scope: scope_id });
        match program {
            Program::Ordinary(top_level_declarations) => {
                for WithModifiers { item: declaration, .. } in top_level_declarations {
                    self.add_type_declaration_and_member_names(declaration, scope_id);
                    let type_id = self.add_type(declaration);
                    self.symbol_table
                        .scope_mut(scope_id)
                        .put(declaration.name().identifier().value.clone(), Entity::Type(type_id));
                }
            }
        }
    }

    fn add_type(&mut self, type_declaration: &'a TypeDeclaration) -> TypeId {
        let id = self.types.len();
        self.types.push(type_declaration);
        TypeId(id)
    }

    fn get_type(&self, type_id: TypeId) -> &'a TypeDeclaration {
        self.types[type_id.0]
    }
}
