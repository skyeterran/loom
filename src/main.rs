use std::io::stdout;
use std::io::{self, Write};

// A conversation which contains streams of dialogue
struct Convo {
    streams: Vec<Stream>,
    active_stream: usize, // The index of the current stream
}

impl Convo {
    fn read_line(&mut self) -> Option<&Line> {
        // Get the next line of dialogue from the active stream
        let maybe_line = self.streams[self.active_stream].read_line();

        // Switch streams if needed
        if let Some(line) = maybe_line {
            if let Some(next_stream) = line.next {
                self.active_stream = next_stream;
            }
        }

        maybe_line
    }
}

// A stream of dialogue
struct Stream {
    lines: Vec<Line>,
    current_line: usize, // The index of the current line
}

impl Stream {
    fn read_line(&mut self) -> Option<&Line> {
        let maybe_line = self.lines.get(self.current_line);
        self.current_line += 1;
        maybe_line
    }
}

// A line of dialogue
#[derive(Debug)]
struct Line {
    content: String,
    next: Option<usize>, // A reference to the next stream to switch to
}

fn main() -> io::Result<()> {
    let mut convo = Convo {
        streams: vec![
            Stream {
                lines: vec![
                    Line { content: format!("Oh, hello there!"), next: None },
                    Line { content: format!("My name is Nobody. Nice to meet you!"), next: None },
                    Line { content: format!("How are you doing today?"), next: Some(1) },
                ],
                current_line: 0,
            },
            Stream {
                lines: vec![
                    Line { content: format!("Guess you're a quiet one!"), next: None },
                ],
                current_line: 0,
            }
        ],
        active_stream: 0,
    };
    
    // Start the story
    let mut line_index = 0;
    println!("{}", convo.read_line().unwrap().content);

    let mut in_buffer = String::new();
    loop {
        // Get user input
        print!("> ");
        stdout().flush().unwrap();
        match io::stdin().read_line(&mut in_buffer) {
            Ok(_) => {
                let input = in_buffer.strip_suffix("\n").unwrap();
                match input {
                    "" => {
                        let line = convo.read_line();
                        match line {
                            Some(l) => {
                                println!("{}", l.content);
                            }
                            None => { break; }
                        }
                    }
                    "q" => {
                        println!("Farewell, traveler!");
                        break;
                    }
                    _ => {
                        println!("Unrecognized in_buffer: `{input}`");
                    }
                }
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
        // Remember to clear the input buffer!
        in_buffer.clear();
    }

    println!("The end.");
    Ok(())
}
