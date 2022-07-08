use std::string::FromUtf8Error;

struct Tokenizer {
    buffer: String,
    index: usize,
    state: State,
}

enum State {
    Invalid,
    Start,
    Identifier,
    Keyword,
    Operator,
    BaseIdentifier,
    Number,
}

struct Token {
    tag: Tag,
    location: Location,
}

enum Tag {
    Invalid,
    Plus,
    XOR,
    SHL,
    SHR,
    Hex,
    Binary,
    Number,
}

struct Location {
    start: usize,
    end: usize
}

impl Tokenizer {
    pub fn init(buffer: &str) -> Self {
        let buffer = buffer.to_string();
        Self {
            buffer,
            index: 0,
            state: State::Start,
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        let start = self.index;
        self.state = State::Start;
        while let Some(current_char) = self.buffer.get(self.index) {
            match self.state {
                State::Start => {
                    match current_char {
                        b'0' => {
                            self.state = State::BaseIdentifier;
                        },
                        b'a'..=b'z' | b'A'..=b'Z' => {
                            self.state = State::Identifier;
                        },
                        b' ' => {
                            self.state = State::Start;
                        }
                        b'1'..=b'9' => {
                            self.state = State::Number;
                        },
                        b'+' | b'-' | b'~' => {
                            //TODO: finish here, operators are always one character!
                        }
                        _ => {
                            self.state = State::Invalid;
                            break;
                        }
                    }
                },
                
                State::Keyword => {
                    match current_char {
                        b'a'..=b'z' | b'A'..=b'Z' => {
                            self.state = State::Identifier;
                        }
                        _ => {
                            self.state = State::Invalid;
                        }
                    }
                },

                State::Operator => {

                },

                State::Identifier => {

                },

                State::BaseIdentifier => {
                    match current_char {
                        b'b' => {
                            
                        }
                        b'x' => {

                        }
                        _ => {
                            self.state = State::Invalid;
                        }
                    }

                },

                State::Number => {
                    match current_char {
                        b'0'..=b'9' => {
                            self.state = State::Number;
                        }
                        b' ' => {
                            //TODO: Finish!
                        }
                        _ => {
                            self.state = State::Invalid;
                        }
                    }

                },

                State::Invalid => {

                },
            }
            self.index += 1; //end of loop, increment index.
        }
        let token = Token::new(, Location::new(start, self.index));
        
        Some(token)
    }
}

impl Token {
    fn new(tag: Tag, location: Location) -> Self {
        Self {
            tag,
            location,
        }
    }
}

impl Location {
    fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
        }
    }
}
