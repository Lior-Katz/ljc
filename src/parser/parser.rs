use crate::ast::{
    Annotation, AnnotationInterfaceDeclaration, ArgumentList, ArrayAccess, ArrayCreationMode,
    ArrayType, AssignmentOp, BinOp, BlockStatements, CatchClause, ClassBodyDeclaration,
    ClassBodyDeclarations, ClassDeclaration, ClassMemberDeclaration, ClassType, ClassTypeList,
    ClassTypePart, CompilationUnit, ComponentPattern, ComponentPatternList, ConstructorBody,
    ConstructorInvocation, ElementValue, ElementValueList, ElementValuePair, EnumBody,
    EnumConstant, EnumDeclaration, Expression, ForInit, ForUpdate, FormalParameter,
    FormalParameterList, Identifier, InterfaceDeclaration, LeftHandSide, MemberAccess, MethodBody,
    MethodCall, MethodDeclaration, MethodReferenceType, Modifiable, Modified, Modifier, Multiple,
    NormalClassDeclaration, NormalInterfaceDeclaration, Pattern, Program, RecordBodyDeclaration,
    RecordComponent, RecordDeclaration, Resource, Statement, Switch, SwitchBlockMember,
    SwitchBlockMembers, SwitchLabel, SwitchRule, TopLevelClassOrInterfaceDeclaration, Type,
    VariableDeclaration, VariableDeclarator, VariableDeclaratorId, VariableDeclaratorList,
    VariableInitializer, VariableInitializerList,
};
use crate::lexer::{LexError, Token};
use crate::lexer::Tokens;
use crate::parser::error::ParseError;
use std::collections::VecDeque;

use bitflags::bitflags;
use std::vec;

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

macro_rules! peek {
    ($self:expr, $($n:expr => $pat:pat $(if $guard:expr)?),+ $(,)?) => {{
        true $(
            && match $self.peek_n($n) {
                Ok(tok) => match tok {
                    $pat $(if $guard)? => true,
                    _ => false,
                },
                Err(_) => false,
            }
        )+
    }};
}

macro_rules! one_of {
    ($($x:expr),+ $(,)?) => {{
        let mut res = Err(ParseError::NoProduction);
        $(
            res = match res {
                Ok(_) => res,
                Err(_) => $x
            };
        )+
        res
    }};
}

macro_rules! one_of_opt {
    ($($x:expr),+ $(,)?) => {{
        $(
            match $x {  Some(v) => return Ok(v), None => {} };
        )+
        Err(ParseError::NoProduction)
    }};
}

