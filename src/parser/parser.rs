use crate::LexicalError;
use crate::ast::{
    Annotation, AnnotationInterfaceDeclaration, ArgumentList, ArrayAccess, ArrayCreationMode,
    ArrayType, AssignmentOp, BinOp, BlockStatements, CatchClause, ClassBodyDeclaration,
    ClassBodyDeclarations, ClassDeclaration, ClassMemberDeclaration, ClassType, TypeList,
    ClassTypePart, ClassTypePartList, CompilationUnit, ComponentPattern, ComponentPatternList,
    ConstructorBody, ConstructorInvocation, ElementValue, ElementValueList, ElementValuePair,
    EnumBody, EnumConstant, EnumDeclaration, Expression, ForInit, ForUpdate, FormalParameter,
    FormalParameterList, Identifier, InterfaceDeclaration, LeftHandSide, MemberAccess, MethodBody,
    MethodCall, MethodDeclaration, MethodReferenceType, Modifiable, Modified, Modifier,
    NormalClassDeclaration, NormalInterfaceDeclaration, Pattern, Program, RecordBodyDeclaration,
    RecordComponent, RecordDeclaration, Resource, Statement, Switch, SwitchBlockMember,
    SwitchBlockMembers, SwitchLabel, SwitchRule, TopLevelClassOrInterfaceDeclaration, Type,
    TypeIdentifier, VariableDeclaration, VariableDeclarator, VariableDeclaratorId,
    VariableDeclaratorList, VariableInitializer, VariableInitializerList,
};
use crate::collections::{AtLeastOne, Multiple, NonEmptyList};
use crate::lexer::{Symbol, Token, Tokens};
use crate::parser::error::{Error, Failure, ParseResult, ResultExtension, SyntaxKind};

use bitflags::bitflags;
use std::collections::VecDeque;
use std::vec;

