use super::exp::LoomExp;
use std::num::ParseFloatError;

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
    Number(f64),
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
                '{' => { words.push_str(" ( do ") },
                '}' => { words.push_str(" ) ") },
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
                    "(" => { tokens.push(LParen); },
                    ")" => { tokens.push(RParen); },
                    "\"" => {
                        consume_string = true;
                    },
                    _ => {
                        let maybe_number: Result<f64, ParseFloatError> = word.parse();
                        match maybe_number {
                            Ok(n) => {
                                tokens.push(Number(n));
                            },
                            Err(_) => {
                                tokens.push(Symbol(word.to_string()));
                            }
                        }
                    },
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

// Creates an object from a stream of tokens
pub fn tokens_to_exp(tokens: Vec<Token>) -> Result<LoomExp, ParseError> {
    use ParseError::*;
    use Token::*;

    // The list we're building up
    let mut list: Vec<LoomExp> = Vec::new();

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
                    list.push(LoomExp::Symbol(s));
                },
                Number(n) => {
                    list.push(LoomExp::Number(n));
                },
                StringToken(s) => {
                    list.push(LoomExp::FString(s));
                },
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
                list.push(tokens_to_exp(substream.clone())?);
                substream = Vec::new();
            }
        }
    }

    Ok(LoomExp::List(list))
}
