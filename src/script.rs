use std::collections::HashMap;
use super::parser::Expr;

#[derive(Debug)]
pub struct Memory {
    pub map: HashMap<String, String>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            map: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub enum Cmd {
    Noop, // Do nothing
    Say(String, String), // Say a line of dialogue
    Let(String, String), // Insert a value into memory
}

#[derive(Debug)]
pub struct Script {
    pub objects: Vec<Expr>,
    pub memory: Memory,
    pub index: usize,
}
