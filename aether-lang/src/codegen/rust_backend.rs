use crate::ir::structure::*;

pub struct RustBackend {
    output: String,
}

impl RustBackend {
    pub fn new() -> Self {
        Self {
            output: String::new(),
        }
    }

    pub fn generate(&mut self, module: &Module) -> String {
        self.output.push_str("use aether_std::{Tensor, Vector, CognitiveError, Result};\n");
        self.output.push_str("use aether_runtime::{detect_runtime, RuntimeTarget};\n\n");
        
        for func in &module.functions {
            self.generate_function(func);
        }
        
        self.output.clone()
    }

    fn generate_function(&mut self, func: &Function) {
        // Function Signature (Assuming main-like for demo)
        if func.name == "digest" {
             self.output.push_str(&format!("pub fn {}() {{\n", func.name));
        } else {
             self.output.push_str(&format!("fn {}() {{\n", func.name));
        }
        
        self.output.push_str("    let runtime = detect_runtime();\n");

        for op in &func.operations {
            self.generate_op(op);
        }

        self.output.push_str("}\n\n");
    }

    fn generate_op(&mut self, op: &Operation) {
        match &op.opcode {
            OpCode::Embed(txt) => {
                let res = self.fmt_val(op.result.as_ref().unwrap());
                self.output.push_str(&format!("    let {} = Vector::from_string(\"{}\".to_string());\n", res, txt));
            }
            OpCode::LoadModel(model) => {
                let res = self.fmt_val(op.result.as_ref().unwrap());
                self.output.push_str(&format!("    let {} = Tensor::new(vec![7_000_000]); // Loaded {}\n", res, model));
            }
            OpCode::ThinkingStart(desc) => {
                self.output.push_str(&format!("    println!(\"Thinking '{}'...\");\n", desc));
                self.output.push_str("    {\n");
            }
            OpCode::ThinkingEnd => {
                self.output.push_str("    }\n");
            }
            OpCode::TensorOp(name) => {
                let res = self.fmt_val(op.result.as_ref().unwrap());
                let args: Vec<String> = op.operands.iter().map(|v| self.fmt_val(v)).collect();
                if name == "predict" {
                    self.output.push_str(&format!("    let {} = {}.predict(&{});\n", res, args[0], args[1]));
                }
            }
            OpCode::Return => {
                if let Some(val) = op.operands.first() {
                    self.output.push_str(&format!("    return {};\n", self.fmt_val(val)));
                } else {
                    self.output.push_str("    return;\n");
                }
            }
            _ => {
                self.output.push_str(&format!("    // Unimplemented Op: {:?}\n", op.opcode));
            }
        }
    }

    fn fmt_val(&self, val: &Value) -> String {
        match val {
            Value::Register(id) => format!("r{}", id),
            Value::Const(lit) => match lit {
                Literal::String(s) => format!("\"{}\"", s),
                Literal::Int(i) => format!("{}", i),
                Literal::Float(f) => format!("{}", f),
            },
        }
    }
}
