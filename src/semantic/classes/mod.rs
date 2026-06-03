use crate::ast::{
    AnnotationInterfaceDeclaration, ClassBodyDeclaration, ClassDeclaration, ClassMemberDeclaration,
    EnumBody, EnumDeclaration, FieldDeclaration, InterfaceDeclaration, Modifiers,
    RecordDeclaration, TypeDeclaration, VariableDeclaratorId, WithModifiers,
};
use crate::error::Diagnose;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::ast_tags::TypeDeclarationAttributes;
use crate::semantic::error::{SemanticResult, UnimplementedFeature};
use crate::semantic::symbol_table::{Entity, ScopeId};

mod class;
mod members;

impl<'a> SemanticAnalyzer<'a> {
    pub(super) fn top_level_class_or_interface_declaration(
        &mut self,
        declaration: &TypeDeclaration,
        modifiers: &Modifiers,
    ) -> SemanticResult {
        let span = declaration.span().clone();
        match declaration {
            TypeDeclaration::Class(c) => self.class_declaration(c, modifiers),
            TypeDeclaration::Record(_) => Err(UnimplementedFeature::RecordClass.at(span).into()),
            TypeDeclaration::Enum(_) => Err(UnimplementedFeature::EnumClass.at(span).into()),
            TypeDeclaration::Interface(_) => Err(UnimplementedFeature::Interface.at(span).into()),
            TypeDeclaration::AnnotationInterface(_) => {
                Err(UnimplementedFeature::AnnotationInterface.at(span).into())
            }
        }
    }

    pub(super) fn add_type_declaration_and_member_names(
        &mut self,
        type_declaration: &'a TypeDeclaration,
        parent: ScopeId,
    ) {
        let scope_id = self.symbol_table.new_child_scope(parent);
        self.attributes
            .insert(type_declaration, TypeDeclarationAttributes { scope: scope_id });
        let members = get_members(type_declaration);
        members.for_each(|member| match member {
            ClassMemberDeclaration::NestedClassOrInterface(ty) => {
                self.symbol_table
                    .scope_mut(scope_id)
                    .put(ty.name().identifier().value.clone(), Entity::Type(ty));
                self.add_type_declaration_and_member_names(ty, scope_id);
            }

            ClassMemberDeclaration::Method(m) => {
                self.symbol_table
                    .scope_mut(scope_id)
                    .put(m.identifier.value.clone(), Entity::Method(m));
            }

            ClassMemberDeclaration::Field(f) => {
                let FieldDeclaration { declarators, .. } = f;
                for declarator in declarators {
                    if let VariableDeclaratorId::Named(id) = &declarator.name {
                        self.symbol_table
                            .scope_mut(scope_id)
                            .put(id.value.clone(), Entity::Field(f));
                    }
                }
            }
            ClassMemberDeclaration::Constructor { .. }
            | ClassMemberDeclaration::CompactConstructor { .. } => {}
        });
    }
}


fn get_members(
    type_declaration: &TypeDeclaration,
) -> Box<dyn Iterator<Item = &ClassMemberDeclaration> + '_> {
    match type_declaration {
        TypeDeclaration::Class(ClassDeclaration { body, .. })
        | TypeDeclaration::Record(RecordDeclaration { body, .. })
        | TypeDeclaration::Enum(EnumDeclaration {
                                    body: EnumBody { body_declarations: body, .. },
                                    ..
                                }) => Box::new(body.iter().filter_map(|d| match d {
            ClassBodyDeclaration::ClassMember(WithModifiers { item, .. }) => Some(item),
            ClassBodyDeclaration::InstanceInitializer(_)
            | ClassBodyDeclaration::StaticInitializer(_) => None,
        })),
        TypeDeclaration::Interface(InterfaceDeclaration { body, .. })
        | TypeDeclaration::AnnotationInterface(AnnotationInterfaceDeclaration { body, .. }) => {
            Box::new(body.iter().map(|WithModifiers { item, .. }| item))
        }
    }
}