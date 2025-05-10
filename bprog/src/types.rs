use std::{fmt::Display, ops::{Add, Div, Mul, Not, Sub}};

use crate::{operations::{self, arithmetic, io, logic}, variables::{self, Variables}};
use crate::stack::Stack;

#[derive(Debug, Clone)]
pub enum Value {
    Float(f64),
    Int(i64),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
    Block(Vec<String>),
    Symbol(String),
}

impl Value {
    pub fn to_string_with_variables(&self, vars: &Variables) -> String {
        match self {
            Value::List(list) => {
                let resolved = list.iter().map(|v| {
                    if let Some(val) = vars.get(&v.to_string()) {
                        val.to_string()
                    } else {
                        v.to_string()
                    }
                }).collect::<Vec<_>>();
                format!("[ {} ]", resolved.join(" "))
            },
            _ => self.to_string(), // fall back to Display
        }
    }

    pub fn and(self, other: Self) -> Result<Value, String> {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a && b)),
            _ => Err(format!("Invalid operation and")),
        }
    }

    pub fn or(self, other: Self) -> Result<Value, String> {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a || b)),
            _ => Err(format!("Invalid operation or")),
        }
    }

    pub fn parse_integer(input: &str) -> Result<Value, String> {
        match input.parse::<i64>() {
            Ok(value) => Ok(Value::Int(value)),
            Err(_) => Err(format!("Invalid integer {}", input)),
        }
    }

    pub fn parse_float(input: &str) -> Result<Value, String> {
        match input.parse::<f64>() {
            Ok(value) => Ok(Value::Float(value)),
            Err(_) => Err(format!("Invalid float {}", input)),
        }
    }

    pub fn head(value: &Value) -> Result<Value, String> {
        let result = match value {
            Value::List(list) if !list.is_empty() => Ok(list[0].clone()),
            Value::String(string) if !string.is_empty() => Ok(Value::String(string.chars().nth(0).unwrap().to_string())),
            Value::String(_) => Err(format!("Empty string")),
            Value::List(_) => Err(format!("Empty list")),
            _ => return Err(format!("Invalid operation")),
        };
        result
    }

    pub fn tail(value: &Value) -> Result<Value, String> {
        match value {
            Value::List(list) if !list.is_empty() => return Ok(Value::List(list[1..].to_vec())),
            Value::String(string) if !string.is_empty() => return Ok(Value::String(string.chars().skip(1).collect())),
            Value::String(_) => return Err(format!("Empty string")),
            Value::List(_) => return Err(format!("Empty list")),
            _ => return Err(format!("Invalid operation")),
        };
    }

    pub fn empty(value: &Value) -> Result<Value, String> {
        let result = match value {
            Value::List(list ) => Ok(Value::Boolean(list.is_empty())),
            Value::String(string) => Ok(Value::Boolean(string.is_empty())),
            _ => Err(format!("Invalid operation")),
        };
        result
    }

    pub fn lenght(value: &Value) -> Result<Value, String> {
        let result = match value {
            Value::List(list) => Ok(Value::Int(list.len() as i64)),
            Value::String(string) => Ok(Value::Int(string.chars().count() as i64)),
            _ => Err(format!("Invalid operation")),
        };
        result
    }

    pub fn cons(list: &mut Vec<Value>, value: Value) -> Result<(), String> {
        list.insert(0,value);
        Ok(())
    }

    pub fn append(list1: &mut Vec<Value>, list2: Vec<Value>) -> Result<(), String> {
        list1.extend(list2);
        Ok(())
    }

    pub fn words(stack: &mut Stack) -> Result<(), String> {
        let input = stack.pop().unwrap();
        let mut words = Vec::new();
        match input {
            Value::String(input) => {
                for word in input.split_whitespace() {
                    words.push(Value::String(word.to_string()));
                }
                stack.push(Value::List(words));

            },
            _ => return Err(format!("Invalid operation")),
        };
        Ok(())
    }


    // This function will exeute a code block
    pub fn exec(&self, stack: &mut Stack, variables: &mut variables::Variables) -> Result<(), String> {
        match self {
            Value::Block(b) => {
                let mut tokens = b.iter();
                while let Some(token) = tokens.next() {
                    if variables.get(&token.to_string()).is_some() {        // If the token is in variables
                        stack.push(variables.get(&token.to_string()).unwrap().clone());        // push the value
                        continue;        // and continue
                    }
                    match token.as_str() {
                        "+" => {
                            arithmetic::add(stack)?;
                        },
                        "-" => {
                            arithmetic::sub(stack)?;
                        },
                        "*" => {
                            arithmetic::mul(stack)?;
                        },
                        "/" => {
                            arithmetic::div(stack)?;
                        },
                        "div" => {
                            arithmetic::int_div(stack)?;
                        },
                        "<" => {
                            arithmetic::lt(stack)?;
                        },
                        ">" => {
                            arithmetic::gt(stack)?;
                        },
                        "==" => {
                            arithmetic::eq(stack)?;
                        },
                        "dup" => {
                            stack.dup()?;
                        },
                        "swap" => {
                            stack.swap()?;
                        },
                        "pop" => {
                            stack.pop()?;
                        },
                        "words" => {
                            Self::words(stack)?;
                        },
                        "print" => {
                            io::print(stack, variables)?;
                        },
                        "read" => {
                            io::read(stack)?;
                        },
                        "parseInteger" => {
                            let input = stack.pop().unwrap();
                            Self::parse_integer(&input.to_string()).unwrap();
                            stack.push(input);
                        },
                        "parseFloat" => {
                            let input = stack.pop().unwrap();
                            Self::parse_float(&input.to_string()).unwrap();
                            stack.push(input);
                        },
                        "&&" => {
                            logic::and(stack)?;
                        },
                        "||" => {
                            logic::or(stack)?;
                        }
                        "not" => {
                            logic::not(stack)?;
                        },
                        "[" => {
                            let input = operations::flow::read_list(&mut tokens)?;
                            stack.push(input);
                        },
                        "{" => {
                            let input = operations::flow::read_block(&mut tokens)?;
                            stack.push(input);
                        },
                        "\"" => {
                            let input = operations::flow::read_string(&mut tokens);
                            stack.push(input);
                        },
                        "head" => {
                            operations::lists::head(stack)?;
                        },
                        "tail" => {
                            operations::lists::tail(stack)?;
                        },
                        "empty" => {
                            operations::lists::empty(stack)?;
                        },
                        "length" => {                            
                            operations::lists::lenght(stack)?;
                        },
                        "cons" => {
                            operations::lists::cons(stack)?;
                        },
                        "append" => {
                            operations::lists::append(stack)?;
                        },
                        "if" => {
                            operations::flow::if_block(&mut tokens, stack, variables)?;
                        },
                        "println" => {
                            io::println(stack, variables)?;
                        },
                        "each" => {
                            operations::lists::each(&mut tokens, stack, variables)?;
                        },
                        "map" => {
                            operations::lists::map(&mut tokens, stack, variables)?;
                        },
                        "foldl" => {
                            operations::lists::foldl(&mut tokens, stack, variables)?;
                        },
                        "loop" => {
                            operations::flow::r#loop(&mut tokens, stack, variables)?;
                        },
                        "times" => {
                            operations::flow::times(&mut tokens, stack, variables)?;
                        },
                        ":=" => {
                            operations::flow::assign(stack, variables)?;
                        },
                        _ => {
                            stack.push(convert(token));
                        }
                    }
                }
                Ok(())
            }
            _ => Err(format!("Invalid operation")),
        }
    }

    pub fn each(self, stack: &mut Stack, block: Self, variables: &mut variables::Variables) -> Result<(), String> {
        match (self, &block) {
            (Value::List(list), Value::Block(_)) => {
                for item in list {
                    stack.push(item.clone());
                    block.exec(stack, variables)?;
                }
                Ok(())
            },
            _ => Err(format!("Invalid operation")),
        }
    }

    pub fn map(self, stack: &mut Stack, block: Self, variables: &mut variables::Variables) -> Result<(), String> {
        match (self, &block) {
            (Value::List(list), Value::Block(_)) => {
                let mut result = Vec::new();
                for item in list {
                    stack.push(item.clone());
                    block.exec(stack, variables).unwrap();
                    result.push(stack.pop().unwrap());
                }
                stack.push(Value::List(result));
                Ok(())
            },
            _ => Err(format!("Invalid operation")),
        }
    }

    pub fn foldl(self, stack: &mut Stack, block: Self, start_value: Self, variables: &mut variables::Variables) -> Result<(), String> {
        match (self, &block) {
            (Value::List(list), Value::Block(_)) => {
                let mut result = start_value;
                for item in list {
                    stack.push(result.clone());
                    stack.push(item.clone());
                    block.exec(stack, variables).unwrap();
                    result = stack.pop().unwrap();
                }
                stack.push(result);
                Ok(())
            },
            _ => Err(format!("Invalid operation")),
        }
    }

}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::List(value)
    }
}

