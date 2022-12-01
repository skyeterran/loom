use super::script::{Memory, Cmd};

/// Takes a Loom source file and formats all dialogue into Lispy function calls
fn lispify(source: String) -> String {
    let mut new_lines: Vec<String> = Vec::new();

    for raw_line in source.split("\n") {
        let line = raw_line.trim();

        // Try to interpret this line as dialogue, otherwise treat it normally
        let Some((speaker, content)) = line.split_once(": ") else {
            new_lines.push(line.to_string());
            continue;
        };

        new_lines.push(format!("(say {speaker} \"{content}\")"));
    }

    new_lines.join("")
}

#[derive(Debug)]
pub enum ParseError {
    UnfinishedString,
    UnexpectedRParen,
    ImproperRoot,
    UnknownType,
    UnknownSymbol,
    WrongArgument,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum Token {
    LParen,
    RParen,
    Symbol(String),
    StringToken(String),
}

/// Takes a Lispy string and turns it into a stream of tokens
pub fn tokenize(source: String) -> Result<Vec<Token>, ParseError> {
    use ParseError::*;
    use Token::*;

    // Lispify the source script and add spaces for ease of parsing
    let mut words = String::new();
    let mut in_string = false;
    for char in lispify(source).chars() {
        if !in_string {
            match char {
                '(' => { words.push_str(" ( ") },
                ')' => { words.push_str(" ) ") },
                '\"' => {
                    in_string = true;
                    words.push_str(" \" ");
                },
                _ => { words.push(char) }
            }
        } else {
            // Don't put spaces around parentheses while in a string literal
            // TODO: What about expressions embedded in formatting strings???
            match char {
                '\"' => {
                    in_string = false;
                    words.push_str(" \" ");
                },
                _ => { words.push(char) }
            }
        }
    }

    let mut tokens: Vec<Token> = Vec::new();

    // Whether to consume incoming tokens as part of a literal string or not
    let mut consume_string = false;
    let mut literal_string = String::new();
    for word in words.split(" ") {
        if !consume_string {
            if !word.is_empty() {
                // Expression parsing
                match word {
                    "(" => { tokens.push(LParen) },
                    ")" => { tokens.push(RParen) },
                    "\"" => {
                        consume_string = true;
                    },
                    _ => { tokens.push(Symbol(word.to_string())) },
                }
            }
        } else {
            // String parsing
            if word == "\"" {
                // Stop consuming the string and move on, clearing the string buffer
                consume_string = false;
                tokens.push(StringToken(literal_string.clone()));
                literal_string = String::new();
                continue;
            } else {
                if !literal_string.is_empty() {
                    literal_string.push(' ');
                }
                literal_string.push_str(word);
            }
        }
    }

    if consume_string {
        // If we're still consuming a string, that means it's missing an end quote
        return Err(UnfinishedString);
    }

    Ok(tokens)
}

#[derive(Debug)]
pub enum LogicError {
    MissingSymbol,
    MissingItem,
    WrongArgument,
    Unknown,
}

#[derive(Debug)]
pub enum Object {
    Symbol(String),
    LitString(String),
    List(Vec<Object>),
}

impl Object {
    // Creates an object from a stream of tokens
    pub fn from_tokens(tokens: Vec<Token>) -> Result<Self, ParseError> {
        use ParseError::*;
        use Token::*;

        // The list we're building up
        let mut list: Vec<Object> = Vec::new();

        let mut consume_substream = false;
        let mut nesting: usize = 0;
        let mut substream: Vec<Token> = Vec::new();
        for token in tokens {
            if !consume_substream {
                // Token parsing
                match token {
                    LParen => {
                        // Create an object from this sub-expression
                        consume_substream = true;
                        nesting += 1;
                    },
                    RParen => {
                        return Err(UnexpectedRParen);
                    },
                    Symbol(s) => {
                        list.push(Object::Symbol(s));
                    },
                    StringToken(s) => {
                        list.push(Object::LitString(s));
                    },
                    _ => {},
                }
            } else {
                // Substream parsing
                match token {
                    LParen => { nesting += 1 },
                    RParen => { nesting -= 1 },
                    _ => {},
                }

                if nesting > 0 {
                    // Consume substream
                    substream.push(token.clone());
                } else {
                    // Parse the substream and clear it
                    consume_substream = false;
                    list.push(Object::from_tokens(substream.clone())?);
                    substream = Vec::new();
                }
            }
        }

        Ok(Object::List(list))
    }

    pub fn evaluate(&self, memory: &Memory) -> Result<Cmd, LogicError> {
        use Object::*;

        match self {
            List(list) => {
                let Some(first_item) = list.first() else {
                    return Err(LogicError::MissingItem);
                };

                match first_item {
                    Symbol(f) => {
                        // Function call
                        match f.as_str() {
                            // Kludgy test logic
                            "say" => {
                                let Some(Symbol(speaker)) = list.get(1) else {
                                    return Err(LogicError::WrongArgument);
                                };
                                let Some(LitString(dialogue)) = list.get(2) else {
                                    return Err(LogicError::WrongArgument);
                                };
                                return Ok(Cmd::Say(speaker.clone(), dialogue.clone()));
                            },
                            "if" => {
                                let Some(Symbol(key)) = list.get(1) else {
                                    return Err(LogicError::WrongArgument);
                                };
                                let mut test: String;
                                // Check the value of the key in the memory
                                match memory.map.get(key) {
                                    Some(v) => { test = v.clone(); },
                                    None => { test = "false".to_string() },
                                }
                                if test == "true" {
                                    println!("Truthy!");
                                    let Some(true_list) = list.get(2) else {
                                        return Err(LogicError::WrongArgument);
                                    };
                                    return true_list.evaluate(memory);
                                } else {
                                    let Some(false_list) = list.get(3) else {
                                        return Ok(Cmd::Noop);
                                    };
                                    return false_list.evaluate(memory);
                                }
                            },
                            "let" => {
                                let Some(Symbol(key)) = list.get(1) else {
                                    return Err(LogicError::WrongArgument);
                                };
                                let Some(LitString(value)) = list.get(2) else {
                                    return Err(LogicError::WrongArgument);
                                };
                                return Ok(Cmd::Let(key.clone(), value.clone()));
                            },
                            "match" => {
                                let Some(Symbol(key)) = list.get(1) else {
                                    return Err(LogicError::WrongArgument);
                                };
                                let val = match memory.map.get(key) {
                                    Some(v) => v.clone(),
                                    None => "false".to_string(),
                                };
                                // Look through the possible choices until one is valid
                                for variant in &list[2..] {
                                    let List(var_list) = variant else {
                                        return Err(LogicError::WrongArgument);
                                    };
                                    let Some(LitString(value)) = var_list.get(0) else {
                                        return Err(LogicError::MissingItem);
                                    };
                                    let Some(path) = var_list.get(1) else {
                                        return Err(LogicError::MissingItem);
                                    };
                                    if &val == value {
                                        return path.evaluate(memory);
                                    } else {
                                        continue;
                                    }
                                }
                                return Ok(Cmd::Noop);
                            },
                            _ => {},
                        }
                    },
                    List(_) => {
                        // Pure list
                        for item in list {
                            item.evaluate(memory)?;
                        }
                    },
                    _ => {
                        return Err(LogicError::Unknown);
                    },
                }
            },
            Symbol(s) => {
                println!("Symbol: {s}");
            },
            LitString(s) => {
                println!("String: {s}");
            },
            _ => { return Err(LogicError::Unknown) },
        }

        Ok(Cmd::Noop)
    }
}
