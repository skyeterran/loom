use std::fmt;
use std::collections::HashMap;

#[derive(Clone)]
pub enum LoomExp {
    True,
    False,
    Symbol(String),
    Number(f64),
    FString(String),
    List(Vec<LoomExp>),
    Func(fn(&[LoomExp], &mut LoomEnv) -> Result<LoomExp, LoomErr>),
    Macro(fn(&[LoomExp], &mut LoomEnv) -> Result<LoomExp, LoomErr>),
}

impl LoomExp {
    pub fn eval(&self, env: &mut LoomEnv) -> Result<LoomExp, LoomErr> {
        match self {
            LoomExp::True => { Ok(LoomExp::True) },
            LoomExp::False => { Ok(LoomExp::False) },
            LoomExp::Symbol(k) => {
                let sym = match env.data.get(k) {
                    Some(v) => v,
                    None => self
                };
                Ok(sym.clone())
            },
            LoomExp::Number(_) => { Ok(self.clone()) },
            LoomExp::FString(_) => { Ok(self.clone()) },
            LoomExp::List(list) => {
                let Some(first_form) = list.first() else {
                    // Empty lists are NIL (false, here)
                    return Ok(LoomExp::False);
                };
                let arg_forms = &list[1..];
                if let LoomExp::List(_) = first_form {
                    // If this is a pure list, do NOT evaluate it.
                    return Ok(LoomExp::True);
                }
                let first_eval = first_form.eval(env)?;
                match first_eval {
                    LoomExp::Func(f) => {
                        let args_eval = arg_forms.iter()
                                                 .map(|x| x.eval(env))
                                                 .collect::<Result<Vec<LoomExp>, LoomErr>>();
                        f(&args_eval?, env)
                    },
                    LoomExp::Macro(m) => {
                        m(&arg_forms, env)
                    },
                    _ => {
                        // Everything else should be true
                        Ok(LoomExp::True)
                    }
                }
            },
            LoomExp::Func(_) => { Err(LoomErr::Reason("Unexpected form".to_string())) },
            LoomExp::Macro(_) => { Err(LoomErr::Reason("Unexpected form".to_string())) },
        }
    }
}

impl fmt::Display for LoomExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            LoomExp::True => { format!("true") },
            LoomExp::False => { format!("false") },
            LoomExp::Symbol(s) => s.clone(),
            LoomExp::Number(n) => n.to_string(),
            LoomExp::FString(fs) => fs.clone(),
            LoomExp::List(list) => {
                let xs: Vec<String> = list.iter()
                                          .map(|x| x.to_string())
                                          .collect();
                format!("({})", xs.join(","))
            },
            LoomExp::Func(_) => "Function {}".to_string(),
            LoomExp::Macro(_) => "Macro {}".to_string(),
        };

        write!(f, "{}", str)
    }
}

impl fmt::Debug for LoomExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoomExp::True => { write!(f, "true") },
            LoomExp::False => { write!(f, "false") },
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
            LoomExp::Macro(_) => { write!(f, "Macro call") },
        }
    }
}

#[derive(Debug)]
pub enum LoomErr {
    Reason(String),
}

#[derive(Debug, Clone)]
pub struct LoomEnv {
    data: HashMap<String, LoomExp>,
}

impl Default for LoomEnv {
    fn default() -> Self {
        let mut data: HashMap<String, LoomExp> = HashMap::new();
        data.insert(
            "+".to_string(),
            LoomExp::Func(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    let floats = parse_float_list(args)?;
                    let sum = floats.iter().fold(0.0, |sum, a| sum + a);

                    Ok(LoomExp::Number(sum))
                }
            )
        );

        data.insert(
            "print".to_string(),
            LoomExp::Func(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    for arg in args {
                        match arg {
                            LoomExp::FString(fs) => { println!("{fs}") },
                            _ => { println!("{arg}") },
                        }
                    }
                    Ok(LoomExp::True)
                }
            )
        );

        data.insert(
            "let".to_string(),
            LoomExp::Macro(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    let Some(LoomExp::Symbol(k)) = args.first() else {
                        return Err(LoomErr::Reason(format!("Expected variable name")));
                    };
                    let Some(v) = args.get(1) else {
                        return Err(LoomErr::Reason(format!("Expected value")));
                    };
                    let v_eval = v.eval(env)?;
                    env.data.insert(k.clone(), v_eval);
                    Ok(LoomExp::True)
                }
            )
        );

        data.insert(
            "say".to_string(),
            LoomExp::Func(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    let Some(LoomExp::FString(speaker)) = args.first() else {
                        return Err(LoomErr::Reason(format!("Expected speaker")));
                    };
                    let Some(LoomExp::FString(dialogue)) = args.get(1) else {
                        return Err(LoomErr::Reason(format!("Expected dialogue")));
                    };
                    println!("{speaker}: {dialogue}");
                    Ok(LoomExp::True)
                }
            )
        );

        data.insert(
            "format".to_string(),
            LoomExp::Func(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    let mut string = String::new();
                    for arg in args {
                        string = format!("{string}{arg}");
                    }
                    Ok(LoomExp::FString(string))
                }
            )
        );

        data.insert(
            "true".to_string(),
            LoomExp::True
        );

        data.insert(
            "false".to_string(),
            LoomExp::False
        );

        data.insert(
            "if".to_string(),
            LoomExp::Macro(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    let Some(condition) = args.first() else {
                        return Err(LoomErr::Reason(format!("\"if\" has no condition")));
                    };
                    match condition.eval(env)? {
                        LoomExp::False => {
                            let Some(falsy) = args.get(2) else {
                                return Ok(LoomExp::False);
                            };
                            Ok(falsy.eval(env)?)
                        },
                        _ => {
                            let Some(truthy) = args.get(1) else {
                                return Err(
                                    LoomErr::Reason(format!("\"if\" has no true value"))
                                );
                            };
                            Ok(truthy.eval(env)?)
                        },
                    }
                }
            )
        );

        data.insert(
            "do".to_string(),
            LoomExp::Func(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    for arg in args {
                        arg.eval(env)?;
                    }
                    Ok(LoomExp::True)
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
