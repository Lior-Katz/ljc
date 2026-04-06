use crate::lexer::{LexError, Token};
use crate::lexer::{Tokens, lex_single_file};
use crate::parser::ast::{
    AssignmentOp, BinOp, ClassBodyDeclaration, ClassDeclaration, ClassMemberDeclaration,
    ClassModifier, CompilationUnit, Expression, FormalParameter, Identifier, LeftHandSide,
    MethodBody, MethodDeclaration, MethodModifiers, MethodResult, NormalClassDeclaration, Program,
    Statement, TopLevelClassOrInterfaceDeclaration, Type, VariableDeclaratorId,
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

    ($self:expr, $($token:expr => $result:expr),+ $(,)?) => {{
        $(
            if $self.accept($token) {
                Ok($result)
            } else
        )+
        {
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

    fn class_body(&mut self) -> Result<Vec<ClassBodyDeclaration>, ParseError> {
        Ok(self.zero_or_more(Self::class_body_declaration))
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
        Ok(MethodBody::Block(self.block()?))
    }

    /// ```text
    /// block:
    ///     [ block_statements ]
    ///
    /// block_statements:
    ///     {block_statement}
    /// ```
    fn block(&mut self) -> Result<Vec<Statement>, ParseError> {
        self.assert(Token::LeftBrace)?;
        let block_statements = self.zero_or_more(Self::block_statement);
        self.assert(Token::RightBrace)?;
        Ok(block_statements)
    }

    /// Original grammar defines:
    /// ```text
    /// block_statement:
    ///     local_class_or_interface_declaration
    ///     local_variable_declaration_statement
    ///     statement
    /// ```
    /// `local_class_or_interface_declaration` has to start with a keyword and is thus easily recognizable,
    /// while `local_variable_declaration_statement` and `statement` are ambiguous. So they are here
    /// unified into [Parser::local_variable_declaration_or_statement]:
    /// ```text
    /// block_statement:
    ///     local_class_or_interface_declaration
    ///     local_variable_declaration_or_statement
    /// ```
    ///
    /// NOTE: This is still ambiguous: both `local_class_or_interface_declaration` and `local_variable_declaration_statement`
    /// (produced in [Parser::statement_starting_with_name]) can start with a sequence of modifiers.
    /// TODO: when implementing `local_class_or_interface_declaration`, the list of modifiers should be factored out
    fn block_statement(&mut self) -> Result<Statement, ParseError> {
        self.local_variable_declaration_or_statement()
    }

    /// from [Parser::block_statement] we get
    /// ```text
    /// local_variable_declaration_or_statement:
    ///     local_variable_declaration_statement
    ///     statement
    /// ```
    /// `statement` is (after expanding `StatementWithoutTrailingSubstatement`):
    /// ```text
    /// statement:
    ///     empty_statement
    ///     block
    ///     if_then_statement
    ///     if_then_else_statement
    ///     while_statement
    ///     for_statement
    ///     assert_statement
    ///     switch_statement
    ///     do_statement
    ///     break_statement
    ///     continue_statement
    ///     return_statement
    ///     synchronized_statement
    ///     throw_statement
    ///     try_statement
    ///     yield_statement
    ///     labeled_statement
    ///     expression_statement
    /// ```
    /// Again, `EmptyStatement`, `Block` are immediately recognizable starting with a `;` or `{` respectively,
    /// while `IfThenStatement`/`IfThenElseStatement`, `WhileStatement`, `ForStatement`, `ExpressionStatement`,
    /// `AssertStatement`, `SwitchStatement`, `DoStatement`, `BreakStatement`, `ContinueStatement`,
    /// `ReturnStatement`, `SynchronizedStatement`, `ThrowStatement`, `TryStatement`, `YieldStatement`
    /// can be recognized by their respective keywords and are grouped into [Parser::simple_statement].
    ///
    /// Additionally, `LabeledStatement`, `ExpressionStatement`, and `LocalVariableDeclarationStatement`
    /// are grouped into [Parser::statement_starting_with_name]
    ///
    /// Lastly, `pre_increment_expression` and `pre_decrement_expression` are extracted out of `expression_statement`
    /// into [Parser::prefix_expression_statement], as they do not start with an identifier and are also immediately recognizable.
    ///
    /// The resulting productions are thus:
    /// ```text
    /// local_variable_declaration_or_statement:
    ///     empty_statement
    ///     block
    ///     prefix_expression_statement
    ///     simple_statement
    ///     statement_starting_with_name
    /// ```
    fn local_variable_declaration_or_statement(&mut self) -> Result<Statement, ParseError> {
        self
            .empty_statement().or_else(|_| self
            .block().map(|v| Statement::Block(v))).or_else(|_| self
            .prefix_expression_statement()).or_else(|_| self
            .simple_statement()).or_else(|_| self
            .statement_starting_with_name())
    }

    fn empty_statement(&mut self) -> Result<Statement, ParseError> {
        self.assert(Token::Semicolon)?;
        Ok(Statement::EmptyStatement)
    }

    /// ```text
    /// prefix_expression_statement:
    ///     prefix_expression ;
    /// ```
    fn prefix_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expression = self.prefix_expression()?;
        self.assert(Token::Semicolon)?;
        Ok(Statement::ExpressionStatement(expression))
    }

    /// from [Parser::local_variable_declaration_or_statement],
    /// ```text
    /// simple_statement:
    ///     if_statement
    ///     while_statement
    ///     for_statement
    ///     assert_statement
    ///     switch_statement
    ///     do_statement
    ///     break_statement
    ///     continue_statement
    ///     return_statement
    ///     synchronized_statement
    ///     throw_statement
    ///     try_statement
    ///     yield_statement
    ///
    /// if_statement:
    ///     if_then_statement
    ///     if_then_else_statement
    /// ```
    fn simple_statement(&mut self) -> Result<Statement, ParseError> {
        Err(ParseError::NoProduction)
    }

    /// From [Parser::local_variable_declaration_or_statement], the remaining kind of statements:
    /// ```text
    /// statement_starting_with_name:
    ///     labeled_statement
    ///     expression_statement
    ///     local_variable_declaration_statement
    /// ```
    fn statement_starting_with_name(&mut self) -> Result<Statement, ParseError> {
        self.expression_statement()
    }

    fn expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expression = self.statement_expression()?;
        self.assert(Token::Semicolon)?;
        Ok(Statement::ExpressionStatement(expression))
    }

    /// `statement_expression` is defined as:
    /// ```text
    /// statement_expression:
    ///     pre_increment_expression
    ///     pre_decrement_expression
    ///     assignment
    ///     post_increment_expression
    ///     post_decrement_expression
    ///     method_invocation
    ///     class_instance_creation_expression
    /// ```
    /// The two prefix expressions were extracted out to [Parser::local_variable_declaration_or_statement]
    /// as [Parser::prefix_expression_statement], so they can be removed from here, resulting in:
    /// ```text
    /// statement_expression:
    ///     assignment
    ///     post_increment_expression
    ///     post_decrement_expression
    ///     method_invocation
    ///     class_instance_creation_expression
    /// ```
    fn statement_expression(&mut self) -> Result<Expression, ParseError> {
        self.statement_expression_starting_with_name()
    }

    /// ```text
    /// statement_expression_starting_with_name:
    ///     assignment
    ///     post_increment_expression
    ///     post_decrement_expression
    ///     method_invocation
    ///     class_instance_creation_expression
    /// ```
    fn statement_expression_starting_with_name(&mut self) -> Result<Expression, ParseError> {
        self.assignment()
    }

    /// ```text
    /// assignment:
    ///     conditional_expression [ assignment_op expression ]
    ///
    /// assignment_op:
    ///     one of:
    ///         =  +=  -=  *=  /=  %=  <<=  >>=  >>>=  &=  ^=  |=
    /// ```
    fn assignment(&mut self) -> Result<Expression, ParseError> {
        // Compound assignments are not strictly equivalent to assigning the result of a binary op,
        // as there can be some differences to how the subexpressions are evaluated.
        // For example in the following expression:
        //     foo().x += 5
        // foo() is evaluated only once.
        // Transforming this expression into
        //     f().x = f().x + 5
        // will evaluate f() twice.
        let expr = self.conditional_expression()?;
        if let Ok(op) = accept_with_value!(self,
            Token::Assign => AssignmentOp::Identity,
            Token::AddAssign => AssignmentOp::Add,
            Token::SubAssign=> AssignmentOp::Subtract,
            Token::MulAssign => AssignmentOp::Multiply,
            Token::DivAssign => AssignmentOp::Divide,
            Token::ModAssign => AssignmentOp::Modulo,
            Token::LeftShiftAssign => AssignmentOp::LeftShift,
            Token::SignedRightShiftAssign => AssignmentOp::SignedRightShift,
            Token::UnsignedRightShiftAssign => AssignmentOp::UnsignedRightShift,
            Token::AndAssign => AssignmentOp::BitwiseAnd,
            Token::XorAssign => AssignmentOp::BitwiseXor,
            Token::OrAssign => AssignmentOp::BitwiseOr,
        ) {
            let lhs = match expr {
                Expression::Name(id) => LeftHandSide::ExpressionName(id),
                _ => return Err(ParseError::NoProduction)
            };
            let rhs = self.assignment()?;
            Ok(Expression::Assignment {
                lhs,
                rhs: Box::new(rhs),
                op,
            })
        } else {
            Ok(expr)
        }
    }

    fn expression(&mut self) -> Result<Expression, ParseError> {
        self.assignment()
    }

    fn conditional_expression(&mut self) -> Result<Expression, ParseError> {
        let condition = self.conditional_or_expression()?;
        if self.accept(Token::QuestionMark) {
            let if_true = self.expression()?;
            self.assert(Token::Colon)?;
            let if_false = self.conditional_expression()?;
            Ok(Expression::ConditionalExpression {
                condition: Box::new(condition),
                if_true: Box::new(if_true),
                if_false: Box::new(if_false),
            })
        } else {
            Ok(condition)
        }
    }

    fn left_associative_binary_operation<F, G>(
        &mut self,
        subexpression: F,
        operation: G,
    ) -> Result<Expression, ParseError>
    where
        F: Fn(&mut Self) -> Result<Expression, ParseError>,
        G: Fn(&mut Self) -> Result<BinOp, ParseError>,
    {
        let mut expr = subexpression(self)?;

        while let Ok(op) = operation(self) {
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                right: Box::new(subexpression(self)?),
                op,
            }
        }
        Ok(expr)
    }

    fn conditional_or_expression(&mut self) -> Result<Expression, ParseError> {
        self.left_associative_binary_operation(
            |this| this.conditional_and_expression(),
            |this| {
                accept_with_value!(this,
                    Token::LogicalOr => BinOp::LogicalOr
                )
            },
        )
    }

    fn conditional_and_expression(&mut self) -> Result<Expression, ParseError> {
        self.left_associative_binary_operation(
            |this| this.inclusive_or_expression(),
            |this| {
                accept_with_value!(this,
                    Token::LogicalAnd => BinOp::LogicalAnd
                )
            },
        )
    }

    fn inclusive_or_expression(&mut self) -> Result<Expression, ParseError> {
        self.left_associative_binary_operation(
            |this| this.exclusive_or_expression(),
            |this| {
                accept_with_value!(this,
                    Token::BitwiseOr => BinOp::BitwiseOr
                )
            },
        )
    }

    fn exclusive_or_expression(&mut self) -> Result<Expression, ParseError> {
        self.left_associative_binary_operation(
            |this| this.and_expression(),
            |this| {
                accept_with_value!(this,
                    Token::BitwiseXor => BinOp::BitwiseXor
                )
            },
        )
    }

    fn and_expression(&mut self) -> Result<Expression, ParseError> {
        self.left_associative_binary_operation(
            |this| this.equality_expression(),
            |this| {
                accept_with_value!(this,
                    Token::BitwiseAnd => BinOp::BitwiseAnd
                )
            },
        )
    }

    fn equality_expression(&mut self) -> Result<Expression, ParseError> {
        self.left_associative_binary_operation(
            |this| this.relational_expression(),
            |this| {
                accept_with_value!(this,
                    Token::Equals => BinOp::Equal,
                    Token::NotEquals => BinOp::NotEqual,
                )
            },
        )
    }

    fn relational_expression(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.shift_expression()?;

        // not using generic left_associative_binary_operation here
        // because in this case there is another case - the instanceof operator
        // which does not take symmetric operands.
        loop {
            if let Ok(op) = accept_with_value!(self,
                Token::LessThan => BinOp::Less,
                Token::GreaterThan => BinOp::Greater,
                Token::LessThanOrEquals => BinOp::LessEqual,
                Token::GreaterThanOrEquals => BinOp::GreaterEqual,
            ) {
                expr = Expression::BinaryOp {
                    left: Box::new(expr),
                    right: Box::new(self.shift_expression()?),
                    op,
                }
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn shift_expression(&mut self) -> Result<Expression, ParseError> {
        self.left_associative_binary_operation(
            |this| this.additive_expression(),
            |this| {
                accept_with_value!(this,
                    Token::LeftShift => BinOp::LeftShift,
                    Token::SignedRightShift => BinOp::SignedRightShift,
                    Token::UnsignedRightShift => BinOp::UnsignedRightShift,
                )
            },
        )
    }

    fn additive_expression(&mut self) -> Result<Expression, ParseError> {
        self.left_associative_binary_operation(
            |this| this.multiplicative_expression(),
            |this| {
                accept_with_value!(this,
                    Token::Plus => BinOp::Add,
                    Token::Minus => BinOp::Subtract,
                )
            },
        )
    }

    fn multiplicative_expression(&mut self) -> Result<Expression, ParseError> {
        self.left_associative_binary_operation(
            |this| this.unary_expression(),
            |this| {
                accept_with_value!(this,
                    Token::Multiply => BinOp::Multiply,
                    Token::Divide   => BinOp::Divide,
                    Token::Modulo   => BinOp::Modulo,
                )
            },
        )
    }

    fn unary_expression(&mut self) -> Result<Expression, ParseError> {
        if self.accept(Token::Tilde) {
            Ok(Expression::BitwiseComplement(Box::new(self.unary_expression()?)))
        } else if self.accept(Token::ExclamationMark) {
            Ok(Expression::LogicalNot(Box::new(self.unary_expression()?)))
        } else if self.accept(Token::Plus) {
            Ok(Expression::UnaryPlus(Box::new(self.unary_expression()?)))
        } else if self.accept(Token::Minus) {
            Ok(Expression::UnaryMinus(Box::new(self.unary_expression()?)))
        } else {
            self
                .prefix_expression().or_else(|_| self
                .postfix_expression())
        }
    }

    /// ```text
    /// prefix_expression:
    ///     ++ unary_expression
    ///     -- unary_exoression
    /// ```
    fn prefix_expression(&mut self) -> Result<Expression, ParseError> {
        if self.accept(Token::Increment) {
            Ok(Expression::PreIncrement(Box::new(self.unary_expression()?)))
        } else if self.accept(Token::Decrement) {
            Ok(Expression::PreDecrement(Box::new(self.unary_expression()?)))
        } else {
            Err(ParseError::NoProduction)
        }
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
            .or_else(|_| {
                self.assert(Token::LeftParen)?;
                let expr = self.expression()?;
                self.assert(Token::RightParen)?;
                Ok(expr)
            })
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

impl Into<Expression> for LeftHandSide {
    fn into(self) -> Expression {
        match self {
            LeftHandSide::ExpressionName(id) => Expression::Name(id),
        }
    }
}
