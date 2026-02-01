use std::collections::HashMap;
use crate::ast::Type;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub ty: Type,
    pub is_mutable: bool,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn insert(&mut self, name: String, ty: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, Symbol { ty, is_mutable: false });
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.get(name) {
                return Some(sym);
            }
        }
        None
    }
}