pub struct Parser<'a> {
    tokens: Tokens<'a>,
    buffer: VecDeque<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            tokens: Tokens::new(input),
            buffer: VecDeque::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        self.compilation_unit()
    }

    fn next(&mut self) -> Result<Token, LexError> {
        if let Some(tok) = self.buffer.pop_front() {
            return Ok(tok);
        }

        self.tokens.next()
    }

    fn peek(&mut self) -> Result<&Token, LexError> {
        self.peek_n(0)
    }

    fn peek_n(&mut self, skip: usize) -> Result<&Token, LexError> {
        while self.buffer.len() <= skip {
            self.buffer.push_back(self.tokens.next()?)
        }
        Ok(&self.buffer[skip])
    }

    fn next_is(&mut self, desired: Token) -> bool {
        self.nth_is(0, desired)
    }

    fn nth_is(&mut self, n: usize, desired: Token) -> bool {
        match self.peek_n(n) {
            Ok(t) if *t == desired => true,
            _ => false,
        }
    }

    fn accept(&mut self, desired: Token) -> bool {
        let matches = self.next_is(desired);
        if matches {
            self.next().unwrap();
        }
        matches
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

    fn opt<T>(
        &mut self,
        cond: impl Fn(&mut Self) -> bool,
        element: impl Fn(&mut Self) -> Result<T, ParseError>,
    ) -> Result<Option<T>, ParseError> {
        if cond(self) { Ok(Some(element(self)?)) } else { Ok(None) }
    }

    fn zero_or_more<T>(&mut self, next: impl Fn(&mut Self) -> Result<T, ParseError>) -> Vec<T> {
        let mut v = Vec::new();
        loop {
            match next(self) {
                Ok(elem) => v.push(elem),
                Err(ParseError::NoProduction) => return v,
            }
        }
    }

    /// ```text
    /// [next {delim next}]
    /// ```
    fn delimited_list<T, S>(
        &mut self,
        next: impl Fn(&mut Self) -> Result<T, ParseError>,
        delim: impl Fn(&mut Self) -> Result<S, ParseError>,
    ) -> Result<Vec<T>, ParseError> {
        let mut list = if let Ok(elem) = next(self) {
            vec![elem]
        } else {
            return Ok(vec![]);
        };
        loop {
            if delim(self).is_err() {
                break;
            }
            if let Ok(elem) = next(self) {
                list.push(elem);
            } else {
                return Err(ParseError::NoProduction);
            }
        }
        Ok(list)
    }

    /// ```text
    /// next {delim next}
    /// ```
    fn delimited_at_least_1<T, S>(
        &mut self,
        next: impl Fn(&mut Self) -> Result<T, ParseError>,
        delim: impl Fn(&mut Self) -> Result<S, ParseError>,
    ) -> Result<Vec<T>, ParseError> {
        match self.delimited_list(next, delim) {
            Ok(l) if !l.is_empty() => Ok(l),
            _ => Err(ParseError::NoProduction),
        }
    }

    fn dot(&mut self) -> Result<(), ParseError> {
        self.assert(Token::Dot)
    }

    fn comma(&mut self) -> Result<(), ParseError> {
        self.assert(Token::Comma)
    }

    fn semicolon(&mut self) -> Result<(), ParseError> {
        self.assert(Token::Semicolon)
    }

    fn pipe(&mut self) -> Result<(), ParseError> {
        self.assert(Token::BitwiseOr)
    }

    fn compilation_unit(&mut self) -> Result<CompilationUnit, ParseError> {
        self.ordinary_compilation_unit()
    }

    fn ordinary_compilation_unit(&mut self) -> Result<CompilationUnit, ParseError> {
        let top_level_class_or_interface_declarations =
            self.zero_or_more(Self::top_level_class_or_interface_declaration);
        Ok(CompilationUnit::Ordinary(top_level_class_or_interface_declarations))
    }

    /// a top level class or interface can be either a class or an interface declaration, both of which
    /// can begin with modifiers, so [modifier parsing](Parser::modifier) is factored out:
    /// ```text
    /// top_level_class_or_interface_declaration:
    ///     {modifier} top_level_class_or_interface_declaration_no_modifier
    ///
    /// top_level_class_or_interface_declaration_no_modifier:
    ///     class_declaration
    ///     interface_declaration
    ///     ;
    /// ```
    fn top_level_class_or_interface_declaration(
        &mut self,
    ) -> Result<Modified<TopLevelClassOrInterfaceDeclaration>, ParseError> {
        while self.accept(Token::Semicolon) {} // §7.6 (p. 231), ignore semicolons at class or interface declarations level

        let modifiers = self.modifiers(ModifierKind::CLASS | ModifierKind::INTERFACE);
        one_of!(
            self.class_declaration().map(ClassDeclaration::into),
            self.interface_declaration().map(InterfaceDeclaration::into)
        )
        .map(|d: TopLevelClassOrInterfaceDeclaration| d.with_modifiers(modifiers))
    }

    fn class_declaration(&mut self) -> Result<ClassDeclaration, ParseError> {
        one_of!(
            self.normal_class_declaration()
                .map(NormalClassDeclaration::into),
            self.record_declaration().map(RecordDeclaration::into),
            self.enum_declaration().map(EnumDeclaration::into),
        )
    }

    fn normal_class_declaration(&mut self) -> Result<NormalClassDeclaration, ParseError> {
        self.assert(Token::Class)?;
        let identifier = self.identifier()?.try_into()?;
        let extends = self.opt_class_extends()?;
        let implements = self.opt_class_implements()?;
        let permits = self.opt_class_permits()?;
        let body = self.class_body()?;
        let class_decl = NormalClassDeclaration {
            identifier,
            extends,
            implements,
            permits,
            body,
        };
        Ok(class_decl)
    }

    fn opt_class_extends(&mut self) -> Result<Option<ClassType>, ParseError> {
        self.opt(|this| this.accept(Token::Extends), Self::class_type)
    }

    fn opt_class_implements(&mut self) -> Result<Option<ClassTypeList>, ParseError> {
        self.opt(
            |this| this.accept(Token::Implements),
            |this| this.delimited_at_least_1(Self::class_type, Self::comma),
        )
    }

    fn opt_class_permits(&mut self) -> Result<Option<ClassTypeList>, ParseError> {
        self.opt(
            |this| {
                let permits = peek!(this, 0 => Token::Id(s) if s.as_str() == "permits");
                if permits {
                    this.next().unwrap();
                }
                permits
            },
            |this| this.delimited_at_least_1(Self::class_type, Self::comma),
        )
    }

    fn modifier(&mut self, modifier_kind: ModifierKind) -> Result<Modifier, ParseError> {
        one_of_opt!(
            self.accept(Token::Public).then_some(Modifier::Public),
            self.accept(Token::Private).then_some(Modifier::Private),
            self.accept(Token::Protected).then_some(Modifier::Protected),
            self.accept(Token::Abstract).then_some(Modifier::Abstract),
            (!self.nth_is(1, Token::LeftBrace) && self.accept(Token::Static))
                .then_some(Modifier::Static),
            self.accept(Token::Final).then_some(Modifier::Final),
            (modifier_kind.contains(ModifierKind::METHOD) && self.accept(Token::Default))
                .then_some(Modifier::Default),
            self.accept(Token::Strictfp).then_some(Modifier::Strictfp),
            self.accept(Token::Native).then_some(Modifier::Native),
            self.accept(Token::Transient).then_some(Modifier::Transient),
            self.accept(Token::Volatile).then_some(Modifier::Volatile),
            (!self.nth_is(1, Token::LeftParen) && self.accept(Token::Synchronized))
                .then_some(Modifier::Synchronized),
            self.is_sealed_class_start().then(|| {
                self.next().unwrap();
                Modifier::Sealed
            }),
            self.is_non_sealed_class_start().then(|| {
                self.next().unwrap();
                self.next().unwrap();
                self.next().unwrap();
                Modifier::NonSealed
            })
        )
        .or_else(|_| self.annotation().map(Annotation::into))
    }

    fn modifiers(&mut self, modifier_kind: ModifierKind) -> Vec<Modifier> {
        self.zero_or_more(|this| this.modifier(modifier_kind))
    }

    fn is_sealed_modifier(&mut self, start: usize) -> bool {
        peek!(self, start => Token::Id(s) if s.as_str() == "sealed")
    }

    fn is_non_sealed_modifier(&mut self, start: usize) -> bool {
        peek!(self,
                start => Token::Id(s) if s.as_str() == "non",
                start + 1 => Token::Minus,
                start + 2 => Token::Id(s) if s.as_str() == "sealed")
    }

    fn is_sealed_class_start(&mut self) -> bool {
        let sealed_modifier = self.is_sealed_modifier(0);
        let next_token_start = 1;
        sealed_modifier && self.is_after_sealed_or_non_sealed(next_token_start)
    }

    fn is_non_sealed_class_start(&mut self) -> bool {
        let non_sealed_modifier = self.is_non_sealed_modifier(0);
        let next_token_start = 3;
        non_sealed_modifier && self.is_after_sealed_or_non_sealed(next_token_start)
    }

    fn is_after_sealed_or_non_sealed(&mut self, next_token_start: usize) -> bool {
        let keyword_class_or_interface_modifier = peek!(self,
            next_token_start => Token::Public
            | Token::Protected
            | Token::Private
            | Token::Abstract
            | Token::Static
            | Token::Final
            | Token::Strictfp
            | Token::At);
        let sealed_modifier = self.is_sealed_modifier(next_token_start);
        let non_sealed_modifier = self.is_non_sealed_modifier(next_token_start);
        let class_or_enum_or_record_or_interface = self.nth_is(next_token_start, Token::Class)
            || self.nth_is(next_token_start, Token::Enum)
            || peek!(self, next_token_start => Token::Id(s) if s.as_str() == "record")
            || self.nth_is(next_token_start, Token::Interface);
        keyword_class_or_interface_modifier
            || sealed_modifier
            || non_sealed_modifier
            || class_or_enum_or_record_or_interface
    }

    /// ```text
    /// annotation:
    ///     @ type_name
    ///     @ type_name ( element_value )
    ///     @ type_name ( [element_value_pair_list] )
    ///
    /// element_value_pair_list:
    ///     identifier = element_value {, identifier = element_value}
    ///
    /// element_v alue:
    ///     { element_value_list }
    ///     conditional_expression
    ///     annotation
    ///
    /// element_value_list:
    ///     [,]
    ///     element_value {, element_value} [,]
    /// ```
    fn annotation(&mut self) -> Result<Annotation, ParseError> {
        if !self.next_is(Token::At) || self.nth_is(1, Token::Interface) {
            // to differentiate from annotation interface declaration
            return Err(ParseError::NoProduction);
        }
        self.assert(Token::At)?;
        let name = self.delimited_at_least_1(Self::identifier, Self::dot)?;
        if !self.accept(Token::LeftParen) {
            return Ok(Annotation::Marker(name));
        }
        if self.accept(Token::RightParen) || peek!(self, 0 => Token::Id(_), 1 => Token::Assign) {
            let values = self.delimited_list(Self::element_value_pair, Self::comma)?;
            self.assert(Token::RightParen)?;
            return Ok(Annotation::Normal { name, values });
        }
        let value = self.element_value()?;
        self.assert(Token::RightParen)?;
        Ok(Annotation::SingleElement { name, value })
    }

    /// ```text
    /// element_value_pair:
    ///     identifier = element_value
    /// ```
    fn element_value_pair(&mut self) -> Result<ElementValuePair, ParseError> {
        let name = self.identifier()?;
        self.assert(Token::Assign)?;
        let value = self.element_value()?;
        Ok(ElementValuePair { name, value })
    }

    /// ```text
    /// element_value:
    ///     conditional_expression
    ///     element_value_array_initializer
    ///     annotation
    /// ```
    fn element_value(&mut self) -> Result<ElementValue, ParseError> {
        one_of!(
            self.conditional_expression().map(Expression::into),
            self.element_value_array_initializer()
                .map(ElementValueList::into),
            self.annotation().map(Annotation::into),
        )
    }

    /// ```text
    /// element_value_array_initializer:
    ///     { element_value_list }
    /// ```
    fn element_value_array_initializer(&mut self) -> Result<ElementValueList, ParseError> {
        self.assert(Token::LeftBrace)?;
        let elements = self.element_value_list()?;
        self.assert(Token::RightBrace)?;
        Ok(elements)
    }

    /// ```text
    /// element_value_list:
    ///     [,]
    ///     element_value {, element_value} [,]
    /// ```
    fn element_value_list(&mut self) -> Result<ElementValueList, ParseError> {
        if self.accept(Token::Comma) {
            // just a single comma
            return Ok(vec![]);
        }

        let mut items = vec![];
        loop {
            if self.next_is(Token::RightBrace) {
                break;
            }
            items.push(self.element_value()?);
            if !self.accept(Token::Comma) {
                break;
            }
        }
        Ok(items)
    }

    fn identifier(&mut self) -> Result<Identifier, ParseError> {
        accept_with_value!(self, Token::Id)
    }

    fn class_body(&mut self) -> Result<ClassBodyDeclarations, ParseError> {
        self.assert(Token::LeftBrace)?;
        let declarations = self.zero_or_more(Self::class_body_declaration);
        self.assert(Token::RightBrace)?;
        Ok(declarations)
    }

    fn class_body_declaration(&mut self) -> Result<ClassBodyDeclaration, ParseError> {
        one_of!(
            self.class_member_declaration().map(Modified::into),
            self.instance_initializer()
                .map(|v| ClassBodyDeclaration::InstanceInitializer(v)),
            self.static_initializer()
                .map(|v| ClassBodyDeclaration::StaticInitializer(v)),
        )
    }

    fn instance_initializer(&mut self) -> Result<BlockStatements, ParseError> {
        self.block()
    }

    fn static_initializer(&mut self) -> Result<BlockStatements, ParseError> {
        if !peek!(
            self,
            0 => Token::Static,
            1 => Token::LeftBrace,
        ) {
            Err(ParseError::NoProduction)
        } else {
            self.assert(Token::Static)?;
            self.block()
        }
    }

    /// class_member_declaration is defined as:
    /// ```text
    /// class_member_declaration:
    ///     field_declaration
    ///     method_declaration
    ///     class_declaration
    ///     interface_declaration
    ///     ;
    /// ```
    /// All four begin with modifiers, so parsing [modifier](Parser::modifier)s is factored out,
    /// while methods and fields both follow with a type so are combined. Thus, we arrive at:
    /// ```text
    /// class_member_declaration:
    ///     {modifier} class_member_declaration_no_modifier:
    ///
    /// class_member_declaration_no_modifier:
    ///     class_declaration
    ///     interface_declaration
    ///     constructor_declaration
    ///     method_or_field_declaration
    ///     ;
    /// ```
    fn class_member_declaration(&mut self) -> Result<Modified<ClassMemberDeclaration>, ParseError> {
        while self.accept(Token::Semicolon) {} // ignore stray semicolons
        let modifiers = self.modifiers(ModifierKind::CLASS_MEMBER);

        one_of!(
            self.class_declaration().map(ClassDeclaration::into),
            self.interface_declaration().map(InterfaceDeclaration::into),
            self.constructor_declaration(),
            self.method_or_field_declaration()
        )
        .map(|d| d.with_modifiers(modifiers))
    }

    /// ```text
    /// record_declaration:
    ///     record type_identifier ( [record_component_list] ) record_body
    ///
    /// record_component_list:
    ///     record_component {, record_component}
    /// ```
    fn record_declaration(&mut self) -> Result<RecordDeclaration, ParseError> {
        if !peek!(
            self,
            0 => Token::Id(s) if s.as_str() == "record",
            1 => Token::Id(_),
        ) {
            return Err(ParseError::NoProduction);
        }
        accept_with_value!(self, Token::Id)?;
        let name = self.identifier()?.try_into()?;
        self.assert(Token::LeftParen)?;
        let components = self.delimited_list(Self::record_component, Self::comma)?;
        self.assert(Token::RightParen)?;
        let implements = self.opt_class_implements()?;
        let body = self.record_body()?;
        Ok(RecordDeclaration {
            name,
            components,
            implements,
            body,
        })
    }

    /// ```text
    /// record_component:
    ///     {annotation} type_term identifier
    ///     {annotation} type_term ... identifier
    fn record_component(&mut self) -> Result<Modified<RecordComponent>, ParseError> {
        let annotations = self.zero_or_more(|this| this.annotation().map(Annotation::into));
        let component_type = self.type_term()?;
        if self.accept(Token::Ellipsis) {
            let name = self.identifier()?;
            Ok(RecordComponent::VariableArity { component_type, name }.with_modifiers(annotations))
        } else {
            let name = self.identifier()?;
            Ok(RecordComponent::Normal { component_type, name }.with_modifiers(annotations))
        }
    }

    fn record_body(&mut self) -> Result<Vec<RecordBodyDeclaration>, ParseError> {
        self.class_body()
    }

    fn enum_declaration(&mut self) -> Result<EnumDeclaration, ParseError> {
        self.assert(Token::Enum)?;
        let name = self.identifier()?.try_into()?;
        let implements = self.opt_class_implements()?;
        let body = self.enum_body()?;
        Ok(EnumDeclaration { name, implements, body })
    }

    /// ```text
    /// enum_body:
    ///     { enum_constant_list [enum_body_declarations] }
    /// ```
    fn enum_body(&mut self) -> Result<EnumBody, ParseError> {
        self.assert(Token::LeftBrace)?;
        let constants = self.enum_constant_list()?;
        let body_declarations = if self.accept(Token::Semicolon) {
            self.zero_or_more(Self::class_body_declaration)
        } else {
            vec![]
        };
        self.assert(Token::RightBrace)?;
        Ok(EnumBody { constants, body_declarations })
    }

    /// ```text
    /// enum_constant_list:
    ///     [,]
    ///     enum_constant {, enum_constant} [,]
    /// ```
    fn enum_constant_list(&mut self) -> Result<Vec<Modified<EnumConstant>>, ParseError> {
        if self.accept(Token::Comma) {
            // just a single comma
            return Ok(vec![]);
        }

        let mut items = vec![];
        loop {
            // enum constants list ends either with the end of the enum (right brace) or the semicolon
            // that starts the enum body declarations
            if self.next_is(Token::RightBrace) || self.next_is(Token::Semicolon) {
                break;
            }
            items.push(self.enum_constant()?);
            if !self.accept(Token::Comma) {
                break;
            }
        }
        Ok(items)
    }

    /// ```text
    /// enum_constant:
    ///     {annotation} identifier [( argument_list )] [class_body]
    /// ```
    fn enum_constant(&mut self) -> Result<Modified<EnumConstant>, ParseError> {
        let annotations = self.zero_or_more(|this| this.annotation().map(Annotation::into));
        let name = self.identifier()?;
        let args = if self.accept(Token::LeftParen) {
            let args = self.argument_list()?;
            self.assert(Token::RightParen)?;
            Some(args)
        } else {
            None
        };
        let body = if self.next_is(Token::LeftBrace) {
            Some(self.class_body()?)
        } else {
            None
        };
        Ok(EnumConstant { name, args, body }.with_modifiers(annotations))
    }

    fn interface_declaration(&mut self) -> Result<InterfaceDeclaration, ParseError> {
        one_of!(
            self.normal_interface_declaration()
                .map(NormalInterfaceDeclaration::into),
            self.annotation_interface_declaration()
                .map(AnnotationInterfaceDeclaration::into),
        )
    }

    fn normal_interface_declaration(&mut self) -> Result<NormalInterfaceDeclaration, ParseError> {
        self.assert(Token::Interface)?;
        let identifier = self.identifier()?.try_into()?;
        let extends = self.opt_interface_extends()?;
        let permits = self.opt_class_permits()?;
        let body = self.interface_body()?;
        Ok(NormalInterfaceDeclaration {
            identifier,
            extends,
            permits,
            body,
        })
    }

    fn opt_interface_extends(&mut self) -> Result<Option<ClassTypeList>, ParseError> {
        self.opt(
            |this| this.accept(Token::Extends),
            |this| this.delimited_at_least_1(Self::class_type, Self::comma),
        )
    }

    fn interface_body(&mut self) -> Result<Vec<Modified<ClassMemberDeclaration>>, ParseError> {
        self.assert(Token::LeftBrace)?;
        let members = self.zero_or_more(Self::class_member_declaration);
        self.assert(Token::RightBrace)?;
        Ok(members)
    }

    fn annotation_interface_declaration(
        &mut self,
    ) -> Result<AnnotationInterfaceDeclaration, ParseError> {
        if !peek!(self, 0 => Token::At, 1 => Token::Interface) {
            return Err(ParseError::NoProduction);
        }
        self.assert(Token::At)?;
        self.assert(Token::Interface)?;
        let name = self.identifier()?.try_into()?;
        self.assert(Token::LeftBrace)?;
        let body = self.zero_or_more(Self::class_member_declaration);
        self.assert(Token::RightBrace)?;
        Ok(AnnotationInterfaceDeclaration { name, body })
    }

    /// ```text
    /// constructor_declaration:
    ///     regular_constructor_declaration
    ///     compact_constructor_declaration
    ///
    /// regular_constructor_declaration:
    ///     identifier ( [formal_parameters] ) constructor_body
    ///
    /// compact_constructor_declaration:
    ///     identifier constructor_body
    /// ```
    fn constructor_declaration(&mut self) -> Result<ClassMemberDeclaration, ParseError> {
        if peek!(self, 0 => Token::Id(_) , 1 => Token::LeftParen | Token::LeftBrace) {
            let name = self.identifier()?.try_into()?;
            if self.accept(Token::LeftParen) {
                let parameters = self.formal_parameters()?;
                self.assert(Token::RightParen)?;
                let throws = self.opt_throws()?;
                let body = self.constructor_body()?;
                Ok(ClassMemberDeclaration::Constructor { name, parameters, throws, body })
            } else {
                let body = self.constructor_body()?;
                Ok(ClassMemberDeclaration::CompactConstructor { name, body })
            }
        } else {
            Err(ParseError::NoProduction)
        }
    }

    /// ```text
    /// method_or_field_declaration:
    ///     method_declaration
    ///     field_declaration
    ///
    /// method_declaration:
    ///     type_term identifier ( [formal_parameters] ) [default_value] method_body
    ///
    /// field_declaration:
    ///     type_term identifier [= variable_initializer] {, identifier [= variable_initializer]}
    /// ```
    fn method_or_field_declaration(&mut self) -> Result<ClassMemberDeclaration, ParseError> {
        let type_term = self.type_term()?;
        let identifier = self.identifier()?;
        if self.accept(Token::LeftParen) {
            let parameters = self.formal_parameters()?;
            self.assert(Token::RightParen)?;
            let throws = self.opt_throws()?;
            let default = self.opt_default()?;
            let body = self.method_body()?;
            Ok(MethodDeclaration {
                result: type_term,
                identifier,
                parameters,
                throws,
                default,
                body,
            }
            .into())
        } else {
            let mut field_declaration = vec![VariableDeclarator {
                name: VariableDeclaratorId::Named(identifier),
                initializer: self
                    .variable_declarator_initializer()
                    .map_or(None, |i| Some(i)),
            }];
            if self.accept(Token::Comma) {
                field_declaration.append(&mut self.variable_declarators_list()?);
            }
            self.assert(Token::Semicolon)?;
            Ok(ClassMemberDeclaration::Field {
                variable_type: type_term,
                declarations: field_declaration,
            })
        }
    }

    fn formal_parameters(&mut self) -> Result<FormalParameterList, ParseError> {
        self.delimited_list(Self::formal_parameter, Self::comma)
    }

    fn formal_parameter(&mut self) -> Result<Modified<FormalParameter>, ParseError> {
        let modifiers = self.modifiers(ModifierKind::VARIABLE);
        let param_type = self.type_term()?;
        if self.accept(Token::Ellipsis) {
            // variable arity
            let identifier = self.identifier()?;
            Ok(FormalParameter::VariableArityParameter(param_type, identifier)
                .with_modifiers(modifiers))
        } else {
            let identifier = self.identifier()?;
            Ok(FormalParameter::NormalParameter(
                param_type,
                VariableDeclaratorId::Named(identifier),
            )
            .with_modifiers(modifiers))
        }
    }

    fn primitive_type(&mut self) -> Result<Type, ParseError> {
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
        } else if self.accept(Token::Void) {
            Ok(Type::Void)
        } else {
            Err(ParseError::NoProduction)
        }
    }

    fn opt_throws(&mut self) -> Result<Multiple<Modified<Type>>, ParseError> {
        if self.accept(Token::Throws) {
            self.delimited_at_least_1(
                |this| {
                    let modifiers = this.modifiers(ModifierKind::VARIABLE);
                    Ok(this.reference_type()?.with_modifiers(modifiers))
                },
                Self::comma,
            )
        } else {
            Ok(vec![])
        }
    }

    /// ```text
    /// default_value:
    ///     default element_value
    /// ```
    fn opt_default(&mut self) -> Result<Option<ElementValue>, ParseError> {
        self.opt(|this| this.accept(Token::Default), Self::element_value)
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
    fn block(&mut self) -> Result<BlockStatements, ParseError> {
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
    /// Lastly, `LabeledStatement`, `ExpressionStatement`, and `LocalVariableDeclarationStatement`
    /// are grouped into [Parser::statement_starting_with_name]
    ///
    /// The resulting productions are thus:
    /// ```text
    /// local_variable_declaration_or_statement:
    ///     empty_statement
    ///     block
    ///     simple_statement
    ///     statement_starting_with_name
    /// ```
    fn local_variable_declaration_or_statement(&mut self) -> Result<Statement, ParseError> {
        one_of!(
            self.empty_statement(),
            self.block().map(|v| Statement::Block(v)),
            self.simple_statement(),
            self.statement_starting_with_name(),
        )
    }

    fn empty_statement(&mut self) -> Result<Statement, ParseError> {
        self.assert(Token::Semicolon)?;
        Ok(Statement::EmptyStatement)
    }

    /// from [Parser::local_variable_declaration_or_statement],
    /// ```text
    /// simple_statement:
    ///     if_statement
    ///     while_statement
    ///     for_statement
    ///     do_statement
    ///     break_statement
    ///     continue_statement
    ///     assert_statement
    ///     return_statement
    ///     try_statement
    ///     throw_statement
    ///     switch_statement
    ///     yield_statement
    ///     synchronized_statement
    ///
    /// if_statement:
    ///     if_then_statement
    ///     if_then_else_statement
    /// ```
    fn simple_statement(&mut self) -> Result<Statement, ParseError> {
        one_of!(
            self.if_statement(),
            self.while_statement(),
            self.for_statement(),
            self.do_statement(),
            self.break_statement(),
            self.continue_statement(),
            self.assert_statement(),
            self.return_statement(),
            self.try_statement(),
            self.throw_statement(),
            self.switch_statement(),
            self.yield_statement(),
            self.synchronized_statement(),
        )
    }

    /// From [Parser::local_variable_declaration_or_statement], the remaining kind of statements:
    /// ```text
    /// statement_starting_with_name:
    ///     labeled_statement
    ///     expression_statement
    ///     local_variable_declaration_statement
    /// ```
    /// All three alternatives can start with an `Identifier`, they look roughly like this:
    /// ```text
    /// labeled_statement:
    ///     identifier : statement
    ///
    /// expression_statement:
    ///     Assignment ;
    ///     PostIncrementExpression ;
    ///     PostDecrementExpression ;
    ///     MethodInvocation ;
    ///     ClassInstanceCreationExpression ;
    ///
    /// local_variable_declaration_statement
    ///     {modifier} type variable_declarator_list ;
    ///```
    /// At this point we cannot distinguish between the identifier part of the `labeled_statement`,
    /// the first primary in some of the possible derivations of `expression_statement`, and the type
    /// of `local_variable_declaration_statement`, so we factor them out into [Parser::term].
    ///
    /// At this point in the parser, `term` should be understood operationally rather than
    /// strictly grammatically: it parses any construct that can begin with an identifier and
    /// form a complete expression, a type, or a standalone identifier.
    /// - simple names: `x`
    /// - qualified names: `a.b.c`
    /// - field accesses: `a.b`
    /// - array accesses: `a[i]`
    /// - method calls: `a.b()`
    /// - assignments: `x = y`, `a.b = c`
    ///
    /// By parsing `term` first, we defer the decision between these alternatives until
    /// additional tokens (such as `:`, `identifier`, or `;`) make the distinction unambiguous.
    /// ```text
    /// statement_starting_with_name:
    ///     {modifier} term variable_declarator {, variable_declarator} ;
    ///     term [statement_ending]
    ///
    /// statement_ending:
    ///     : statement // labeled statement
    ///     ; // just a term - in this case it's a complete expression_statement
    ///
    /// variable_declarator:
    ///     identifier [= variable_initializer]
    ///     _          [= variable_initializer]
    /// ```
    fn statement_starting_with_name(&mut self) -> Result<Statement, ParseError> {
        let modifiers = self.modifiers(ModifierKind::VARIABLE);
        let expression = self.term()?;
        if let Ok(var_declarations) = self.variable_declarators_list() {
            self.assert(Token::Semicolon)?;
            return Ok(Statement::VariableDeclaration(
                VariableDeclaration {
                    variable_type: expression.try_into()?,
                    declarators: var_declarations,
                }
                .with_modifiers(modifiers),
            ));
        }

        if self.accept(Token::Colon) {
            return match expression {
                Expression::Name(id) => {
                    let body = Box::new(self.block_statement()?);
                    Ok(Statement::Labeled { label: id, body })
                }
                _ => Err(ParseError::NoProduction),
            };
        }

        if self.accept(Token::Semicolon) {
            return Ok(Statement::ExpressionStatement(expression));
        }

        Err(ParseError::NoProduction)
    }

    /// `term` defines the maximal construct we can parse at this point without yet knowing
    /// whether it is:
    /// - an assignment
    /// - a value-producing construct,
    /// - a type,
    /// - or the start of a labeled statement.
    ///
    /// While it often begins with an identifier, it may also start with other constructs
    /// (e.g. primitive types, parenthesized forms, casts). From that starting point,
    /// `term` continues consuming input as long as it can legally extend the construct
    /// through qualified names, member accesses, ternary/binary/unary operators, etc.
    ///
    /// The minimum precedence construct consumed are the ternary `conditional_expression`, the binary
    /// operators, the left side of an assignment, and type names
    ///
    /// ```text
    /// term:
    ///     left_hand_side = term
    ///     conditional_expression
    ///
    /// left_hand_side:
    ///     identifier {. identifier}
    ///     field_access
    ///     array_access
    /// ```
    fn term(&mut self) -> Result<Expression, ParseError> {
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
            let lhs = expr.try_into()?;
            let rhs = self.term()?;
            /*
            Compound assignments are not strictly equivalent to assigning the result of a binary op,
            as there can be some differences to how the subexpressions are evaluated.
            For example in the following expression:
                f().x += 5
                f() is evaluated only once.
            Transforming this expression into
                f().x = f().x + 5
            will evaluate f() twice.
            */
            Ok(Expression::Assignment { lhs, rhs: Box::new(rhs), op })
        } else {
            Ok(expr)
        }
    }

    fn expression(&mut self) -> Result<Expression, ParseError> {
        self.term()
    }

    /// ```text
    /// conditional_expression:
    ///     conditional_or_expression [? expression : conditional_expression]
    /// ```
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
        self.left_associative_binary_operation(Self::conditional_and_expression, |this| {
            accept_with_value!(this,
                Token::LogicalOr => BinOp::LogicalOr
            )
        })
    }

    fn conditional_and_expression(&mut self) -> Result<Expression, ParseError> {
        self.left_associative_binary_operation(Self::inclusive_or_expression, |this| {
            accept_with_value!(this,
                Token::LogicalAnd => BinOp::LogicalAnd
            )
        })
    }

    fn inclusive_or_expression(&mut self) -> Result<Expression, ParseError> {
        self.left_associative_binary_operation(Self::exclusive_or_expression, |this| {
            accept_with_value!(this,
                Token::BitwiseOr => BinOp::BitwiseOr
            )
        })
    }

    fn exclusive_or_expression(&mut self) -> Result<Expression, ParseError> {
        self.left_associative_binary_operation(Self::and_expression, |this| {
            accept_with_value!(this,
                Token::BitwiseXor => BinOp::BitwiseXor
            )
        })
    }

    fn and_expression(&mut self) -> Result<Expression, ParseError> {
        self.left_associative_binary_operation(Self::equality_expression, |this| {
            accept_with_value!(this,
                Token::BitwiseAnd => BinOp::BitwiseAnd
            )
        })
    }

    fn equality_expression(&mut self) -> Result<Expression, ParseError> {
        self.left_associative_binary_operation(Self::relational_expression, |this| {
            accept_with_value!(this,
                Token::Equals => BinOp::Equal,
                Token::NotEquals => BinOp::NotEqual,
            )
        })
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
        self.left_associative_binary_operation(Self::additive_expression, |this| {
            accept_with_value!(this,
                Token::LeftShift => BinOp::LeftShift,
                Token::SignedRightShift => BinOp::SignedRightShift,
                Token::UnsignedRightShift => BinOp::UnsignedRightShift,
            )
        })
    }

    fn additive_expression(&mut self) -> Result<Expression, ParseError> {
        self.left_associative_binary_operation(Self::multiplicative_expression, |this| {
            accept_with_value!(this,
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Subtract,
            )
        })
    }

    fn multiplicative_expression(&mut self) -> Result<Expression, ParseError> {
        self.left_associative_binary_operation(Self::unary_expression, |this| {
            accept_with_value!(this,
                Token::Multiply => BinOp::Multiply,
                Token::Divide   => BinOp::Divide,
                Token::Modulo   => BinOp::Modulo,
            )
        })
    }

    /// ```text
    /// unary_expression:
    ///     switch
    ///     {prefix_oprerator} postfix_expression
    ///
    /// prefix_operator:
    ///     one of:
    ///         ~  !  +  -  ++  --
    /// ```
    fn unary_expression(&mut self) -> Result<Expression, ParseError> {
        if let Ok(switch) = self.switch() {
            return Ok(switch.into());
        }
        if self.accept(Token::Tilde) {
            Ok(Expression::BitwiseComplement(Box::new(self.unary_expression()?)))
        } else if self.accept(Token::ExclamationMark) {
            Ok(Expression::LogicalNot(Box::new(self.unary_expression()?)))
        } else if self.accept(Token::Plus) {
            Ok(Expression::UnaryPlus(Box::new(self.unary_expression()?)))
        } else if self.accept(Token::Minus) {
            Ok(Expression::UnaryMinus(Box::new(self.unary_expression()?)))
        } else if self.accept(Token::Increment) {
            Ok(Expression::PreIncrement(Box::new(self.unary_expression()?)))
        } else if self.accept(Token::Decrement) {
            Ok(Expression::PreDecrement(Box::new(self.unary_expression()?)))
        } else {
            self.postfix_expression()
        }
    }

    /// ```text
    /// postfix_expression:
    ///     primary {selector} [postfix_operator]
    ///
    /// postfix_operator:
    ///     ++
    ///     --
    /// ```
    fn postfix_expression(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.primary()?;
        expr = self.parse_selectors(expr)?;
        if self.accept(Token::Increment) {
            expr = Expression::PostIncrement(Box::new(expr));
        } else if self.accept(Token::Decrement) {
            expr = Expression::PostDecrement(Box::new(expr));
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        one_of!(
            self.literal(),
            self.primitive_type().map(Type::into),
            self.parenthesized_expression(),
            self.instance_creation_expression(),
            self.identifier_expression(),
            self.this_access(),
        )
    }

    /// ```text
    /// literal:
    ///     integer_literal
    ///     long_literal
    ///     boolean_literal
    ///     char_literal
    ///     string_literal
    ///     null_literal
    /// ```
    fn literal(&mut self) -> Result<Expression, ParseError> {
        one_of!(
            self.integer_literal()
                .map(|v| Expression::IntegerLiteral(v)),
            self.long_literal().map(|v| Expression::LongLiteral(v)),
            self.boolean_literal()
                .map(|v| Expression::BooleanLiteral(v)),
            self.char_literal().map(|v| Expression::CharLiteral(v)),
            self.string_literal().map(|v| Expression::StringLiteral(v)),
            self.assert(Token::NullLiteral)
                .map(|_| Expression::NullLiteral)
        )
    }

    fn parenthesized_expression(&mut self) -> Result<Expression, ParseError> {
        if self.accept(Token::LeftParen) {
            let expr = self.expression()?; // assuming you have this
            self.assert(Token::RightParen)?;
            Ok(expr)
        } else {
            Err(ParseError::NoProduction)
        }
    }

    /// ```text
    /// selector:
    ///     . this
    ///     . class // class literal
    ///     . super
    ///     . identfier // field access
    ///     . identifier ( [arg_list] ) // method invocation
    ///     [ expression ] // array access
    ///     [ ] // array type
    ///     ( [arg_list] ) // bare method invocation
    ///     :: identifier // named method reference
    ///     :: new // constructor method reference
    /// ```
    fn parse_selectors(&mut self, expr: Expression) -> Result<Expression, ParseError> {
        let mut expr = expr;
        loop {
            if self.accept(Token::Dot) {
                if let Ok(id) = accept_with_value!(self, Token::Id) {
                    if self.accept(Token::LeftParen) {
                        let arg_list = self.argument_list()?;
                        self.assert(Token::RightParen)?;
                        expr = Expression::MethodCall(MethodCall {
                            target: Some(Box::new(expr)),
                            name: id,
                            arguments: arg_list,
                        })
                    } else {
                        expr = Expression::MemberAccess(MemberAccess {
                            target: Box::new(expr),
                            name: id,
                        })
                    }
                } else if self.accept(Token::Class) {
                    expr = Expression::ClassLiteral(Type::try_from(expr)?)
                } else if self.accept(Token::This) {
                    expr = Expression::QualifiedThis(Type::try_from(expr)?)
                }
            } else if self.accept(Token::LeftBracket) {
                if self.accept(Token::RightBracket) {
                    expr = Type::from(ArrayType {
                        element_type: Box::new(Type::try_from(expr)?),
                    })
                    .into()
                } else {
                    let index = self.expression()?;
                    self.assert(Token::RightBracket)?;
                    expr = ArrayAccess {
                        target: Box::new(expr),
                        index: Box::new(index),
                    }
                    .into();
                }
            } else if self.accept(Token::LeftParen) {
                let arg_list = self.argument_list()?;
                self.assert(Token::RightParen)?;
                expr = Expression::MethodCall(MethodCall {
                    target: None,
                    name: expr.try_into()?,
                    arguments: arg_list,
                })
            } else if self.accept(Token::DoubleColon) {
                let target = Box::new(expr);
                let method = if self.accept(Token::New) {
                    MethodReferenceType::Constructor
                } else {
                    let name = self.identifier()?;
                    MethodReferenceType::Named(name)
                };
                expr = Expression::MethodReference { target, method };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn identifier_expression(&mut self) -> Result<Expression, ParseError> {
        Ok(Expression::Name(self.identifier()?))
    }

    fn variable_declarators_list(&mut self) -> Result<VariableDeclaratorList, ParseError> {
        self.delimited_at_least_1(
            |this| {
                Ok(VariableDeclarator {
                    name: this.variable_declarator_id()?,
                    initializer: this
                        .variable_declarator_initializer()
                        .map_or(None, |i| Some(i)),
                })
            },
            Self::comma,
        )
    }

    fn variable_declarator_id(&mut self) -> Result<VariableDeclaratorId, ParseError> {
        if let Ok(name) = accept_with_value!(self, Token::Id) {
            Ok(VariableDeclaratorId::Named(name))
        } else if self.accept(Token::Underscore) {
            Ok(VariableDeclaratorId::Unnamed)
        } else {
            Err(ParseError::NoProduction)
        }
    }

    fn variable_declarator_initializer(&mut self) -> Result<VariableInitializer, ParseError> {
        self.assert(Token::Assign)?;
        self.variable_initializer()
    }

    fn variable_initializer(&mut self) -> Result<VariableInitializer, ParseError> {
        one_of!(
            self.expression().map(Expression::into),
            self.array_initializer()
                .map(|i| VariableInitializer::ArrayInitializer(i)),
        )
    }

    fn argument_list(&mut self) -> Result<ArgumentList, ParseError> {
        self.delimited_list(Self::expression, Self::comma)
    }

    /// ```text
    /// constructor_body:
    ///     { {block_statement} [constructor_invocation] {block_statement} }
    ///
    /// constructor_invocation:
    ///     this ( [argument_list] ) ;
    /// ```
    fn constructor_body(&mut self) -> Result<ConstructorBody, ParseError> {
        self.assert(Token::LeftBrace)?;
        let first_part = self.zero_or_more(Self::block_statement);
        let constructor_invocation = if self.accept(Token::This) {
            self.assert(Token::LeftParen)?;
            let arguments = self.argument_list()?;
            self.assert(Token::RightParen)?;
            self.assert(Token::Semicolon)?;
            Some(ConstructorInvocation::Alternate { arguments })
        } else {
            None
        };
        let (prologue, epilogue) = match constructor_invocation {
            Some(_) => {
                let prologue = if first_part.is_empty() { None } else { Some(first_part) };

                let epilogue = self.zero_or_more(Self::block_statement);

                (prologue, epilogue)
            }
            None => {
                // No constructor call → everything is epilogue
                (None, first_part)
            }
        };
        self.assert(Token::RightBrace)?;
        Ok(ConstructorBody {
            prologue,
            constructor_invocation,
            epilogue,
        })
    }

    /// ```text
    /// unqualified_class_instance_creation_expression:
    ///     new base_type ( argument_list )
    ///     new base_type array_creation
    ///
    /// base_type:
    ///     primitive_type
    ///     reference_type
    /// ```
    fn instance_creation_expression(&mut self) -> Result<Expression, ParseError> {
        self.assert(Token::New)?;
        // not using type_term here because we want to get the base type only, without possible brackets
        let type_to_instantiate = one_of!(self.primitive_type(), self.reference_type())?;
        if self.next_is(Token::LeftParen) {
            self.class_instance_creation(type_to_instantiate)
        } else if self.next_is(Token::LeftBracket) {
            self.array_creation(type_to_instantiate)
        } else {
            Err(ParseError::NoProduction)
        }
    }

    fn class_instance_creation(
        &mut self,
        type_to_instantiate: Type,
    ) -> Result<Expression, ParseError> {
        self.assert(Token::LeftParen)?;
        let arguments = self.argument_list()?;
        self.assert(Token::RightParen)?;
        Ok(Expression::InstanceCreation { type_to_instantiate, arguments })
    }

    /// ```text
    /// array_creation:
    ///     dim_expression {dim_expression} {dims}
    ///     dims {dims} array_initializer
    ///
    /// dim_expression:
    ///     [ expression ]
    ///
    /// dims:
    ///     [ ]
    /// ```
    fn array_creation(&mut self, mut element_type: Type) -> Result<Expression, ParseError> {
        // FIXME: can be refactored with peek_n
        self.assert(Token::LeftBracket)?;
        let array_creation_mode = if self.accept(Token::RightBracket) {
            element_type = Type::from(ArrayType {
                element_type: Box::new(element_type),
            });
            while self.accept(Token::LeftBracket) {
                self.assert(Token::RightBracket)?;
                element_type = Type::from(ArrayType {
                    element_type: Box::new(element_type),
                });
            }
            let initializer = self.array_initializer()?;
            ArrayCreationMode::Initialized(initializer)
        } else {
            let mut sized_dimensions = vec![self.expression()?];
            let mut unsized_dimensions = 0;
            self.assert(Token::RightBracket)?;
            loop {
                if !self.accept(Token::LeftBracket) {
                    break;
                }
                if self.accept(Token::RightBracket) {
                    unsized_dimensions += 1;
                    break;
                }
                sized_dimensions.push(self.expression()?);
                self.assert(Token::RightBracket)?;
            }
            while self.accept(Token::LeftBracket) {
                self.assert(Token::RightBracket)?;
                unsized_dimensions += 1;
            }
            ArrayCreationMode::Sized {
                sized_dimensions,
                unsized_dimensions,
            }
        };
        Ok(Expression::ArrayCreation {
            element_type,
            array_creation_mode,
        })
    }

    /// ```text
    /// array_initializer:
    ///     { [variable_initializer_list] [,] }
    ///
    /// variable_initializer_list:
    ///     variable_initializer {, variable_initializer}
    /// ```
    fn array_initializer(&mut self) -> Result<VariableInitializerList, ParseError> {
        self.assert(Token::LeftBrace)?;
        let mut items = vec![];

        // {,}
        if self.accept(Token::Comma) {
            self.assert(Token::RightBrace)?;
            return Ok(items);
        }

        loop {
            if self.next_is(Token::RightBrace) {
                break;
            }
            items.push(self.variable_initializer()?);
            if !self.accept(Token::Comma) {
                break;
            }
        }
        self.assert(Token::RightBrace)?;
        Ok(items)
    }

    fn this_access(&mut self) -> Result<Expression, ParseError> {
        if self.next_is(Token::This) && !self.nth_is(1, Token::LeftParen) {
            self.next()?;
            Ok(Expression::This)
        } else {
            Err(ParseError::NoProduction)
        }
    }

    /// The general structure of the if statement is as follows:
    /// ```text
    /// if_statement:
    ///     if ( expression ) statement [else_clause]
    ///
    /// else_clause:
    ///     else statement
    /// ```
    /// To solve the dangling else problem, Java defines that the else clause belongs to the
    /// innermost `if` ([§7.6](https://docs.oracle.com/javase/specs/jls/se26/html/jls-14.html#jls-14.5))
    /// This means that if the else clause exists, the middle statement (then clause) must not end
    /// in a short-if statement (without an `else` clause).
    /// Here this is achieved simply by a recursive call, which consumes the else clause if it
    /// appears.
    fn if_statement(&mut self) -> Result<Statement, ParseError> {
        self.assert(Token::If)?;
        self.assert(Token::LeftParen)?;
        let condition = self.expression()?;
        self.assert(Token::RightParen)?;
        let if_true = Box::new(self.block_statement()?);
        let if_false = if self.accept(Token::Else) {
            Some(Box::new(self.block_statement()?))
        } else {
            None
        };
        Ok(Statement::If { condition, if_true, if_false })
    }

    fn while_statement(&mut self) -> Result<Statement, ParseError> {
        self.assert(Token::While)?;
        self.assert(Token::LeftParen)?;
        let condition = self.expression()?;
        self.assert(Token::RightParen)?;
        let statement = Box::new(self.block_statement()?);
        Ok(Statement::While { condition, statement })
    }

    /// ```text
    /// for_statement:
    ///     for ( for_header ) statement
    /// ```
    fn for_statement(&mut self) -> Result<Statement, ParseError> {
        self.assert(Token::For)?;
        self.assert(Token::LeftParen)?;
        let header = self.for_header()?;
        self.assert(Token::RightParen)?;
        let statement = Box::new(self.block_statement()?);
        match header {
            ForHeader::BasicForHeader { initializer, condition, update } => Ok(Statement::For {
                initializer,
                condition,
                update,
                statement,
            }),
            ForHeader::ForEachHeader { variable_declaration, iterable } => Ok(Statement::ForEach {
                variable_declaration,
                iterable,
                statement,
            }),
        }
    }

    /// ```text
    /// for_header:
    ///     [for_init] ; [expression] ; [statement_expression_list] ;
    ///     local_variable_declaration : expression
    ///
    /// for_init:
    ///     statement_expression_list
    ///     local_variable_declaration
    ///
    /// statement_expression_list:
    ///     term {, term}
    /// ```
    fn for_header(&mut self) -> Result<ForHeader, ParseError> {
        let modifiers = self.modifiers(ModifierKind::VARIABLE);

        if self.accept(Token::Semicolon) {
            // basic for, empty init
            let initializer = ForInit::Expressions(vec![]);
            let (condition, update) = self.basic_for_condition_and_update()?;
            return Ok(ForHeader::BasicForHeader { initializer, condition, update });
        }

        let expression = self.term()?;

        if self.accept(Token::Comma) {
            // basic for, init is a statement_expression_list
            let mut init_expressions = vec![expression];
            init_expressions.extend(self.statement_expression_list()?);
            let initializer = ForInit::Expressions(init_expressions);
            self.assert(Token::Semicolon)?;
            let (condition, update) = self.basic_for_condition_and_update()?;
            return Ok(ForHeader::BasicForHeader { initializer, condition, update });
        }

        if self.accept(Token::Semicolon) {
            // basic for, single expression init
            let initializer = ForInit::Expressions(vec![expression]);
            let (condition, update) = self.basic_for_condition_and_update()?;
            return Ok(ForHeader::BasicForHeader { initializer, condition, update });
        }

        // either a basic for with local_variable_declaration init, or a for-each
        let var_declarators = self.variable_declarators_list()?;
        let var_declarations = VariableDeclaration {
            variable_type: expression.try_into()?,
            declarators: var_declarators,
        }
        .with_modifiers(modifiers);
        if self.accept(Token::Semicolon) {
            // basic for init is a local_variable_declaration
            let (condition, update) = self.basic_for_condition_and_update()?;
            return Ok(ForHeader::BasicForHeader {
                initializer: ForInit::LocalVarDeclaration(var_declarations),
                condition,
                update,
            });
        }

        // for each
        self.assert(Token::Colon)?;
        let iterable = self.expression()?;
        Ok(ForHeader::ForEachHeader {
            variable_declaration: var_declarations,
            iterable,
        })
    }

    //noinspection DuplicatedCode
    fn basic_for_condition_and_update(
        &mut self,
    ) -> Result<(Option<Expression>, ForUpdate), ParseError> {
        let condition = if self.accept(Token::Semicolon) {
            None
        } else {
            let expression = self.expression()?;
            self.assert(Token::Semicolon)?;
            Some(expression)
        };
        let update = self.statement_expression_list()?;
        Ok((condition, update))
    }

    fn statement_expression_list(&mut self) -> Result<Vec<Expression>, ParseError> {
        self.delimited_list(Self::term, Self::comma)
    }

    fn do_statement(&mut self) -> Result<Statement, ParseError> {
        self.assert(Token::Do)?;
        let statement = Box::new(self.block_statement()?);
        self.assert(Token::While)?;
        self.assert(Token::LeftParen)?;
        let condition = self.expression()?;
        self.assert(Token::RightParen)?;
        self.assert(Token::Semicolon)?;
        Ok(Statement::DoWhile { statement, condition })
    }

    fn break_statement(&mut self) -> Result<Statement, ParseError> {
        self.assert(Token::Break)?;
        let label = self.identifier().ok();
        self.assert(Token::Semicolon)?;
        Ok(Statement::Break(label))
    }

    fn continue_statement(&mut self) -> Result<Statement, ParseError> {
        self.assert(Token::Continue)?;
        let label = self.identifier().ok();
        self.assert(Token::Semicolon)?;
        Ok(Statement::Continue(label))
    }

    fn assert_statement(&mut self) -> Result<Statement, ParseError> {
        self.assert(Token::Assert)?;
        let condition = self.expression()?;
        let detail_message = if self.accept(Token::Colon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.assert(Token::Semicolon)?;
        Ok(Statement::Assert { condition, detail_message })
    }

    //noinspection DuplicatedCode
    fn return_statement(&mut self) -> Result<Statement, ParseError> {
        self.assert(Token::Return)?;
        let expression = if self.accept(Token::Semicolon) {
            None
        } else {
            let expression = self.expression()?;
            self.assert(Token::Semicolon)?;
            Some(expression)
        };
        Ok(Statement::Return(expression))
    }

    /// ```text
    /// try_statement:
    ///     try [( resource_list )] block {catch_clause} [finally]
    ///
    /// resource_list:
    ///     resource {; resource} [;]
    /// ```
    fn try_statement(&mut self) -> Result<Statement, ParseError> {
        self.assert(Token::Try)?;
        let resources = if self.accept(Token::LeftParen) {
            let resources = self.delimited_at_least_1(Self::try_resource, Self::semicolon)?;
            self.accept(Token::Semicolon);
            self.assert(Token::RightParen)?;
            resources
        } else {
            vec![]
        };
        let body = self.block()?;
        let catch_clauses = self.zero_or_more(Self::catch_clause);
        let finally_block = if self.accept(Token::Finally) {
            Some(self.block()?)
        } else {
            None
        };
        Ok(Statement::Try {
            resources,
            try_block: body,
            exception_handlers: catch_clauses,
            finally_block,
        })
    }

    /// ```text
    /// resource:
    ///     local_variable_declaration
    ///     variable_access
    /// ```
    fn try_resource(&mut self) -> Result<Resource, ParseError> {
        let modifiers = self.modifiers(ModifierKind::VARIABLE);
        let expression = self.term()?;
        if let Ok(var_declarations) = self.variable_declarators_list() {
            Ok(Resource::VariableDeclaration(
                VariableDeclaration {
                    variable_type: expression.try_into()?,
                    declarators: var_declarations,
                }
                .with_modifiers(modifiers),
            ))
        } else {
            Ok(Resource::VariableAccess(expression))
        }
    }

    /// ```text
    /// catch_clause:
    ///     catch ( catch_type variable_declarator_id ) block
    ///
    /// catch_type:
    ///     catch_type_part {| catch_type_part}
    ///
    /// catch_type_part:
    ///     {modifier} type_term
    /// ```
    fn catch_clause(&mut self) -> Result<CatchClause, ParseError> {
        self.assert(Token::Catch)?;
        self.assert(Token::LeftParen)?;
        let catch_type = self.delimited_at_least_1(
            |this| {
                let modifiers = this.modifiers(ModifierKind::VARIABLE);
                Ok(this.type_term()?.with_modifiers(modifiers))
            },
            Self::pipe,
        )?;
        let var_id = self.variable_declarator_id()?;
        self.assert(Token::RightParen)?;
        let body = self.block()?;
        Ok(CatchClause { catch_type, var_id, body })
    }

    /// ```text
    /// type_term:
    ///     primitive_type {dims}
    ///     reference_type {dims}
    ///
    /// dims:
    ///     [ ]
    /// ```
    fn type_term(&mut self) -> Result<Type, ParseError> {
        let mut type_term = one_of!(self.primitive_type(), self.reference_type())?;
        while self.accept(Token::LeftBracket) {
            self.assert(Token::RightBracket)?;
            type_term = Type::from(ArrayType {
                element_type: Box::new(type_term),
            })
        }
        Ok(type_term)
    }

    /// ```text
    /// reference_type:
    ///     type_part {. type_part}
    /// ```
    fn reference_type(&mut self) -> Result<Type, ParseError> {
        let type_parts = self.delimited_at_least_1(Self::type_part, Self::dot)?;
        Ok(Type::Class(type_parts))
    }

    /// ```text
    /// type_part:
    ///     identifier
    /// ```
    fn type_part(&mut self) -> Result<ClassTypePart, ParseError> {
        Ok(ClassTypePart { identifier: self.identifier()? })
    }

    fn class_type(&mut self) -> Result<ClassType, ParseError> {
        match self.type_term() {
            Ok(Type::Class(class_type)) => Ok(class_type),
            Ok(_) => Err(ParseError::NoProduction),
            Err(e) => Err(e),
        }
    }

    fn throw_statement(&mut self) -> Result<Statement, ParseError> {
        self.assert(Token::Throw)?;
        let expression = self.expression()?;
        self.assert(Token::Semicolon)?;
        Ok(Statement::Throw(expression))
    }

    /// ```text
    /// synchronized_statement:
    ///     synchronized ( expression ) block:
    ///
    /// ```
    fn synchronized_statement(&mut self) -> Result<Statement, ParseError> {
        if !peek!(
            self,
            0 => Token::Synchronized,
            1 => Token::LeftParen,
        ) {
            Err(ParseError::NoProduction)
        } else {
            self.assert(Token::Synchronized)?;
            self.assert(Token::LeftParen)?;
            let lock = self.expression()?;
            self.assert(Token::RightParen)?;
            let body = self.block()?;
            Ok(Statement::Synchronized { lock, body })
        }
    }

    fn yield_statement(&mut self) -> Result<Statement, ParseError> {
        if !self.is_yield_statement() {
            return Err(ParseError::NoProduction);
        }
        self.next()?;
        let expression = self.expression()?;
        self.assert(Token::Semicolon)?;
        Ok(Statement::Yield(expression))
    }

    fn is_yield_statement(&mut self) -> bool {
        let yield_token = peek!(self, 0 => Token::Id(s) if s.as_str() == "yield");
        let mut expression_start = peek!(self, 1 =>
            Token::Plus | Token::Minus | Token::StringLiteral(_) | Token::CharLiteral(_) |
            Token::IntegerLiteral(_) | Token::LongLiteral(_) | Token::FloatingPointLiteral |
            Token::NullLiteral | Token::Id(_) | Token::Underscore | Token::BooleanLiteral(_) |
            Token::New | Token::Switch | Token::This | Token::Super |
            Token::Byte | Token::Char | Token::Short | Token::Int | Token::Long | Token::Float
            | Token::Double | Token::Void | Token::Boolean | Token::Tilde | Token::ExclamationMark);

        expression_start |= peek!(self,1 => Token::Increment | Token::Decrement )
            && !self.nth_is(2, Token::Semicolon);

        /* TODO: at this point additional lookahead can yield better error diagnostics,
        by checking if whatever comes after looks like a method call or like an expression */
        expression_start |= self.nth_is(1, Token::LeftParen);

        /* TODO: can be used for error recovery here, or for providing better error diagnostics */
        expression_start |= self.nth_is(1, Token::Semicolon);

        yield_token && expression_start
    }

    fn switch_statement(&mut self) -> Result<Statement, ParseError> {
        self.switch().map(Statement::from)
    }

    /// ```text
    /// switch_expression:
    ///     switch ( expression ) switch_block
    /// ```
    fn switch(&mut self) -> Result<Switch, ParseError> {
        self.assert(Token::Switch)?;
        self.assert(Token::LeftParen)?;
        let expression = self.expression()?;
        self.assert(Token::RightParen)?;
        let block = self.switch_block()?;
        Ok(Switch { expression, block })
    }

    /// ```text
    /// switch_block:
    ///     { {switch_block_member} }
    /// ```
    fn switch_block(&mut self) -> Result<SwitchBlockMembers, ParseError> {
        self.assert(Token::LeftBrace)?;
        let members = self.zero_or_more(Self::switch_block_member);
        self.assert(Token::RightBrace)?;
        Ok(members)
    }

    /// ```text
    /// switch_block_member:
    ///     switch_block_statement_group
    ///
    /// switch_block_statement_group:
    ///     switch_label : {switch_label :} {block_statement}
    /// ```
    fn switch_block_member(&mut self) -> Result<SwitchBlockMember, ParseError> {
        let label = self.switch_label()?;
        if self.accept(Token::Colon) {
            let mut labels = vec![label];
            let mut additional_labels = self.zero_or_more(|this| {
                let label = this.switch_label()?;
                this.assert(Token::Colon)?;
                Ok(label)
            });
            labels.append(&mut additional_labels);
            let statements = self.zero_or_more(Self::block_statement);
            Ok(SwitchBlockMember::LabeledStatements { labels, statements })
        } else if self.accept(Token::Arrow) {
            let rule = one_of!(
                self.switch_rule_expression().map(SwitchRule::from),
                self.block().map(SwitchRule::from),
                self.throw_statement().map(SwitchRule::try_from).flatten(),
            )?;
            Ok(SwitchBlockMember::Rule { case: label, rule })
        } else {
            Err(ParseError::NoProduction)
        }
    }

    fn switch_rule_expression(&mut self) -> Result<Expression, ParseError> {
        let expression = self.expression()?;
        self.assert(Token::Semicolon)?;
        Ok(expression)
    }

    /// ```text
    /// switch_label:
    ///     switch_case_label
    ///     default
    /// ```
    fn switch_label(&mut self) -> Result<SwitchLabel, ParseError> {
        one_of!(
            self.switch_case_label(),
            self.assert(Token::Default).map(|_| SwitchLabel::Default),
        )
    }

    /// ```text
    /// switch_case_label:
    ///     case null [, default]
    ///     case conditional_expression {, conditional_expression}
    ///     case switch_case_pattern_label
    /// ```
    fn switch_case_label(&mut self) -> Result<SwitchLabel, ParseError> {
        self.assert(Token::Case)?;
        if self.accept(Token::NullLiteral) {
            let default = self.accept(Token::Comma) && self.assert(Token::Default).map(|_| true)?;
            return Ok(SwitchLabel::Null { default });
        }

        if self.check_pattern() {
            self.switch_case_pattern_label()
        } else {
            let labels = self.delimited_at_least_1(Self::conditional_expression, Self::comma)?;
            Ok(SwitchLabel::Constants(labels))
        }
    }

    fn check_pattern(&mut self) -> bool {
        let mut lookahead = 0;
        let mut paren_depth = 0;
        let mut temp_result = false;
        loop {
            let token = match self.peek_n(lookahead) {
                Ok(token) => token,
                Err(_) => {
                    return false;
                }
            };
            match token {
                Token::Byte
                | Token::Short
                | Token::Int
                | Token::Long
                | Token::Float
                | Token::Double
                | Token::Boolean
                | Token::Char
                | Token::Void
                | Token::Id(_) => match self.peek_n(lookahead + 1) {
                    Ok(Token::Id(_)) | Ok(Token::Underscore) if paren_depth == 0 => return true,
                    Ok(Token::Id(_)) | Ok(Token::Underscore) => temp_result = true,
                    Ok(Token::Arrow) | Ok(Token::Comma) if paren_depth == 0 => return false,
                    Err(_) => return false,
                    _ => {}
                },
                Token::Underscore => match self.peek_n(lookahead + 1) {
                    Ok(Token::RightParen) | Ok(Token::Comma) => return true,
                    Ok(Token::Id(_)) | Ok(Token::Underscore) if paren_depth == 0 => return true,
                    Ok(Token::Id(_)) | Ok(Token::Underscore) => temp_result = true,
                    Err(_) => return false,
                    _ => {}
                },
                Token::Dot | Token::QuestionMark | Token::Extends | Token::Super | Token::Comma => {
                }
                Token::LessThan
                | Token::LeftShift
                | Token::GreaterThan
                | Token::SignedRightShift
                | Token::UnsignedRightShift => return false,
                Token::At => lookahead = self.skip_annotation(lookahead),
                Token::LeftBracket => {
                    if peek!(self, lookahead + 1 => Token::RightBracket, lookahead + 2 => Token::Id(_) | Token::Underscore)
                    {
                        return true;
                    } else if self.nth_is(lookahead + 1, Token::RightBracket) {
                        lookahead += 1;
                    } else {
                        return temp_result;
                    }
                }
                Token::LeftParen => {
                    if self.nth_is(lookahead + 1, Token::RightParen) {
                        return paren_depth == 0 || !self.nth_is(lookahead + 2, Token::Arrow);
                    }
                    paren_depth += 1;
                }
                Token::RightParen => {
                    paren_depth -= 1;
                    if paren_depth == 0
                        && peek!(self, lookahead + 1 => Token::Id(s) if s.as_str() == "when")
                    {
                        return true;
                    }
                }
                Token::Arrow => return if paren_depth > 0 { false } else { temp_result },
                Token::Final => {
                    if paren_depth > 0 {
                        return true;
                    }
                }
                _ => return temp_result,
            }
            lookahead += 1;
        }
    }

    fn skip_annotation(&mut self, mut lookahead: usize) -> usize {
        if !self.nth_is(lookahead, Token::At) {
            return lookahead;
        }
        lookahead += 2; // skip @ and identifier

        // skip full name
        while self.nth_is(lookahead, Token::Dot) {
            lookahead += 2;
        }
        let mut nesting = 0;
        loop {
            match self.peek_n(lookahead) {
                Ok(Token::EOF) => break,
                Ok(Token::LeftParen) => nesting += 1,
                Ok(Token::RightParen) => {
                    nesting -= 1;
                    if nesting == 0 {
                        break;
                    }
                }
                Err(_) => break,
                _ => {}
            }
        }
        lookahead
    }

    /// ```text
    /// switch_case_pattern_label:
    ///     pattern {, pattern} [guard]
    /// ```
    fn switch_case_pattern_label(&mut self) -> Result<SwitchLabel, ParseError> {
        let patterns = self.delimited_at_least_1(Self::pattern, Self::comma)?;
        let guard = if peek!(self, 0 => Token::Id(s) if s.as_str() == "when") {
            self.next()?;
            Some(self.expression()?)
        } else {
            None
        };
        Ok(SwitchLabel::Pattern { patterns, guard })
    }

    /// ```text
    /// component_pattern_list:
    ///     component_pattern {, component_pattern}
    /// ```
    fn record_component_pattern_list(&mut self) -> Result<ComponentPatternList, ParseError> {
        self.delimited_at_least_1(Self::record_component_pattern, Self::comma)
    }

    /// ```text
    /// component_pattern:
    ///     pattern
    ///     _
    /// ```
    fn record_component_pattern(&mut self) -> Result<ComponentPattern, ParseError> {
        if self.accept(Token::Underscore) {
            Ok(ComponentPattern::MatchAll)
        } else {
            Ok(ComponentPattern::Pattern(self.pattern()?))
        }
    }

    /// ```text
    /// pattern:
    ///     local_varaiable_declaration
    ///     reference_type ( component_pattern_list )
    /// ```
    fn pattern(&mut self) -> Result<Pattern, ParseError> {
        let modifiers = self.modifiers(ModifierKind::VARIABLE);
        let type_term = self.type_term()?;
        if self.accept(Token::LeftParen) {
            let reference_type = type_term.try_into().map_err(|_| ParseError::NoProduction)?;
            let components = self.record_component_pattern_list()?;
            self.assert(Token::RightParen)?;
            return Ok(Pattern::Record { reference_type, components });
        }

        if let Ok(var_id) = self.variable_declarator_id() {
            let variable_type = type_term.try_into().map_err(|_| ParseError::NoProduction)?;
            let declarators = vec![VariableDeclarator {
                name: var_id,
                initializer: None,
            }];
            let var_declaration =
                VariableDeclaration { variable_type, declarators }.with_modifiers(modifiers);

            return Ok(Pattern::Type(var_declaration));
        }

        Err(ParseError::NoProduction)
    }
}

enum ForHeader {
    BasicForHeader {
        initializer: ForInit,
        condition: Option<Expression>,
        update: ForUpdate,
    },
    ForEachHeader {
        variable_declaration: Modified<VariableDeclaration>,
        iterable: Expression,
    },
}

macro_rules! bitflag_combination {
    ($($n:ident),+ $(,)?) => {
        0 $(
            | Self::$n.bits()
        )+
    };
}

bitflags! {
    #[derive(Copy, Clone)]
    struct ModifierKind: u8 {
        const CLASS         = 1 << 0;
        const METHOD        = 1 << 1;
        const FIELD         = 1 << 2;
        const VARIABLE      = 1 << 3;
        const INTERFACE     = 1 << 4;
        const ANNOTATION    = 1 << 5;
        const CLASS_MEMBER = bitflag_combination!(CLASS, INTERFACE, FIELD, METHOD);
    }
}

impl From<LexError> for ParseError {
    fn from(_e: LexError) -> Self {
        ParseError::NoProduction
    }
}

impl From<Identifier> for Type {
    fn from(value: Identifier) -> Self {
        Self::Class(vec![ClassTypePart { identifier: value }])
    }
}

impl TryFrom<Expression> for Identifier {
    type Error = ParseError;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::Name(id) => Ok(id),
            _ => Err(ParseError::NoProduction),
        }
    }
}

impl TryFrom<Expression> for Type {
    type Error = ParseError;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::Type(t) => Ok(t),
            Expression::Name(n) => Ok(Self::from(n)),
            Expression::MemberAccess(MemberAccess { target, name }) => {
                match Self::try_from(*target)? {
                    Type::Class(mut ct) => {
                        ct.push(ClassTypePart { identifier: name });
                        Ok(Type::Class(ct))
                    }
                    _ => Err(ParseError::NoProduction),
                }
            }
            _ => Err(ParseError::NoProduction),
        }
    }
}

