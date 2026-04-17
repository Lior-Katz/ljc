use crate::parser::ast::{
    ArrayType, AssignmentOp, BinOp, CatchClause, ClassBodyDeclaration, ClassDeclaration,
    ClassMemberDeclaration, ClassTypePart, CompilationUnit, ConstructorBody, ConstructorInvocation,
    Expression, ForInit, FormalParameter, LeftHandSide, MemberAccess, MethodBody, MethodCall,
    MethodDeclaration, Modified, Modifier, Modifiers, NormalClassDeclaration, Resource, Statement,
    TopLevelClassOrInterfaceDeclaration, Type, VariableDeclaration, VariableDeclarator,
    VariableDeclaratorId, VariableInitializer,
};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

pub trait AstNode<Context = ()> {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result;
    fn fmt_tree_with_context(
        &self,
        f: &mut Formatter<'_>,
        prefix: &str,
        is_last: bool,
        _context: &Context,
    ) -> fmt::Result {
        self.fmt_tree(f, prefix, is_last)
    }
}

impl<T: AstNode<Modifiers>> AstNode<Modifiers> for Modified<T> {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        self.item
            .fmt_tree_with_context(f, prefix, is_last, &self.modifiers)
    }
}

impl<T, C> AstNode<C> for Vec<T>
where
    T: AstNode<C>,
{
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        for (i, stmt) in self.iter().enumerate() {
            stmt.fmt_tree(f, &prefix, i == self.len() - 1 && is_last)?;
        }
        Ok(())
    }
}

impl<T, C> AstNode<C> for Box<T>
where
    T: AstNode<C>,
{
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        self.deref().fmt_tree(f, prefix, is_last)
    }
}

impl<T, C> AstNode<C> for &T
where
    T: AstNode<C>,
{
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        <T as AstNode<C>>::fmt_tree(self, f, prefix, is_last)
    }
}

impl Display for dyn AstNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_tree(f, "", false)
    }
}

macro_rules! impl_display_via_ast_node {
    ($t:ty) => {
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.fmt_tree(f, "", true)
            }
        }
    };
}

impl_display_via_ast_node!(CompilationUnit);

fn branch(prefix: &str, is_last: bool) -> (String, String) {
    let child_prefix = "├──";
    let last_child_prefix = "└──";
    let child_indent = "│   ";
    let last_child_indent = "    ";

    if is_last {
        (format!("{prefix}{last_child_prefix}"), format!("{prefix}{last_child_indent}"))
    } else {
        (format!("{prefix}{child_prefix}"), format!("{prefix}{child_indent}"))
    }
}

impl AstNode for CompilationUnit {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        let (line_prefix, new_prefix) = branch(&prefix, is_last);

        match self {
            CompilationUnit::Ordinary(classes) => {
                writeln!(f, "{line_prefix}Compilation Unit")?;

                for (i, class) in classes.iter().enumerate() {
                    class.fmt_tree(f, &new_prefix, i == classes.len() - 1)?;
                }
            }
        }
        Ok(())
    }
}

impl AstNode for TopLevelClassOrInterfaceDeclaration {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        match self {
            TopLevelClassOrInterfaceDeclaration::ClassDeclaration(c) => {
                c.fmt_tree(f, prefix, is_last)
            }
        }
    }
}

impl AstNode<Modifiers> for ClassDeclaration {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        self.fmt_tree_with_context(f, prefix, is_last, &vec![])
    }

    fn fmt_tree_with_context(
        &self,
        f: &mut Formatter<'_>,
        prefix: &str,
        is_last: bool,
        modifiers: &Modifiers,
    ) -> fmt::Result {
        match self {
            ClassDeclaration::NormalClassDeclaration(c) => {
                c.fmt_tree_with_context(f, prefix, is_last, modifiers)
            }
        }
    }
}

