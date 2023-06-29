use std::fs;
use std::error::Error;
use loom_reader::parse::{
    Exp, read_expressions
};

fn main() -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string("test.loom")?;
    let expressions = read_expressions(source)?;

    for x in expressions {
        println!("{x}");
        //println!("{x:#?}\n");
    }

    Ok(())
}
