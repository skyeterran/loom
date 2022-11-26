#![allow(unused_imports)]

use std::env;
use std::fs;

#[derive(Debug)]
enum Line {
    Dialogue(String),
    Command(String),
}

#[derive(Debug)]
struct Script {
    lines: Vec<Line>,
}

impl Script {
    pub fn from_source(source: String) -> Self {
        let mut lines: Vec<Line> = Vec::new();
        let raw_lines = source.split("\n");
        
        for raw_line in raw_lines {
            if let Some(first_char) = raw_line.chars().next() {
                match first_char {
                    '(' => { lines.push(Line::Command(raw_line.to_string())) },
                    _ => { lines.push(Line::Dialogue(raw_line.to_string())) },
                }
            }
        }

        Script {
            lines: lines
        }
    }
}

fn main() {
    let source = fs::read_to_string("script.loom").expect("Couldn't load file!");
    //println!("Source:\n```\n{}\n```", source);

    let mut script = Script::from_source(source);
    println!("{:#?}", script);
}
