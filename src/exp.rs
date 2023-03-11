use std::fmt;
use std::collections::HashMap;
use rand::prelude::*;

use crate::parser::tokens_to_exp;

#[derive(Clone)]
pub enum LoomExp {
    True,
    Nil,
    Error,
    Symbol(String),
    Name(String),
    Number(f64),
    FString(String),
    Object(HashMap<String, LoomExp>),
    List(Vec<LoomExp>),
    Func(fn(&[LoomExp], &mut LoomEnv) -> Result<LoomExp, LoomErr>),
    Macro(fn(&[LoomExp], &mut LoomEnv) -> Result<LoomExp, LoomErr>),
}

impl LoomExp {
    pub fn eval(&self, env: &mut LoomEnv) -> Result<LoomExp, LoomErr> {
        match self {
            LoomExp::True => { Ok(LoomExp::True) },
            LoomExp::Nil => { Ok(LoomExp::Nil) },
            LoomExp::Error => { Ok(LoomExp::Error) },
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
            LoomExp::Name(_) => { Ok(self.clone()) },
            LoomExp::Number(_) => { Ok(self.clone()) },
            LoomExp::FString(_) => { Ok(self.clone()) },
            LoomExp::List(list) => {
                let Some(first_form) = list.first() else {
                    // Empty lists are nil
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
            LoomExp::Object(_) => { Ok(self.clone()) },
            LoomExp::Func(_) => { Err(LoomErr::Reason("Unexpected form".to_string())) },
            LoomExp::Macro(_) => { Err(LoomErr::Reason("Unexpected form".to_string())) },
        }
    }
}

impl fmt::Display for LoomExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            LoomExp::True => { format!("True") },
            LoomExp::Nil => { format!("Nil") },
            LoomExp::Error => { format!("Error") },
            LoomExp::Symbol(s) => s.clone(),
            LoomExp::Name(n) => format!("#{n}"),
            LoomExp::Number(n) => n.to_string(),
            LoomExp::FString(fs) => fs.clone(),
            LoomExp::List(list) => {
                let xs: Vec<String> = list.iter()
                                          .map(|x| x.to_string())
                                          .collect();
                format!("[{}]", xs.join(", "))
            },
            LoomExp::Object(map) => {
                let mut obj = "(object".to_string();
                for (i, (key, value)) in map.iter().enumerate() {
                    let mut value_text = format!("{value}");
                    match value {
                        LoomExp::Object(_) => {
                            value_text = value_text.replace("\n", "\n    ");
                        },
                        _ => {}
                    }
                    /*if i == 0 {
                        obj = format!("{obj} ({key} {value_text})");
                    } else {
                        let indent = " ".repeat(4);
                        obj = format!("{obj}\n{indent}({key} {value_text})");
                    }*/
                    let indent = " ".repeat(4);
                    obj = format!("{obj}\n{indent}({key} {value_text})");
                }
                format!("{obj}\n)")
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
            LoomExp::True => { write!(f, "True") },
            LoomExp::Nil => { write!(f, "Nil") },
            LoomExp::Error => { write!(f, "Error") },
            LoomExp::Symbol(s) => { write!(f, "Symbol({s})") },
            LoomExp::Name(n) => { write!(f, "Name({n})") },
            LoomExp::Number(n) => { write!(f, "Number({n})") },
            LoomExp::FString(fs) => { write!(f, "FString(\"{}\")", fs) },
            LoomExp::List(list) => {
                let mut lines: Vec<String> = Vec::new();
                for item in list {
                    lines.push(format!("{:?}", item));
                }
                write!(f, "List({})", lines.join(", "))
            },
            LoomExp::Object(obj) => { write!(f, "Object({:#?})", obj) },
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
            LoomExp::Name(n) => {
                match other {
                    LoomExp::Name(o_n) => { n == o_n },
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
            LoomExp::Object(obj) => {
                match other {
                    LoomExp::Object(o_obj) => { obj == o_obj },
                    _ => { false }
                }
            }
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
                    Ok(LoomExp::Nil)
                }
            )
        );

        data.insert(
            "debug".to_string(),
            LoomExp::Func(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    let Some(x) = args.first() else {
                        return Err(LoomErr::Reason(format!("Can't debug nothing!")));
                    };
                    println!("{:?}", x);
                    Ok(LoomExp::Nil)
                }
            )
        );

