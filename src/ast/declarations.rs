use crate::ast::expressions::{Expression, VariableInitializer};
use crate::ast::identifiers::{Identifier, TypeIdentifier};
use crate::ast::modifiers::Modified;
use crate::ast::statements::{BlockStatements, ConstructorInvocation};
use crate::ast::types::Type;

pub type FormalParameterList = Vec<Modified<FormalParameter>>;
pub type VariableDeclaratorList = Vec<VariableDeclarator>;
pub type MethodResult = Expression;
pub type RecordComponentList = Vec<RecordComponent>;
pub type RecordBodyDeclaration = ClassBodyDeclaration;

#[derive(Debug)]
pub enum TopLevelClassOrInterfaceDeclaration {
    Class(Modified<ClassDeclaration>),
    Interface(Modified<InterfaceDeclaration>),
}

#[derive(Debug)]
pub enum ClassDeclaration {
    NormalClass(NormalClassDeclaration),
    Record(RecordDeclaration),
}

#[derive(Debug)]
pub struct NormalClassDeclaration {
    pub identifier: TypeIdentifier,
    pub body: Vec<ClassBodyDeclaration>,
}

#[derive(Debug)]
pub enum ClassBodyDeclaration {
    ClassMember(Modified<ClassMemberDeclaration>),
}

#[derive(Debug)]
pub enum ClassMemberDeclaration {
    Method(MethodDeclaration),
    NestedClass(ClassDeclaration),
    NestedInterface(InterfaceDeclaration),
    Field {
        variable_type: Expression,
        declarations: VariableDeclaratorList,
    },
    Constructor {
        name: TypeIdentifier, // this is just for validating that the name matches the class
        parameters: FormalParameterList,
        body: ConstructorBody,
    },
}

#[derive(Debug)]
pub enum InterfaceDeclaration {
    NormalInterface(NormalInterfaceDeclaration),
}

#[derive(Debug)]
pub struct NormalInterfaceDeclaration {
    pub identifier: TypeIdentifier,
    pub body: Vec<Modified<ClassMemberDeclaration>>,
}

#[derive(Debug)]
pub struct RecordDeclaration {
    pub name: TypeIdentifier,
    pub components: RecordComponentList,
    pub body: Vec<RecordBodyDeclaration>,
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
pub struct MethodDeclaration {
    pub result: MethodResult,
    pub identifier: Identifier,
    pub parameters: FormalParameterList,
    pub body: MethodBody,
}

#[derive(Debug)]
pub enum FormalParameter {
    NormalParameter(Type, VariableDeclaratorId),
    VariableArityParameter(Type, Identifier),
}

#[derive(Debug)]
pub enum MethodBody {
    Semicolon,
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
    pub variable_type: Expression,
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
