use std::collections::HashMap;

use crate::types::Value;

pub struct Variables {
    pub variables: HashMap<String, Value>,
}

impl Variables {
    pub fn new() -> Self {
        Variables {
            variables: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: &str, value: Value) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        if self.variables.contains_key(name) {
            Some(&self.variables[name])
        } else {
            None
        }
    }
}