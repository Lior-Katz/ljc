use std::fmt;
use std::fmt::{Display, Formatter};
use crate::parser::ast::{ClassBodyDeclaration, ClassDeclaration, ClassMemberDeclaration, CompilationUnit, Expression, FormalParameter, LeftHandSide, MethodBody, MethodDeclaration, MethodResult, NormalClassDeclaration, Statement, TopLevelClassOrInterfaceDeclaration, Type};

pub trait AstNode{
    // fn to_string(&self, prefix: String, is_last: bool) -> String;
    fn fmt_tree(
        &self,
        f: &mut Formatter<'_>,
        prefix: &str,
        is_last: bool,
    ) -> fmt::Result;
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
impl_display_via_ast_node!(TopLevelClassOrInterfaceDeclaration);
impl_display_via_ast_node!(ClassDeclaration);
impl_display_via_ast_node!(NormalClassDeclaration);
impl_display_via_ast_node!(ClassBodyDeclaration);
impl_display_via_ast_node!(ClassMemberDeclaration);
impl_display_via_ast_node!(MethodDeclaration);
impl_display_via_ast_node!(FormalParameter);
impl_display_via_ast_node!(MethodBody);
impl_display_via_ast_node!(Statement);

fn branch(prefix: &str, is_last: bool) -> (String, String) {
    let child_prefix = "├──";
    let last_child_prefix = "└──";
    let child_indent = "│   ";
    let last_child_indent = "    ";

    if is_last {
        (
            format!("{prefix}{last_child_prefix}"),
            format!("{prefix}{last_child_indent}"),
        )
    } else {
        (
            format!("{prefix}{child_prefix}"),
            format!("{prefix}{child_indent}"),
        )
    }
}

impl AstNode for CompilationUnit {
    fn fmt_tree(
        &self,
        f: &mut Formatter<'_>,
        prefix: &str,
        is_last: bool,
    ) -> fmt::Result {
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
    fn fmt_tree(
        &self,
        f: &mut Formatter<'_>,
        prefix: &str,
        is_last: bool,
    ) -> fmt::Result {
        match self {
            TopLevelClassOrInterfaceDeclaration::ClassDeclaration(c) => {
                c.fmt_tree(f, prefix, is_last)
            }
        }
    }
}

impl AstNode for ClassDeclaration {
    fn fmt_tree(
        &self,
        f: &mut Formatter<'_>,
        prefix: &str,
        is_last: bool,
    ) -> fmt::Result {
        match self {
            ClassDeclaration::NormalClassDeclaration(c) => {
                c.fmt_tree(f, prefix, is_last)?;
            }
        }
        Ok(())
    }
}

impl AstNode for NormalClassDeclaration {
    fn fmt_tree(
        &self,
        f: &mut Formatter<'_>,
        prefix: &str,
        is_last: bool,
    ) -> fmt::Result {
        let (line_prefix, new_prefix) = branch(&prefix, is_last);

        writeln!(f, "{line_prefix}Class {} {:?}", self.identifier, self.modifiers)?;
        let total = self.body.class_body_declarations.len();

        for (i, decl) in self.body.class_body_declarations.iter().enumerate() {
            decl.fmt_tree(f, &new_prefix, i == total - 1)?;
        }
        Ok(())
    }
}

impl AstNode for ClassBodyDeclaration {
    fn fmt_tree(
        &self,
        f: &mut Formatter<'_>,
        prefix: &str,
        is_last: bool,
    ) -> fmt::Result {
        match self {
            ClassBodyDeclaration::ClassMemberDeclaration(m) => {
                m.fmt_tree(f, prefix, is_last)
            }
        }
    }
}

impl AstNode for ClassMemberDeclaration {
    fn fmt_tree(
        &self,
        f: &mut Formatter<'_>,
        prefix: &str,
        is_last: bool,
    ) -> fmt::Result {
        let (line_prefix, new_prefix) = branch(&prefix, is_last);

        match self {
            ClassMemberDeclaration::MethodDeclaration(m) => {
                writeln!(f, "{line_prefix}Method {}->{} {:?}", m.identifier, m.result, m.modifiers)?;
                m.fmt_tree(f, &new_prefix, true)
            }
        }
    }
}

impl Display for MethodResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MethodResult::Void => write!(f, "void"),
            MethodResult::Type(t) => write!(f, "{}", t),
        }
    }
}

impl AstNode for MethodDeclaration {
    fn fmt_tree(
        &self,
        f: &mut Formatter<'_>,
        prefix: &str,
        _is_last: bool,
    ) -> fmt::Result {
        let total = self.parameters.len() + 1;

        for (i, param) in self.parameters.iter().enumerate() {
            param.fmt_tree(f, &prefix, i == total - 1)?;
        }

        self.body.fmt_tree(f, &prefix, true)
    }
}

impl AstNode for FormalParameter {
    fn fmt_tree(
        &self,
        f: &mut Formatter<'_>,
        prefix: &str,
        is_last: bool,
    ) -> fmt::Result {
        let (line_prefix, _) = branch(&prefix, is_last);

        match self {
            FormalParameter::NormalFormalParameter(t, id) => {
                writeln!(f, "{line_prefix}Param {} {}", t, id.identifier)
            }
            FormalParameter::VariableArityParameter(_, id) => {
                writeln!(f, "{line_prefix}VarArg {}", id)
            }
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl AstNode for MethodBody {
    fn fmt_tree(
        &self,
        f: &mut Formatter<'_>,
        prefix: &str,
        is_last: bool,
    ) -> fmt::Result {
        let (line_prefix, new_prefix) = branch(&prefix, is_last);

        match self {
            MethodBody::Semicolon => {
                writeln!(f, "{line_prefix};")
            }
            MethodBody::Block(stmts) => {
                writeln!(f, "{line_prefix}Block")?;

                for (i, stmt) in stmts.iter().enumerate() {
                    stmt.fmt_tree(f, &new_prefix, i == stmts.len() - 1)?;
                }
                Ok(())
            }
        }
    }
}

impl AstNode for Statement {
    fn fmt_tree(
        &self,
        f: &mut Formatter<'_>,
        prefix: &str,
        is_last: bool,
    ) -> fmt::Result {
        let (line_prefix, _) = branch(&prefix, is_last);

        match self {
            Statement::EmptyStatement => {
                writeln!(f, "{line_prefix}EmptyStatement")
            },
            Statement::ExpressionStatement(e) => {
                e.fmt_tree(f, &prefix, is_last)
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
            Expression::Assignment { lhs, rhs } => {
                writeln!(f, "{line_prefix}Assignment")?;
                <LeftHandSide as Into<Expression>>::into(lhs.clone()).fmt_tree(f, &new_prefix, false)?;
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
        }
    }
}









