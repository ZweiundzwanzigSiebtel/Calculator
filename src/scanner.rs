use std::fmt;
use std::str::Chars;

#[derive(Clone)]
pub struct Scanner<'a> {
    buffer: Chars<'a>,
    lookup: String,
    initial_len: usize,
}

const EOF_CHAR: char = '\0';

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    //single-character Tokens
    LeftParen,
    RightParen,
    Minus,
    Plus,

    //more character Tokens
    ShiftLeft,
    ShiftRight,

    //Literals
    BinaryNumber(u32),
    DecimalNumber(u32),
    HexNumber(u32),

    //Keywords
    And,
    Or,
    Xor,
    Nor,

    Error(usize, usize),
    KeywordNotFound,
    Eof,
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

impl<'a> Scanner<'a> {
    /// Creates a new scanner instance
    pub fn new(input: &'a str) -> Self {
        Self {
            buffer: input.chars(),
            lookup: input.to_string(), //is just a copy of the buffer, because the chars iterator
                                       //consumes the values
            initial_len: input.len(),
        }
    }

    /// returns the next Token from the buffer the scanner was instantiated with.
    /// # Example:
    /// ``let mut sc = Scanner("13 37");
    /// assert_eq!(sc.next(), Token::DecimalNumber(13));
    /// assert_eq!(sc.next(), Token::DecimalNumber(37));```
    pub fn next(&mut self) -> Token {
        self.eat_while(char::is_whitespace);
        let result_token;
        let mut state = State::Start;

        let token_start = self.buffer.as_str().len();
        loop {
            match state {
                State::Start => {
                    let ch = self.buffer.next();
                    match ch {
                        Some('(') => {
                            result_token = Token::LeftParen;
                            break;
                        }
                        Some(')') => {
                            result_token = Token::RightParen;
                            break;
                        }
                        Some('+') => {
                            result_token = Token::Plus;
                            break;
                        }
                        Some('-') => {
                            result_token = Token::Minus;
                            break;
                        }
                        Some('>') => state = State::ExpectShiftRight,
                        Some('<') => state = State::ExpectShiftLeft,
                        Some('0') => state = State::ExpectBase,
                        Some('1'..='9') => state = State::DecimalNumber,
                        Some('a'..='z' | 'A'..='Z') => {
                            state = State::Keyword;
                        }
                        None => {
                            result_token = Token::Eof;
                            break;
                        }
                        _ => {
                            let start = self.initial_len - token_start;
                            let token_len = self.initial_len - self.buffer.as_str().len();
                            result_token = Token::Error(start, token_len);
                            break;
                        }
                    }
                }
                State::ExpectShiftRight => match self.buffer.next() {
                    Some('>') => {
                        result_token = Token::ShiftRight;
                        break;
                    }
                    None => {
                        result_token = Token::ShiftRight;
                        break;
                    }
                    _ => {
                        result_token = Token::ShiftRight;
                        break;
                    }
                },
                State::ExpectShiftLeft => match self.buffer.next() {
                    Some('<') => {
                        result_token = Token::ShiftLeft;
                        break;
                    }
                    None => {
                        result_token = Token::ShiftLeft;
                        break;
                    }
                    _ => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::Error(start, token_len);
                        break;
                    }
                },
                State::Keyword => {
                    let next_char = self.peek_char();
                    match next_char {
                        'a'..='z' | 'A'..='Z' => {
                            self.buffer.next();
                        },
                        ch if ch.is_whitespace() => {
                            let start = self.initial_len - token_start;
                            let token_len = self.initial_len - self.buffer.as_str().len();
                            result_token = self.get_keyword(&self.clone().lookup[start..token_len]);
                            break;
                        }
                        EOF_CHAR => {
                            let start = self.initial_len - token_start;
                            let token_len = self.initial_len - self.buffer.as_str().len();
                            result_token = self.get_keyword(&self.clone().lookup[start..token_len]);
                            break;
                        }
                        _ => {
                            let start = self.initial_len - token_start;
                            let token_len = self.initial_len - self.buffer.as_str().len();
                            result_token = Token::Error(start, token_len);
                            break;
                        }
                    }
                }
                State::ExpectBase => match self.buffer.next() {
                    Some('b') => state = State::BinaryNumber,
                    Some('x') => state = State::HexNumber,
                    _ => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::Error(start, token_len);
                        break;
                    }
                },
                State::DecimalNumber => match self.peek_char() {
                    '0'..='9' => {
                        state = State::DecimalNumber;
                        self.buffer.next();
                    },
                    ch if ch.is_whitespace()
                            || ch == ')'
                            || ch == '('
                            || ch == '>'
                            || ch == '<' => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::DecimalNumber(self.lookup[start..token_len].parse().unwrap());
                        break;
                    }
                    EOF_CHAR => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::DecimalNumber(self.lookup[start..token_len].parse().unwrap());
                        break;
                    }
                    _ => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::Error(start, token_len);
                        break;
                    }
                },
                State::BinaryNumber => match self.peek_char() {
                    '0'..='1' => {
                        state = State::BinaryNumber;
                        self.buffer.next();
                    },
                    ch
                        if ch.is_whitespace()
                            || ch == ')'
                            || ch == '('
                            || ch == '>'
                            || ch == '<' =>
                    {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::BinaryNumber(self.lookup[start+2..token_len].parse().unwrap());
                        break;
                    }
                    EOF_CHAR => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::BinaryNumber(self.lookup[start+2..token_len].parse().unwrap());
                        break;
                    }
                    _ => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::Error(start, token_len);
                        break;
                    }
                },
                State::HexNumber => match self.peek_char() {
                    '0'..='9' | 'a'..='f' | 'A'..='F' => {
                        state = State::HexNumber;
                        self.buffer.next();
                    },
                    ch if ch.is_whitespace()
                            || ch == ')'
                            || ch == '('
                            || ch == '>'
                            || ch == '<' =>
                    {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::HexNumber(self.lookup[start+2..token_len].parse().unwrap());
                        break;
                    }
                    EOF_CHAR => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::HexNumber(self.lookup[start+2..token_len].parse().unwrap());
                        break;
                    }
                    _ => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::Error(start, token_len);
                        break;
                    }
                },
            }
        }
        result_token
    }

    pub fn peek(&mut self) -> Token {
        self.clone().next()
    }

    fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.peek_char()) && !self.buffer.as_str().is_empty() {
            self.buffer.next();
        }
    }

    fn peek_char(&mut self) -> char {
        self.buffer.clone().next().unwrap_or(EOF_CHAR)
    }

    fn get_keyword(&mut self, possible_keyword: &str) -> Token {
        match possible_keyword {
            "and" | "AND" => Token::And, 
            "or" | "OR" => Token::Or, 
            "nor" | "NOR" => Token::Nor, 
            "xor" | "XOR" => Token::Xor, 
            _ => Token::KeywordNotFound,
        }
    }
}

