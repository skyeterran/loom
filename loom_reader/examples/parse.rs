use std::fs;
use std::error::Error;
use std::collections::HashMap;
use loom_reader::{
    parse::{
        Exp, read_expressions
    },
    forms::Form,
};

#[derive(Debug)]
enum Prim {
    Nil,
    Keyword(String),
    Value(String),
    Object {
        kind: String,
        props: HashMap<String, Prim>,
    },
}

#[derive(Debug)]
struct SignalSource(String);

// TODO: Build a macro reflection system for loom to use rust types?
// There needs to be a more streamlined way of doing this - traits?
#[derive(Debug)]
enum Image {
    Tex { path: String },
    TexSequence { paths: Vec<String>, time: SignalSource },
}

impl Prim {
    pub fn from_exp(x: Exp) -> Self {
        match x {
            Exp::Nil => Self::Nil,
            Exp::Atom(contents) => {
                if let Some(keyword) = contents.strip_prefix('@') {
                    Self::Keyword(keyword.to_string())
                } else {
                    Self::Value(contents.to_string())
                }
            }
            Exp::SExp { kind, args, kwargs } => {
                let mut props: HashMap<String, Prim> = HashMap::new();
                for (k, v) in kwargs.iter() {
                    props.insert(k.clone(), Prim::from_exp(v.clone()));
                }
                Self::Object { kind: kind.as_symbol().unwrap(), props }
            }
            _ => Self::Nil, // TODO: Remove this
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string("lazer.loom")?;
    //println!("SOURCE:\n{}", source);
    let expressions = read_expressions(source)?;

    for x in expressions {
        println!("{:#?}", Prim::from_exp(x));
    }

    Ok(())
}