impl From<Vec<String>> for Value {
    fn from(value: Vec<String>) -> Self {
        Value::Block(value)
    }
}

impl Into<String> for Value {
    fn into(self) -> String {
        match self {
            Value::String(value) => value,
            Value::Int(value) => value.to_string(),
            Value::Float(value) => value.to_string(),
            Value::Boolean(value) => value.to_string(),
            Value::List(value) => format!("[{}]", value.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" ")),
            Value::Block(value) => format!("{:?}", value),
            Value::Symbol(value) => value,
        }
    }
}

impl Into<i64> for Value {
    fn into(self) -> i64 {
        match self {
            Value::Int(value) => value,
            Value::Float(value) => value as i64,
            _ => panic!("Invalid conversion"),
        }
    }
}

impl Into<bool> for Value {
    fn into(self) -> bool {
        match self {
            Value::Boolean(value) => value,
            Value::Int(value) => value != 0,
            Value::Float(value) => value != 0.0,
            _ => panic!("Invalid conversion"),
        }
    }
}

impl Add for Value {
    type Output = Result<Value, String>;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::List(mut a), Value::List(b)) => {
                a.extend(b);
                Ok(Value::List(a))
            },
            _ => Err(format!("Invalid operation +")),
        }
    }
}

impl Sub for Value {
    type Output = Result<Value, String>;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            _ => Err(format!("Invalid operation -")),
        }
    }
}