macro_rules! accept_with_value {
    ($self:expr, $variant:path) => {{
        match $self.peek() {
            Ok($variant(_)) => {
                let Ok($variant(v)) = $self.next() else {
                    unreachable!()
                };
                Ok(v)
            }
            Ok(_) => Err(Failure::NoProduction),
            Err(e) => Err(e.into()),
        }
    }};

    ($self:expr, $($token:expr => $result:expr),+ $(,)?) => {{
        $(
            if $self.accept($token) {
                Ok($result)
            } else
        )+
        {
            Err(Failure::NoProduction)
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
        let mut res = Err(Failure::NoProduction);
        $(
            res = match res {
                Ok(_) => res,
                Err(Failure::Error(_)) => res,
                Err(Failure::NoProduction) => $x
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
        Err(Failure::NoProduction)
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

    pub fn parse(&mut self) -> Result<Program, Error> {
        self.compilation_unit()
    }

    fn next(&mut self) -> Result<Token, LexicalError> {
        if let Some(tok) = self.buffer.pop_front() {
            return Ok(tok);
        }

        self.tokens.next()
    }

    fn peek(&mut self) -> Result<&Token, LexicalError> {
        self.peek_n(0)
    }

    fn peek_n(&mut self, skip: usize) -> Result<&Token, LexicalError> {
        while self.buffer.len() <= skip {
            self.buffer.push_back(self.tokens.next()?)
        }
        Ok(&self.buffer[skip])
    }

    fn next_is(&mut self, desired: Symbol) -> bool {
        self.nth_is(0, desired)
    }

    fn nth_is(&mut self, n: usize, desired: Symbol) -> bool {
        match self.peek_n(n) {
            Ok(Token::Symbol(s)) if *s == desired => true,
            _ => false,
        }
    }

    fn accept(&mut self, desired: Symbol) -> bool {
        let matches = self.next_is(desired);
        if matches {
            self.next().unwrap();
        }
        matches
    }

    fn integer_literal(&mut self) -> ParseResult<u64> {
        accept_with_value!(self, Token::IntegerLiteral)
    }

    fn long_literal(&mut self) -> ParseResult<u64> {
        accept_with_value!(self, Token::LongLiteral)
    }

    fn boolean_literal(&mut self) -> ParseResult<bool> {
        accept_with_value!(self, Token::BooleanLiteral)
    }

    fn char_literal(&mut self) -> ParseResult<char> {
        accept_with_value!(self, Token::CharLiteral)
    }

    fn string_literal(&mut self) -> ParseResult<String> {
        accept_with_value!(self, Token::StringLiteral)
    }

    fn expect(&mut self, desired: Symbol) -> ParseResult<()> {
        if self.accept(desired) {
            Ok(())
        } else {
            Err(Failure::NoProduction)
        }
    }

    fn assert(&mut self, desired: Symbol) -> ParseResult<()> {
        if self.accept(desired.clone()) {
            Ok(())
        } else {
            Err(Error::SymbolExpected(desired).into())
        }
    }

    fn opt<T>(
        &mut self,
        cond: impl Fn(&mut Self) -> bool,
        element: impl Fn(&mut Self) -> ParseResult<T>,
    ) -> ParseResult<Option<T>> {
        if cond(self) { Ok(Some(element(self)?)) } else { Ok(None) }
    }

    fn zero_or_more<T>(
        &mut self,
        next: impl Fn(&mut Self) -> ParseResult<T>,
    ) -> Result<Vec<T>, Error> {
        let mut v = Vec::new();
        loop {
            match next(self) {
                Ok(elem) => v.push(elem),
                Err(Failure::NoProduction) => return Ok(v),
                Err(Failure::Error(cause)) => return Err(cause),
            }
        }
    }

    /// ```text
    /// [next {delim next}]
    /// ```
    fn delimited_list<T>(
        &mut self,
        next: impl Fn(&mut Self) -> ParseResult<T>,
        delim: Symbol,
    ) -> ParseResult<Vec<T>> {
        let mut list = match next(self) {
            Ok(elem) => vec![elem],
            Err(Failure::NoProduction) => return Ok(Vec::new()),
            Err(Failure::Error(err)) => return Err(err.into()),
        };
        loop {
            if !self.accept(delim) {
                break;
            }
            let elem = next(self).assert(Error::MissingElementAfterDelimiter(delim))?;
            list.push(elem);
        }
        Ok(list)
    }

    /// ```text
    /// next {delim next}
    /// ```
    fn delimited_at_least_1<T>(
        &mut self,
        next: impl Fn(&mut Self) -> ParseResult<T>,
        delim: Symbol,
    ) -> ParseResult<AtLeastOne<T>> {
        match self.delimited_list(next, delim) {
            Ok(l) => NonEmptyList::from_vec(l).map_err(|_| Failure::NoProduction),
            Err(e) => Err(e),
        }
    }

    fn compilation_unit(&mut self) -> Result<CompilationUnit, Error> {
        self.ordinary_compilation_unit()
    }

    fn ordinary_compilation_unit(&mut self) -> Result<CompilationUnit, Error> {
        let top_level_class_or_interface_declarations =
            self.zero_or_more(Self::top_level_class_or_interface_declaration)?;
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
    ) -> ParseResult<Modified<TopLevelClassOrInterfaceDeclaration>> {
        while self.accept(Symbol::Semicolon) {} // §7.6 (p. 231), ignore semicolons at class or interface declarations level

        let modifiers = self.modifiers(ModifierKind::CLASS | ModifierKind::INTERFACE)?;
        one_of!(
            self.class_declaration().map(ClassDeclaration::into),
            self.interface_declaration().map(InterfaceDeclaration::into)
        )
        .map(|d: TopLevelClassOrInterfaceDeclaration| d.with_modifiers(modifiers))
    }

    fn class_declaration(&mut self) -> ParseResult<ClassDeclaration> {
        one_of!(
            self.normal_class_declaration()
                .map(NormalClassDeclaration::into),
            self.record_declaration().map(RecordDeclaration::into),
            self.enum_declaration().map(EnumDeclaration::into),
        )
    }

    fn normal_class_declaration(&mut self) -> ParseResult<NormalClassDeclaration> {
        self.expect(Symbol::Class)?;
        let identifier = self
            .identifier()
            .assert(Error::IdentifierExpected)?
            .try_into()?;
        let extends = self.opt_class_extends()?;
        let implements = self.opt_class_implements()?;
        let permits = self.opt_class_permits()?;
        let body = self.class_body().assert(Error::MissingClassBody)?;
        let class_decl = NormalClassDeclaration {
            identifier,
            extends,
            implements,
            permits,
            body,
        };
        Ok(class_decl)
    }

    fn opt_class_extends(&mut self) -> ParseResult<Option<Type>> {
        self.opt(|this| this.accept(Symbol::Extends), Self::type_term)
    }

    fn opt_class_implements(&mut self) -> ParseResult<Option<TypeList>> {
        self.opt(
            |this| this.accept(Symbol::Implements),
            |this| this.delimited_at_least_1(Self::type_term, Symbol::Comma),
        )
    }

    fn opt_class_permits(&mut self) -> ParseResult<Option<TypeList>> {
        self.opt(
            |this| {
                let permits = peek!(this, 0 => Token::Id(s) if s.as_str() == "permits");
                if permits {
                    this.next().unwrap();
                }
                permits
            },
            |this| this.delimited_at_least_1(Self::type_term, Symbol::Comma),
        )
    }

    fn modifier(&mut self, modifier_kind: ModifierKind) -> ParseResult<Modifier> {
        one_of_opt!(
            self.accept(Symbol::Public).then_some(Modifier::Public),
            self.accept(Symbol::Private).then_some(Modifier::Private),
            self.accept(Symbol::Protected)
                .then_some(Modifier::Protected),
            self.accept(Symbol::Abstract).then_some(Modifier::Abstract),
            (!self.nth_is(1, Symbol::LeftBrace) && self.accept(Symbol::Static))
                .then_some(Modifier::Static),
            self.accept(Symbol::Final).then_some(Modifier::Final),
            (modifier_kind.contains(ModifierKind::METHOD) && self.accept(Symbol::Default))
                .then_some(Modifier::Default),
            self.accept(Symbol::Strictfp).then_some(Modifier::Strictfp),
            self.accept(Symbol::Native).then_some(Modifier::Native),
            self.accept(Symbol::Transient)
                .then_some(Modifier::Transient),
            self.accept(Symbol::Volatile).then_some(Modifier::Volatile),
            (!self.nth_is(1, Symbol::LeftParen) && self.accept(Symbol::Synchronized))
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

    fn modifiers(&mut self, modifier_kind: ModifierKind) -> Result<Vec<Modifier>, Error> {
        self.zero_or_more(|this| this.modifier(modifier_kind))
    }

    fn is_sealed_modifier(&mut self, start: usize) -> bool {
        peek!(self, start => Token::Id(s) if s.as_str() == "sealed")
    }

    fn is_non_sealed_modifier(&mut self, start: usize) -> bool {
        peek!(self,
                start => Token::Id(s) if s.as_str() == "non",
                start + 1 => symbol!(Minus),
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
            next_token_start => Token::Symbol(
                Symbol::Public
                | Symbol::Protected
                | Symbol::Private
                | Symbol::Abstract
                | Symbol::Static
                | Symbol::Final
                | Symbol::Strictfp
                | Symbol::At)
        );
        let sealed_modifier = self.is_sealed_modifier(next_token_start);
        let non_sealed_modifier = self.is_non_sealed_modifier(next_token_start);
        let class_or_enum_or_record_or_interface = self.nth_is(next_token_start, Symbol::Class)
            || self.nth_is(next_token_start, Symbol::Enum)
            || peek!(self, next_token_start => Token::Id(s) if s.as_str() == "record")
            || self.nth_is(next_token_start, Symbol::Interface);
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
    fn annotation(&mut self) -> ParseResult<Annotation> {
        if !self.next_is(Symbol::At) || self.nth_is(1, Symbol::Interface) {
            // to differentiate from annotation interface declaration
            return Err(Failure::NoProduction);
        }
        self.expect(Symbol::At)?;
        let name = self
            .delimited_at_least_1(Self::identifier, Symbol::Dot)
            .assert(Error::IdentifierExpected)?;
        if !self.accept(Symbol::LeftParen) {
            return Ok(Annotation::Marker(name));
        }
        if self.accept(Symbol::RightParen) || peek!(self, 0 => Token::Id(_), 1 => symbol!(Assign)) {
            let values = self.delimited_list(Self::element_value_pair, Symbol::Comma)?;
            self.assert(Symbol::RightParen)?;
            return Ok(Annotation::Normal { name, values });
        }
        let value = self.element_value()?;
        self.assert(Symbol::RightParen)?;
        Ok(Annotation::SingleElement { name, value })
    }

    /// ```text
    /// element_value_pair:
    ///     identifier = element_value
    /// ```
    fn element_value_pair(&mut self) -> ParseResult<ElementValuePair> {
        let name = self.identifier()?;
        self.assert(Symbol::Assign)?;
        let value = self.element_value()?;
        Ok(ElementValuePair { name, value })
    }

    /// ```text
    /// element_value:
    ///     conditional_expression
    ///     element_value_array_initializer
    ///     annotation
    /// ```
    fn element_value(&mut self) -> ParseResult<ElementValue> {
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
    fn element_value_array_initializer(&mut self) -> ParseResult<ElementValueList> {
        self.expect(Symbol::LeftBrace)?;
        let elements = self.element_value_list()?;
        self.assert(Symbol::RightBrace)?;
        Ok(elements)
    }

    /// ```text
    /// element_value_list:
    ///     [,]
    ///     element_value {, element_value} [,]
    /// ```
    fn element_value_list(&mut self) -> ParseResult<ElementValueList> {
        if self.accept(Symbol::Comma) {
            // just a single comma
            return Ok(vec![]);
        }

        let mut items = vec![];
        loop {
            if self.next_is(Symbol::RightBrace) {
                break;
            }
            items.push(self.element_value()?);
            if !self.accept(Symbol::Comma) {
                break;
            }
        }
        Ok(items)
    }

    fn identifier(&mut self) -> ParseResult<Identifier> {
        accept_with_value!(self, Token::Id)
    }

    fn class_body(&mut self) -> ParseResult<ClassBodyDeclarations> {
        self.expect(Symbol::LeftBrace)?;
        let declarations = self.zero_or_more(Self::class_body_declaration)?;
        self.assert(Symbol::RightBrace)?;
        Ok(declarations)
    }

    fn class_body_declaration(&mut self) -> ParseResult<ClassBodyDeclaration> {
        one_of!(
            self.class_member_declaration().map(Modified::into),
            self.instance_initializer()
                .map(|v| ClassBodyDeclaration::InstanceInitializer(v)),
            self.static_initializer()
                .map(|v| ClassBodyDeclaration::StaticInitializer(v)),
        )
    }

    fn instance_initializer(&mut self) -> ParseResult<BlockStatements> {
        self.block()
    }

    fn static_initializer(&mut self) -> ParseResult<BlockStatements> {
        if !peek!(
            self,
            0 => symbol!(Static),
            1 => symbol!(LeftBrace),
        ) {
            Err(Failure::NoProduction)
        } else {
            self.assert(Symbol::Static)?;
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
    fn class_member_declaration(&mut self) -> ParseResult<Modified<ClassMemberDeclaration>> {
        while self.accept(Symbol::Semicolon) {} // ignore stray semicolons
        let modifiers = self.modifiers(ModifierKind::CLASS_MEMBER)?;

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
    fn record_declaration(&mut self) -> ParseResult<RecordDeclaration> {
        if !peek!(
            self,
            0 => Token::Id(s) if s.as_str() == "record",
            1 => Token::Id(_),
        ) {
            return Err(Failure::NoProduction);
        }
        accept_with_value!(self, Token::Id)?;
        let name = self.identifier()?.try_into()?;
        self.assert(Symbol::LeftParen)?;
        let components = self.delimited_list(Self::record_component, Symbol::Comma)?;
        self.assert(Symbol::RightParen)?;
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
    fn record_component(&mut self) -> ParseResult<Modified<RecordComponent>> {
        let annotations = self.zero_or_more(|this| this.annotation().map(Annotation::into))?;
        let component_type = self.type_term()?;
        if self.accept(Symbol::Ellipsis) {
            let name = self.identifier()?;
            Ok(RecordComponent::VariableArity { component_type, name }.with_modifiers(annotations))
        } else {
            let name = self.identifier()?;
            Ok(RecordComponent::Normal { component_type, name }.with_modifiers(annotations))
        }
    }

    fn record_body(&mut self) -> ParseResult<Vec<RecordBodyDeclaration>> {
        self.class_body()
    }

    fn enum_declaration(&mut self) -> ParseResult<EnumDeclaration> {
        self.expect(Symbol::Enum)?;
        let name = self.identifier()?.try_into()?;
        let implements = self.opt_class_implements()?;
        let body = self.enum_body()?;
        Ok(EnumDeclaration { name, implements, body })
    }

    /// ```text
    /// enum_body:
    ///     { enum_constant_list [enum_body_declarations] }
    /// ```
    fn enum_body(&mut self) -> ParseResult<EnumBody> {
        self.expect(Symbol::LeftBrace)?;
        let constants = self.enum_constant_list()?;
        let body_declarations = if self.accept(Symbol::Semicolon) {
            self.zero_or_more(Self::class_body_declaration)?
        } else {
            vec![]
        };
        self.assert(Symbol::RightBrace)?;
        Ok(EnumBody { constants, body_declarations })
    }

    /// ```text
    /// enum_constant_list:
    ///     [,]
    ///     enum_constant {, enum_constant} [,]
    /// ```
    fn enum_constant_list(&mut self) -> ParseResult<Vec<Modified<EnumConstant>>> {
        if self.accept(Symbol::Comma) {
            // just a single comma
            return Ok(vec![]);
        }

        let mut items = vec![];
        loop {
            // enum constants list ends either with the end of the enum (right brace) or the semicolon
            // that starts the enum body declarations
            if self.next_is(Symbol::RightBrace) || self.next_is(Symbol::Semicolon) {
                break;
            }
            items.push(self.enum_constant()?);
            if !self.accept(Symbol::Comma) {
                break;
            }
        }
        Ok(items)
    }

    /// ```text
    /// enum_constant:
    ///     {annotation} identifier [( argument_list )] [class_body]
    /// ```
    fn enum_constant(&mut self) -> ParseResult<Modified<EnumConstant>> {
        let annotations = self.zero_or_more(|this| this.annotation().map(Annotation::into))?;
        let name = self.identifier()?;
        let args = if self.accept(Symbol::LeftParen) {
            let args = self.argument_list()?;
            self.assert(Symbol::RightParen)?;
            Some(args)
        } else {
            None
        };
        let body = if self.next_is(Symbol::LeftBrace) {
            Some(self.class_body()?)
        } else {
            None
        };
        Ok(EnumConstant { name, args, body }.with_modifiers(annotations))
    }

    fn interface_declaration(&mut self) -> ParseResult<InterfaceDeclaration> {
        one_of!(
            self.normal_interface_declaration()
                .map(NormalInterfaceDeclaration::into),
            self.annotation_interface_declaration()
                .map(AnnotationInterfaceDeclaration::into),
        )
    }

    fn normal_interface_declaration(&mut self) -> ParseResult<NormalInterfaceDeclaration> {
        self.expect(Symbol::Interface)?;
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

    fn opt_interface_extends(&mut self) -> ParseResult<Option<TypeList>> {
        self.opt(
            |this| this.accept(Symbol::Extends),
            |this| this.delimited_at_least_1(Self::type_term, Symbol::Comma),
        )
    }

    fn interface_body(&mut self) -> ParseResult<Vec<Modified<ClassMemberDeclaration>>> {
        self.expect(Symbol::LeftBrace)?;
        let members = self.zero_or_more(Self::class_member_declaration)?;
        self.assert(Symbol::RightBrace)?;
        Ok(members)
    }

    fn annotation_interface_declaration(&mut self) -> ParseResult<AnnotationInterfaceDeclaration> {
        if !peek!(self, 0 => symbol!(At), 1 => symbol!(Interface)) {
            return Err(Failure::NoProduction);
        }
        self.expect(Symbol::At)?;
        self.expect(Symbol::Interface)?;
        let name = self
            .identifier()
            .assert(Error::IdentifierExpected)?
            .try_into()?;
        self.assert(Symbol::LeftBrace)?;
        let body = self.zero_or_more(Self::class_member_declaration)?;
        self.assert(Symbol::RightBrace)?;
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
    fn constructor_declaration(&mut self) -> ParseResult<ClassMemberDeclaration> {
        if peek!(self, 0 => Token::Id(_) , 1 => symbol!(LeftParen | LeftBrace)) {
            let name = self.identifier()?.try_into()?;
            if self.accept(Symbol::LeftParen) {
                let parameters = self.formal_parameters()?;
                self.assert(Symbol::RightParen)?;
                let throws = self.opt_throws()?;
                let body = self.constructor_body()?;
                Ok(ClassMemberDeclaration::Constructor { name, parameters, throws, body })
            } else {
                let body = self.constructor_body()?;
                Ok(ClassMemberDeclaration::CompactConstructor { name, body })
            }
        } else {
            Err(Failure::NoProduction)
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
    fn method_or_field_declaration(&mut self) -> ParseResult<ClassMemberDeclaration> {
        let type_term = self.type_term()?;
        let identifier = self.identifier().assert(Error::IdentifierExpected)?;
        if self.accept(Symbol::LeftParen) {
            let parameters = self.formal_parameters()?;
            self.assert(Symbol::RightParen)?;
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
            let mut field_declaration = NonEmptyList::new(VariableDeclarator {
                name: VariableDeclaratorId::Named(identifier),
                initializer: self
                    .variable_declarator_initializer()
                    .map(|i| Some(i))
                    .or_else(|e| match e {
                        Failure::NoProduction => Ok(None),
                        Failure::Error(e) => Err(e),
                    })?,
            });
            if self.accept(Symbol::Comma) {
                field_declaration.append(self.variable_declarators_list()?);
            }
            self.assert(Symbol::Semicolon)?;
            Ok(ClassMemberDeclaration::Field {
                variable_type: type_term,
                declarations: field_declaration,
            })
        }
    }

    fn formal_parameters(&mut self) -> ParseResult<FormalParameterList> {
        self.delimited_list(Self::formal_parameter, Symbol::Comma)
    }

    fn formal_parameter(&mut self) -> ParseResult<Modified<FormalParameter>> {
        let modifiers = self.modifiers(ModifierKind::VARIABLE)?;
        let param_type = self.type_term()?;
        if self.accept(Symbol::Ellipsis) {
            // variable arity
            let identifier = self.identifier().assert(Error::IdentifierExpected)?;
            Ok(FormalParameter::VariableArityParameter(param_type, identifier)
                .with_modifiers(modifiers))
        } else {
            let identifier = self.identifier().assert(Error::IdentifierExpected)?;
            Ok(FormalParameter::NormalParameter(
                param_type,
                VariableDeclaratorId::Named(identifier),
            )
            .with_modifiers(modifiers))
        }
    }

    fn primitive_type(&mut self) -> ParseResult<Type> {
        if self.accept(Symbol::Byte) {
            Ok(Type::Byte)
        } else if self.accept(Symbol::Short) {
            Ok(Type::Short)
        } else if self.accept(Symbol::Int) {
            Ok(Type::Int)
        } else if self.accept(Symbol::Long) {
            Ok(Type::Long)
        } else if self.accept(Symbol::Char) {
            Ok(Type::Char)
        } else if self.accept(Symbol::Float) {
            Ok(Type::Float)
        } else if self.accept(Symbol::Double) {
            Ok(Type::Double)
        } else if self.accept(Symbol::Boolean) {
            Ok(Type::Boolean)
        } else if self.accept(Symbol::Void) {
            Ok(Type::Void)
        } else {
            Err(Failure::NoProduction)
        }
    }

    fn opt_throws(&mut self) -> ParseResult<Multiple<Modified<Type>>> {
        if self.accept(Symbol::Throws) {
            self.delimited_at_least_1(
                |this| {
                    let modifiers = this.modifiers(ModifierKind::VARIABLE)?;
                    Ok(this.reference_type()?.with_modifiers(modifiers))
                },
                Symbol::Comma,
            )
            .map(|non_empty| non_empty.into())
        } else {
            Ok(vec![])
        }
    }

    /// ```text
    /// default_value:
    ///     default element_value
    /// ```
    fn opt_default(&mut self) -> ParseResult<Option<ElementValue>> {
        self.opt(|this| this.accept(Symbol::Default), Self::element_value)
    }

    fn method_body(&mut self) -> ParseResult<MethodBody> {
        if self.accept(Symbol::Semicolon) {
            return Ok(MethodBody::Semicolon);
        }
        let statements = self
            .block()
            .assert(Error::SymbolExpected2(Symbol::Semicolon, Symbol::LeftBrace))?;
        Ok(MethodBody::Block(statements))
    }

    /// ```text
    /// block:
    ///     [ block_statements ]
    ///
    /// block_statements:
    ///     {block_statement}
    /// ```
    fn block(&mut self) -> ParseResult<BlockStatements> {
        self.expect(Symbol::LeftBrace)?;
        let block_statements = self.zero_or_more(Self::block_statement)?;
        self.assert(Symbol::RightBrace)?;
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
    fn block_statement(&mut self) -> ParseResult<Statement> {
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
    fn local_variable_declaration_or_statement(&mut self) -> ParseResult<Statement> {
        one_of!(
            self.empty_statement(),
            self.block().map(|v| Statement::Block(v)),
            self.simple_statement(),
            self.statement_starting_with_name(),
        )
    }

    fn empty_statement(&mut self) -> ParseResult<Statement> {
        self.expect(Symbol::Semicolon)?;
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
    fn simple_statement(&mut self) -> ParseResult<Statement> {
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
    fn statement_starting_with_name(&mut self) -> ParseResult<Statement> {
        let modifiers = self.modifiers(ModifierKind::VARIABLE)?;
        let expression = self.term()?;

        if self.accept(Symbol::Colon) {
            return match expression {
                Expression::Name(id) => {
                    let body = Box::new(self.block_statement()?);
                    Ok(Statement::Labeled { label: id, body })
                }
                _ => Err(Failure::NoProduction),
            };
        }

        if self.accept(Symbol::Semicolon) {
            return Ok(Statement::ExpressionStatement(expression));
        }

        let var_declarations = self.variable_declarators_list()?;
        self.assert(Symbol::Semicolon)?;
        Ok(Statement::VariableDeclaration(
            VariableDeclaration {
                variable_type: expression.try_into()?,
                declarators: var_declarations,
            }
            .with_modifiers(modifiers),
        ))
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
    fn term(&mut self) -> ParseResult<Expression> {
        let expr = self.conditional_expression()?;
        if let Ok(op) = accept_with_value!(self,
            Symbol::Assign => AssignmentOp::Identity,
            Symbol::AddAssign => AssignmentOp::Add,
            Symbol::SubAssign=> AssignmentOp::Subtract,
            Symbol::MulAssign => AssignmentOp::Multiply,
            Symbol::DivAssign => AssignmentOp::Divide,
            Symbol::ModAssign => AssignmentOp::Modulo,
            Symbol::LeftShiftAssign => AssignmentOp::LeftShift,
            Symbol::SignedRightShiftAssign => AssignmentOp::SignedRightShift,
            Symbol::UnsignedRightShiftAssign => AssignmentOp::UnsignedRightShift,
            Symbol::AndAssign => AssignmentOp::BitwiseAnd,
            Symbol::XorAssign => AssignmentOp::BitwiseXor,
            Symbol::OrAssign => AssignmentOp::BitwiseOr,
        ) {
            let lhs = expr.try_into()?;
            let rhs = self
                .term()
                .assert(Error::SyntaxExpectedAfter(SyntaxKind::Expression, Symbol::Assign))?;
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

    fn expression(&mut self) -> ParseResult<Expression> {
        self.term()
    }

    /// ```text
    /// conditional_expression:
    ///     conditional_or_expression [? expression : conditional_expression]
    /// ```
    fn conditional_expression(&mut self) -> ParseResult<Expression> {
        let condition = self.conditional_or_expression()?;
        if self.accept(Symbol::QuestionMark) {
            let if_true = self
                .expression()
                .assert(Error::SyntaxExpectedAfter(SyntaxKind::Expression, Symbol::QuestionMark))?;
            self.assert(Symbol::Colon)?;
            let if_false = self
                .conditional_expression()
                .assert(Error::SyntaxExpectedAfter(SyntaxKind::Expression, Symbol::Colon))?;
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
    ) -> ParseResult<Expression>
    where
        F: Fn(&mut Self) -> ParseResult<Expression>,
        G: Fn(&mut Self) -> ParseResult<BinOp>,
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

    fn conditional_or_expression(&mut self) -> ParseResult<Expression> {
        self.left_associative_binary_operation(Self::conditional_and_expression, |this| {
            accept_with_value!(this,
                Symbol::LogicalOr => BinOp::LogicalOr
            )
        })
    }

    fn conditional_and_expression(&mut self) -> ParseResult<Expression> {
        self.left_associative_binary_operation(Self::inclusive_or_expression, |this| {
            accept_with_value!(this,
                Symbol::LogicalAnd => BinOp::LogicalAnd
            )
        })
    }

    fn inclusive_or_expression(&mut self) -> ParseResult<Expression> {
        self.left_associative_binary_operation(Self::exclusive_or_expression, |this| {
            accept_with_value!(this,
                Symbol::BitwiseOr => BinOp::BitwiseOr
            )
        })
    }

    fn exclusive_or_expression(&mut self) -> ParseResult<Expression> {
        self.left_associative_binary_operation(Self::and_expression, |this| {
            accept_with_value!(this,
                Symbol::BitwiseXor => BinOp::BitwiseXor
            )
        })
    }

    fn and_expression(&mut self) -> ParseResult<Expression> {
        self.left_associative_binary_operation(Self::equality_expression, |this| {
            accept_with_value!(this,
                Symbol::BitwiseAnd => BinOp::BitwiseAnd
            )
        })
    }

    fn equality_expression(&mut self) -> ParseResult<Expression> {
        self.left_associative_binary_operation(Self::relational_expression, |this| {
            accept_with_value!(this,
                Symbol::Equals => BinOp::Equal,
                Symbol::NotEquals => BinOp::NotEqual,
            )
        })
    }

    fn relational_expression(&mut self) -> ParseResult<Expression> {
        let mut expr = self.shift_expression()?;

        // not using generic left_associative_binary_operation here
        // because in this case there is another case - the instanceof operator
        // which does not take symmetric operands.
        loop {
            if let Ok(op) = accept_with_value!(self,
                Symbol::LessThan => BinOp::Less,
                Symbol::GreaterThan => BinOp::Greater,
                Symbol::LessThanOrEquals => BinOp::LessEqual,
                Symbol::GreaterThanOrEquals => BinOp::GreaterEqual,
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

    fn shift_expression(&mut self) -> ParseResult<Expression> {
        self.left_associative_binary_operation(Self::additive_expression, |this| {
            accept_with_value!(this,
                Symbol::LeftShift => BinOp::LeftShift,
                Symbol::SignedRightShift => BinOp::SignedRightShift,
                Symbol::UnsignedRightShift => BinOp::UnsignedRightShift,
            )
        })
    }

    fn additive_expression(&mut self) -> ParseResult<Expression> {
        self.left_associative_binary_operation(Self::multiplicative_expression, |this| {
            accept_with_value!(this,
                Symbol::Plus => BinOp::Add,
                Symbol::Minus => BinOp::Subtract,
            )
        })
    }

    fn multiplicative_expression(&mut self) -> ParseResult<Expression> {
        self.left_associative_binary_operation(Self::unary_expression, |this| {
            accept_with_value!(this,
                Symbol::Multiply => BinOp::Multiply,
                Symbol::Divide   => BinOp::Divide,
                Symbol::Modulo   => BinOp::Modulo,
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
    fn unary_expression(&mut self) -> ParseResult<Expression> {
        if let Ok(switch) = self.switch() {
            return Ok(switch.into());
        }
        if self.accept(Symbol::Tilde) {
            Ok(Expression::BitwiseComplement(Box::new(self.unary_expression()?)))
        } else if self.accept(Symbol::ExclamationMark) {
            Ok(Expression::LogicalNot(Box::new(self.unary_expression()?)))
        } else if self.accept(Symbol::Plus) {
            Ok(Expression::UnaryPlus(Box::new(self.unary_expression()?)))
        } else if self.accept(Symbol::Minus) {
            Ok(Expression::UnaryMinus(Box::new(self.unary_expression()?)))
        } else if self.accept(Symbol::Increment) {
            Ok(Expression::PreIncrement(Box::new(self.unary_expression()?)))
        } else if self.accept(Symbol::Decrement) {
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
    fn postfix_expression(&mut self) -> ParseResult<Expression> {
        let mut expr = self.primary()?;
        expr = self.parse_selectors(expr)?;
        if self.accept(Symbol::Increment) {
            expr = Expression::PostIncrement(Box::new(expr));
        } else if self.accept(Symbol::Decrement) {
            expr = Expression::PostDecrement(Box::new(expr));
        }

        Ok(expr)
    }

    fn primary(&mut self) -> ParseResult<Expression> {
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
    fn literal(&mut self) -> ParseResult<Expression> {
        one_of!(
            self.integer_literal()
                .map(|v| Expression::IntegerLiteral(v)),
            self.long_literal().map(|v| Expression::LongLiteral(v)),
            self.boolean_literal()
                .map(|v| Expression::BooleanLiteral(v)),
            self.char_literal().map(|v| Expression::CharLiteral(v)),
            self.string_literal().map(|v| Expression::StringLiteral(v)),
            self.expect(Symbol::NullLiteral)
                .map(|_| Expression::NullLiteral)
        )
    }

    fn parenthesized_expression(&mut self) -> ParseResult<Expression> {
        if self.accept(Symbol::LeftParen) {
            let expr = self.expression()?; // assuming you have this
            self.assert(Symbol::RightParen)?;
            Ok(expr)
        } else {
            Err(Failure::NoProduction)
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
    fn parse_selectors(&mut self, expr: Expression) -> ParseResult<Expression> {
        let mut expr = expr;
        loop {
            if self.accept(Symbol::Dot) {
                if let Ok(id) = accept_with_value!(self, Token::Id) {
                    if self.accept(Symbol::LeftParen) {
                        let arg_list = self.argument_list()?;
                        self.assert(Symbol::RightParen)?;
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
                } else if self.accept(Symbol::Class) {
                    expr = Expression::ClassLiteral(Type::try_from(expr)?)
                } else if self.accept(Symbol::This) {
                    expr = Expression::QualifiedThis(Type::try_from(expr)?)
                } else {
                    return Err(Error::IdentifierExpected.into());
                }
            } else if self.accept(Symbol::LeftBracket) {
                if self.accept(Symbol::RightBracket) {
                    expr = Type::from(ArrayType {
                        element_type: Box::new(Type::try_from(expr)?),
                    })
                    .into()
                } else {
                    let index = self.expression()?;
                    self.assert(Symbol::RightBracket)?;
                    expr = ArrayAccess {
                        target: Box::new(expr),
                        index: Box::new(index),
                    }
                    .into();
                }
            } else if self.accept(Symbol::LeftParen) {
                let arg_list = self.argument_list()?;
                self.assert(Symbol::RightParen)?;
                expr = Expression::MethodCall(MethodCall {
                    target: None,
                    name: expr.try_into()?,
                    arguments: arg_list,
                })
            } else if self.accept(Symbol::DoubleColon) {
                let target = Box::new(expr);
                let method = if self.accept(Symbol::New) {
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

    fn identifier_expression(&mut self) -> ParseResult<Expression> {
        Ok(Expression::Name(self.identifier()?))
    }

    fn variable_declarators_list(&mut self) -> ParseResult<VariableDeclaratorList> {
        self.delimited_at_least_1(
            |this| {
                Ok(VariableDeclarator {
                    name: this.variable_declarator_id()?,
                    initializer: this
                        .variable_declarator_initializer()
                        .map(|i| Some(i))
                        .or_else(|e| match e {
                            Failure::NoProduction => Ok(None),
                            Failure::Error(e) => Err(e),
                        })?,
                })
            },
            Symbol::Comma,
        )
    }

    fn variable_declarator_id(&mut self) -> ParseResult<VariableDeclaratorId> {
        if let Ok(name) = accept_with_value!(self, Token::Id) {
            Ok(VariableDeclaratorId::Named(name))
        } else if self.accept(Symbol::Underscore) {
            Ok(VariableDeclaratorId::Unnamed)
        } else {
            Err(Failure::NoProduction)
        }
    }

    fn variable_declarator_initializer(&mut self) -> ParseResult<VariableInitializer> {
        self.expect(Symbol::Assign)?;
        self.variable_initializer()
            .assert(Error::SyntaxExpectedAfter(SyntaxKind::Expression, Symbol::Assign))
    }

    fn variable_initializer(&mut self) -> ParseResult<VariableInitializer> {
        one_of!(
            self.expression().map(Expression::into),
            self.array_initializer()
                .map(|i| VariableInitializer::ArrayInitializer(i)),
        )
    }

    fn argument_list(&mut self) -> ParseResult<ArgumentList> {
        self.delimited_list(Self::expression, Symbol::Comma)
    }

    /// ```text
    /// constructor_body:
    ///     { {block_statement} [constructor_invocation] {block_statement} }
    ///
    /// constructor_invocation:
    ///     this ( [argument_list] ) ;
    /// ```
    fn constructor_body(&mut self) -> ParseResult<ConstructorBody> {
        self.expect(Symbol::LeftBrace)?;
        let first_part = self.zero_or_more(Self::block_statement)?;
        let constructor_invocation = if self.accept(Symbol::This) {
            self.assert(Symbol::LeftParen)?;
            let arguments = self.argument_list()?;
            self.assert(Symbol::RightParen)?;
            self.assert(Symbol::Semicolon)?;
            Some(ConstructorInvocation::Alternate { arguments })
        } else {
            None
        };
        let (prologue, epilogue) = match constructor_invocation {
            Some(_) => {
                let prologue = if first_part.is_empty() { None } else { Some(first_part) };

                let epilogue = self.zero_or_more(Self::block_statement)?;

                (prologue, epilogue)
            }
            None => {
                // No constructor call → everything is epilogue
                (None, first_part)
            }
        };
        self.assert(Symbol::RightBrace)?;
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
    fn instance_creation_expression(&mut self) -> ParseResult<Expression> {
        self.expect(Symbol::New)?;
        // not using type_term here because we want to get the base type only, without possible brackets
        let type_to_instantiate = one_of!(self.primitive_type(), self.reference_type())?;
        if self.next_is(Symbol::LeftParen) {
            self.class_instance_creation(type_to_instantiate)
        } else if self.next_is(Symbol::LeftBracket) {
            self.array_creation(type_to_instantiate)
        } else {
            Err(Error::SymbolExpected2(Symbol::LeftParen, Symbol::LeftBracket).into())
        }
    }

    fn class_instance_creation(&mut self, type_to_instantiate: Type) -> ParseResult<Expression> {
        self.expect(Symbol::LeftParen)?;
        let arguments = self.argument_list()?;
        self.assert(Symbol::RightParen)?;
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
    fn array_creation(&mut self, mut element_type: Type) -> ParseResult<Expression> {
        // FIXME: can be refactored with peek_n
        self.expect(Symbol::LeftBracket)?;
        let array_creation_mode = if self.accept(Symbol::RightBracket) {
            element_type = Type::from(ArrayType {
                element_type: Box::new(element_type),
            });
            while self.accept(Symbol::LeftBracket) {
                self.assert(Symbol::RightBracket)?;
                element_type = Type::from(ArrayType {
                    element_type: Box::new(element_type),
                });
            }
            let initializer = self.array_initializer()?;
            ArrayCreationMode::Initialized(initializer)
        } else {
            let mut sized_dimensions = vec![self.expression()?];
            let mut unsized_dimensions = 0;
            self.assert(Symbol::RightBracket)?;
            loop {
                if !self.accept(Symbol::LeftBracket) {
                    break;
                }
                if self.accept(Symbol::RightBracket) {
                    unsized_dimensions += 1;
                    break;
                }
                sized_dimensions.push(self.expression()?);
                self.assert(Symbol::RightBracket)?;
            }
            while self.accept(Symbol::LeftBracket) {
                self.assert(Symbol::RightBracket)?;
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
    fn array_initializer(&mut self) -> ParseResult<VariableInitializerList> {
        self.expect(Symbol::LeftBrace)?;
        let mut items = vec![];

        // {,}
        if self.accept(Symbol::Comma) {
            self.assert(Symbol::RightBrace)?;
            return Ok(items);
        }

        loop {
            if self.next_is(Symbol::RightBrace) {
                break;
            }
            items.push(self.variable_initializer()?);
            if !self.accept(Symbol::Comma) {
                break;
            }
        }
        self.assert(Symbol::RightBrace)?;
        Ok(items)
    }

    fn this_access(&mut self) -> ParseResult<Expression> {
        if self.next_is(Symbol::This) && !self.nth_is(1, Symbol::LeftParen) {
            self.next()?;
            Ok(Expression::This)
        } else {
            Err(Failure::NoProduction)
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
    fn if_statement(&mut self) -> ParseResult<Statement> {
        self.expect(Symbol::If)?;
        self.assert(Symbol::LeftParen)?;
        let condition = self
            .expression()
            .assert(Error::SyntaxExpectedAfter(SyntaxKind::Expression, Symbol::LeftParen))?;
        self.assert(Symbol::RightParen)?;
        let if_true = Box::new(
            self.block_statement()
                .assert(Error::SyntaxExpectedAfter(SyntaxKind::Statement, Symbol::If))?,
        );
        let if_false = if self.accept(Symbol::Else) {
            Some(Box::new(
                self.block_statement()
                    .assert(Error::SyntaxExpectedAfter(SyntaxKind::Statement, Symbol::Else))?,
            ))
        } else {
            None
        };
        Ok(Statement::If { condition, if_true, if_false })
    }

    fn while_statement(&mut self) -> ParseResult<Statement> {
        self.expect(Symbol::While)?;
        self.assert(Symbol::LeftParen)?;
        let condition = self.expression()?;
        self.assert(Symbol::RightParen)?;
        let statement = Box::new(self.block_statement()?);
        Ok(Statement::While { condition, statement })
    }

    /// ```text
    /// for_statement:
    ///     for ( for_header ) statement
    /// ```
    fn for_statement(&mut self) -> ParseResult<Statement> {
        self.expect(Symbol::For)?;
        self.assert(Symbol::LeftParen)?;
        let header = self.for_header()?;
        self.assert(Symbol::RightParen)?;
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
    fn for_header(&mut self) -> ParseResult<ForHeader> {
        let modifiers = self.modifiers(ModifierKind::VARIABLE)?;

        if self.accept(Symbol::Semicolon) {
            // basic for, empty init
            let initializer = ForInit::Expressions(vec![]);
            let (condition, update) = self.basic_for_condition_and_update()?;
            return Ok(ForHeader::BasicForHeader { initializer, condition, update });
        }

        let expression = self.term()?;

        if self.accept(Symbol::Comma) {
            // basic for, init is a statement_expression_list
            let mut init_expressions = vec![expression];
            init_expressions.extend(self.statement_expression_list()?);
            let initializer = ForInit::Expressions(init_expressions);
            self.assert(Symbol::Semicolon)?;
            let (condition, update) = self.basic_for_condition_and_update()?;
            return Ok(ForHeader::BasicForHeader { initializer, condition, update });
        }

        if self.accept(Symbol::Semicolon) {
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
        if self.accept(Symbol::Colon) {
            // for each
            let iterable = self.expression()?;
            return Ok(ForHeader::ForEachHeader {
                variable_declaration: var_declarations,
                iterable,
            });
        }

        // basic for init is a local_variable_declaration
        self.assert(Symbol::Semicolon)?;
        let (condition, update) = self.basic_for_condition_and_update()?;
        Ok(ForHeader::BasicForHeader {
            initializer: ForInit::LocalVarDeclaration(var_declarations),
            condition,
            update,
        })
    }

    //noinspection DuplicatedCode
    fn basic_for_condition_and_update(&mut self) -> ParseResult<(Option<Expression>, ForUpdate)> {
        let condition = if self.accept(Symbol::Semicolon) {
            None
        } else {
            let expression = self.expression()?;
            self.assert(Symbol::Semicolon)?;
            Some(expression)
        };
        let update = self.statement_expression_list()?;
        Ok((condition, update))
    }

    fn statement_expression_list(&mut self) -> ParseResult<Vec<Expression>> {
        self.delimited_list(Self::term, Symbol::Comma)
    }

    fn do_statement(&mut self) -> ParseResult<Statement> {
        self.expect(Symbol::Do)?;
        let statement = Box::new(self.block_statement()?);
        self.assert(Symbol::While)?;
        self.assert(Symbol::LeftParen)?;
        let condition = self.expression()?;
        self.assert(Symbol::RightParen)?;
        self.assert(Symbol::Semicolon)?;
        Ok(Statement::DoWhile { statement, condition })
    }

    fn break_statement(&mut self) -> ParseResult<Statement> {
        self.expect(Symbol::Break)?;
        let label = self.identifier().ok();
        self.assert(Symbol::Semicolon)?;
        Ok(Statement::Break(label))
    }

    fn continue_statement(&mut self) -> ParseResult<Statement> {
        self.expect(Symbol::Continue)?;
        let label = self.identifier().ok();
        self.assert(Symbol::Semicolon)?;
        Ok(Statement::Continue(label))
    }

    fn assert_statement(&mut self) -> ParseResult<Statement> {
        self.expect(Symbol::Assert)?;
        let condition = self.expression()?;
        let detail_message = if self.accept(Symbol::Colon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.assert(Symbol::Semicolon)?;
        Ok(Statement::Assert { condition, detail_message })
    }

    //noinspection DuplicatedCode
    fn return_statement(&mut self) -> ParseResult<Statement> {
        self.expect(Symbol::Return)?;
        let expression = if self.accept(Symbol::Semicolon) {
            None
        } else {
            let expression = self.expression()?;
            self.assert(Symbol::Semicolon)?;
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
    fn try_statement(&mut self) -> ParseResult<Statement> {
        self.expect(Symbol::Try)?;
        let resources = if self.accept(Symbol::LeftParen) {
            let resources = self.delimited_at_least_1(Self::try_resource, Symbol::Semicolon)?;
            self.accept(Symbol::Semicolon);
            self.assert(Symbol::RightParen)?;
            Some(resources)
        } else {
            None
        };
        let body = self.block().assert(Error::MissingTryBlock)?;
        let catch_clauses = self.zero_or_more(Self::catch_clause)?;
        let finally_block = if self.accept(Symbol::Finally) {
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
    fn try_resource(&mut self) -> ParseResult<Resource> {
        let modifiers = self.modifiers(ModifierKind::VARIABLE)?;
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
    fn catch_clause(&mut self) -> ParseResult<CatchClause> {
        self.expect(Symbol::Catch)?;
        self.assert(Symbol::LeftParen)?;
        let catch_type = self.delimited_at_least_1(
            |this| {
                let modifiers = this.modifiers(ModifierKind::VARIABLE)?;
                Ok(this.type_term()?.with_modifiers(modifiers))
            },
            Symbol::BitwiseOr,
        )?;
        let var_id = self.variable_declarator_id()?;
        self.assert(Symbol::RightParen)?;
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
    fn type_term(&mut self) -> ParseResult<Type> {
        let mut type_term = one_of!(self.primitive_type(), self.reference_type())?;
        while self.accept(Symbol::LeftBracket) {
            self.assert(Symbol::RightBracket)?;
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
    fn reference_type(&mut self) -> ParseResult<Type> {
        let type_parts = self.delimited_at_least_1(Self::type_part, Symbol::Dot)?;
        Ok(Type::Class(type_parts.try_into()?))
    }

    /// ```text
    /// type_part:
    ///     identifier
    /// ```
    fn type_part(&mut self) -> ParseResult<ClassTypePart> {
        Ok(ClassTypePart { identifier: self.identifier()? })
    }

    fn throw_statement(&mut self) -> ParseResult<Statement> {
        self.expect(Symbol::Throw)?;
        let expression = self.expression()?;
        self.assert(Symbol::Semicolon)?;
        Ok(Statement::Throw(expression))
    }

    /// ```text
    /// synchronized_statement:
    ///     synchronized ( expression ) block:
    ///
    /// ```
    fn synchronized_statement(&mut self) -> ParseResult<Statement> {
        if !peek!(
            self,
            0 => symbol!(Synchronized),
            1 => symbol!(LeftParen),
        ) {
            Err(Failure::NoProduction)
        } else {
            self.assert(Symbol::Synchronized)?;
            self.assert(Symbol::LeftParen)?;
            let lock = self.expression()?;
            self.assert(Symbol::RightParen)?;
            let body = self.block()?;
            Ok(Statement::Synchronized { lock, body })
        }
    }

    fn yield_statement(&mut self) -> ParseResult<Statement> {
        if !self.is_yield_statement() {
            return Err(Failure::NoProduction);
        }
        self.next()?;
        let expression = self.expression()?;
        self.assert(Symbol::Semicolon)?;
        Ok(Statement::Yield(expression))
    }

    fn is_yield_statement(&mut self) -> bool {
        let yield_token = peek!(self, 0 => Token::Id(s) if s.as_str() == "yield");
        let mut expression_start = peek!(self, 1 =>
            symbol!(Plus | Minus | NullLiteral | Underscore | New | Switch | This | Super | Byte
                | Char | Short | Int | Long | Float | Double | Void | Boolean | Tilde
                | ExclamationMark)
            | Token::Id(_) | Token::BooleanLiteral(_) | Token::StringLiteral(_)
            | Token::CharLiteral(_) | Token::IntegerLiteral(_) | Token::LongLiteral(_)
            | Token::FloatingPointLiteral
        );

        expression_start |=
            peek!(self,1 => symbol!(Increment | Decrement)) && !self.nth_is(2, Symbol::Semicolon);

        /* TODO: at this point additional lookahead can yield better error diagnostics,
        by checking if whatever comes after looks like a method call or like an expression */
        expression_start |= self.nth_is(1, Symbol::LeftParen);

        /* TODO: can be used for error recovery here, or for providing better error diagnostics */
        expression_start |= self.nth_is(1, Symbol::Semicolon);

        yield_token && expression_start
    }

    fn switch_statement(&mut self) -> ParseResult<Statement> {
        self.switch().map(Statement::from)
    }

    /// ```text
    /// switch_expression:
    ///     switch ( expression ) switch_block
    /// ```
    fn switch(&mut self) -> ParseResult<Switch> {
        self.expect(Symbol::Switch)?;
        self.assert(Symbol::LeftParen)?;
        let expression = self.expression()?;
        self.assert(Symbol::RightParen)?;
        let block = self.switch_block()?;
        Ok(Switch { expression, block })
    }

    /// ```text
    /// switch_block:
    ///     { {switch_block_member} }
    /// ```
    fn switch_block(&mut self) -> ParseResult<SwitchBlockMembers> {
        self.expect(Symbol::LeftBrace)?;
        let members = self.zero_or_more(Self::switch_block_member)?;
        self.assert(Symbol::RightBrace)?;
        Ok(members)
    }

    /// ```text
    /// switch_block_member:
    ///     switch_block_statement_group
    ///     switch_rule
    ///
    /// switch_block_statement_group:
    ///     switch_label : {switch_label :} {block_statement}
    ///
    /// switch_rule:
    ///     switch_label -> expression ;
    ///     switch_label -> block
    ///     switch_label -> throw_statement
    /// ```
    fn switch_block_member(&mut self) -> ParseResult<SwitchBlockMember> {
        let label = self.switch_label()?;
        if self.accept(Symbol::Colon) {
            let mut labels = NonEmptyList::new(label);
            let additional_labels = self.zero_or_more(|this| {
                let label = this.switch_label()?;
                this.assert(Symbol::Colon)?;
                Ok(label)
            })?;
            labels.append_vec(additional_labels);
            let statements = self.zero_or_more(Self::block_statement)?;
            Ok(SwitchBlockMember::LabeledStatements { labels, statements })
        } else if self.accept(Symbol::Arrow) {
            let rule = one_of!(
                self.switch_rule_expression().map(SwitchRule::from),
                self.block().map(SwitchRule::from),
                self.throw_statement().map(SwitchRule::try_from).flatten(),
            )?;
            Ok(SwitchBlockMember::Rule { case: label, rule })
        } else {
            Err(Error::SymbolExpected2(Symbol::Colon, Symbol::Arrow).into())
        }
    }

    fn switch_rule_expression(&mut self) -> ParseResult<Expression> {
        let expression = self.expression()?;
        self.assert(Symbol::Semicolon)?;
        Ok(expression)
    }

    /// ```text
    /// switch_label:
    ///     switch_case_label
    ///     default
    /// ```
    fn switch_label(&mut self) -> ParseResult<SwitchLabel> {
        one_of!(
            self.switch_case_label(),
            self.expect(Symbol::Default).map(|_| SwitchLabel::Default),
        )
    }

    /// ```text
    /// switch_case_label:
    ///     case null [, default]
    ///     case conditional_expression {, conditional_expression}
    ///     case switch_case_pattern_label
    /// ```
    fn switch_case_label(&mut self) -> ParseResult<SwitchLabel> {
        self.expect(Symbol::Case)?;
        if self.accept(Symbol::NullLiteral) {
            let default =
                self.accept(Symbol::Comma) && self.assert(Symbol::Default).map(|_| true)?;
            return Ok(SwitchLabel::Null { default });
        }

        if self.check_pattern() {
            self.switch_case_pattern_label()
        } else {
            let labels = self.delimited_at_least_1(Self::conditional_expression, Symbol::Comma)?;
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
                Token::Symbol(
                    Symbol::Byte
                    | Symbol::Short
                    | Symbol::Int
                    | Symbol::Long
                    | Symbol::Float
                    | Symbol::Double
                    | Symbol::Boolean
                    | Symbol::Char
                    | Symbol::Void,
                )
                | Token::Id(_) => match self.peek_n(lookahead + 1) {
                    Ok(Token::Id(_)) | Ok(symbol!(Underscore)) if paren_depth == 0 => return true,
                    Ok(Token::Id(_)) | Ok(symbol!(Underscore)) => temp_result = true,
                    Ok(symbol!(Arrow | Comma)) if paren_depth == 0 => {
                        return false;
                    }
                    Err(_) => return false,
                    _ => {}
                },
                symbol!(Underscore) => match self.peek_n(lookahead + 1) {
                    Ok(symbol!(RightParen | Comma)) => return true,
                    Ok(Token::Id(_) | symbol!(Underscore)) if paren_depth == 0 => return true,
                    Ok(Token::Id(_) | symbol!(Underscore)) => temp_result = true,
                    Err(_) => return false,
                    _ => {}
                },
                symbol!(Dot | QuestionMark | Extends | Super | Comma) => {}
                symbol!(
                    LessThan | LeftShift | GreaterThan | SignedRightShift | UnsignedRightShift
                ) => return false,
                symbol!(At) => lookahead = self.skip_annotation(lookahead),
                symbol!(LeftBracket) => {
                    if peek!(self, lookahead + 1 => symbol!(RightBracket), lookahead + 2 => Token::Id(_) | symbol!(Underscore))
                    {
                        return true;
                    } else if self.nth_is(lookahead + 1, Symbol::RightBracket) {
                        lookahead += 1;
                    } else {
                        return temp_result;
                    }
                }
                symbol!(LeftParen) => {
                    if self.nth_is(lookahead + 1, Symbol::RightParen) {
                        return paren_depth == 0 || !self.nth_is(lookahead + 2, Symbol::Arrow);
                    }
                    paren_depth += 1;
                }
                symbol!(RightParen) => {
                    paren_depth -= 1;
                    if paren_depth == 0
                        && peek!(self, lookahead + 1 => Token::Id(s) if s.as_str() == "when")
                    {
                        return true;
                    }
                }
                symbol!(Arrow) => return if paren_depth > 0 { false } else { temp_result },
                symbol!(Final) => {
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
        if !self.nth_is(lookahead, Symbol::At) {
            return lookahead;
        }
        lookahead += 2; // skip @ and identifier

        // skip full name
        while self.nth_is(lookahead, Symbol::Dot) {
            lookahead += 2;
        }
        let mut nesting = 0;
        loop {
            match self.peek_n(lookahead) {
                Ok(Token::EOF) => break,
                Ok(symbol!(LeftParen)) => nesting += 1,
                Ok(symbol!(RightParen)) => {
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
    fn switch_case_pattern_label(&mut self) -> ParseResult<SwitchLabel> {
        let patterns = self.delimited_at_least_1(Self::pattern, Symbol::Comma)?;
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
    fn record_component_pattern_list(&mut self) -> ParseResult<ComponentPatternList> {
        self.delimited_list(Self::record_component_pattern, Symbol::Comma)
    }

    /// ```text
    /// component_pattern:
    ///     pattern
    ///     _
    /// ```
    fn record_component_pattern(&mut self) -> ParseResult<ComponentPattern> {
        if self.accept(Symbol::Underscore) {
            Ok(ComponentPattern::MatchAll)
        } else {
            Ok(ComponentPattern::Pattern(self.pattern()?))
        }
    }

    /// ```text
    /// pattern:
    ///     local_varaiable_declaration
    ///     reference_type ( [component_pattern_list] )
    /// ```
    fn pattern(&mut self) -> ParseResult<Pattern> {
        let modifiers = self.modifiers(ModifierKind::VARIABLE)?;
        let type_term = self.type_term()?;
        if self.accept(Symbol::LeftParen) {
            let reference_type = type_term.try_into().map_err(|_| Failure::NoProduction)?;
            let components = self.record_component_pattern_list()?;
            self.assert(Symbol::RightParen)?;
            return Ok(Pattern::Record { reference_type, components });
        }

        if let Ok(var_id) = self.variable_declarator_id() {
            let variable_type = type_term.try_into().map_err(|_| Failure::NoProduction)?;
            let declarators = NonEmptyList::new(VariableDeclarator {
                name: var_id,
                initializer: None,
            });
            let var_declaration =
                VariableDeclaration { variable_type, declarators }.with_modifiers(modifiers);

            return Ok(Pattern::Type(var_declaration));
        }

        Err(Failure::NoProduction)
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

impl From<TypeIdentifier> for Type {
    fn from(value: TypeIdentifier) -> Self {
        Self::Class(ClassType {
            namespace: vec![],
            name: ClassTypePart { identifier: value },
        })
    }
}

impl From<ClassType> for Type {
    fn from(value: ClassType) -> Self {
        Type::Class(value)
    }
}

impl TryFrom<AtLeastOne<ClassTypePart>> for ClassType {
    type Error = Error;

    fn try_from(parts: AtLeastOne<ClassTypePart>) -> Result<Self, Self::Error> {
        let (namespace, last) = parts.split_last();
        let name = ClassTypePart::try_from(last)?;
        Ok(Self { namespace, name })
    }
}

impl TryFrom<ClassTypePart<Identifier>> for ClassTypePart<TypeIdentifier> {
    type Error = Error;
    fn try_from(value: ClassTypePart<Identifier>) -> Result<Self, Self::Error> {
        Ok(ClassTypePart {
            identifier: value.identifier.try_into()?,
        })
    }
}

impl TryFrom<Expression> for Identifier {
    type Error = Failure;

    fn try_from(value: Expression) -> ParseResult<Self> {
        match value {
            Expression::Name(id) => Ok(id),
            _ => Err(Failure::NoProduction),
        }
    }
}

impl TryFrom<Expression> for Type {
    type Error = Failure;

    fn try_from(value: Expression) -> ParseResult<Self> {
        match value {
            Expression::Type(t) => Ok(t),
            Expression::Name(n) => Ok(TypeIdentifier::try_from(n)?.into()),
            Expression::MemberAccess(MemberAccess { target, name }) => Ok(ClassType {
                name: ClassTypePart { identifier: name.try_into()? },
                namespace: ClassTypePartList::try_from(*target)?,
            }
            .into()),
            _ => Err(Failure::NoProduction),
        }
    }
}

impl TryFrom<Expression> for ClassTypePartList {
    type Error = Error;
    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::MemberAccess(MemberAccess { target, name }) => {
                let mut parts = Self::try_from(*target)?;
                parts.push(ClassTypePart { identifier: name });
                Ok(parts)
            }
            _ => Err(Error::IdentifierExpected),
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
    type Error = Failure;

    fn try_from(value: Expression) -> ParseResult<Self> {
        match value {
            Expression::Name(id) => Ok(LeftHandSide::ExpressionName(id)),
            Expression::MemberAccess(member_access) => {
                Ok(LeftHandSide::MemberAccess(member_access))
            }
            Expression::ArrayAccess(array_access) => Ok(LeftHandSide::ArrayAccess(array_access)),
            _ => Err(Failure::NoProduction),
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
    type Error = Failure;
    fn try_from(value: Statement) -> ParseResult<Self> {
        match value {
            Statement::Throw(_) => Ok(SwitchRule::Throw(value)),
            _ => Err(Failure::NoProduction),
        }
    }
}

impl TryFrom<Identifier> for TypeIdentifier {
    type Error = Error;
    fn try_from(value: Identifier) -> Result<Self, Self::Error> {
        TypeIdentifier::from(value).map_err(|_| Error::RestrictedTypeName)
    }
}
