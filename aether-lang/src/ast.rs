// Abstract Syntax Tree for Aether

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevel {
    Agent(AgentDef),
    Function(FunctionDef),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AgentDef {
    pub name: String,
    pub decorators: Vec<Decorator>,
    pub state: Option<StateBlock>,
    pub functions: Vec<FunctionDef>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Decorator {
    pub name: String,
    pub arguments: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StateBlock {
    pub fields: Vec<StateField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StateField {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Tensor(Vec<usize>), // Tensor<[1, 10]>
    Vector(String, usize), // Vector<f32, 1536>
    Primitive(String), // String, i32, f32
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDef {
    pub name: String,
    pub args: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(String, Expr),
    Return(Expr),
    Expr(Expr),
    Thinking(String, Block), // thinking "Reasoning" { ... }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Variable(String),
    Call(String, Vec<Expr>),
    MethodCall(Box<Expr>, String, Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Int(i64),
    Float(f64),
}
