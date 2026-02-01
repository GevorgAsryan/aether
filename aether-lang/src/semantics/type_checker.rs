use crate::ast::*;
use crate::semantics::symbol_table::SymbolTable;

pub struct TypeChecker {
    symbols: SymbolTable,
    errors: Vec<String>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            symbols: SymbolTable::new(),
            errors: Vec::new(),
        }
    }

    pub fn check_program(&mut self, program: &[TopLevel]) {
        for item in program {
            match item {
                TopLevel::Agent(agent) => self.check_agent(agent),
                TopLevel::Function(func) => self.check_function(func),
            }
        }
    }

    fn check_agent(&mut self, agent: &AgentDef) {
        self.symbols.enter_scope();
        // Register state fields
        if let Some(state) = &agent.state {
             for field in &state.fields {
                 self.symbols.insert(field.name.clone(), field.ty.clone());
             }
        }
        
        for func in &agent.functions {
            self.check_function(func);
        }
        self.symbols.exit_scope();
    }

    fn check_function(&mut self, func: &FunctionDef) {
        self.symbols.enter_scope();
        for (name, ty) in &func.args {
            self.symbols.insert(name.clone(), ty.clone());
        }

        self.check_block(&func.body);
        self.symbols.exit_scope();
    }

    fn check_block(&mut self, block: &Block) {
        for stmt in &block.statements {
            match stmt {
                Statement::Let(name, expr) => {
                    let ty = self.infer_type(expr);
                    self.symbols.insert(name.clone(), ty);
                }
                Statement::Expr(expr) => {
                    self.infer_type(expr);
                }
                Statement::Return(expr) => {
                    self.infer_type(expr);
                }
                Statement::Thinking(_, sub_block) => {
                    self.check_block(sub_block);
                }
            }
        }
    }

    fn infer_type(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::Literal(lit) => match lit {
                Literal::String(_) => Type::Primitive("String".into()),
                Literal::Int(_) => Type::Primitive("i64".into()),
                Literal::Float(_) => Type::Primitive("f64".into()),
            },
            Expr::Variable(name) => {
                if let Some(sym) = self.symbols.lookup(name) {
                    sym.ty.clone()
                } else {
                    self.errors.push(format!("Undefined variable: {}", name));
                    Type::Primitive("Unknown".into())
                }
            },
            Expr::Call(_, _) => Type::Primitive("TODO".into()), // Function lookups
            Expr::MethodCall(lhs, method, args) => {
                // Heuristic for AI demo:
                // If method is "predict", return Tensor
                if method == "predict" {
                     return Type::Tensor(vec![1, 1]); 
                }
                Type::Primitive("Unknown".into())
            }
        }
    }
}
