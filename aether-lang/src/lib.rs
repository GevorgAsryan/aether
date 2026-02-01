pub mod ast;
pub mod lexer;
pub mod parser;
pub mod semantics;
pub mod ir;
pub mod codegen;

use logos::Logos;
use chumsky::Parser;

pub fn compile(source: &str) -> String {
    println!("Compiling Aether Source...");
    
    // 1. Lexing
    let lexer = lexer::Token::lexer(source);
    let tokens: Vec<_> = lexer.filter_map(|t| t.ok()).collect();

    // 2. Parsing
    let parser = parser::parser();
    match parser.parse(tokens) {
        Ok(ast) => {
            
            // 3. Semantics
            let mut checker = semantics::type_checker::TypeChecker::new();
            checker.check_program(&ast);

            // 4. IR Generation
            let mut lowering = ir::lowering::LoweringContext::new();
            let module = lowering.lower_program(&ast);
            
            // 5. Code Generation (Rust)
            let mut backend = codegen::rust_backend::RustBackend::new();
            let code = backend.generate(&module);
            
            println!("Compilation Successful. Output:\n{}", code);
            code
        },
        Err(errs) => {
            println!("Parse Errors: {:?}", errs);
            String::new()
        }
    }
}
