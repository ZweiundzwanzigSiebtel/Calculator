use phf::phf_map;
use std::str;


//#[derive(Debug)]
struct Scanner {
    buffer: String,
    start: usize,
    current: usize,
}

#[derive(Debug, PartialEq, Clone)]
enum TokenType {
    //single-character Tokens
    LeftParen,
    RightParen,
    Minus,
    Plus,

    //more character Tokens
    ShiftLeft,
    ShiftRight,

    //Literals
    BinaryNumber,
    DecimalNumber,
    HexNumber,

    //Keywords
    And,
    Or,
    Xor,
    Nor,

    Error,
    Eof,
    NotImplemented,
}

#[derive(Debug)]
enum State {
    Start,
    ExpectShiftLeft,
    ExpectShiftRight,
    Keyword,
    ExpectBase,
    BinaryNumber,
    HexNumber,
    DecimalNumber,
}

#[derive(Debug, PartialEq)]
struct Token {
    typ: TokenType,
    start: usize,
    length: usize,
}

impl Scanner {
    fn new(buffer: &str) -> Self {
        Self {
            buffer: buffer.to_string(),
            start: 0,
            current: 0,
        }
    }

    fn next(&mut self) -> Token {
        self.skip_whitespaces();
        self.start = self.current;

        let mut result_token = Token::new(TokenType::Error, self.start, self.current);
        let mut state = State::Start;
        loop {
            match state {
                State::Start => {
                    match self.advance() { //TODO: Remove unwrap here...
                        Some('(') => {
                            result_token = Token::new(TokenType::LeftParen, self.start, self.current);
                            break;
                        },
                        Some(')') => {
                            result_token = Token::new(TokenType::RightParen, self.start, self.current);
                            break;
                        },
                        Some('+') => {
                            result_token = Token::new(TokenType::Plus, self.start, self.current);
                            break;
                        },
                        Some('-') => {
                            result_token = Token::new(TokenType::Minus, self.start, self.current);
                            break;
                        },
                        Some('>') => state = State::ExpectShiftRight,
                        Some('<') => state = State::ExpectShiftLeft,
                        Some('0') => state = State::ExpectBase,
                        Some('1'..='9') => state = State::DecimalNumber,
                        Some('a'..='z' | 'A'..='Z') => state = State::Keyword,
                        None => {
                            result_token = Token::new(TokenType::Eof, self.start, self.current);
                            break;
                        },
                        _ => {
                            result_token = Token::new(TokenType::Error, self.start, self.current);
                            break;
                        },
                    }
                },
                State::ExpectShiftRight => {
                    match self.advance() {
                        Some('>') => {
                            result_token = Token::new(TokenType::ShiftRight, self.start, self.current);
                            break;
                        },
                        None => {
                            result_token = Token::new(TokenType::Eof, self.start, self.current);
                            break;
                        },
                        _ => {
                            result_token = Token::new(TokenType::Error, self.start, self.current);
                            break;
                        },
                    }
                },
                State::ExpectShiftLeft => {
                    match self.advance() {
                        Some('<') => {
                            result_token = Token::new(TokenType::ShiftLeft, self.start, self.current);
                            break;
                        },
                        None => {
                            result_token = Token::new(TokenType::Eof, self.start, self.current);
                            break;
                        },
                        _ => {
                            result_token = Token::new(TokenType::Error, self.start, self.current);
                            break;
                        },
                    }
                },
                State::Keyword => {
                    match self.advance() {
                        Some('a'..='z' | 'A'..='Z') => {},
                        Some(ch) if ch.is_whitespace() => {
                            self.current -= 1;
                            let keyword_type = self.get_keyword();
                            result_token = Token::new(keyword_type, self.start, self.current);
                            break;
                        },
                        None => {
                            self.current -= 1;
                            let keyword_type = self.get_keyword();
                            result_token = Token::new(keyword_type, self.start, self.current);
                            break;
                        }
                        _ => {
                            result_token = Token::new(TokenType::Error, self.start, self.current);
                            break;
                        },
                    }
                },
                State::ExpectBase => {
                    match self.advance() {
                        Some('b') => state = State::BinaryNumber,
                        Some('x') => state = State::HexNumber,
                        _ => {
                            result_token = Token::new(TokenType::Error, self.start, self.current);
                            break;
                        },
                    }

                },
                State::DecimalNumber => {
                    match self.advance() {
                        Some('0'..='9') => state = State::DecimalNumber,
                        Some(ch) if ch.is_whitespace() || ch == ')' || ch =='(' || ch == '>' || ch == '<' => {
                            self.current -= 1;
                            result_token = Token::new(TokenType::DecimalNumber, self.start, self.current);
                            break;
                        },
                        None => {
                            self.current -= 1;
                            result_token = Token::new(TokenType::DecimalNumber, self.start, self.current);
                            break;
                        }
                        _ => {
                            result_token = Token::new(TokenType::Error, self.start, self.current);
                            break;
                        },
                    }
                },
                State::BinaryNumber => {
                    match self.advance() {
                        Some('0'..='1') => state = State::BinaryNumber,
                        Some(ch) if ch.is_whitespace() || ch == ')' || ch =='(' || ch == '>' || ch == '<' => {
                            self.current -= 1;
                            result_token = Token::new(TokenType::BinaryNumber, self.start, self.current);
                            break;
                        },
                        None => {
                            self.current -= 1;
                            result_token = Token::new(TokenType::BinaryNumber, self.start, self.current);
                            break;
                        }
                        _ => {
                            result_token = Token::new(TokenType::Error, self.start, self.current);
                            break;
                        },
                    }
                },
                State::HexNumber => {
                    match self.advance() {
                        Some('0'..='9' | 'a'..='f' | 'A'..='F') => state = State::HexNumber,
                        Some(ch) if ch.is_whitespace() || ch == ')' || ch =='(' || ch == '>' || ch == '<' => {
                            self.current -= 1;
                            result_token = Token::new(TokenType::HexNumber, self.start, self.current);
                            break;
                        },
                        None => {
                            self.current -= 1;
                            result_token = Token::new(TokenType::HexNumber, self.start, self.current);
                            break;
                        }
                        _ => {
                            result_token = Token::new(TokenType::Error, self.start, self.current);
                            break;
                        },
                    }
                }
            }
        }
        result_token
    }

