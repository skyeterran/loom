#![allow(unused_imports, dead_code, unused_mut)]

use std::env;
use std::fs;
use loom::script::Script;

fn main() {
    let source = fs::read_to_string("script.loom").expect("Couldn't load file!");

    //let mut script = Script::parse(source);
    //println!("{:#?}", script);

    let mut new_lines: Vec<String> = Vec::new();
    for line in source.split("\n") {
        let raw = line.trim();
        let Some(first_char) = raw.chars().next() else {
            continue;
        };
        let Some((speaker, content)) = raw.split_once(": ") else {
            new_lines.push(raw.to_string());
            continue;
        };
        new_lines.push(format!("(say {speaker} \"{content}\")"));
    }
    for line in new_lines {
        println!("{line}");
    }
}
