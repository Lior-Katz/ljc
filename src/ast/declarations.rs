use crate::ast::expressions::{ArgumentList, VariableInitializer};
use crate::ast::identifiers::{Identifier, TypeIdentifier};
use crate::ast::modifiers::{ElementValue, Modified};
use crate::ast::statements::{Block, BlockStatements, ConstructorInvocation};
use crate::ast::types::{Type, TypeList, TypeOrVoid};
use crate::collections::{AtLeastOne, Multiple};
use crate::file::Span;

pub type ClassBodyDeclarations = Vec<ClassBodyDeclaration>;
pub type FieldDeclaration = VariableDeclaration;
pub type FormalParameterList = Vec<Modified<FormalParameter>>;
pub type VariableDeclaratorList = AtLeastOne<VariableDeclarator>;
pub type MethodResult = TypeOrVoid;
pub type RecordComponentList = Vec<Modified<RecordComponent>>;
pub type RecordBodyDeclaration = ClassBodyDeclaration;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum TypeDeclaration {
    Class(ClassDeclaration),
    Record(RecordDeclaration),
    Enum(EnumDeclaration),
    Interface(InterfaceDeclaration),
    AnnotationInterface(AnnotationInterfaceDeclaration),
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct ClassDeclaration {
    pub name: TypeIdentifier,
    pub extends: Option<Type>,
    pub implements: Option<TypeList>,
    pub permits: Option<TypeList>,
    pub body: ClassBodyDeclarations,
    pub span: Span,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum ClassBodyDeclaration {
    ClassMember(Modified<ClassMemberDeclaration>),
    InstanceInitializer(Block),
    StaticInitializer(Block),
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum ClassMemberDeclaration {
    Method(MethodDeclaration),
    NestedClassOrInterface(TypeDeclaration),
    Field(FieldDeclaration),
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

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct InterfaceDeclaration {
    pub name: TypeIdentifier,
    pub extends: Option<TypeList>,
    pub permits: Option<TypeList>,
    pub body: Vec<Modified<ClassMemberDeclaration>>,
    pub span: Span,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct AnnotationInterfaceDeclaration {
    pub name: TypeIdentifier,
    pub body: Vec<Modified<ClassMemberDeclaration>>,
    pub span: Span,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct RecordDeclaration {
    pub name: TypeIdentifier,
    pub components: RecordComponentList,
    pub implements: Option<TypeList>,
    pub body: Vec<RecordBodyDeclaration>,
    pub span: Span,
}

#[derive(Debug, Hash, Eq, PartialEq)]
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

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct EnumDeclaration {
    pub name: TypeIdentifier,
    pub implements: Option<TypeList>,
    pub body: EnumBody,
    pub span: Span,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct EnumBody {
    pub constants: Vec<Modified<EnumConstant>>,
    pub body_declarations: ClassBodyDeclarations,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct EnumConstant {
    pub name: Identifier,
    pub args: Option<ArgumentList>,
    pub body: Option<ClassBodyDeclarations>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct MethodDeclaration {
    pub result: MethodResult,
    pub identifier: Identifier,
    pub parameters: FormalParameterList,
    pub throws: Multiple<Modified<Type>>,
    pub body: MethodBody,
    pub default: Option<ElementValue>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum FormalParameter {
    NormalParameter(Type, VariableDeclaratorId),
    VariableArityParameter(Type, Identifier),
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum MethodBody {
    Semicolon(Span),
    Block(BlockStatements),
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct ConstructorBody {
    pub prologue: Option<BlockStatements>,
    pub constructor_invocation: Option<ConstructorInvocation>,
    pub epilogue: BlockStatements,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct VariableDeclaration {
    pub variable_type: Type,
    pub declarators: VariableDeclaratorList,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct VariableDeclarator {
    pub name: VariableDeclaratorId,
    pub initializer: Option<VariableInitializer>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
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

    pub fn name(&self) -> &TypeIdentifier {
        match self {
            TypeDeclaration::Class(ClassDeclaration { name, .. })
            | TypeDeclaration::Enum(EnumDeclaration { name, .. })
            | TypeDeclaration::Record(RecordDeclaration { name, .. })
            | TypeDeclaration::Interface(InterfaceDeclaration { name, .. })
            | TypeDeclaration::AnnotationInterface(AnnotationInterfaceDeclaration {
                name, ..
            }) => name,
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
            ClassMemberDeclaration::Field(FieldDeclaration { variable_type, .. }) => {
                variable_type.span()
            }
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