impl AstNode<Modifiers> for NormalClassDeclaration {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        self.fmt_tree_with_context(f, prefix, is_last, &vec![])
    }

    fn fmt_tree_with_context(
        &self,
        f: &mut Formatter<'_>,
        prefix: &str,
        is_last: bool,
        modifiers: &Modifiers,
    ) -> fmt::Result {
        let (line_prefix, new_prefix) = branch(&prefix, is_last);

        writeln!(f, "{line_prefix}Class {} {:?}", self.identifier, modifiers)?;
        let total = self.body.len();

        for (i, decl) in self.body.iter().enumerate() {
            decl.fmt_tree(f, &new_prefix, i == total - 1)?;
        }
        Ok(())
    }
}

impl AstNode for ClassBodyDeclaration {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        match self {
            ClassBodyDeclaration::ClassMemberDeclaration(m) => m.fmt_tree(f, prefix, is_last),
        }
    }
}

impl AstNode<Modifiers> for ClassMemberDeclaration {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        self.fmt_tree_with_context(f, prefix, is_last, &vec![])
    }
    fn fmt_tree_with_context(
        &self,
        f: &mut Formatter<'_>,
        prefix: &str,
        is_last: bool,
        modifiers: &Modifiers,
    ) -> fmt::Result {
        let (line_prefix, new_prefix) = branch(&prefix, is_last);

        match self {
            ClassMemberDeclaration::MethodDeclaration(m) => {
                writeln!(f, "{line_prefix}Method {} {:?}", m.identifier, modifiers)?;
                m.result.fmt_tree(f, &new_prefix, false)?;
                m.fmt_tree(f, &new_prefix, true)
            }
            ClassMemberDeclaration::NestedClassDeclaration(c) => {
                c.fmt_tree_with_context(f, prefix, is_last, modifiers)
            }
            ClassMemberDeclaration::FieldDeclaration { variable_type, declarations } => {
                writeln!(f, "{line_prefix}Field declaration {:?}", modifiers)?;
                variable_type.fmt_tree(f, &new_prefix, false)?;
                declarations.fmt_tree(f, &new_prefix, true)
            }
            ClassMemberDeclaration::ConstructorDeclaration { parameters, body, name: _ } => {
                writeln!(f, "{line_prefix}Constructor declaration {:?}", modifiers)?;
                parameters.fmt_tree(f, &new_prefix, false)?;
                body.fmt_tree(f, &new_prefix, true)
            }
        }
    }
}

impl AstNode for MethodDeclaration {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, _is_last: bool) -> fmt::Result {
        let total = self.parameters.len() + 1;

        for (i, param) in self.parameters.iter().enumerate() {
            param.fmt_tree(f, &prefix, i == total - 1)?;
        }

        self.body.fmt_tree(f, &prefix, true)
    }
}

impl AstNode<Modifiers> for FormalParameter {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        self.fmt_tree_with_context(f, prefix, is_last, &vec![])
    }

    fn fmt_tree_with_context(
        &self,
        f: &mut Formatter<'_>,
        prefix: &str,
        is_last: bool,
        modifiers: &Modifiers,
    ) -> fmt::Result {
        let (line_prefix, new_prefix) = branch(&prefix, is_last);
        match self {
            FormalParameter::NormalFormalParameter(t, id) => {
                writeln!(f, "{line_prefix}Param {}", id)?;
                t.fmt_tree(f, &new_prefix, modifiers.is_empty())?;
            }
            FormalParameter::VariableArityParameter(_, id) => {
                writeln!(f, "{line_prefix}VarArg {}", id)?;
            }
        };
        modifiers.fmt_tree(f, &new_prefix, true)
    }
}

