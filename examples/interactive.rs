#![allow(unused_imports, dead_code, unused_mut, unused_variables)]

use std::io::stdout;
use std::io::{self, Write};
use std::env;
use std::fs;
use loom::parser::{tokenize, Expr, ParseError, ParseError::*};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let source = fs::read_to_string("test.loom").expect("Couldn't load file!");
    let tokens = tokenize(source).unwrap();
    let Expr::List(exprs) = Expr::from_tokens(tokens).unwrap() else { panic!() };

    let mut in_buffer = String::new();
    let mut i: usize = 0;
    loop {
        // Get user input
        print!("> ");
        stdout().flush().unwrap();
        match io::stdin().read_line(&mut in_buffer) {
            Ok(_) => {
                let Some(input) = in_buffer.strip_suffix("\n") else {
                    println!("No input!");
                    break;
                };
                match input {
                    "" => {
                        println!("Next!");
                    },
                    "q" => {
                        println!("Farewell, traveler!");
                        break;
                    },
                    "mem" => {
                        println!("{:#?}", script.memory);
                    },
                    _ => {
                        println!("Unrecognized input: `{input}`");
                    },
                }
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
        // Remember to clear the input buffer!
        in_buffer.clear();
    }

    println!("The end.");
    Ok(())
}