//impl fmt::Display for Token {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        match self.typ {
//            TokenType::Or => write!(f, "OR"),
//            TokenType::And => write!(f, "AND"),
//            TokenType::Xor => write!(f, "XOR"),
//            TokenType::Nor => write!(f, "NOR"),
//            TokenType::Eof => write!(f, "EOF"),
//            TokenType::Plus => write!(f, "+"),
//            TokenType::Minus => write!(f, "-"),
//            TokenType::Error => write!(f, "Error"),
//            TokenType::LeftParen => write!(f, "("),
//            TokenType::RightParen => write!(f, ")"),
//            TokenType::ShiftLeft => write!(f, "<<"),
//            TokenType::ShiftRight => write!(f, ">>"),
//            TokenType::DecimalNumber(_) => write!(f, "DecimalNumber"),
//            TokenType::BinaryNumber(_) => write!(f, "BinaryNumber"),
//            TokenType::HexNumber(_) => write!(f, "HexNumber"),
//            TokenType::KeywordNotFound => write!(f, "KeywordNotFound"),
//        }
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek_char_behaviour() {
        let mut sc = Scanner::new("(12 + 12)");
        assert_eq!(sc.next(), Token::LeftParen);
        assert_eq!(sc.next(), Token::DecimalNumber(12));
        assert_eq!(sc.next(), Token::Plus);
        assert_eq!(sc.next(), Token::DecimalNumber(12));
        assert_eq!(sc.next(), Token::RightParen);
    }

    #[test]
    fn test_whitespaces_only() {
        let mut sc = Scanner::new("  ");
        assert_eq!(sc.next(), Token::Eof);
    }

    #[test]
    fn test_single_character_token() {
        let mut sc = Scanner::new("(");
        assert_eq!(sc.next(), Token::LeftParen);
    }

    #[test]
    fn test_shift_right() {
        let mut sc = Scanner::new(">>");
        assert_eq!(sc.next(), Token::ShiftRight);
    }

    #[test]
    fn test_few_parens() {
        let mut sc = Scanner::new("()()");
        assert_eq!(sc.next(), Token::LeftParen);
        assert_eq!(sc.next(), Token::RightParen);
        assert_eq!(sc.next(), Token::LeftParen);
        assert_eq!(sc.next(), Token::RightParen);
    }

    #[test]
    fn test_dec_number() {
        let mut sc = Scanner::new("1234");
        assert_eq!(sc.next(), Token::DecimalNumber(1234));
    }

    #[test]
    fn test_hex_number() {
        let mut sc = Scanner::new("0x1234");
        assert_eq!(sc.next(), Token::HexNumber(1234));
    }

    #[test]
    fn test_bin_number() {
        let mut sc = Scanner::new("0b1010");
        assert_eq!(sc.next(), Token::BinaryNumber(1010));
    }

    #[test]
    fn test_xor_keyword() {
        let mut sc = Scanner::new("XOR");
        assert_eq!(sc.next(), Token::Xor);
    }

    #[test]
    fn test_shifts() {
        let mut sc = Scanner::new(">><<");
        assert_eq!(sc.next(), Token::ShiftRight);
        assert_eq!(sc.next(), Token::ShiftLeft);
    }

