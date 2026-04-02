pub type Program = CompilationUnit;
pub type Identifier = String;
pub type TypeIdentifier = Identifier;
type BlockStatements = Vec<Statement>;

#[derive(Debug)]
pub enum CompilationUnit {
    Ordinary(Vec<TopLevelClassOrInterfaceDeclaration>),
}

#[derive(Debug)]
pub enum TopLevelClassOrInterfaceDeclaration {
    ClassDeclaration(ClassDeclaration),
}

#[derive(Debug)]
pub enum ClassDeclaration {
    NormalClassDeclaration(NormalClassDeclaration),
}

#[derive(Debug)]
pub struct NormalClassDeclaration {
    pub identifier: TypeIdentifier,
    pub body: ClassBody,
}

#[derive(Debug)]
pub struct ClassBody {
    pub class_body_declarations: Vec<ClassBodyDeclaration>,
}

#[derive(Debug)]
pub enum ClassBodyDeclaration {
    ClassMemberDeclaration(ClassMemberDeclaration),
}

#[derive(Debug)]
pub enum ClassMemberDeclaration {
    MethodDeclaration(MethodDeclaration),
}

#[derive(Debug)]
pub struct MethodDeclaration {
    pub result: MethodResult,
    pub identifier: Identifier,
    pub body: MethodBody,
}

#[derive(Debug)]
pub enum MethodResult {
    Void,
}

#[derive(Debug)]
pub enum MethodBody {
    Semicolon,
    Block(BlockStatements),
}

#[derive(Debug)]
pub enum Statement {
    EmptyStatement,
}
