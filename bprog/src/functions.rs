use std::collections::HashMap;
use crate::types::Value;

pub struct Functions {
    pub functions: HashMap<String, Value>,
}

impl Functions {
    pub fn new() -> Self {
        Functions {
            functions: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: &str, value: Value) {
        self.functions.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        if self.functions.contains_key(name) {
            Some(&self.functions[name])
        } else {
            None
        }
    }
}