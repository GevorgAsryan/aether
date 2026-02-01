use std::env;
use std::fs;
use aether_lang::compile;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: aetherc <file.ae>");
        return;
    }

    let filename = &args[1];
    println!("Reading {}", filename);

    let source = fs::read_to_string(filename).expect("Unable to read file");
    
    let output_code = compile(&source);
    
    let output_filename = filename.replace(".ae", ".rs");
    fs::write(&output_filename, output_code).expect("Unable to write output");
    
    println!("Compiled to {}", output_filename);
}
