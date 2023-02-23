use std::io::stdout;
use std::io::{self, Write};
use std::env;
use std::fs;
use std::collections::HashMap;

enum Line {
    Normal(&'static str),
    Choice((&'static str, usize), (&'static str, usize)),
}

struct Conversation {
    index: (usize, usize),
    lines: Vec<Vec<Line>>,
}

impl Conversation {
    // To feed the convo a response
    fn say<S: Into<String>>(&mut self, input: S) -> Option<String> {
        let in_string = input.into();
        let Some(branch) = self.lines.get(self.index.0) else { return None };
        match branch.get(self.index.1) {
            Some(line) => {
                match line {
                    Line::Normal(s) => {
                        self.index = (self.index.0, self.index.1 + 1);
                        Some(s.to_string())
                    },
                    Line::Choice((choice_a, index_a), (choice_b, index_b)) => {
                        if in_string == *choice_a {
                            self.index = (*index_a, 1);
                            let Some(next_branch) = self.lines.get(self.index.0) else {
                                return None;
                            };
                            if let Some(Line::Normal(next_line)) = next_branch.first() {
                                return Some(next_line.to_string());
                            } else {
                                return None;
                            }
                        }
                        if in_string == *choice_b {
                            self.index = (*index_b, 1);
                            let Some(next_branch) = self.lines.get(self.index.0) else {
                                return None;
                            };
                            if let Some(Line::Normal(next_line)) = next_branch.first() {
                                return Some(next_line.to_string());
                            } else {
                                return None;
                            }
                        }
                        Some("Sorry, what?".to_string()) 
                    }
                }
            },
            None => None
        }
    }
}

fn main() -> io::Result<()> {
    use Line::*;

    // Make a state machine
    let mut convo = Conversation {
        index: (0, 0),
        lines: vec![
            vec![
                Normal("Hello, there!"),
                Normal("How are you? (good / bad)"),
                Choice(("good", 1), ("bad", 2)),
            ],
            vec![
                Normal("Great to hear it!"),
            ],
            vec![
                Normal("Oh, no! What's wrong? (dog ate lunch / just sad)"),
                Choice(("dog ate lunch", 3), ("just sad", 4)),
            ],
            vec![
                Normal("...How did that even happen?"),
            ],
            vec![
                Normal("Well, just know that I care about you. *hug*"),
            ],
        ],
    };

    // Make an env
    let mut data = HashMap::<String, String>::new();

    // Let's just print the first line for now
    match convo.say("") {
        Some(response) => {
            println!("{}", response);
        },
        None => {},
    }

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
                        println!("Farewell, traveler!");
                        break;
                    },
                    "env" => {
                        println!("{:#?}", data);
                    },
                    _ => {
                        if let Some((k, v)) = input.split_once(": ") {
                            data.insert(k.to_string(), v.to_string());
                        } else {
                            match convo.say(input) {
                                Some(response) => {
                                    println!("{}", response);
                                },
                                None => { break; },
                            }
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

    println!("The end.");
    Ok(())
}
