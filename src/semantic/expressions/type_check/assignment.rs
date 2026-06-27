use crate::ast::{AssignmentOp, BinOp, Expression, LeftHandSide};
use crate::error::Diagnose;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::error::{SemanticResult, TypeMismatch, UnimplementedFeature};
use crate::semantic::expressions::ExpressionResult;
use crate::semantic::symbol_table::ScopeId;

impl SemanticAnalyzer<'_> {
    pub fn assignment(
        &mut self,
        lhs: &LeftHandSide,
        rhs: &Box<Expression>,
        op: &AssignmentOp,
        scope: ScopeId,
    ) -> SemanticResult<ExpressionResult> {
        match op {
            AssignmentOp::Identity => self.simple_assignment(lhs, rhs, scope),
            AssignmentOp::Add => self.binary_op(&lhs.into(), rhs, &BinOp::Add, scope),
            AssignmentOp::Subtract => self.binary_op(&lhs.into(), rhs, &BinOp::Subtract, scope),
            AssignmentOp::Multiply => self.binary_op(&lhs.into(), rhs, &BinOp::Multiply, scope),
            AssignmentOp::Divide => self.binary_op(&lhs.into(), rhs, &BinOp::Divide, scope),
            AssignmentOp::Modulo => self.binary_op(&lhs.into(), rhs, &BinOp::Modulo, scope),
            AssignmentOp::LeftShift => self.binary_op(&lhs.into(), rhs, &BinOp::LeftShift, scope),
            AssignmentOp::SignedRightShift => {
                self.binary_op(&lhs.into(), rhs, &BinOp::SignedRightShift, scope)
            }
            AssignmentOp::UnsignedRightShift => {
                self.binary_op(&lhs.into(), rhs, &BinOp::UnsignedRightShift, scope)
            }
            AssignmentOp::BitwiseAnd => self.binary_op(&lhs.into(), rhs, &BinOp::BitwiseAnd, scope),
            AssignmentOp::BitwiseXor => self.binary_op(&lhs.into(), rhs, &BinOp::BitwiseXor, scope),
            AssignmentOp::BitwiseOr => self.binary_op(&lhs.into(), rhs, &BinOp::BitwiseOr, scope),
        }
    }

    fn simple_assignment(
        &mut self,
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
            LeftHandSide::MemberAccess(_) => Err(UnimplementedFeature::MemberAssignment
                .at(*left_hand_side.span())
                .into()),
            LeftHandSide::ArrayAccess(_) => Err(UnimplementedFeature::ArrayAccess
                .at(*left_hand_side.span())
                .into()),
        }
    }
}

impl From<&LeftHandSide> for Expression {
    fn from(value: &LeftHandSide) -> Self {
        match value {
            LeftHandSide::ExpressionName(name) => Expression::Name(name.clone()),
            LeftHandSide::MemberAccess(member_access) => {
                Expression::MemberAccess(member_access.clone())
            }
            LeftHandSide::ArrayAccess(array_access) => {
                Expression::ArrayAccess(array_access.clone())
            }
        }
    }
}