impl AstNode<Modifiers> for Type {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        self.fmt_tree_with_context(f, prefix, is_last, &vec![])
    }

    fn fmt_tree_with_context(
        &self,
        f: &mut Formatter<'_>,
        prefix: &str,
        is_last: bool,
        modifiers: &Modifiers,
    ) -> fmt::Result {
        let (line_prefix, new_prefix) = branch(&prefix, is_last);

        writeln!(f, "{line_prefix}Type")?;
        modifiers.fmt_tree(f, &new_prefix, false)?;

        let (type_line_prefix, type_prefix) = branch(&new_prefix, true);
        match self {
            Type::Byte => writeln!(f, "{type_line_prefix}byte"),
            Type::Short => writeln!(f, "{type_line_prefix}short"),
            Type::Int => writeln!(f, "{type_line_prefix}int"),
            Type::Long => writeln!(f, "{type_line_prefix}long"),
            Type::Char => writeln!(f, "{type_line_prefix}char"),
            Type::Float => writeln!(f, "{type_line_prefix}float"),
            Type::Double => writeln!(f, "{type_line_prefix}double"),
            Type::Boolean => writeln!(f, "{type_line_prefix}boolean"),
            Type::Void => writeln!(f, "{type_line_prefix}void"),
            Type::ClassType(c) => c.fmt_tree(f, &new_prefix, true),
            Type::ArrayType(ArrayType { element_type, }) => {
                writeln!(f, "{type_line_prefix}ArrayType")?;
                element_type.fmt_tree(f, &type_prefix, true)
            }
        }
    }
}

impl AstNode for Modifier {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        let (line_prefix, _) = branch(&prefix, is_last);
        match self {
            Modifier::Public => writeln!(f, "{line_prefix}public"),
            Modifier::Protected => writeln!(f, "{line_prefix}protected"),
            Modifier::Private => writeln!(f, "{line_prefix}private"),
            Modifier::Abstract => writeln!(f, "{line_prefix}abstract"),
            Modifier::Static => writeln!(f, "{line_prefix}static"),
            Modifier::Final => writeln!(f, "{line_prefix}final"),
        }
    }
}

impl AstNode for ClassTypePart {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        let (line_prefix, _) = branch(&prefix, is_last);
        writeln!(f, "{line_prefix}{}", self.identifier)
    }
}

impl AstNode for MethodBody {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        let (line_prefix, new_prefix) = branch(&prefix, is_last);

        match self {
            MethodBody::Semicolon => {
                writeln!(f, "{line_prefix};")
            }
            MethodBody::Block(stmts) => {
                writeln!(f, "{line_prefix}Block")?;
                stmts.fmt_tree(f, &new_prefix, is_last)
            }
        }
    }
}

impl AstNode for Statement {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        let (line_prefix, new_prefix) = branch(&prefix, is_last);

