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
    Op { kind: OpKind, args: Vec<ID> },
    Fn(Function),
}

#[derive(Debug)]
enum OpKind {
    Global(String), // Named
    Local(ID), // ID'ed
}

impl fmt::Display for OpKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpKind::Global(name) => write!(f, "{name}"),
            OpKind::Local(id) => write!(f, "{id}")
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ID {
    Nil,
    Var(usize),
    Param(usize),
}

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ID::Nil => write!(f, "nil"),
            ID::Var(i) => write!(f, "v{i}"),
            ID::Param(i) => write!(f, "p{i}"),
        }
    }
}

#[derive(Debug)]
struct Function {
    parameters: Vec<()>,
    variables: Vec<Prim>,
    ids: HashMap<String, ID>, // Names to variable IDs
}

impl Function {
    pub fn new() -> Self {
        Self {
            parameters: Vec::new(),
            variables: Vec::new(),
            ids: HashMap::new(),
        }
    }
    pub fn add_param(&mut self, name: String) -> ID {
        self.parameters.push(());
        let id = ID::Param(self.parameters.len() - 1);
        self.ids.insert(name, id);
        id
    }
    pub fn add_nil(&mut self) -> ID {
        self.variables.push(Prim::Nil);
        ID::Var(self.variables.len() - 1)
    }
    pub fn add_value(&mut self, x: String) -> ID {
        self.variables.push(Prim::Value(x));
        ID::Var(self.variables.len() - 1)
    }
    pub fn add_function(&mut self, f: Function) -> ID {
        self.variables.push(Prim::Fn(f));
        ID::Var(self.variables.len() - 1)
    }
    pub fn add_op(&mut self, kind: String, args: Vec<ID>) -> ID {
        let kind: OpKind = match self.ids.get(&kind) {
            Some(id) => OpKind::Local(*id),
            None => OpKind::Global(kind),
        };
        self.variables.push(Prim::Op { kind, args });
        ID::Var(self.variables.len() - 1)
    }
    pub fn add_exp(&mut self, x: Exp) -> ID {
        match x {
            Exp::Nil => {
                self.add_nil()
            }
            Exp::SExp { .. } => {
                let Some(kind) = x.car_symbol() else { todo!() };
                match kind.as_str() {
                    "def" => {
                        let Some(var_key) = x.arg_symbol(0) else { todo!() };
                        let Some(var_exp) = x.arg(1) else { todo!() };
                        let op_id = self.add_exp(var_exp);
                        self.ids.insert(var_key, op_id);
                        ID::Nil
                    }
                    "fn" => {
                        let mut f = Function::new();
                        // Extract parameters
                        let Some(Exp::List(params)) = x.arg(0) else { todo!() };
                        for p in params {
                            let Some(p_name) = p.as_symbol() else { todo!() };
                            f.add_param(p_name);
                        }
                        // Extract body expressions
                        if let Some(body) = x.args() {
                            let mut bi = 0;
                            for b in body {
                                if bi > 0 {
                                    f.add_exp(b.clone());
                                }
                                bi += 1;
                            }
                        }
                        self.add_function(f)
                    }
                    _ => {
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
                                ids.push(ID::Var(self.variables.len() - 1));
                            }
                        }
                        self.add_op(kind, ids)
                    }
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

fn debug_fn(f: &Function, nesting: usize) -> String {
    let mut result = String::new();
    for (i, v) in f.variables.iter().enumerate() {
        let indent = " ".repeat(nesting * 4);
        let val = match v {
            Prim::Nil => format!("NIL"),
            Prim::Value(val) => format!("{val}"),
            Prim::Op { kind, args } => {
                let args: Vec<String> = args.iter().map(|x| format!("{x}")).collect();
                let args_string = args.connect(", ");
                format!("{kind}({args_string})")
            }
            Prim::Fn(f) => {
                format!("{{{}\n{}}}", debug_fn(f, nesting + 1), indent)
            }
        };
        result = format!("{result}\n{indent}v{i} = {val}");
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string("basic.loom")?;
    println!("SOURCE:\n{}", source);
    let expressions = read_expressions(source)?;

    let mut main = Function::new();

    for x in expressions {
        main.add_exp(x);
    }

    println!("COMPILED:{}", debug_fn(&main, 0));

    Ok(())
}
