#![allow(unused_imports, dead_code, unused_mut)]

use std::env;
use std::fs;
use loom::parser::{tokenize, Object, ParseError};

fn main() -> Result<(), ParseError> {
    let source = fs::read_to_string("script.loom").expect("Couldn't load file!");
    
    let tokens = tokenize(source)?;
    //println!("{:#?}", tokens);

    let object = Object::from_tokens(tokens)?;
    println!("{:#?}", object);

    Ok(())
}
