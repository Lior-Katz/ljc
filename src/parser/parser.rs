use crate::lexer::{LexError, Token};
use crate::lexer::{Tokens, lex_single_file};
use crate::parser::ast::{
    ClassBody, ClassBodyDeclaration, ClassDeclaration, ClassMemberDeclaration, ClassModifier,
    CompilationUnit, Expression, FormalParameter, Identifier, LeftHandSide, MethodBody,
    MethodDeclaration, MethodModifiers, MethodResult, NormalClassDeclaration, Program, Statement,
    TopLevelClassOrInterfaceDeclaration, Type, VariableDeclaratorId,
};
use crate::parser::error::ParseError;
use std::path::Path;

macro_rules! accept_with_value {
    ($self:expr, $variant:path) => {{
        if let Ok($variant(_)) = $self.peek() {
            if let $variant(v) = $self.next()? {
                Ok(v)
            } else {
                unreachable!()
            }
        } else {
            Err(ParseError::NoProduction)
        }
    }};
}

pub fn parse_single_file(path: &Path) -> Result<Program, ParseError> {
    let mut parser = Parser::new(path).unwrap();
    parser.parse()
}

pub struct Parser {
    tokens: Tokens,
    buffer: Vec<Token>,
}

impl Parser {
    pub fn new(path: &Path) -> Result<Self, std::io::Error> {
        Ok(Self {
            tokens: lex_single_file(path)?,
            buffer: Vec::new(),
        })
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        self.compilation_unit()
    }

    fn next(&mut self) -> Result<Token, LexError> {
        if let Some(tok) = self.buffer.pop() {
            return Ok(tok);
        }

        self.tokens.next()
    }

    fn peek(&mut self) -> Result<&Token, LexError> {
        if self.buffer.is_empty() {
            self.buffer.push(self.tokens.next()?)
        }
        Ok(&self.buffer[0])
    }

    fn accept(&mut self, desired: Token) -> bool {
        match self.peek() {
            Ok(current) if *current == desired => {
                self.next().unwrap();
                true
            }
            _ => false,
        }
    }

    fn integer_literal(&mut self) -> Result<u64, ParseError> {
        accept_with_value!(self, Token::IntegerLiteral)
    }

    fn long_literal(&mut self) -> Result<u64, ParseError> {
        accept_with_value!(self, Token::LongLiteral)
    }

    fn boolean_literal(&mut self) -> Result<bool, ParseError> {
        accept_with_value!(self, Token::BooleanLiteral)
    }

    fn char_literal(&mut self) -> Result<char, ParseError> {
        accept_with_value!(self, Token::CharLiteral)
    }

    fn string_literal(&mut self) -> Result<String, ParseError> {
        accept_with_value!(self, Token::StringLiteral)
    }

    fn assert(&mut self, desired: Token) -> Result<(), ParseError> {
        if self.accept(desired) {
            Ok(())
        } else {
            Err(ParseError::NoProduction)
        }
    }

    fn zero_or_more<T: std::fmt::Debug>(
        &mut self,
        next: impl Fn(&mut Self) -> Result<T, ParseError>,
    ) -> Vec<T> {
        let mut v = Vec::new();
        loop {
            match next(self) {
                Ok(elem) => v.push(elem),
                Err(ParseError::NoProduction) => return v,
            }
        }
    }

    fn compilation_unit(&mut self) -> Result<CompilationUnit, ParseError> {
        self.ordinary_compilation_unit()
    }

    fn ordinary_compilation_unit(&mut self) -> Result<CompilationUnit, ParseError> {
        let top_level_class_or_interface_declarations =
            self.zero_or_more(Self::top_level_class_or_interface_declaration);
        Ok(CompilationUnit::Ordinary(
            top_level_class_or_interface_declarations,
        ))
    }

    fn top_level_class_or_interface_declaration(
        &mut self,
    ) -> Result<TopLevelClassOrInterfaceDeclaration, ParseError> {
        while self.accept(Token::Semicolon) {} // §7.6 (p. 231), ignore semicolons at class or interface declarations level
        self.class_declaration()
    }

    fn class_declaration(&mut self) -> Result<TopLevelClassOrInterfaceDeclaration, ParseError> {
        self.normal_class_declaration()/*.or_else(|_| self.enum_declaration()).or_else(|_| self.record_declaration())*/
            .map(|class_decl| TopLevelClassOrInterfaceDeclaration::ClassDeclaration(ClassDeclaration::NormalClassDeclaration(class_decl)))
    }

    fn normal_class_declaration(&mut self) -> Result<NormalClassDeclaration, ParseError> {
        let modifiers = self.zero_or_more(Self::class_modifier);
        self.assert(Token::Class)?;
        let identifier = self.identifier()?;
        self.assert(Token::LeftBrace)?;
        let body = self.class_body()?;
        self.assert(Token::RightBrace)?;
        let class_decl = NormalClassDeclaration {
            modifiers,
            identifier,
            body,
        };
        Ok(class_decl)
    }