impl Mul for Value {
    type Output = Result<Value, String>;
    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            _ => Err(format!("Invalid operation *")),
        }
    }
}

impl Div for Value {
    type Output = Result<Value, String>;
    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => {
                if b == 0 {
                    Err(format!("Division by zero"))
                } else {
                    Ok(Value::Int(a / b))
                }
            },
            (Value::Float(a), Value::Float(b)) => {
                if b == 0.0 || a.is_nan() {
                    Err(format!("Division by zero or NaN"))
                } else {
                    Ok(Value::Float(a / b))
                }
            },
            _ => Err(format!("Invalid operation /")),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::List(a), Value::List(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.partial_cmp(b),
            (Value::Float(a), Value::Float(b)) => a.partial_cmp(b),
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
            (Value::Boolean(a), Value::Boolean(b)) => a.partial_cmp(b),
            (Value::List(a), Value::List(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Float(a) => write!(f, "{}", a),
            Value::Int(a) => write!(f, "{}", a),
            Value::String(a) => write!(f, "\" {} \"", a),
            Value::Boolean(a) => write!(f, "{}", a),
            Value::List(a) => {
                write!(f, "[ {} ]", a.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "))
            },
            Value::Block(a) => {
                write!{f, "{:?}", a}
            },
            Value::Symbol(a) => write!(f, "{}", a),
        }
    }
}

impl Not for Value {
    type Output = Value;

    fn not(self) -> Self::Output {
        match self {
            Value::Boolean(value) => Value::Boolean(!value),
            _ => panic!("Invalid operation"),
        }
    }
}

pub fn convert(value: &str) -> Value {
    if let Ok(value) = value.parse::<i64>() {
        Value::Int(value)
    } else if let Ok(value) = value.parse::<f64>() {
        Value::Float(value)
    } else if value == "true" {
        Value::Boolean(true)
    } else if value == "false" {
        Value::Boolean(false)
    } else {
        Value::Symbol(value.to_string())
    }
}