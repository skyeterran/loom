use std::env;
use std::fs;
use std::io::stdout;
use std::io::{self, Write};
use loom::parser::{tokenize, tokens_to_exp};
use loom::exp::{LoomEnv, LoomExp, LoomErr};

fn main() -> Result<(), LoomErr> {
    let mut env = LoomEnv::default();

    let mut in_buffer = String::new();
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
                    "q" => {
                        println!("Farewell!");
                        break;
                    },
                    _ => {
                        let tokens = tokenize(input.to_string()).unwrap();
                        match tokens_to_exp(tokens) {
                            Ok(exp) => {
                                    match exp.eval(&mut env) {
                                    Ok(r) => { println!("{r}"); },
                                    Err(e) => { println!("EVALUATION ERROR:\n{:?}", e); }
                                }
                            }
                            Err(e) => { println!("PARSING ERROR:\n{:?}", e); }
                        }
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

    Ok(())
}
