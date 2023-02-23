use std::fmt;
use std::collections::HashMap;
use rand::prelude::*;

#[derive(Clone)]
pub enum LoomExp {
    True,
    Nil,
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
            LoomExp::Nil => { Ok(LoomExp::Nil) },
            LoomExp::Symbol(k) => {
                match env.data.get(k) {
                    Some(v) => {
                        Ok(v.clone())
                    },
                    None => {
                        Err(LoomErr::Reason(format!("Unexpected symbol: {k}")))
                    }
                }
            },
            LoomExp::Number(_) => { Ok(self.clone()) },
            LoomExp::FString(_) => { Ok(self.clone()) },
            LoomExp::List(list) => {
                let Some(first_form) = list.first() else {
                    // Empty lists are NIL (false, here)
                    return Ok(LoomExp::Nil);
                };
                let arg_forms = &list[1..];
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
                        // If this is a pure list, do NOT evaluate it.
                        return Ok(self.clone());
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
            LoomExp::Nil => { format!("false") },
            LoomExp::Symbol(s) => s.clone(),
            LoomExp::Number(n) => n.to_string(),
            LoomExp::FString(fs) => fs.clone(),
            LoomExp::List(list) => {
                let xs: Vec<String> = list.iter()
                                          .map(|x| x.to_string())
                                          .collect();
                format!("({})", xs.join(", "))
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
            LoomExp::Nil => { write!(f, "false") },
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

impl PartialEq for LoomExp {
    fn eq(&self, other: &Self) -> bool {
        match self {
            LoomExp::True => {
                match other {
                    LoomExp::True => { true },
                    _ => { false }
                }
            },
            LoomExp::Nil => {
                match other {
                    LoomExp::Nil => { true },
                    _ => { false }
                }
            },
            LoomExp::Symbol(s) => {
                match other {
                    LoomExp::Symbol(o_s) => { s == o_s },
                    _ => { false }
                }
            },
            LoomExp::FString(fs) => {
                match other {
                    LoomExp::FString(o_fs) => { fs == o_fs },
                    _ => { false }
                }
            },
            LoomExp::List(list) => {
                match other {
                    LoomExp::List(o_list) => {
                        if list.len() != o_list.len() { return false; }
                        let mut items_match = true;
                        for i in 0..(list.len() - 1) {
                            if list.get(i) != o_list.get(i) {
                                items_match = false;
                                break;
                            }
                        }
                        items_match
                    },
                    _ => { false }
                }
            },
            _ => { false }
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
            LoomExp::Nil
        );

        data.insert(
            "if".to_string(),
            LoomExp::Macro(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    let Some(condition) = args.first() else {
                        return Err(LoomErr::Reason(format!("\"if\" has no condition")));
                    };
                    match condition.eval(env)? {
                        LoomExp::Nil => {
                            let Some(falsy) = args.get(2) else {
                                return Ok(LoomExp::Nil);
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

        data.insert(
            "random".to_string(),
            LoomExp::Macro(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    let Some(first_arg) = args.first() else {
                        return Err(
                            LoomErr::Reason(format!("Not enough arguments to random"))
                        );
                    };
                    let LoomExp::List(list) = first_arg.eval(env)? else {
                        return Err(
                            LoomErr::Reason(format!("Random expects a list as input"))
                        );
                    };
                    let mut rng = rand::thread_rng();
                    let die = rand::distributions::Uniform::from(0..list.len());
                    let choice = die.sample(&mut rng);
                    let Some(chosen_result) = list.get(choice) else {
                        return Err(LoomErr::Reason(format!("Could not choose anything")));
                    };
                    let chosen_eval = chosen_result.eval(env)?;
                    Ok(chosen_eval.clone())
                }
            )
        );

        data.insert(
            "match".to_string(),
            LoomExp::Macro(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    let Some(first_arg) = args.first() else {
                        return Err(
                            LoomErr::Reason(format!("Not enough arguments to match"))
                        );
                    };
                    let condition = first_arg.eval(env)?;
                    let Some(LoomExp::List(paths)) = args.get(1) else {
                        return Err(
                            LoomErr::Reason(format!("Match expects a list of paths"))
                        );
                    };
                    for path in paths {
                        let LoomExp::List(path_list) = path else {
                            return Err(
                                LoomErr::Reason(format!("Match paths must be lists"))
                            );
                        };
                        let Some(path_cond) = path_list.first() else {
                            return Err(
                                LoomErr::Reason(format!("Match path is empty"))
                            );
                        };
                        if *path_cond == condition {
                            // Evaluate this path
                            let Some(path_body) = path_list.get(1) else {
                                return Err(
                                    LoomErr::Reason(format!("Match path has no body"))
                                );
                            };
                            return Ok(path_body.eval(env)?)
                        }
                    }
                    Ok(LoomExp::Nil)
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