    fn class_modifier(&mut self) -> Result<ClassModifier, ParseError> {
        self.accept(Token::Public)
            .then_some(ClassModifier::Public)
            .or(self
                .accept(Token::Private)
                .then_some(ClassModifier::Private))
            .or(self
                .accept(Token::Protected)
                .then_some(ClassModifier::Protected))
            .ok_or(ParseError::NoProduction)
    }

    fn identifier(&mut self) -> Result<Identifier, ParseError> {
        accept_with_value!(self, Token::Id)
    }

    fn class_body(&mut self) -> Result<ClassBody, ParseError> {
        Ok(ClassBody {
            class_body_declarations: self.zero_or_more(Self::class_body_declaration),
        })
    }

    fn class_body_declaration(&mut self) -> Result<ClassBodyDeclaration, ParseError> {
        self.class_member_declaration()
            .map(|m| ClassBodyDeclaration::ClassMemberDeclaration(m))
    }

    fn class_member_declaration(&mut self) -> Result<ClassMemberDeclaration, ParseError> {
        self.method_declaration()
            .map(|m| ClassMemberDeclaration::MethodDeclaration(m))
    }

    fn method_declaration(&mut self) -> Result<MethodDeclaration, ParseError> {
        let modifiers = self.zero_or_more(Self::method_modifier);
        let result = self.result()?;
        let identifier = self.identifier()?;
        self.assert(Token::LeftParen)?;
        let parameters = self.formal_parameters();
        self.assert(Token::RightParen)?;
        let body = self.method_body()?;
        Ok(MethodDeclaration {
            modifiers,
            result,
            identifier,
            parameters,
            body,
        })
    }

    fn method_modifier(&mut self) -> Result<MethodModifiers, ParseError> {
        self.accept(Token::Public)
            .then_some(MethodModifiers::Public)
            .or(self
                .accept(Token::Private)
                .then_some(MethodModifiers::Private))
            .or(self
                .accept(Token::Protected)
                .then_some(MethodModifiers::Protected))
            .ok_or(ParseError::NoProduction)
    }

    fn result(&mut self) -> Result<MethodResult, ParseError> {
        if self.accept(Token::Void) {
            Ok(MethodResult::Void)
        } else {
            Ok(MethodResult::Type(self.unannotated_type()?))
        }
    }

    fn formal_parameters(&mut self) -> Vec<FormalParameter> {
        let mut v = Vec::new();
        if let Ok(formal_parameter) = self.formal_parameter() {
            v.push(formal_parameter);
        } else {
            return v;
        }
        loop {
            if !self.accept(Token::Comma) {
                break;
            }
            if let Ok(fp) = self.formal_parameter() {
                v.push(fp);
            } else {
                // TODO: if not a format parameter, should get "identifier or type expected" error
                break;
            }
        }
        v
    }

    fn formal_parameter(&mut self) -> Result<FormalParameter, ParseError> {
        let param_type = self.unannotated_type()?;
        if self.accept(Token::Ellipsis) {
            // variable arity
            let identifier = self.identifier()?;
            Ok(FormalParameter::VariableArityParameter(
                param_type, identifier,
            ))
        } else {
            let identifier = self.identifier()?;
            Ok(FormalParameter::NormalFormalParameter(
                param_type,
                VariableDeclaratorId { identifier },
            ))
        }
    }

    fn unannotated_type(&mut self) -> Result<Type, ParseError> {
        self.unannotated_primitive_type()
    }

    fn unannotated_primitive_type(&mut self) -> Result<Type, ParseError> {
        if self.accept(Token::Byte) {
            Ok(Type::Byte)
        } else if self.accept(Token::Short) {
            Ok(Type::Short)
        } else if self.accept(Token::Int) {
            Ok(Type::Int)
        } else if self.accept(Token::Long) {
            Ok(Type::Long)
        } else if self.accept(Token::Char) {
            Ok(Type::Char)
        } else if self.accept(Token::Float) {
            Ok(Type::Float)
        } else if self.accept(Token::Double) {
            Ok(Type::Double)
        } else if self.accept(Token::Boolean) {
            Ok(Type::Boolean)
        } else {
            Err(ParseError::NoProduction)
        }
    }

    fn method_body(&mut self) -> Result<MethodBody, ParseError> {
        if self.accept(Token::Semicolon) {
            return Ok(MethodBody::Semicolon);
        }
        self.assert(Token::LeftBrace)?;
        let block_statements = self.zero_or_more(Self::block_statement);
        self.assert(Token::RightBrace)?;
        Ok(MethodBody::Block(block_statements))
    }

    fn block_statement(&mut self) -> Result<Statement, ParseError> {
        self.statement()
    }

    fn statement(&mut self) -> Result<Statement, ParseError> {
        self.statement_without_trailing_substatement()
    }

    fn statement_without_trailing_substatement(&mut self) -> Result<Statement, ParseError> {
        self.empty_statement()
            .or_else(|_| self.expression_statement())
    }

