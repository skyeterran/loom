#[derive(Debug)]
pub struct ParseError;

#[derive(Debug)]
pub enum Phrase {
    Dialogue(Line),
    Command(String),
}

impl Phrase {
    fn parse(source: String) -> Result<Self, ParseError> {
        if let Some(first_char) = source.chars().next() {
            match first_char {
                '(' => { return Ok(Phrase::Command(source.to_string())) },
                _ => {
                    let Some(line) = source.split_once(": ") else {
                        return Err(ParseError)
                    };
                    return Ok(Phrase::Dialogue(
                            Line {
                                speaker: line.0.to_string(),
                                content: line.1.to_string(),
                            })
                        );
                },
            }
        } else { return Err(ParseError) }
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
            let Ok(phrase) = Phrase::parse(raw_line.to_string()) else { continue; };
            lines.push(phrase);
        }

        Script {
            lines: lines
        }
    }
}
