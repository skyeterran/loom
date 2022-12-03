#![allow(unused_imports, dead_code, unused_mut)]

use std::env;
use std::fs;
use loom::parser::{tokenize, tokens_to_exp};
use loom::exp::{LoomEnv, LoomExp, LoomErr};

fn main() -> Result<(), LoomErr> {
    let source = fs::read_to_string("eval.loom").expect("Couldn't load file!");
    let tokens = tokenize(source).unwrap();
    let exp = tokens_to_exp(tokens).unwrap();
    let mut env = LoomEnv::default();

    match exp {
        LoomExp::List(list) => {
            for item in list {
                match item.eval(&mut env) {
                    Ok(_) => {},
                    Err(e) => { println!("Error: {:?}", e) },
                }
            }
        },
        _ => { return Ok(()) },
    }

    Ok(())
}