//    #[test]
//    fn test_complete() {
//        let input = "()0x1234 << 10";
//        let mut sc = Scanner::new(input);
//        assert_eq!(sc.next(), Token::new(TokenType::LeftParen, 0, 1));
//        assert_eq!(sc.next(), Token::new(TokenType::RightParen, 1, 2));
//        assert_eq!(sc.next(), Token::new(TokenType::HexNumber, 2, 8));
//        assert_eq!(sc.next(), Token::new(TokenType::ShiftLeft, 9, 11));
//        assert_eq!(
//            sc.next(),
//            Token::new(TokenType::DecimalNumber, 12, input.len())
//        );
//    }
//
//    #[test]
//    fn test_extended() {
//        let input = "(0b1010 + 0xFF) and (2 OR 0b10) << 12";
//        let mut sc = Scanner::new(input);
//        assert_eq!(sc.next(), Token::new(TokenType::LeftParen, 0, 1));
//        assert_eq!(sc.next(), Token::new(TokenType::BinaryNumber, 1, 7));
//        assert_eq!(sc.next(), Token::new(TokenType::Plus, 8, 9));
//        assert_eq!(sc.next(), Token::new(TokenType::HexNumber, 10, 14));
//        assert_eq!(sc.next(), Token::new(TokenType::RightParen, 14, 15));
//        assert_eq!(sc.next(), Token::new(TokenType::And, 16, 19));
//        assert_eq!(sc.next(), Token::new(TokenType::LeftParen, 20, 21));
//        assert_eq!(sc.next(), Token::new(TokenType::DecimalNumber, 21, 22));
//        assert_eq!(sc.next(), Token::new(TokenType::Or, 23, 25));
//        assert_eq!(sc.next(), Token::new(TokenType::BinaryNumber, 26, 30));
//        assert_eq!(sc.next(), Token::new(TokenType::RightParen, 30, 31));
//        assert_eq!(sc.next(), Token::new(TokenType::ShiftLeft, 32, 34));
//        assert_eq!(
//            sc.next(),
//            Token::new(TokenType::DecimalNumber, 35, input.len())
//        );
//    }
}
