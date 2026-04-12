use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Value {
    Number(i32),
    Bool(bool),
    String(String),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Self::Number(value) => value.to_string(),
            Self::Bool(value) => value.to_string(),
            Self::String(value) => (*value).clone(),
        }
    }
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OpCode {
    Push(Value),
    Syscall,
    Add,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BytecodeFile {
    pub lines: Vec<OpCode>,
    pub version: String,
}
