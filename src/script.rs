#[derive(Debug)]
pub enum ParseError {
    EmptyLine,
    BadLine,
    Unknown,
}

#[derive(Debug)]
pub enum Phrase {
    Dialogue(Line),
    Expression(String),
}

impl Phrase {
    fn parse(source: String) -> Result<Self, ParseError> {
        let raw = source.trim();
        let Some(first_char) = raw.chars().next() else {
            return Err(ParseError::EmptyLine);
        };
        match first_char {
            '(' => { return Ok(Phrase::Expression(raw.to_string())) },
            ')' => { return Ok(Phrase::Expression(raw.to_string())) },
            _ => {
                let Some((speaker, content)) = raw.split_once(": ") else {
                    return Err(ParseError::Unknown)
                };
                return Ok(Phrase::Dialogue(
                        Line {
                            speaker: speaker.to_string(),
                            content: content.to_string(),
                        })
                    );
            },
        }
    }
}

#[derive(Debug)]
pub struct Line {
    pub speaker: String,
    pub content: String,
}

#[derive(Debug)]
pub struct Script {
    pub phrases: Vec<Phrase>,
}

impl Script {
    pub fn parse(source: String) -> Self {
        let mut phrases: Vec<Phrase> = Vec::new();
        let raw_lines = source.split("\n");
        
        for raw_line in raw_lines {
            match Phrase::parse(raw_line.to_string()) {
                Ok(phrase) => { phrases.push(phrase) },
                Err(error) => {},
            }
        }

        Script {
            phrases: phrases
        }
    }
}
