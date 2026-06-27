use crate::ast::{Expression, ExpressionOrTypeOrVoid, Identifier, MemberAccess};
use crate::error::Diagnose;
use crate::file::Span;
use crate::semantic::error::{Error, NameResolutionKind, SemanticResult, UnimplementedFeature};
use crate::semantic::expressions::ExpressionResult;
use crate::semantic::symbol_table::{Entity, ScopeId};
use crate::semantic::types::Type;
use crate::semantic::{SemanticAnalyzer, TypeId};
use std::ops::Deref;

impl<'a> SemanticAnalyzer<'a> {
    pub(super) fn member_access(
        &mut self,
        member_access: &MemberAccess,
        scope: ScopeId,
    ) -> SemanticResult<ExpressionResult> {
        let target_type = self.resolve_target_type(member_access.target.deref(), scope, member_access._dot_span)?;
        self.check_member_access(target_type, &member_access.name)
    }

    fn resolve_target_type(
        &mut self,
        target: &ExpressionOrTypeOrVoid,
        scope: ScopeId,
        dot_span: Span,
    ) -> SemanticResult<TypeId> {
        match target {
            ExpressionOrTypeOrVoid::Expression(Expression::Name(id)) => {
                let Some(entity) = self.symbol_table.lookup(&id.value, scope) else {
                    return Err(Error::UnknownSymbol(id.value.clone()).at(id.span).into());
                };
                match entity {
                    Entity::Type(ty) => Ok(*ty),
                    Entity::Method(_) => Err(Error::NameNotMemberNamespace(
                        id.value.clone(),
                        NameResolutionKind::Method,
                    )
                    .at(dot_span)
                    .into()),
                    Entity::Field(field_declaration) => {
                        let field_type = self.field_type(field_declaration, scope)?;
                        self.as_reference_type(&field_type, dot_span)
                    }
                    Entity::Variable(var_declaration) => {
                        let var_type = &self.attributes.get(*var_declaration).unwrap().ty;
                        self.as_reference_type(var_type, dot_span)
                    }
                }
            }
            ExpressionOrTypeOrVoid::Expression(Expression::MemberAccess(member_access)) => {
                self.resolve_member_access_target(member_access, scope, dot_span)
            }
            ExpressionOrTypeOrVoid::Expression(e) => match self.expression(e, scope)? {
                ExpressionResult::Void => {
                    Err(Error::VoidExpressionDereference.at(dot_span).into())
                }
                ExpressionResult::Value(ty) | ExpressionResult::Variable(ty) => {
                    self.as_reference_type(&ty, dot_span)
                }
            },
            ExpressionOrTypeOrVoid::Type(ty) => {
                self.as_reference_type(&self.resolve(ty, scope)?, dot_span)
            }
            ExpressionOrTypeOrVoid::Void(_) => {
                Err(Error::VoidLiteralDereference.at(dot_span).into())
            }
        }
    }

    pub fn check_member_access(
        &mut self,
        target_type: TypeId,
        member_name: &Identifier,
    ) -> SemanticResult<ExpressionResult> {
        let declaration = self.get_type(target_type);
        let scope = self.attributes.get(declaration).unwrap().scope;
        let Some(entity) = self.symbol_table.lookup(&member_name.value, scope) else {
            return Err(Error::UnknownMember {
                name: member_name.value.clone(),
                ty: declaration.name().identifier().value.clone(),
            }
            .at(member_name.span)
            .into());
        };
        match entity {
            Entity::Field(field) => Ok(ExpressionResult::Variable(self.field_type(field, scope)?)),
            Entity::Type(_) => {
                Err(Error::MemberNameNotField(member_name.value.clone(), NameResolutionKind::Type)
                    .at(member_name.span)
                    .into())
            }
            Entity::Method(_) => Err(Error::MemberNameNotField(
                member_name.value.clone(),
                NameResolutionKind::Method,
            )
            .at(member_name.span)
            .into()),
            Entity::Variable(_) => {
                unreachable!("Variables cannot be members of a class or interface")
            }
        }
    }

    fn resolve_member_access_target(
        &mut self,
        member_access: &MemberAccess,
        scope: ScopeId,
        dot_span: Span,
    ) -> SemanticResult<TypeId> {
        let target_type = self.resolve_target_type(member_access.target.deref(), scope, member_access._dot_span)?;
        let declaration = self.get_type(target_type);
        let scope = self.attributes.get(declaration).unwrap().scope;
        let Some(entity) = self.symbol_table.lookup(&member_access.name.value, scope) else {
            return Err(Error::UnknownMember {
                name: member_access.name.value.clone(),
                ty: declaration.name().identifier().value.clone(),
            }
            .at(member_access.name.span)
            .into());
        };
        match entity {
            Entity::Type(ty) => Ok(*ty),
            Entity::Method(_) => Err(Error::NameNotMemberNamespace(
                member_access.name.value.clone(),
                NameResolutionKind::Method,
            )
            .at(dot_span)
            .into()),
            Entity::Field(field) => {
                let ty = self.field_type(field, scope)?;
                self.as_reference_type(&ty, dot_span)
            }
            Entity::Variable(_) => {
                unreachable!("Variables cannot be members of a class or interface")
            }
        }
    }

    fn as_reference_type(&self, ty: &Type, dot_span: Span) -> SemanticResult<TypeId> {
        match ty {
            Type::Reference(reference) => Ok(reference.id()),
            Type::Boxed(_) => Err(UnimplementedFeature::BoxedDereference.at(dot_span).into()),
            Type::Primitive(_) => Err(Error::PrimitiveTypeDereference.at(dot_span).into()),
            Type::Null => Err(Error::NullLiteralDereference.at(dot_span).into()),
        }
    }
}
