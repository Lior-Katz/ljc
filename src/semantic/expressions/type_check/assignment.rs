use crate::ast::{AssignmentOp, Expression, LeftHandSide};
use crate::error::Diagnose;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::error::{SemanticResult, TypeMismatch, UnimplementedFeature};
use crate::semantic::expressions::ExpressionResult;
use crate::semantic::symbol_table::ScopeId;

impl SemanticAnalyzer<'_> {
    pub fn assignment(
        &self,
        lhs: &LeftHandSide,
        rhs: &Box<Expression>,
        op: &AssignmentOp,
        scope: ScopeId,
    ) -> SemanticResult<ExpressionResult> {
        match op {
            AssignmentOp::Identity => self.simple_assignment(lhs, rhs, scope),
            _ => Err(UnimplementedFeature::CompoundAssignment
                .at(*lhs.span())
                .into()),
        }
    }

    fn simple_assignment(
        &self,
        lhs: &LeftHandSide,
        rhs: &Box<Expression>,
        scope: ScopeId,
    ) -> SemanticResult<ExpressionResult> {
        let lhs_type = match self.left_hand_side(lhs, scope)? {
            ExpressionResult::Void => Err(TypeMismatch::VoidExpression),
            ExpressionResult::Value(_) => Err(TypeMismatch::NeedVariableFoundValue),
            ExpressionResult::Variable(ty) => Ok(ty),
        }
        .map_err(|e| e.at(*lhs.span()))?;
        let rhs_type = match self.type_check(rhs, scope)? {
            ExpressionResult::Void => Err(TypeMismatch::VoidExpression),
            ExpressionResult::Value(ty) | ExpressionResult::Variable(ty) => Ok(ty),
        }
        .map_err(|e| e.at(*rhs.span()))?;
        if !lhs_type.is_assignable_from(&rhs_type) {
            return Err(TypeMismatch::IncompatibleAssignment.at(*rhs.span()).into());
        }
        Ok(ExpressionResult::Value(lhs_type))
    }

    fn left_hand_side(
        &self,
        left_hand_side: &LeftHandSide,
        scope: ScopeId,
    ) -> SemanticResult<ExpressionResult> {
        match left_hand_side {
            LeftHandSide::ExpressionName(name) => self.name_expression(name, scope),
            LeftHandSide::MemberAccess(_) => Err(UnimplementedFeature::MemberAccess
                .at(*left_hand_side.span())
                .into()),
            LeftHandSide::ArrayAccess(_) => Err(UnimplementedFeature::ArrayAccess
                .at(*left_hand_side.span())
                .into()),
        }
    }
}
