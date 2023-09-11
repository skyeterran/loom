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
    Modulo(Box<Expr>, Box<Expr>),
    IfElse(Box<Expr>, Vec<Expr>, Vec<Expr>),
    WhileLoop(Box<Expr>, Vec<Expr>),
    Call(String, Vec<Expr>),
    GlobalDataAddr(String),
    Sequence(Vec<Expr>),
    MakeArray(u32),
    GetArrayElem(Box<Expr>, Box<Expr>),
    SetArrayElem(Box<Expr>, Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn from_exp(x: &Exp) -> Self {
        match x {
            Exp::Symbol { contents } => {
                match contents.parse::<i32>() {
                    Ok(n) => Expr::Literal(format!("{n}")),
                    Err(_) => {
                        if contents.starts_with("&") {
                            Expr::GlobalDataAddr(contents.clone()
                                                         .strip_prefix("&")
                                                         .unwrap()
                                                         .to_string())
                        } else {
                            Expr::Identifier(contents.clone())
                        }
                    }
                }
            }
            Exp::Literal { contents } => {
                Expr::Literal(contents.clone())
            }
            Exp::SExp { car, cdr } => {
                let args: Vec<Box<Expr>> = cdr.iter().map(|i| {
                    Box::new(Expr::from_exp(i))
                }).collect();
                let mut name = String::new();
                match *car.clone() {
                    Exp::Symbol { contents } => {
                        name = contents;
                    }
                    _ =>  todo!()
                }
                match name.as_str() {
                    "+" => Expr::Add(args[0].clone(), args[1].clone()),
                    "-" => Expr::Sub(args[0].clone(), args[1].clone()),
                    "*" => Expr::Mul(args[0].clone(), args[1].clone()),
                    "/" => Expr::Div(args[0].clone(), args[1].clone()),
                    "=" => Expr::Eq(args[0].clone(), args[1].clone()),
                    "!=" => Expr::Ne(args[0].clone(), args[1].clone()),
                    "<" => Expr::Lt(args[0].clone(), args[1].clone()),
                    "<=" => Expr::Le(args[0].clone(), args[1].clone()),
                    ">" => Expr::Gt(args[0].clone(), args[1].clone()),
                    ">=" => Expr::Ge(args[0].clone(), args[1].clone()),
                    "%" => Expr::Modulo(args[0].clone(), args[1].clone()),
                    "if" => {
                        let truthy = *args[1].clone();
                        let truthy = match truthy {
                            Expr::Sequence(contents) => contents,
                            _ => vec![truthy]
                        };
                        if args.len() > 2 {
                            let falsy = *args[2].clone();
                            let falsy = match falsy {
                                Expr::Sequence(contents) => contents,
                                _ => vec![falsy]
                            };
                            Expr::IfElse(
                                args[0].clone(),
                                truthy,
                                falsy
                            )
                        } else {
                            Expr::IfElse(
                                args[0].clone(),
                                truthy,
                                vec![]
                            )
                        }
                    },
                    "set" => {
                        let varname: String = match *args[0].clone() {
                            Expr::Identifier(n) => n,
                            _ => todo!()
                        };
                        Expr::Assign(varname, args[1].clone())
                    },
                    "while" => {
                        let mut body: Vec<Expr> = Vec::new();
                        let mut i: usize = 0;
                        for a in &args {
                            if i > 0 {
                                body.push(*a.clone());
                            }
                            i += 1;
                        }
                        Expr::WhileLoop(args[0].clone(), body)
                    },
                    "do" => {
                        let mut body: Vec<Expr> = Vec::new();
                        for a in &args {
                            body.push(*a.clone());
                        }
                        Expr::Sequence(body)
                    }
                    "array" => {
                        let Some(length) = args.get(0) else { todo!() };
                        match *length.clone() {
                            Expr::Literal(contents) => {
                                let length: u32 = contents.parse().unwrap();
                                Expr::MakeArray(length)
                            }
                            _ => todo!()
                        }
                    }
                    "array_get" => {
                        let Some(addr) = args.get(0) else { todo!() };
                        let Some(index) = args.get(1) else { todo!() };
                        Expr::GetArrayElem(addr.clone(), index.clone())
                    }
                    "array_set" => {
                        let Some(addr) = args.get(0) else { todo!() };
                        let Some(index) = args.get(1) else { todo!() };
                        let Some(value) = args.get(2) else { todo!() };
                        Expr::SetArrayElem(addr.clone(), index.clone(), value.clone())
                    }
                    _ => {
                        let mut body: Vec<Expr> = Vec::new();
                        for a in &args {
                            body.push(*a.clone());
                        }
                        Expr::Call(name, body)
                    }
                }
            }
            _ => {
                Expr::Literal(format!("-1"))
            }
        }
    }
}