        match self {
            Statement::EmptyStatement => {
                writeln!(f, "{line_prefix}EmptyStatement")
            }
            Statement::ExpressionStatement(e) => e.fmt_tree(f, &prefix, is_last),
            Statement::Block(statements) => {
                writeln!(f, "{line_prefix}BlockStatement")?;
                statements.fmt_tree(f, &new_prefix, true)
            }
            Statement::VariableDeclaration(v) => v.fmt_tree(f, &prefix, is_last),
            Statement::IfStatement { condition, if_true, if_false } => {
                writeln!(f, "{line_prefix}IfStatement")?;
                let children = Children::new()
                    .push("Condition", condition)
                    .push("if_true", if_true)
                    .push_opt("if_false", if_false);
                children.fmt_tree(f, &new_prefix, true)
            }
            Statement::WhileStatement { condition, statement } => {
                writeln!(f, "{line_prefix}WhileStatement")?;
                let (condition_label_prefix, condition_prefix) = branch(&new_prefix, false);
                let (statement_label_prefix, statement_prefix) = branch(&new_prefix, true);

                writeln!(f, "{condition_label_prefix}Condition")?;
                condition.fmt_tree(f, &condition_prefix, true)?;

                writeln!(f, "{statement_label_prefix}Body")?;
                statement.fmt_tree(f, &statement_prefix, true)
            }
            Statement::ForStatement {
                initializer,
                condition,
                update,
                statement,
            } => {
                writeln!(f, "{line_prefix}ForStatement")?;
                let (initializer_label_prefix, initializer_prefix) = branch(&new_prefix, false);
                let (condition_label_prefix, condition_prefix) = branch(&new_prefix, false);
                let (update_label_prefix, update_prefix) = branch(&new_prefix, false);
                let (statement_label_prefix, statement_prefix) = branch(&new_prefix, true);

                writeln!(f, "{initializer_label_prefix}Initializer")?;
                initializer.fmt_tree(f, &initializer_prefix, true)?;

                if let Some(condition) = condition {
                    writeln!(f, "{condition_label_prefix}Condition")?;
                    condition.fmt_tree(f, &condition_prefix, true)?;
                }

                writeln!(f, "{update_label_prefix}Update")?;
                update.fmt_tree(f, &update_prefix, true)?;

                writeln!(f, "{statement_label_prefix}Body")?;
                statement.fmt_tree(f, &statement_prefix, true)
            }
            Statement::ForEachStatement {
                variable_declaration,
                iterable,
                statement,
            } => {
                writeln!(f, "{line_prefix}ForEachStatement")?;
                let (var_declaration_label_prefix, var_declaration_prefix) =
                    branch(&new_prefix, false);
                let (iterable_label_prefix, iterable_prefix) = branch(&new_prefix, false);
                let (statement_label_prefix, statement_prefix) = branch(&new_prefix, true);

                writeln!(f, "{var_declaration_label_prefix}Initializer")?;
                variable_declaration.fmt_tree(f, &var_declaration_prefix, true)?;

                writeln!(f, "{iterable_label_prefix}Iterable")?;
                iterable.fmt_tree(f, &iterable_prefix, true)?;

                writeln!(f, "{statement_label_prefix}Body")?;
                statement.fmt_tree(f, &statement_prefix, true)
            }
            Statement::DoWhile { statement, condition } => {
                writeln!(f, "{line_prefix}DoWhileStatement")?;
                let (statement_label_prefix, statement_prefix) = branch(&new_prefix, false);
                let (condition_label_prefix, condition_prefix) = branch(&new_prefix, true);

                writeln!(f, "{statement_label_prefix}Body")?;
                statement.fmt_tree(f, &statement_prefix, true)?;

                writeln!(f, "{condition_label_prefix}Condition")?;
                condition.fmt_tree(f, &condition_prefix, true)
            }
            Statement::Labeled { label, body } => {
                writeln!(f, "{line_prefix}LabeledStatement: {label}")?;
                body.fmt_tree(f, &new_prefix, true)
            }
            Statement::Break(label) => {
                let label = match label {
                    None => "",
                    Some(v) => &format!(" {}", &v),
                };
                writeln!(f, "{line_prefix}BreakStatement{label}")
            }
            Statement::Continue(label) => {
                let label = match label {
                    None => "",
                    Some(v) => &format!(" {}", &v),
                };
                writeln!(f, "{line_prefix}ContinueStatement{label}")
            }
            Statement::Assert { condition, detail_message } => {
                writeln!(f, "{line_prefix}AssertStatement")?;
                let children = Children::new()
                    .push("Condition", condition)
                    .push_opt("DetailMessage", detail_message);
                children.fmt_tree(f, &new_prefix, true)
            }
            Statement::Return(e) => {
                writeln!(f, "{line_prefix}ReturnStatement")?;
                if let Some(e) = e {
                    e.fmt_tree(f, &new_prefix, true)?;
                }
                Ok(())
            }
            Statement::Try {
                resource,
                try_block,
                exception_handlers,
                finally_block,
            } => {
                writeln!(f, "{line_prefix}TryStatement")?;
                let resources = if resource.is_empty() { None } else { Some(resource) };

                let exception_handlers = if exception_handlers.is_empty() {
                    None
                } else {
                    Some(exception_handlers)
                };

                let children = Children::new()
                    .push_opt("Resources", &resources)
                    .push("TryBlock", try_block)
                    .push_opt("ExceptionHandlers", &exception_handlers)
                    .push_opt("FinallyBlock", finally_block);

                children.fmt_tree(f, &new_prefix, true)
            }
            Statement::Throw(e) => {
                writeln!(f, "{line_prefix}ThrowStatement")?;
                e.fmt_tree(f, &new_prefix, true)
            }
        }
    }
}

impl AstNode for Expression {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        let (line_prefix, new_prefix) = branch(&prefix, is_last);

