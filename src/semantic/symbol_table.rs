use crate::ast::{FieldDeclaration, MethodDeclaration, TypeDeclaration};
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
pub struct SymbolTable<'a> {
    scopes: Vec<Scope<'a>>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        Self { scopes: Vec::new() }
    }

    pub fn new_scope(&mut self) -> ScopeId {
        let id = self.scopes.len();
        self.scopes.push(Scope::new());
        ScopeId(id)
    }

    pub fn new_child_scope(&mut self, parent: ScopeId) -> ScopeId {
        let id = self.scopes.len();
        self.scopes.push(Scope::with_parent(parent));
        ScopeId(id)
    }

    #[allow(dead_code)]
    fn scope(&self, id: ScopeId) -> &Scope<'a> {
        &self.scopes[id.0]
    }

    pub fn scope_mut(&mut self, id: ScopeId) -> &mut Scope<'a> {
        &mut self.scopes[id.0]
    }

    #[allow(dead_code)]
    pub fn lookup(&'_ self, name: &str, scope_id: ScopeId) -> Option<&'_ Entity<'_>> {
        let scope = self.scope(scope_id);
        scope.lookup(name).or_else(|| {
            scope
                .parent
                .and_then(|parent_id| self.lookup(name, parent_id))
        })
    }
}

#[derive(Debug)]
pub struct Scope<'a> {
    entities: HashMap<String, Entity<'a>>,
    #[allow(dead_code)]
    parent: Option<ScopeId>,
}

#[derive(Debug, Copy, Clone)]
pub struct ScopeId(usize);

impl<'a> Scope<'a> {
    fn new() -> Self {
        Self {
            entities: HashMap::new(),
            parent: None,
        }
    }

    fn with_parent(parent: ScopeId) -> Self {
        Self {
            entities: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn put(&mut self, name: String, entity: Entity<'a>) {
        self.entities.insert(name, entity);
    }

    fn lookup(&'a self, name: &str) -> Option<&'a Entity<'a>> {
        self.entities.get(name)
    }
}

#[expect(dead_code)]
#[derive(Debug)]
pub enum Entity<'a> {
    Type(&'a TypeDeclaration),
    Method(&'a MethodDeclaration),
    Field(&'a FieldDeclaration),
}
