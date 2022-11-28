#![allow(unused_imports, dead_code, unused_mut)]

use std::env;
use std::fs;
use loom::parser::tokenize;

fn main() {
    let source = fs::read_to_string("script.loom").expect("Couldn't load file!");
    println!("{:#?}", tokenize(source));
}
