use std::env;
use std::fs;

#[derive(Debug)]
enum Token {
    Integer(i64),
    Symbol(String),
    LParen,
    RParen,
    Quote,
}

#[derive(Debug)]
enum Object {
    Nil,
    Integer(i64),
    Bool(bool),
    LitString(String),
    Symbol(String),
    Lambda(Vec<String>, Vec<Object>),
    List(Vec<Object>),
}

#[derive(Debug)]
struct ParseError {
    err: String,
}

#[derive(Debug)]
struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    fn from_source(source: String) -> Self {
        let words = source.replace("(", " ( ")
                          .replace(")", " ) ")
                          .replace("\"", " \" ");
        let words = words.split_whitespace();

        let mut tokens: Vec<Token> = Vec::new();
        for word in words {
            match word {
                "(" => tokens.push(Token::LParen),
                ")" => tokens.push(Token::RParen),
                "\"" => tokens.push(Token::Quote),
                _ => {
                    // Try to treat this as an integer
                    let t = word.parse::<i64>();
                    if t.is_ok() {
                        tokens.push(Token::Integer(t.unwrap()));
                    } else {
                        tokens.push(Token::Symbol(word.to_string()));
                    }
                }
            }
        }

        TokenStream { tokens: tokens }
    }

    fn parse(&mut self) -> Result<Object, ParseError> {
        // DEBUG
        println!("Attempting to parse!");

        let Some(token) = self.tokens.pop() else {
            return Err(ParseError {
                err: format!("Ran out of tokens!")
            });
        };

        // DEBUG
        println!("Parsing {:?}", token);

        match token {
            Token::RParen => {},
            _ => {
                println!("Not RParen!!!");
                return Err(ParseError {
                    err: format!("Expected RParen, found '{:?}'", token)
                });
            }
        }

        let mut list: Vec<Object> = Vec::new();
        while !self.tokens.is_empty() {
            let token = self.tokens.pop();

            // DEBUG
            println!("Parsing: {:?}", token);

            if token.is_none() {
                return Err(ParseError {
                    err: "Not enough self.tokens!".to_string()
                });
            }
            let t = token.unwrap();
            match t {
                Token::Integer(n) => { list.push(Object::Integer(n)); },
                Token::Symbol(s) => { list.push(Object::Symbol(s)); },
                Token::Quote => {
                    // Consume literal string
                    let mut lit_string = String::new();
                    loop {
                        match self.tokens.pop() {
                            Some(next_token) => {
                                match next_token {
                                    Token::Symbol(s) => {
                                        lit_string = format!("{s} {lit_string}");
                                    }
                                    Token::Quote => {
                                        lit_string = lit_string.strip_suffix(" ").unwrap().to_string();
                                        list.push(Object::LitString(lit_string));
                                        break;
                                    }
                                    _ => {
                                        return Err(ParseError {
                                            err: format!("Encountered unknown token during literal string parsing!"),
                                        });
                                    }
                                }
                            }
                            None => {
                                return Err(ParseError {
                                    err: format!("Literal string never closed!"),
                                });
                            }
                        }
                    }
                },
                Token::RParen => {
                    self.tokens.push(Token::RParen);
                    let sub_list = self.parse()?;
                    list.push(sub_list);
                },
                Token::LParen => { return Ok(Object::List(list)); }
            }
        }
        Ok(Object::List(list))
    }
}

fn main() {
    let source = fs::read_to_string("script.loom").expect("Couldn't load file!");
    println!("{}", source);

    let mut token_stream = TokenStream::from_source(source);
    println!("{:?}", token_stream);

    let parsed_object = token_stream.parse();
    println!("{:#?}", parsed_object);
}
