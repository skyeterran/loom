use std::fs;
use std::error::Error;
use loom_reader::{
    parse::{
        Exp, read_expressions
    },
    forms::Form,
};

#[derive(Debug)]
enum Prim {
    Nil,
    Value(String),
    Op { kind: String, args: Vec<usize> },
}

#[derive(Debug)]
struct Compiler {
    variables: Vec<Prim>
}

impl Compiler {
    pub fn new() -> Self {
        Self { variables: Vec::new() }
    }
    pub fn add_nil(&mut self) -> usize {
        self.variables.push(Prim::Nil);
        self.variables.len() - 1
    }
    pub fn add_value(&mut self, x: String) -> usize {
        self.variables.push(Prim::Value(x));
        self.variables.len() - 1
    }
    pub fn add_op(&mut self, kind: String, args: Vec<usize>) -> usize {
        self.variables.push(Prim::Op { kind, args });
        self.variables.len() - 1
    }
    pub fn add_exp(&mut self, x: Exp) -> usize {
        match x {
            Exp::SExp { .. } => {
                let mut ids: Vec<usize> = Vec::new();
                if let Some(args) = x.args() {
                    for arg in args {
                        self.add_exp(arg);
                        ids.push(self.variables.len() - 1);
                    }
                }
                let Some(kind) = x.car_symbol() else { todo!() };
                self.add_op(kind, ids)
            }
            Exp::Atom(v) => {
                self.add_value(v)
            }
            _ => {
                self.add_nil()
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
                    args_string = format!("{} v{}", args_string, arg);
                }
                format!("{kind}{args_string}")
            }
        };
        println!("v{i} = {val}");
    }

    Ok(())
}
