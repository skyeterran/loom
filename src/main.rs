#![allow(dead_code, unused_mut, unused_variables)]

use std::io::stdout;
use std::io::{self, Write};
use std::collections::HashMap;

#[derive(Debug)]
struct Convo {
    blocks: Vec<Block>,
}

impl Convo {
    fn get_line(&self, index: &mut (usize, usize)) -> Option<&String> {
        // Try to get a block and return early if impossible
        let Some(block) = self.blocks.get(index.0) else { return None };
        let Some(line) = block.lines.get(index.1) else {
            // We've run out of lines in this block, go to the next one
            index.0 += 1;
            index.1 = 0;
            return self.get_line(index);
        };

        // Move the line index forward
        index.1 += 1;

        Some(line)
    }
}

#[derive(Debug)]
struct Block {
    lines: Vec<String>,
}

#[derive(Debug)]
struct KeyStore {
    map: HashMap<&'static str, bool>,
}

impl KeyStore {
    fn check(&self, key: &'static str) -> Option<&bool> {
        self.map.get(key)
    }
}

fn main() -> io::Result<()> {
    let mut keys = KeyStore {
        map: HashMap::new(),
    };
    
    let mut convo = Convo {
        blocks: vec![
            Block {
                lines: vec![
                    format!("Oh, hi there!"),
                    format!("What's your name?"),
                ],
            },
            Block {
                lines: vec![
                    format!("This is the second block!"),
                    format!("I'm glad you reached this place.")
                ],
            },
            Block {
                lines: vec![
                    format!("Now you're in the third block."),
                    format!("Hell yeah, sister.")
                ],
            },
        ],
    };
    
    // Start the story
    let mut line_index: (usize, usize) = (0, 0);
    println!("{}", convo.get_line(&mut line_index).unwrap());

    let mut in_buffer = String::new();
    loop {
        // Get user input
        print!("> ");
        stdout().flush().unwrap();
        match io::stdin().read_line(&mut in_buffer) {
            Ok(_) => {
                let input = in_buffer.strip_suffix("\n").unwrap();
                match input {
                    "" => {
                        let line = convo.get_line(&mut line_index);
                        match line {
                            Some(l) => {
                                println!("{}", l);
                            }
                            None => { break; }
                        }
                    }
                    "q" => {
                        println!("Farewell, traveler!");
                        break;
                    }
                    _ => {
                        println!("Unrecognized input: `{input}`");
                    }
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
