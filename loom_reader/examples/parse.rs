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
    Keyword(String),
}

fn main() -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string("lazer.loom")?;
    //println!("SOURCE:\n{}", source);
    let expressions = read_expressions(source)?;

    for x in expressions {
        println!("{:#?}", x);
    }

    Ok(())
}