        match self {
            Expression::IntegerLiteral(v) => writeln!(f, "{line_prefix}int {}", v),
            Expression::LongLiteral(v) => writeln!(f, "{line_prefix}long {}", v),
            Expression::BooleanLiteral(v) => writeln!(f, "{line_prefix}boolean {}", v),
            Expression::CharLiteral(v) => writeln!(f, "{line_prefix}char '{}'", v),
            Expression::StringLiteral(v) => writeln!(f, "{line_prefix}String \"{}\"", v),
            Expression::NullLiteral => writeln!(f, "{line_prefix}null"),
            Expression::Name(v) => writeln!(f, "{line_prefix}{}", v),
            Expression::Assignment { lhs, rhs, op } => {
                writeln!(f, "{line_prefix}Assignment {op}")?;
                lhs.fmt_tree(f, &new_prefix, false)?;
                rhs.fmt_tree(f, &new_prefix, true)
            }
            Expression::PostIncrement(e) => {
                writeln!(f, "{line_prefix}PostIncrement")?;
                e.fmt_tree(f, &new_prefix, true)
            }
            Expression::PostDecrement(e) => {
                writeln!(f, "{line_prefix}PostDecrement")?;
                e.fmt_tree(f, &new_prefix, true)
            }
            Expression::PreIncrement(e) => {
                writeln!(f, "{line_prefix}PreIncrement")?;
                e.fmt_tree(f, &new_prefix, true)
            }
            Expression::PreDecrement(e) => {
                writeln!(f, "{line_prefix}PreDecrement")?;
                e.fmt_tree(f, &new_prefix, true)
            }
            Expression::BitwiseComplement(e) => {
                writeln!(f, "{line_prefix}BitwiseComplement")?;
                e.fmt_tree(f, &new_prefix, true)
            }
            Expression::LogicalNot(e) => {
                writeln!(f, "{line_prefix}LogicalNot")?;
                e.fmt_tree(f, &new_prefix, true)
            }
            Expression::UnaryPlus(e) => {
                writeln!(f, "{line_prefix}UnaryPlus")?;
                e.fmt_tree(f, &new_prefix, true)
            }
            Expression::UnaryMinus(e) => {
                writeln!(f, "{line_prefix}UnaryMinus")?;
                e.fmt_tree(f, &new_prefix, true)
            }
            Expression::BinaryOp { left, op, right } => {
                op.fmt_tree(f, &line_prefix, is_last)?;
                left.fmt_tree(f, &new_prefix, false)?;
                right.fmt_tree(f, &new_prefix, true)
            }
            Expression::ConditionalExpression { condition, if_true, if_false } => {
                writeln!(f, "{line_prefix}ConditionalExpression")?;
                condition.fmt_tree(f, &new_prefix, false)?;
                if_true.fmt_tree(f, &new_prefix, false)?;
                if_false.fmt_tree(f, &new_prefix, true)
            }
            Expression::Type(t) => t.fmt_tree(f, &prefix, is_last),
            Expression::MemberAccess(v) => v.fmt_tree(f, &prefix, is_last),
            Expression::MethodCall(v) => v.fmt_tree(f, &prefix, is_last),
            Expression::InstanceCreation { type_to_instantiate, arguments } => {
                writeln!(f, "{line_prefix}NewInstance")?;
                type_to_instantiate.fmt_tree(f, &new_prefix, arguments.is_empty())?;
                arguments.fmt_tree(f, &new_prefix, true)
            }
        }
    }
}

impl AstNode for Modified<Expression> {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        let (line_prefix, new_prefix) = branch(&prefix, is_last);
        writeln!(f, "{line_prefix}Modifiers {:?}", self.modifiers)?;
        <Expression as AstNode<()>>::fmt_tree(&self.item, f, &new_prefix, true)
    }
}