impl Into<TopLevelClassOrInterfaceDeclaration> for ClassDeclaration {
    fn into(self) -> TopLevelClassOrInterfaceDeclaration {
        TopLevelClassOrInterfaceDeclaration::Class(self)
    }
}

impl Into<TopLevelClassOrInterfaceDeclaration> for InterfaceDeclaration {
    fn into(self) -> TopLevelClassOrInterfaceDeclaration {
        TopLevelClassOrInterfaceDeclaration::Interface(self)
    }
}

impl Into<ClassDeclaration> for NormalClassDeclaration {
    fn into(self) -> ClassDeclaration {
        ClassDeclaration::NormalClass(self)
    }
}

impl Into<ClassDeclaration> for RecordDeclaration {
    fn into(self) -> ClassDeclaration {
        ClassDeclaration::Record(self)
    }
}

impl Into<ClassDeclaration> for EnumDeclaration {
    fn into(self) -> ClassDeclaration {
        ClassDeclaration::Enum(self)
    }
}

impl Into<ClassBodyDeclaration> for Modified<ClassMemberDeclaration> {
    fn into(self) -> ClassBodyDeclaration {
        ClassBodyDeclaration::ClassMember(self)
    }
}

impl Into<InterfaceDeclaration> for NormalInterfaceDeclaration {
    fn into(self) -> InterfaceDeclaration {
        InterfaceDeclaration::NormalInterface(self)
    }
}

