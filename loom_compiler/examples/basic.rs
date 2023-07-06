use std::fs;
use std::error::Error;
use loom_reader::parse::{
    Exp, read_expressions
};
use loom_compiler::frontend::Expr;

const test_code: &str = r#"
    (set i 0)
    (set b 2)
    (if (= i 0)
        (do
            (set b 9)
            (set b 3)
        )
    )
    (while (> i 10)
        (set i (+ i 1))
    )
"#;

fn main() -> Result<(), Box<dyn Error>> {
    //let source = fs::read_to_string("test.loom")?;
    let source = test_code.to_string();
    let expressions = read_expressions(source)?;

    for x in expressions {
        println!("{x}");
        println!("--> {:?}", Expr::from_exp(&x));
    }

    Ok(())
}
