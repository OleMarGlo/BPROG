use std::collections::HashMap;
use crate::types::Value;

/**
 * The Functions struct is a wrapper around a HashMap of function names and their corresponding values.
 * It is used to store functions that can be called from within the program.
 * It holds a hashmap of the name and the block of the function.
 */
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