impl Display for AssignmentOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AssignmentOp::Identity => write!(f, "="),
            AssignmentOp::Add => write!(f, "+="),
            AssignmentOp::Subtract => write!(f, "-="),
            AssignmentOp::Multiply => write!(f, "*="),
            AssignmentOp::Divide => write!(f, "/="),
            AssignmentOp::Modulo => write!(f, "%="),
            AssignmentOp::LeftShift => write!(f, "<<="),
            AssignmentOp::SignedRightShift => write!(f, ">>="),
            AssignmentOp::UnsignedRightShift => write!(f, ">>>="),
            AssignmentOp::BitwiseAnd => write!(f, "&="),
            AssignmentOp::BitwiseXor => write!(f, "^="),
            AssignmentOp::BitwiseOr => write!(f, "|="),
        }
    }
}

impl LeftHandSide {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        let (line_prefix, _new_prefix) = branch(&prefix, is_last);
        match self {
            LeftHandSide::ExpressionName(v) => writeln!(f, "{line_prefix}{}", v),
            LeftHandSide::MemberAccess(v) => v.fmt_tree(f, prefix, is_last),
        }
    }
}

impl AstNode for BinOp {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, _is_last: bool) -> fmt::Result {
        match self {
            BinOp::Add => writeln!(f, "{prefix} +"),
            BinOp::Subtract => writeln!(f, "{prefix} -"),
            BinOp::Multiply => writeln!(f, "{prefix} *"),
            BinOp::Divide => writeln!(f, "{prefix} /"),
            BinOp::Modulo => writeln!(f, "{prefix} %"),
            BinOp::LeftShift => writeln!(f, "{prefix} <<"),
            BinOp::SignedRightShift => writeln!(f, "{prefix} >>"),
            BinOp::UnsignedRightShift => writeln!(f, "{prefix} >>>"),
            BinOp::Less => writeln!(f, "{prefix} <"),
            BinOp::Greater => writeln!(f, "{prefix} >"),
            BinOp::LessEqual => writeln!(f, "{prefix} <="),
            BinOp::GreaterEqual => writeln!(f, "{prefix} >="),
            BinOp::Equal => writeln!(f, "{prefix} =="),
            BinOp::NotEqual => writeln!(f, "{prefix} !="),
            BinOp::BitwiseAnd => writeln!(f, "{prefix} &"),
            BinOp::BitwiseXor => writeln!(f, "{prefix} ^"),
            BinOp::BitwiseOr => writeln!(f, "{prefix} |"),
            BinOp::LogicalAnd => writeln!(f, "{prefix} &&"),
            BinOp::LogicalOr => writeln!(f, "{prefix} ||"),
        }
    }
}

impl AstNode<Modifiers> for VariableDeclaration {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        self.fmt_tree_with_context(f, prefix, is_last, &vec![])
    }

    fn fmt_tree_with_context(
        &self,
        f: &mut Formatter<'_>,
        prefix: &str,
        is_last: bool,
        modifiers: &Modifiers,
    ) -> fmt::Result {
        let (line_prefix, new_prefix) = branch(&prefix, is_last);

        writeln!(f, "{line_prefix}VariableDeclaration {:?}", modifiers)?;

        self.variable_type.fmt_tree(f, &new_prefix, false)?;
        self.declarators.fmt_tree(f, &new_prefix, true)
    }
}

impl AstNode for VariableDeclarator {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        let (line_prefix, new_prefix) = branch(&prefix, is_last);
        writeln!(f, "{line_prefix}{}", self.name)?;
        if let Some(initializer) = &self.initializer {
            initializer.fmt_tree(f, &new_prefix, true)?;
        }
        Ok(())
    }
}

impl AstNode for VariableInitializer {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        match self {
            VariableInitializer::Expression(e) => e.fmt_tree(f, prefix, is_last),
        }
    }
}

impl Display for VariableDeclaratorId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            VariableDeclaratorId::Named(name) => name,
            VariableDeclaratorId::Unnamed => "unnamed",
        };
        write!(f, "{}", name)
    }
}

impl AstNode for MemberAccess {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        let (line_prefix, new_prefix) = branch(prefix, is_last);

        writeln!(f, "{line_prefix}MemberAccess")?;

        let (field_prefix, _) = branch(&new_prefix, false);
        writeln!(f, "{field_prefix}field: {}", self.name)?;

