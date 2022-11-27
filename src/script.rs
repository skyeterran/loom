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
            '@' => {
                let Some((speaker, content)) = raw.split_once(": ") else {
                    return Err(ParseError::BadLine)
                };
                let Some(speaker) = speaker.strip_prefix("@") else {
                    return Err(ParseError::BadLine);
                };
                return Ok(Phrase::Dialogue(
                        Line {
                            speaker: speaker.to_string(),
                            content: content.to_string(),
                        })
                    );
            },
            _ => { return Err(ParseError::Unknown) },
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
    pub lines: Vec<Phrase>,
}

impl Script {
    pub fn parse(source: String) -> Self {
        let mut lines: Vec<Phrase> = Vec::new();
        let raw_lines = source.split("\n");
        
        for raw_line in raw_lines {
            match Phrase::parse(raw_line.to_string()) {
                Ok(phrase) => { lines.push(phrase) },
                Err(error) => { println!("{:?}", error) },
            }
        }

        Script {
            lines: lines
        }
    }
}