    fn advance(&mut self) -> Option<char> {
        let temp = self.current;
        self.current += 1;
        self.buffer.chars().nth(temp)
    }

    fn skip_whitespaces(&mut self) {
        while self.buffer.chars().nth(self.current).unwrap().is_whitespace() {
            self.current += 1;
        }
    }

    fn get_keyword(&mut self) -> TokenType {
        static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
            "and" => TokenType::And,
            "AND" => TokenType::And,
            "or" => TokenType::Or,
            "OR" => TokenType::Or,
            "nor" => TokenType::Nor,
            "NOR" => TokenType::Nor,
            "xor" => TokenType::Xor,
            "XOR" => TokenType::Xor,
        };
        if let Some(typ) = KEYWORDS.get(self.buffer.get_mut(self.start..self.current).unwrap()).cloned() {
            typ
        } else {
            TokenType::Error
        }
    }
}

impl Token {
    fn new(typ: TokenType, start: usize, end: usize) -> Self {
        Self {
            typ,
            start,
            length: end - start,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespaces() {
        let mut sc = Scanner::new("    >");
        sc.skip_whitespaces();
        assert_eq!(sc.current, 4);
    }

    #[test]
    fn test_single_character_token() {
        let mut sc = Scanner::new("(");
        assert_eq!(sc.next(), Token::new(TokenType::LeftParen, 0, 1));
    }

    #[test]
    fn test_shift_right() {
        let mut sc = Scanner::new(" >>");
        assert_eq!(sc.next(), Token::new(TokenType::ShiftRight, 1, 3));
    }

    #[test]
    fn test_few_parens() {
        let mut sc = Scanner::new("()()");
        assert_eq!(sc.next(), Token::new(TokenType::LeftParen, 0, 1));
        assert_eq!(sc.next(), Token::new(TokenType::RightParen, 1, 2));
        assert_eq!(sc.next(), Token::new(TokenType::LeftParen, 2, 3));
        assert_eq!(sc.next(), Token::new(TokenType::RightParen, 3, 4));
    }

    #[test]
    fn test_dec_number() {
        let mut sc = Scanner::new("1234");
        assert_eq!(sc.next(), Token::new(TokenType::DecimalNumber, 0, 4));
    }

    #[test]
    fn test_hex_number() {
        let mut sc = Scanner::new("0x1234");
        assert_eq!(sc.next(), Token::new(TokenType::HexNumber, 0, 6));
    }

    #[test]
    fn test_bin_number() {
        let mut sc = Scanner::new("0b1010");
        assert_eq!(sc.next(), Token::new(TokenType::BinaryNumber, 0, 6));
    }

    #[test]
    fn test_xor_keyword() {
        let mut sc = Scanner::new("XOR");
        assert_eq!(sc.next(), Token::new(TokenType::Xor, 0, 3));
    }

    #[test]
    fn test_shifts() {
        let mut sc = Scanner::new(">><<");
        assert_eq!(sc.next(), Token::new(TokenType::ShiftRight, 0, 2));
        assert_eq!(sc.next(), Token::new(TokenType::ShiftLeft, 2, 4));
    }

    #[test]
    fn test_complete() {
        let input = "()0x1234 << 10";
        let mut sc = Scanner::new(input);
        assert_eq!(sc.next(), Token::new(TokenType::LeftParen, 0, 1));
        assert_eq!(sc.next(), Token::new(TokenType::RightParen, 1, 2));
        assert_eq!(sc.next(), Token::new(TokenType::HexNumber, 2, 8));
        assert_eq!(sc.next(), Token::new(TokenType::ShiftLeft, 9, 11));
        assert_eq!(sc.next(), Token::new(TokenType::DecimalNumber, 12, input.len()));
    }

    #[test]
    fn test_extended() {
        let input = "(0b1010 + 0xFF) and (2 OR 0b10) << 12";
        let mut sc = Scanner::new(input);
        assert_eq!(sc.next(), Token::new(TokenType::LeftParen, 0, 1));
        assert_eq!(sc.next(), Token::new(TokenType::BinaryNumber, 1, 7));
        assert_eq!(sc.next(), Token::new(TokenType::Plus, 8, 9));
        assert_eq!(sc.next(), Token::new(TokenType::HexNumber, 10, 14));
        assert_eq!(sc.next(), Token::new(TokenType::RightParen, 14, 15));
        assert_eq!(sc.next(), Token::new(TokenType::And, 16, 19));
        assert_eq!(sc.next(), Token::new(TokenType::LeftParen, 20, 21));
        assert_eq!(sc.next(), Token::new(TokenType::DecimalNumber, 21, 22));
        assert_eq!(sc.next(), Token::new(TokenType::Or, 23, 25));
        assert_eq!(sc.next(), Token::new(TokenType::BinaryNumber, 26, 30));
        assert_eq!(sc.next(), Token::new(TokenType::RightParen, 30, 31));
        assert_eq!(sc.next(), Token::new(TokenType::ShiftLeft, 32, 34));
        assert_eq!(sc.next(), Token::new(TokenType::DecimalNumber, 35, input.len()));
    }
}
