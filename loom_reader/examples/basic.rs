use std::fs;
use std::error::Error;
use loom_reader::{
    parse::{
        Exp, read_expressions
    },
    forms::Form,
};


fn main() -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string("basic.loom")?;
    let expressions = read_expressions(source)?;

    for x in expressions {
        println!("{x}");
        //println!("{x:#?}\n");
        let form = Form::from_exp(x);
        println!("{form:?}");
    }

    Ok(())
}