        let (target_prefix, target_new_prefix) = branch(&new_prefix, true);
        writeln!(f, "{target_prefix}target:")?;
        self.target.fmt_tree(f, &target_new_prefix, true)
    }
}

impl AstNode for MethodCall {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        let (line_prefix, new_prefix) = branch(prefix, is_last);
        let has_args = self.arguments.is_empty();

        writeln!(f, "{line_prefix}MethodInvocation")?;

        let (method_prefix, _) = branch(&new_prefix, false);
        writeln!(f, "{method_prefix}method: {}", self.name)?;

        let (target_prefix, target_new_prefix) = branch(&new_prefix, has_args);
        writeln!(f, "{target_prefix}target:")?;
        self.target.fmt_tree(f, &target_new_prefix, true)?;

        if !has_args {
            let (args_prefix, args_new_prefix) = branch(&new_prefix, true);
            writeln!(f, "{args_prefix}args:")?;
            self.arguments.fmt_tree(f, &args_new_prefix, true)?;
        }

        Ok(())
    }
}

impl AstNode for ConstructorBody {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        let (line_prefix, new_prefix) = branch(prefix, is_last);
        writeln!(f, "{line_prefix}ConstructorBody")?;

        let epilogue = if self.epilogue.is_empty() {
            None
        } else {
            Some(&self.epilogue)
        };

        let children = Children::new()
            .push_opt("Prologue", &self.prologue)
            .push_opt("ConstructorInvocation", &self.constructor_invocation)
            .push_opt("Epilogue", &epilogue);
        children.fmt_tree(f, &new_prefix, true)
    }
}

impl AstNode for ConstructorInvocation {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        // let (line_prefix, _) = branch(prefix, is_last);
        match self {
            ConstructorInvocation::Alternate { arguments } => {
                arguments.fmt_tree(f, &prefix, is_last)
            }
        }
    }
}

impl AstNode for ForInit {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        match self {
            ForInit::LocalVarDeclaration(v) => v.fmt_tree(f, prefix, is_last),
            ForInit::Expressions(e) => e.fmt_tree(f, prefix, is_last),
        }
    }
}

impl AstNode for Resource {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        match self {
            Resource::VariableDeclaration(v) => v.fmt_tree(f, prefix, is_last),
            Resource::VariableAccess(v) => v.fmt_tree(f, prefix, is_last),
        }
    }
}

impl AstNode for CatchClause {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> fmt::Result {
        let (line_prefix, new_prefix) = branch(prefix, is_last);
        // let children = Children::new().push()
        writeln!(f, "{line_prefix}CatchClause")?;
        let (catch_parameter_label_prefix, catch_parameter_prefix) = branch(&new_prefix, false);
        writeln!(f, "{catch_parameter_label_prefix}{}", self.var_id)?;

        writeln!(f, "{catch_parameter_label_prefix}CatchType")?;
        self.catch_type.fmt_tree(f, &catch_parameter_prefix, true)?;
        self.body.fmt_tree(f, &new_prefix, true)
    }
}

struct Children<'a> {
    inner: Vec<(&'a str, &'a dyn AstNode)>,
}

impl<'a> Children<'a> {
    fn new() -> Self {
        Self { inner: Vec::new() }
    }

    fn push(mut self, label: &'a str, node: &'a dyn AstNode) -> Self {
        self.inner.push((label, node));
        self
    }

    fn push_opt<T>(mut self, label: &'a str, node: &'a Option<T>) -> Self
    where
        T: AstNode,
    {
        if let Some(n) = node {
            self.inner.push((label, n));
        }
        self
    }
}

impl AstNode for Children<'_> {
    fn fmt_tree(&self, f: &mut Formatter<'_>, prefix: &str, _is_last: bool) -> fmt::Result {
        for (i, (label, node)) in self.inner.iter().enumerate() {
            let is_last_child = i == self.inner.len() - 1;
            let (label_prefix, child_prefix) = branch(prefix, is_last_child);

            writeln!(f, "{label_prefix}{label}")?;
            node.fmt_tree(f, &child_prefix, true)?;
        }
        Ok(())
    }
}