        data.insert(
            "env".to_string(),
            LoomExp::Func(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    println!("{:#?}", env);
                    Ok(LoomExp::Nil)
                }
            )
        );

        data.insert(
            // (load "test.loom")
            // TODO: Make this load into an object instead of running it
            "load".to_string(),
            LoomExp::Func(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    let Some(LoomExp::FString(filename)) = args.first() else {
                        return Err(LoomErr::Reason(format!("Expected a filename!")));
                    };
                    let source = std::fs::read_to_string(filename).expect(format!("Couldn't load {filename}").as_str());
                    let tokens = crate::parser::tokenize(source).unwrap();
                    let exp = tokens_to_exp(tokens, true).unwrap();
                    match exp {
                        LoomExp::List(list) => {
                            for item in list {
                                match item.eval(env) {
                                    Ok(_) => {},
                                    Err(e) => { println!("Error: {:?}", e) },
                                }
                                //println!("Env:\n{:#?}", env);
                            }
                        },
                        _ => {},
                    }
                    Ok(LoomExp::Nil)
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
                    Ok(LoomExp::Nil)
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
            "nil".to_string(),
            LoomExp::Nil
        );

        data.insert(
            "exit".to_string(),
            LoomExp::Func(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    println!("Goodbye for now!");
                    std::process::exit(0);
                }
            )
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
            "object".to_string(),
            LoomExp::Macro(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    let mut hashmap: HashMap<String, LoomExp> = HashMap::new();
                    for arg in args {
                        match arg {
                            LoomExp::List(l) => {
                                let Some(LoomExp::Symbol(key)) = l.first() else {
                                    return Err(LoomErr::Reason(format!("Incorrect type for object variable key")));
                                };
                                let Some(value) = l.get(1) else {
                                    return Err(LoomErr::Reason(format!("Missing value argument for object!")));
                                };
                                hashmap.insert(key.clone(), value.eval(env)?);
                            },
                            _ => { return Err(LoomErr::Reason(format!("object arguments must be lists of key-value pairs"))); }
                        }
                    }
                    Ok(LoomExp::Object(hashmap))
                }
            )
        );

        data.insert(
            // (get object #key)
            "get".to_string(),
            LoomExp::Macro(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    let Some(obj_sym) = args.first() else {
                        return Err(LoomErr::Reason(format!("First argument to get is missing")));
                    };
                    let LoomExp::Object(obj) = obj_sym.eval(env)? else {
                        return Err(LoomErr::Reason(format!("First argument to get is not an object!")));
                    };
                    let Some(LoomExp::Symbol(key)) = args.get(1) else {
                        return Err(LoomErr::Reason(format!("Second argument to get must be a name")));
                    };
                    let mut value: Option<&LoomExp> = obj.get(key).clone();
                    for name in args[2..args.len()].iter() {
                        let LoomExp::Symbol(key) = name else {
                            return Err(LoomErr::Reason(format!("Bad argument!")));
                        };
                        let Some(LoomExp::Object(map)) = value else {
                            return Err(LoomErr::Reason(format!("Bad thing!")));
                        };
                        value = map.get(key).clone();
                    }
                    match value {
                        Some(contents) => Ok(contents.clone()),
                        None => Ok(LoomExp::Nil)
                    }
                }
            )
        );

        data.insert(
            "eval".to_string(),
            LoomExp::Macro(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    args.first().unwrap().eval(env)
                }
            )
        );

        data.insert(
            "list".to_string(),
            LoomExp::Func(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    let mut list: Vec<LoomExp> = Vec::new();
                    for arg in args {
                        list.push(arg.clone());
                    }
                    Ok(LoomExp::List(list))
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
                    let (_, paths): (&LoomExp, &[LoomExp]) = args.split_first().unwrap();
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
