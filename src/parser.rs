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
}

#[derive(Debug)]
pub enum Token {
    LParen,
    RParen,
    Symbol(String),
    LitString(String),
}

/// Takes a Lispy string and turns it into a stream of tokens
pub fn tokenize(source: String) -> Result<Vec<Token>, ParseError> {
    use ParseError::*;
    use Token::*;

    // Lispify the source script and add spaces for ease of parsing
    let words = lispify(source).replace("(", " ( ")
                               .replace(")", " ) ")
                               .replace("\"", " \" ");

    let mut tokens: Vec<Token> = Vec::new();

    // Whether to consume incoming tokens as part of a literal string or not
    let mut consume_string = false;
    let mut literal_string = String::new();
    for word in words.split(" ") {
        if !word.is_empty() {
            if !consume_string {
                // Expression parsing
                match word {
                    "(" => { tokens.push(LParen) },
                    ")" => { tokens.push(RParen) },
                    "\"" => {
                        consume_string = true;
                    },
                    _ => { tokens.push(Symbol(word.to_string())) },
                }
            } else {
                // String parsing
                if word == "\"" {
                    // Stop consuming the string and move on, clearing the string buffer
                    consume_string = false;
                    tokens.push(LitString(literal_string.clone()));
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
    }

    if consume_string {
        // If we're still consuming a string, that means it's missing an end quote
        return Err(UnfinishedString);
    }

    Ok(tokens)
}
