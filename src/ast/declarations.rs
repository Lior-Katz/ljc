use crate::ast::{Block, TypeOrVoid};
use crate::ast::expressions::{ArgumentList, VariableInitializer};
use crate::ast::identifiers::{Identifier, TypeIdentifier};
use crate::ast::modifiers::{ElementValue, Modified};
use crate::ast::statements::{BlockStatements, ConstructorInvocation};
use crate::ast::types::{Type, TypeList};
use crate::collections::{AtLeastOne, Multiple};
use crate::file::Span;

pub type ClassBodyDeclarations = Vec<ClassBodyDeclaration>;
pub type FormalParameterList = Vec<Modified<FormalParameter>>;
pub type VariableDeclaratorList = AtLeastOne<VariableDeclarator>;
pub type MethodResult = TypeOrVoid;
pub type RecordComponentList = Vec<Modified<RecordComponent>>;
pub type RecordBodyDeclaration = ClassBodyDeclaration;

#[derive(Debug)]
pub enum TypeDeclaration {
    Class(ClassDeclaration),
    Record(RecordDeclaration),
    Enum(EnumDeclaration),
    Interface(InterfaceDeclaration),
    AnnotationInterface(AnnotationInterfaceDeclaration),
}

#[derive(Debug)]
pub struct ClassDeclaration {
    pub name: TypeIdentifier,
    pub extends: Option<Type>,
    pub implements: Option<TypeList>,
    pub permits: Option<TypeList>,
    pub body: ClassBodyDeclarations,
    pub span: Span,
}

#[derive(Debug)]
pub enum ClassBodyDeclaration {
    ClassMember(Modified<ClassMemberDeclaration>),
    InstanceInitializer(Block),
    StaticInitializer(Block),
}

#[derive(Debug)]
pub enum ClassMemberDeclaration {
    Method(MethodDeclaration),
    NestedClassOrInterface(TypeDeclaration),
    Field {
        variable_type: Type,
        declarations: VariableDeclaratorList,
    },
    Constructor {
        name: TypeIdentifier, // this is just for validating that the name matches the class
        parameters: FormalParameterList,
        throws: Multiple<Modified<Type>>,
        body: ConstructorBody,
    },
    CompactConstructor {
        name: TypeIdentifier,
        body: ConstructorBody,
    },
}

#[derive(Debug)]
pub struct InterfaceDeclaration {
    pub name: TypeIdentifier,
    pub extends: Option<TypeList>,
    pub permits: Option<TypeList>,
    pub body: Vec<Modified<ClassMemberDeclaration>>,
    pub span: Span,
}

#[derive(Debug)]
pub struct AnnotationInterfaceDeclaration {
    pub name: TypeIdentifier,
    pub body: Vec<Modified<ClassMemberDeclaration>>,
    pub span: Span,
}

#[derive(Debug)]
pub struct RecordDeclaration {
    pub name: TypeIdentifier,
    pub components: RecordComponentList,
    pub implements: Option<TypeList>,
    pub body: Vec<RecordBodyDeclaration>,
    pub span: Span,
}

#[derive(Debug)]
pub enum RecordComponent {
    Normal {
        component_type: Type,
        name: Identifier,
    },
    VariableArity {
        component_type: Type,
        name: Identifier,
    },
}

#[derive(Debug)]
pub struct EnumDeclaration {
    pub name: TypeIdentifier,
    pub implements: Option<TypeList>,
    pub body: EnumBody,
    pub span: Span,
}

#[derive(Debug)]
pub struct EnumBody {
    pub constants: Vec<Modified<EnumConstant>>,
    pub body_declarations: ClassBodyDeclarations,
}

#[derive(Debug)]
pub struct EnumConstant {
    pub name: Identifier,
    pub args: Option<ArgumentList>,
    pub body: Option<ClassBodyDeclarations>,
}

#[derive(Debug)]
pub struct MethodDeclaration {
    pub result: MethodResult,
    pub identifier: Identifier,
    pub parameters: FormalParameterList,
    pub throws: Multiple<Modified<Type>>,
    pub body: MethodBody,
    pub default: Option<ElementValue>,
}

#[derive(Debug)]
pub enum FormalParameter {
    NormalParameter(Type, VariableDeclaratorId),
    VariableArityParameter(Type, Identifier),
}

#[derive(Debug)]
pub enum MethodBody {
    Semicolon(Span),
    Block(BlockStatements),
}

#[derive(Debug)]
pub struct ConstructorBody {
    pub prologue: Option<BlockStatements>,
    pub constructor_invocation: Option<ConstructorInvocation>,
    pub epilogue: BlockStatements,
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub variable_type: Type,
    pub declarators: VariableDeclaratorList,
}

#[derive(Debug)]
pub struct VariableDeclarator {
    pub name: VariableDeclaratorId,
    pub initializer: Option<VariableInitializer>,
}

#[derive(Debug)]
pub enum VariableDeclaratorId {
    Named(Identifier),
    Unnamed,
}

impl TypeDeclaration {
    pub fn span(&self) -> &Span {
        match self {
            TypeDeclaration::Class(c) => &c.span,
            TypeDeclaration::Record(r) => &r.span,
            TypeDeclaration::Enum(e) => &e.span,
            TypeDeclaration::Interface(i) => &i.span,
            TypeDeclaration::AnnotationInterface(a) => &a.span,
        }
    }
}

impl ClassBodyDeclaration {
    pub fn span(&self) -> &Span {
        match self {
            ClassBodyDeclaration::ClassMember(member) => match member.modifiers.first() {
                Some(m) => m.span(),
                None => member.item.span(),
            },
            ClassBodyDeclaration::InstanceInitializer(Block { span, .. })
            | ClassBodyDeclaration::StaticInitializer(Block { span, .. }) => span,
        }
    }
}

impl ClassMemberDeclaration {
    pub fn span(&self) -> &Span {
        match self {
            ClassMemberDeclaration::Method(MethodDeclaration { result, .. }) => result.span(),
            ClassMemberDeclaration::NestedClassOrInterface(c) => c.span(),
            ClassMemberDeclaration::Field { variable_type, .. } => variable_type.span(),
            ClassMemberDeclaration::Constructor { name, .. }
            | ClassMemberDeclaration::CompactConstructor { name, .. } => name.span(),
        }
    }
}

impl VariableDeclaration {
    pub fn span(&self) -> &Span {
        self.variable_type.span()
    }
}