    fn empty_statement(&mut self) -> Result<Statement, ParseError> {
        self.assert(Token::Semicolon)?;
        Ok(Statement::EmptyStatement)
    }

    fn expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expression = self.statement_expression()?;
        self.assert(Token::Semicolon)?;
        Ok(Statement::ExpressionStatement(expression))
    }

    fn statement_expression(&mut self) -> Result<Expression, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, ParseError> {
        let lhs = self.left_hand_side()?;
        let op = self.assignment_operator()?;
        let rhs = self.expression()?;
        let expression = op.to_expr(lhs.clone(), rhs);
        Ok(Expression::Assignment {
            lhs,
            rhs: Box::new(expression),
        })
    }

    fn left_hand_side(&mut self) -> Result<LeftHandSide, ParseError> {
        Ok(LeftHandSide::ExpressionName(self.expression_name()?))
    }

    fn assignment_operator(&mut self) -> Result<AssignmentOperator, ParseError> {
        if self.accept(Token::Assign) {
            Ok(AssignmentOperator::Identity)
        } else {
            Err(ParseError::NoProduction)
        }
    }

    fn expression(&mut self) -> Result<Expression, ParseError> {
        self.assignment_expression()
    }

    fn assignment_expression(&mut self) -> Result<Expression, ParseError> {
        self.conditional_expression()
    }

    fn conditional_expression(&mut self) -> Result<Expression, ParseError> {
        self.conditional_or_expression()
    }

    fn conditional_or_expression(&mut self) -> Result<Expression, ParseError> {
        self.conditional_and_expression()
    }

    fn conditional_and_expression(&mut self) -> Result<Expression, ParseError> {
        self.inclusive_or_expression()
    }

    fn inclusive_or_expression(&mut self) -> Result<Expression, ParseError> {
        self.exclusive_or_expression()
    }

    fn exclusive_or_expression(&mut self) -> Result<Expression, ParseError> {
        self.and_expression()
    }

    fn and_expression(&mut self) -> Result<Expression, ParseError> {
        self.equality_expression()
    }

    fn equality_expression(&mut self) -> Result<Expression, ParseError> {
        self.relational_expression()
    }

    fn relational_expression(&mut self) -> Result<Expression, ParseError> {
        self.shift_expression()
    }

    fn shift_expression(&mut self) -> Result<Expression, ParseError> {
        self.additive_expression()
    }

    fn additive_expression(&mut self) -> Result<Expression, ParseError> {
        self.multiplicative_expression()
    }

    fn multiplicative_expression(&mut self) -> Result<Expression, ParseError> {
        self.unary_expression()
    }

    fn unary_expression(&mut self) -> Result<Expression, ParseError> {
        self.unary_not_plus_minus_expression()
    }

    fn unary_not_plus_minus_expression(&mut self) -> Result<Expression, ParseError> {
        self.postfix_expression()
    }

    fn postfix_expression(&mut self) -> Result<Expression, ParseError> {
        let mut expression = self
            .primary().or_else(|_| self
            .expression_name().map(|name| Expression::Name(name)))?;

        // the semantic structure doesn't actually allow multiple consecutive postfix operators
        // but the grammar is defined in a way that does.
        // the openJDK parser also allows multiple, maybe it helps in error diagnostics
        // maybe we can get away without this loop.
        loop {
            if self.accept(Token::Increment) {
                expression = Expression::PostIncrement(Box::new(expression));
            } else if self.accept(Token::Decrement) {
                expression = Expression::PostDecrement(Box::new(expression));
            } else {
                break;
            }
        }
        Ok(expression)
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        self.primary_no_new_array()
    }

    fn primary_no_new_array(&mut self) -> Result<Expression, ParseError> {
        self
            .integer_literal().map(|v| Expression::IntegerLiteral(v)).or_else(|_| self
            .long_literal().map(|v| Expression::LongLiteral(v))).or_else(|_| self
            .boolean_literal().map(|v| Expression::BooleanLiteral(v))).or_else(|_| self
            .char_literal().map(|v| Expression::CharLiteral(v))).or_else(|_| self
            .string_literal().map(|v| Expression::StringLiteral(v))).or_else(|_| self
            .accept(Token::NullLiteral).then_some(Expression::NullLiteral).ok_or(ParseError::NoProduction))
    }

    fn expression_name(&mut self) -> Result<Identifier, ParseError> {
        self.identifier()
    }
}

impl From<LexError> for ParseError {
    fn from(_e: LexError) -> Self {
        ParseError::NoProduction
    }
}

enum AssignmentOperator {
    Identity,
}

impl Into<Expression> for LeftHandSide {
    fn into(self) -> Expression {
        match self {
            LeftHandSide::ExpressionName(id) => Expression::Name(id),
        }
    }
}

impl AssignmentOperator {
    fn to_expr(&self, _lhs: LeftHandSide, rhs: Expression) -> Expression {
        match self {
            AssignmentOperator::Identity => rhs,
        }
    }
}
