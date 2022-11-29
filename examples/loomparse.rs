#![allow(unused_imports, dead_code, unused_mut)]

use std::env;
use std::fs;
use loom::parser::{tokenize, Object, Object::*, ParseError, ParseError::*};

fn main() -> Result<(), ParseError> {
    let source = fs::read_to_string("script.loom").expect("Couldn't load file!");
    
    let tokens = tokenize(source)?;
    //println!("{:#?}", tokens);

    let object = Object::from_tokens(tokens)?;
    //println!("{:#?}", object);

    let List(elements) = object else {
        return Err(ImproperRoot);
    };
    for element in elements {
        match element {
            List(items) => {
                let Some(func) = items.first() else {
                    return Err(UnknownType);
                };
                let Symbol(s) = func else {
                    return Err(UnknownSymbol);
                };
                match s.as_str() {
                    "say" => {
                        let Some(speaker) = items.get(1) else {
                            return Err(MissingArgument);
                        };
                        let Some(dialogue) = items.get(2) else {
                            return Err(MissingArgument);
                        };
                        match (speaker, dialogue) {
                            (Symbol(s), LitString(d)) => {
                                println!("{s}: {d}");
                            },
                            _ => {
                                return Err(Unknown);
                            }
                        }
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }

    Ok(())
}
