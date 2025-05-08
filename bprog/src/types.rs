use std::{fmt::Display, ops::{Add, Div, Mul, Not, Sub}};

use crate::operations::{self, arithmetic, io, logic};
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

    pub fn exec(&self) -> Result<Self, String> {
        match self {
            Value::Block(b) => {
                let mut stack = Stack::new();
                let mut tokens = b.iter();
                while let Some(token) = tokens.next() {
                    match token.as_str() {
                        "+" => {
                            arithmetic::add(&mut stack)?;
                        },
                        "-" => {
                            arithmetic::sub(&mut stack)?;
                        },
                        "*" => {
                            arithmetic::mul(&mut stack)?;
                        },
                        "/" => {
                            arithmetic::div(&mut stack)?;
                        },
                        "div" => {
                            arithmetic::int_div(&mut stack)?;
                        },
                        "<" => {
                            arithmetic::lt(&mut stack)?;
                        },
                        ">" => {
                            arithmetic::gt(&mut stack)?;
                        },
                        "==" => {
                            arithmetic::eq(&mut stack)?;
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
                            Self::words(&mut stack)?;
                        },
                        "print" => {
                            io::print(&mut stack)?;
                        },
                        "read" => {
                            io::read(&mut stack)?;
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
                            logic::and(&mut stack)?;
                        },
                        "||" => {
                            logic::or(&mut stack)?;
                        }
                        "not" => {
                            logic::not(&mut stack)?;
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
                        _ => {
                            stack.push(convert(token));
                        }
                    }
                }
                if stack.len() == 1 {
                    Ok(stack.pop()?)
                } else {
                    panic!("Invalid stack size")
                }
            }
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