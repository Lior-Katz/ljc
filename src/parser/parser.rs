use crate::lexer::{LexError, Token};
use crate::lexer::{Tokens, lex_single_file};
use crate::parser::ast::{
    ClassBody, ClassBodyDeclaration, ClassDeclaration, ClassMemberDeclaration, ClassModifier,
    CompilationUnit, FormalParameter, Identifier, MethodBody, MethodDeclaration, MethodModifiers,
    MethodResult, NormalClassDeclaration, Program, Statement, TopLevelClassOrInterfaceDeclaration,
    Type, VariableDeclaratorId,
};
use crate::parser::error::ParseError;
use std::path::Path;

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
        // FIXME: should clean this up
        if let Ok(Token::Id(_)) = self.peek() {
            if let Token::Id(id) = self.next()? {
                return Ok(id);
            }
        }
        Err(ParseError::NoProduction)
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
            return Ok(MethodResult::Void);
        }
        Err(ParseError::NoProduction)
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
    }

    fn empty_statement(&mut self) -> Result<Statement, ParseError> {
        self.assert(Token::Semicolon)?;
        Ok(Statement::EmptyStatement)
    }
}

impl From<LexError> for ParseError {
    fn from(_e: LexError) -> Self {
        ParseError::NoProduction
    }
}
