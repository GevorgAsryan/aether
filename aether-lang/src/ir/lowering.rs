use crate::ast::*;
use crate::ir::structure::*;

pub struct LoweringContext {
    register_count: usize,
    operations: Vec<Operation>,
}

impl LoweringContext {
    pub fn new() -> Self {
        Self {
            register_count: 0,
            operations: Vec::new(),
        }
    }

    fn next_reg(&mut self) -> Value {
        let r = Value::Register(self.register_count);
        self.register_count += 1;
        r
    }

    pub fn lower_program(mut self, program: &[TopLevel]) -> Module {
        let mut functions = Vec::new();
        
        for item in program {
            if let TopLevel::Agent(agent) = item {
                for func in &agent.functions {
                    let ir_func = self.lower_function(func);
                    functions.push(ir_func);
                }
            }
        }
        
        Module { functions }
    }

    fn lower_function(&mut self, func: &FunctionDef) -> Function {
        self.operations.clear();
        self.register_count = 0;
        
        // Lower arguments (implicitly bound to registers, assumed)
        
        self.lower_block(&func.body);
        
        Function {
            name: func.name.clone(),
            operations: self.operations.clone(),
        }
    }

    fn lower_block(&mut self, block: &Block) {
        for stmt in &block.statements {
            match stmt {
                Statement::Let(name, expr) => {
                    let val = self.lower_expr(expr);
                    // In a real compiler, we'd map 'name' to 'val' in a symbol table here
                    // For now, we assume the value remains in the register
                }
                Statement::Return(expr) => {
                    let val = self.lower_expr(expr);
                    self.emit(OpCode::Return, vec![val], None);
                }
                Statement::Expr(expr) => {
                    self.lower_expr(expr);
                }
                Statement::Thinking(desc, blk) => {
                    // Start Span
                    self.emit(OpCode::ThinkingStart(desc.clone()), vec![], None);
                    self.lower_block(blk);
                    self.emit(OpCode::ThinkingEnd, vec![], None);
                }
            }
        }
    }

    fn lower_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Literal(lit) => {
                 let l = match lit {
                     crate::ast::Literal::Int(i) => Literal::Int(*i),
                     crate::ast::Literal::Float(f) => Literal::Float(*f),
                     crate::ast::Literal::String(s) => Literal::String(s.clone()),
                 };
                 Value::Const(l)
            }
            Expr::Variable(_) => {
                // In full implementation, lookup register for variable
                Value::Register(0) // Dummy
            }
            Expr::MethodCall(lhs, method, args) => {
                let mut operands = vec![self.lower_expr(lhs)];
                for arg in args {
                    operands.push(self.lower_expr(arg));
                }
                
                let res = self.next_reg();
                
                if method == "predict" {
                    self.emit(OpCode::TensorOp("predict".into()), operands, Some(res.clone()));
                } else {
                    self.emit(OpCode::Call(method.clone()), operands, Some(res.clone()));
                }
                res
            }
            _ => Value::Register(0), // TODO
        }
    }

    fn emit(&mut self, opcode: OpCode, operands: Vec<Value>, result: Option<Value>) {
        self.operations.push(Operation {
            opcode,
            operands,
            result
        });
    }
}
