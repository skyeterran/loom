use loom_reader::parse::Exp;

pub fn eval_expression(x: Exp) -> Exp {
    match x {
        Exp::SExp { car, cdr } => {
            match *car {
                Exp::Symbol { contents } => {
                    match contents.as_str() {
                        "print" => {
                            for i in cdr {
                                println!("{i}");
                            }
                            Exp::Nil
                        }
                        _ => Exp::Nil
                    }
                }
                _ => Exp::Nil
            }
        }
        _ => x
    }
}
