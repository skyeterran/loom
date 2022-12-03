use std::fmt;
use std::collections::HashMap;

#[derive(Clone)]
pub enum LoomExp {
    Unit,
    Nil,
    Symbol(String),
    Number(f64),
    FString(String),
    List(Vec<LoomExp>),
    Func(fn(&[LoomExp]) -> Result<LoomExp, LoomErr>),
}

impl LoomExp {
    pub fn eval(&self, env: &mut LoomEnv) -> Result<LoomExp, LoomErr> {
        match self {
            LoomExp::Unit => { Ok(LoomExp::Unit) },
            LoomExp::Nil => { Ok(LoomExp::Nil) },
            LoomExp::Symbol(k) => {
                env.data.get(k)
                .ok_or(
                    LoomErr::Reason(format!("Unexpected symbol: '{}'", k))
                )
                .map(|x| x.clone())
            },
            LoomExp::Number(_) => { Ok(self.clone()) },
            LoomExp::FString(_) => { Ok(self.clone()) },
            LoomExp::List(list) => {
                let first_form = list.first()
                                     .ok_or(LoomErr::Reason(
                                             "Expected a non-empty list".to_string()
                                             ))?;
                let arg_forms = &list[1..];
                let first_eval = first_form.eval(env)?;
                match first_eval {
                    LoomExp::Func(f) => {
                        let args_eval = arg_forms.iter()
                                                 .map(|x| x.eval(env))
                                                 .collect::<Result<Vec<LoomExp>, LoomErr>>();
                        f(&args_eval?)
                    },
                    _ => Err(LoomErr::Reason("First form must be a function name".to_string()))
                }
            },
            LoomExp::Func(_) => { Err(LoomErr::Reason("Unexpected form".to_string())) },
        }
    }
}

impl fmt::Display for LoomExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            LoomExp::Unit => { format!("Unit") },
            LoomExp::Nil => { format!("Nil") },
            LoomExp::Symbol(s) => s.clone(),
            LoomExp::Number(n) => n.to_string(),
            LoomExp::FString(fs) => format!("\"{fs}\""),
            LoomExp::List(list) => {
                let xs: Vec<String> = list.iter()
                                          .map(|x| x.to_string())
                                          .collect();
                format!("({})", xs.join(","))
            },
            LoomExp::Func(_) => "Function {}".to_string(),
        };

        write!(f, "{}", str)
    }
}

impl fmt::Debug for LoomExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoomExp::Unit => { write!(f, "Unit") },
            LoomExp::Nil => { write!(f, "Nil") },
            LoomExp::Symbol(s) => { write!(f, "Symbol({s})") },
            LoomExp::Number(n) => { write!(f, "Number({n})") },
            LoomExp::FString(fs) => { write!(f, "FString(\"{}\")", fs) },
            LoomExp::List(list) => {
                let mut lines: Vec<String> = Vec::new();
                for item in list {
                    lines.push(format!("{:?}", item));
                }
                write!(f, "List({})", lines.join(", "))
            },
            LoomExp::Func(_) => { write!(f, "Function call") },
        }
    }
}

#[derive(Debug)]
pub enum LoomErr {
    Reason(String),
}

#[derive(Clone)]
pub struct LoomEnv {
    data: HashMap<String, LoomExp>,
}

impl Default for LoomEnv {
    fn default() -> Self {
        let mut data: HashMap<String, LoomExp> = HashMap::new();
        data.insert(
            "+".to_string(),
            LoomExp::Func(
                |args: &[LoomExp]| -> Result<LoomExp, LoomErr> {
                    let floats = parse_float_list(args)?;
                    let sum = floats.iter().fold(0.0, |sum, a| sum + a);

                    Ok(LoomExp::Number(sum))
                }
            )
        );

        data.insert(
            "print".to_string(),
            LoomExp::Func(
                |args: &[LoomExp]| -> Result<LoomExp, LoomErr> {
                    for arg in args {
                        match arg {
                            LoomExp::FString(fs) => { println!("{fs}") },
                            LoomExp::Number(n) => { println!("{n}") },
                            _ => {},
                        }
                    }
                    Ok(LoomExp::Unit)
                }
            )
        );

        LoomEnv { data }
    }
}

fn parse_float_list(args: &[LoomExp]) -> Result<Vec<f64>, LoomErr> {
    args.iter()
        .map(|x| parse_float(x))
        .collect()
}

fn parse_float(exp: &LoomExp) -> Result<f64, LoomErr> {
    match exp {
        LoomExp::Number(n) => Ok(*n),
        _ => Err(LoomErr::Reason("Expected a number".to_string()))
    }
}
