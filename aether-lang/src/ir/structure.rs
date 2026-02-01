// Aether IR (SSA Form)

#[derive(Debug, Clone)]
pub struct Module {
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub operations: Vec<Operation>,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub opcode: OpCode,
    pub operands: Vec<Value>,
    pub result: Option<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    // Standard
    Constant(Literal),
    Call(String),
    Return,
    
    // Aether Specific
    LoadModel(String),
    Embed(String),
    TensorOp(String), // matmul, add, etc.
    ThinkingStart(String),
    ThinkingEnd,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Register(usize), // %0, %1
    Const(Literal),
}
