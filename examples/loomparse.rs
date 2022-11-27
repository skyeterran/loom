#![allow(unused_imports)]

use std::env;
use std::fs;

#[derive(Debug)]
struct ParseError;

#[derive(Debug)]
enum Phrase {
    Dialogue(Line),
    Command(String),
}

impl Phrase {
    fn parse(source: String) -> Result<Self, ParseError> {
        if let Some(first_char) = source.chars().next() {
            match first_char {
                '(' => { return Ok(Phrase::Command(source.to_string())) },
                _ => {
                    let Some(line) = source.split_once(": ") else {
                        return Err(ParseError)
                    };
                    return Ok(Phrase::Dialogue(
                            Line {
                                speaker: line.0.to_string(),
                                content: line.1.to_string(),
                            })
                        );
                },
            }
        } else { return Err(ParseError) }
    }
}

#[derive(Debug)]
struct Line {
    speaker: String,
    content: String,
}

#[derive(Debug)]
struct Script {
    lines: Vec<Phrase>,
}

impl Script {
    pub fn parse(source: String) -> Self {
        let mut lines: Vec<Phrase> = Vec::new();
        let raw_lines = source.split("\n");
        
        for raw_line in raw_lines {
            let Ok(phrase) = Phrase::parse(raw_line.to_string()) else { continue; };
            lines.push(phrase);
        }

        Script {
            lines: lines
        }
    }
}

fn main() {
    let source = fs::read_to_string("script.loom").expect("Couldn't load file!");
    //println!("Source:\n```\n{}\n```", source);

    let mut script = Script::parse(source);
    println!("{:#?}", script);
}
