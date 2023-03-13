use std::fmt;
use std::path::Path;
use std::collections::HashMap;
use rand::prelude::*;

use crate::parser::tokens_to_exp;

#[derive(Clone)]
pub enum LoomExp {
    True,
    Nil,
    Error,
    Symbol(String),
    Keyword(String),
    Number(f64),
    FString(String),
    Table(HashMap<String, LoomExp>),
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
            LoomExp::Keyword(_) => { Ok(self.clone()) },
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
            LoomExp::Table(_) => { Ok(self.clone()) },
            LoomExp::Func(_) => { Err(LoomErr::Reason("Unexpected form".to_string())) },
            LoomExp::Macro(_) => { Err(LoomErr::Reason("Unexpected form".to_string())) },
        }
    }
}

impl fmt::Display for LoomExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            LoomExp::True => { format!("true") },
            LoomExp::Nil => { format!("nil") },
            LoomExp::Error => { format!("error") },
            LoomExp::Symbol(s) => s.clone(),
            LoomExp::Keyword(k) => format!("#{k}"),
            LoomExp::Number(n) => n.to_string(),
            LoomExp::FString(fs) => format!("\"{fs}\""),
            LoomExp::List(list) => {
                let xs: Vec<String> = list.iter()
                                          .map(|x| x.to_string())
                                          .collect();
                format!("[{}]", xs.join(" "))
            },
            LoomExp::Table(map) => {
                let mut t = "{".to_string();
                for (i, (key, value)) in map.iter().enumerate() {
                    let mut value_text = format!("{value}");
                    match value {
                        LoomExp::Table(_) => {
                            value_text = value_text.replace("\n", "\n    ");
                        },
                        _ => {}
                    }
                    let indent = " ".repeat(4);
                    t = format!("{t}\n{indent}#{key} {value_text}");
                }
                format!("{t}\n}}")
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
            LoomExp::Keyword(n) => { write!(f, "Keyword({n})") },
            LoomExp::Number(n) => { write!(f, "Number({n})") },
            LoomExp::FString(fs) => { write!(f, "FString(\"{}\")", fs) },
            LoomExp::List(list) => {
                let mut lines: Vec<String> = Vec::new();
                for item in list {
                    lines.push(format!("{:?}", item));
                }
                write!(f, "List({})", lines.join(", "))
            },
            LoomExp::Table(t) => { write!(f, "Table({:#?})", t) },
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
            LoomExp::Keyword(k) => {
                match other {
                    LoomExp::Keyword(o_k) => { k == o_k },
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
            LoomExp::Table(obj) => {
                match other {
                    LoomExp::Table(o_obj) => { obj == o_obj },
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
            // (run "test.loom")
            "run".to_string(),
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
            // (load "test.loom")
            // (load "x.loom" x)
            "load".to_string(),
            LoomExp::Macro(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    let Some(first) = args.first() else {
                        return Err(LoomErr::Reason(format!("Not enough arguments!")));
                    };
                    let LoomExp::FString(filename) = first.eval(env)? else {
                        return Err(LoomErr::Reason(format!("Expected a filename!")));
                    };
                    let source = std::fs::read_to_string(filename.clone()).expect(format!("Couldn't load {filename}").as_str());
                    let tokens = crate::parser::tokenize(source).unwrap();
                    let exp = tokens_to_exp(tokens, false).unwrap();

                    match exp.eval(env) {
                        Ok(data) => {
                            // If a symbol is given as the second argument, save that data into that symbol
                            if let Some(LoomExp::Symbol(k)) = args.get(1) {
                                env.data.insert(k.clone(), data.clone());
                            }
                            Ok(data)
                        },
                        Err(e) => {
                            Err(e)
                        }
                    }
                }
            )
        );

        data.insert(
            // (save x "x.loom")
            "save".to_string(),
            LoomExp::Func(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    let Some(input) = args.first() else {
                        return Err(LoomErr::Reason(format!("Expected input data!")));
                    };
                    let Some(LoomExp::FString(filename)) = args.get(1) else {
                        return Err(LoomErr::Reason(format!("Expected a filename!")));
                    };
                    let data = format!("{input}");
                    let path = Path::new(filename);
                    std::fs::create_dir_all(path.parent().to_owned().expect("Couldn't get parent path!")).expect("Couldn't create parent path!");
                    std::fs::write(filename, data).expect("Unable to write to disk!");
                    Ok(LoomExp::Nil)
                }
            )
        );

        data.insert(
            "set".to_string(),
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
            "=".to_string(),
            LoomExp::Func(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    let Some(first) = args.first() else {
                        return Err(LoomErr::Reason(format!("Not enough arguments to = !")));
                    };
                    for other in args[1..args.len()].iter() {
                        if first != other { return Ok(LoomExp::Nil); }
                    }
                    Ok(LoomExp::True)
                }
            )
        );

        data.insert(
            "not".to_string(),
            LoomExp::Func(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    let Some(x) = args.first() else {
                        return Err(LoomErr::Reason(format!("Not enough arguments to = !")));
                    };
                    if args.len() > 1 {
                        return Err(LoomErr::Reason(format!("Too many arguments to = !")));
                    }
                    match x {
                        LoomExp::Nil => Ok(LoomExp::True),
                        _ => Ok(LoomExp::Nil)
                    }
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
            "table".to_string(),
            LoomExp::Macro(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    let mut hashmap: HashMap<String, LoomExp> = HashMap::new();
                    if args.len() % 2 != 0 {
                        return Err(LoomErr::Reason(format!("Key-value pairs should add up to an even count!")));
                    }
                    let mut keyvals: Vec<(String, LoomExp)> = Vec::new();
                    let mut current_kv: (String, LoomExp) = (String::new(), LoomExp::Nil);
                    for i in 0..(args.len()) {
                        if i % 2 == 0 {
                            // This is a key
                            let Some(LoomExp::Keyword(key)) = args.get(i) else {
                                return Err(LoomErr::Reason(format!("Invalid keyword!")));
                            };
                            current_kv.0 = key.clone();
                        } else {
                            // This is a value
                            current_kv.1 = args.get(i).expect("Invalid value!").eval(env)?;
                            keyvals.push(current_kv.clone());
                        }
                    }
                    for (k, v) in keyvals {
                        hashmap.insert(k, v);
                    }
                    Ok(LoomExp::Table(hashmap))
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
                    let LoomExp::Table(obj) = obj_sym.eval(env)? else {
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
                        let Some(LoomExp::Table(map)) = value else {
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
            LoomExp::Func(
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
            "quote".to_string(),
            LoomExp::Macro(
                |args: &[LoomExp], env: &mut LoomEnv| -> Result<LoomExp, LoomErr> {
                    if args.len() > 1 {
                        let mut list: Vec<LoomExp> = Vec::new();
                        for arg in args {
                            list.push(arg.clone());
                        }
                        Ok(LoomExp::List(list))
                    } else {
                        match args.first() {
                            Some(exp) => Ok(exp.clone()),
                            None => Ok(LoomExp::Nil)
                        }
                    }
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
