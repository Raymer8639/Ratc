use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OpCode {
    Push(i32),
    Add,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BytecodeFile {
    pub lines: Vec<OpCode>,
    pub version: String,
}
