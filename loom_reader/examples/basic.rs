use std::fs;
use std::fmt;
use std::error::Error;
use loom_reader::{
    parse::{
        Exp, read_expressions
    },
    forms::Form,
};
use std::collections::HashMap;

#[derive(Debug)]
enum Prim {
    Nil,
    Value(String),
    Op { kind: String, args: Vec<ID> },
}

#[derive(Debug)]
struct Compiler {
    variables: Vec<Prim>,
    ids: HashMap<String, ID>, // Names to variable IDs
}

#[derive(Debug, Clone, Copy)]
enum ID {
    Nil,
    Index(usize),
}

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ID::Nil => write!(f, "nil"),
            ID::Index(i) => write!(f, "v{i}")
        }
    }
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
            ids: HashMap::new(),
        }
    }
    pub fn add_nil(&mut self) -> ID {
        self.variables.push(Prim::Nil);
        ID::Index(self.variables.len() - 1)
    }
    pub fn add_value(&mut self, x: String) -> ID {
        self.variables.push(Prim::Value(x));
        ID::Index(self.variables.len() - 1)
    }
    pub fn add_op(&mut self, kind: String, args: Vec<ID>) -> ID {
        self.variables.push(Prim::Op { kind, args });
        ID::Index(self.variables.len() - 1)
    }
    pub fn add_exp(&mut self, x: Exp) -> ID {
        match x {
            Exp::Nil => {
                self.add_nil()
            }
            Exp::SExp { .. } => {
                let Some(kind) = x.car_symbol() else { todo!() };
                if kind == "def" {
                    let Some(var_key) = x.arg_symbol(0) else { todo!() };
                    let Some(var_exp) = x.arg(1) else { todo!() };
                    let op_id = self.add_exp(var_exp);
                    self.ids.insert(var_key, op_id);
                    ID::Nil
                } else {
                    let mut ids: Vec<ID> = Vec::new();
                    if let Some(args) = x.args() {
                        for arg in args {
                            // Check if this is a named variable
                            if let Some(var_key) = arg.as_symbol() {
                                if let Some(var_id) = self.ids.get(&var_key) {
                                    ids.push(*var_id);
                                    continue;
                                }
                            }
                            self.add_exp(arg);
                            ids.push(ID::Index(self.variables.len() - 1));
                        }
                    }
                    self.add_op(kind, ids)
                }
            }
            Exp::Atom(v) => {
                self.add_value(v)
            }
            _ => {
                ID::Nil
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string("basic.loom")?;
    let expressions = read_expressions(source)?;

    let mut compiler = Compiler::new();

    for x in expressions {
        compiler.add_exp(x);
    }

    for (i, v) in compiler.variables.iter().enumerate() {
        let val = match v {
            Prim::Nil => format!("NIL"),
            Prim::Value(val) => format!("\"{val}\""),
            Prim::Op { kind, args } => {
                let mut args_string = format!("");
                for arg in args {
                    args_string = format!("{} {}", args_string, arg);
                }
                format!("{kind}{args_string}")
            }
        };
        println!("v{i} = {val}");
    }

    Ok(())
}