impl Into<InterfaceDeclaration> for AnnotationInterfaceDeclaration {
    fn into(self) -> InterfaceDeclaration {
        InterfaceDeclaration::AnnotationInterface(self)
    }
}

impl Into<ClassMemberDeclaration> for ClassDeclaration {
    fn into(self) -> ClassMemberDeclaration {
        ClassMemberDeclaration::NestedClass(self)
    }
}

impl Into<ClassMemberDeclaration> for InterfaceDeclaration {
    fn into(self) -> ClassMemberDeclaration {
        ClassMemberDeclaration::NestedInterface(self)
    }
}

impl Into<ClassMemberDeclaration> for MethodDeclaration {
    fn into(self) -> ClassMemberDeclaration {
        ClassMemberDeclaration::Method(self)
    }
}

impl Into<Expression> for Type {
    fn into(self) -> Expression {
        Expression::Type(self)
    }
}

impl Into<Expression> for ArrayAccess {
    fn into(self) -> Expression {
        Expression::ArrayAccess(self)
    }
}

impl From<ArrayType> for Type {
    fn from(value: ArrayType) -> Self {
        Type::Array(value)
    }
}

impl TryFrom<Expression> for LeftHandSide {
    type Error = ParseError;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::Name(id) => Ok(LeftHandSide::ExpressionName(id)),
            Expression::MemberAccess(member_access) => {
                Ok(LeftHandSide::MemberAccess(member_access))
            }
            Expression::ArrayAccess(array_access) => Ok(LeftHandSide::ArrayAccess(array_access)),
            _ => Err(ParseError::NoProduction),
        }
    }
}

