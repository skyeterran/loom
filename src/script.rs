use std::collections::HashMap;
use super::parser::Object;

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
    pub objects: Vec<Object>,
    pub memory: Memory,
    pub index: usize,
}

impl Script {
    pub fn progress(&mut self) {
        match self.objects.get(self.index).unwrap().evaluate(&self.memory) {
            Ok(cmd) => {
                //println!("{:?}", cmd);
                match cmd {
                    Cmd::Noop => {},
                    Cmd::Say(s, d) => {
                        println!("{s}: {d}");
                    },
                    Cmd::Let(k, v) => {
                        println!("let {k} = {v}");
                        self.memory.map.insert(k, v);
                    }
                }
            },
            Err(e) => { println!("Error: {:?}", e) },
        }
        self.index += 1;
    }
}
