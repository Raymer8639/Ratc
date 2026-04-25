use rkyv::{Archive, Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Archive, Debug, Clone)]
pub enum Value {
    Number(i32),
    Bool(bool),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => write!(f, "{}", value),
            Self::Bool(value) => write!(f, "{}", value),
            Self::String(value) => write!(f, "{}", value),
        }
    }
}

impl Value {
    pub fn parse(input: &str) -> Self {
        if let Ok(b) = input.parse::<bool>() {
            return Value::Bool(b);
        }
        if let Ok(n) = input.parse::<i32>() {
            return Value::Number(n);
        }
        Value::String(input.to_string())
    }
}

#[derive(Serialize, Deserialize, Archive, Debug, Clone)]
pub enum OpCode {
    Push(Value),
    Syscall,
    Add,
    Rm,
    Null,
}
#[derive(Serialize, Deserialize, Archive, Debug, Clone)]
pub struct BytecodeFile {
    pub lines: Vec<OpCode>,
    pub version: String,
}
