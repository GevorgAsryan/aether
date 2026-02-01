use aether_runtime::UnifiedMemory;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct Tensor {
    pub id: u64,
    pub shape: Vec<usize>,
}

impl Tensor {
    pub fn new(shape: Vec<usize>) -> Self {
        Self {
            id: 0,
            shape
        }
    }

    pub fn predict(&self, input: &Vector) -> Tensor {
        println!("Thinking... (Tensor Ops)");
        Tensor::new(vec![1, 10])
    }
}

#[derive(Debug, Clone)]
pub struct Vector {
    pub data: Vec<f32>,
}

impl Vector {
    pub fn from_string(s: String) -> Self {
        println!("Embedding content: '{}'", s);
        Self {
            data: vec![0.1; 1536],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CognitiveError {
    pub protocol: String,
    pub r#type: String,
    pub error_code: String,
    pub severity: String,
    pub message: String,
    pub context: ErrorContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorContext {
    pub function: String,
    pub suggestion: String,
}

impl CognitiveError {
    pub fn new(code: &str, msg: &str, func: &str) -> Self {
        Self {
            protocol: "aether/v1".to_string(),
            r#type: "error".to_string(),
            error_code: code.to_string(),
            severity: "error".to_string(),
            message: msg.to_string(),
            context: ErrorContext {
                function: func.to_string(),
                suggestion: "Check input dimensions".to_string(),
            }
        }
    }

    pub fn report(&self) {
        if std::env::var("AETHER_AI_MODE").unwrap_or_default() == "1" {
            let json = serde_json::to_string_pretty(self).unwrap();
            eprintln!("{}", json);
        } else {
            eprintln!("Cognitive Error: {}", self.message);
        }
    }
}

pub type Result<T> = std::result::Result<T, CognitiveError>;
