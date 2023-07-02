use loom_reader::parse::Exp;

/// The AST node for expressions.
#[derive(Debug, Clone)]
pub enum Expr {
    Literal(String),
    Identifier(String),
    Assign(String, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Ne(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Le(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Ge(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    IfElse(Box<Expr>, Vec<Expr>, Vec<Expr>),
    WhileLoop(Box<Expr>, Vec<Expr>),
    Call(String, Vec<Expr>),
    GlobalDataAddr(String),
}

impl Expr {
    pub fn from_exp(x: Exp) -> Self {
        match x {
            Exp::Symbol { contents } => {
                match contents.parse::<i32>() {
                    Ok(n) => Expr::Literal(format!("{n}")),
                    Err(_) => {
                        Expr::Identifier(contents)
                    }
                }
            }
            Exp::Literal { contents } => {
                Expr::Literal(contents)
            }
            Exp::SExp { car, cdr } => {
                let args: Vec<Box<Expr>> = cdr.iter().map(|i| {
                    Box::new(Expr::from_exp(i.clone()))
                }).collect();
                let mut name = String::new();
                match *car {
                    Exp::Symbol { contents } => {
                        name = contents;
                    }
                    _ => { todo!(); }
                }
                match name.as_str() {
                    "+" => Expr::Add(args[0].clone(), args[1].clone()),
                    "-" => Expr::Sub(args[0].clone(), args[1].clone()),
                    "*" => Expr::Mul(args[0].clone(), args[1].clone()),
                    "/" => Expr::Div(args[0].clone(), args[1].clone()),
                    "if" => {
                        if args.len() > 2 {
                            Expr::IfElse(
                                args[0].clone(),
                                vec![*args[1].clone()],
                                vec![*args[2].clone()]
                            )
                        } else {
                            Expr::IfElse(
                                args[0].clone(),
                                vec![*args[1].clone()],
                                vec![]
                            )
                        }
                    },
                    _ => { todo!() }
                }
            }
            _ => {
                Expr::Literal(format!("-1"))
            }
        }
    }
}