impl Into<VariableInitializer> for Expression {
    fn into(self) -> VariableInitializer {
        VariableInitializer::Expression(self)
    }
}

impl Into<ElementValue> for Expression {
    fn into(self) -> ElementValue {
        ElementValue::ConditionalExpression(self)
    }
}

impl Into<ElementValue> for ElementValueList {
    fn into(self) -> ElementValue {
        ElementValue::ElementValueList(self)
    }
}

impl Into<ElementValue> for Annotation {
    fn into(self) -> ElementValue {
        ElementValue::Annotation(Box::new(self))
    }
}

impl From<Switch> for Statement {
    fn from(value: Switch) -> Self {
        Statement::Switch(value)
    }
}

impl From<Switch> for Expression {
    fn from(value: Switch) -> Self {
        Expression::Switch(Box::new(value))
    }
}

impl From<Expression> for SwitchRule {
    fn from(value: Expression) -> Self {
        SwitchRule::Expression(value)
    }
}

impl From<BlockStatements> for SwitchRule {
    fn from(value: BlockStatements) -> Self {
        SwitchRule::Block(value)
    }
}

impl TryFrom<Statement> for SwitchRule {
    type Error = ParseError;
    fn try_from(value: Statement) -> Result<Self, Self::Error> {
        match value {
            Statement::Throw(_) => Ok(SwitchRule::Throw(value)),
            _ => Err(ParseError::NoProduction),
        }
    }
}
