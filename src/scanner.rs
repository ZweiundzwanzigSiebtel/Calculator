//!
//!
//!
//! TODO the `advance()` function cals chars().nth(x) every time it gets called.
//! This leads to multiple iterations per input read.
//! For short inputs this shouldn't be a problem, but longer input could take a few miliseconds to
//! read...
//!
//!
//!
//!
//!

use phf::phf_map;
use std::str::Chars;

//#[derive(Debug)]
pub struct Scanner<'a> {
    buffer: Chars<'a>,
    initial_len: usize,
}

const EOF_CHAR: char = '\0';

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
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

#[derive(Debug, PartialEq)]
pub struct Token {
    pub typ: TokenType,
    start: usize,
    length: usize,
}

impl<'a> Scanner<'a> {
    /// Creates a new scanner instance
    pub fn new(input: &'a str) -> Self {
        Self {
            buffer: input.chars(),
            initial_len: input.len(),
        }
    }

    /// returns the next Token from the buffer the scanner was instantiated with.
    /// # Example:
    /// ``let mut sc = Scanner("13 37");
    /// assert_eq!(sc.next(), Token::new(TokenType::DecimalNumber, 0, 2));
    /// assert_eq!(sc.next(), Token::new(TokenType::DecimalNumber, 3, 5));```
    pub fn next(&mut self) -> Token {
        self.eat_while(char::is_whitespace);
        let mut result_token;
        let mut state = State::Start;

        let token_start = self.buffer.as_str().len();
        loop {
            match state {
                State::Start => match self.buffer.next() {
                    Some('(') => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::LeftParen, start, token_len);
                        break;
                    }
                    Some(')') => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::RightParen, start, token_len);
                        break;
                    }
                    Some('+') => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::Plus, start, token_len);
                        break;
                    }
                    Some('-') => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::Minus, start, token_len);
                        break;
                    }
                    Some('>') => state = State::ExpectShiftRight,
                    Some('<') => state = State::ExpectShiftLeft,
                    Some('0') => state = State::ExpectBase,
                    Some('1'..='9') => state = State::DecimalNumber,
                    Some('a'..='z' | 'A'..='Z') => state = State::Keyword,
                    None => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::Eof, start, token_len);
                        break;
                    }
                    _ => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::Error, start, token_len);
                        break;
                    }
                },
                State::ExpectShiftRight => match self.buffer.next() {
                    Some('>') => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::ShiftRight, start, token_len);
                        break;
                    }
                    None => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::ShiftRight, start, token_len);
                        break;
                    }
                    _ => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::ShiftRight, start, token_len);
                        break;
                    }
                },
                State::ExpectShiftLeft => match self.buffer.next() {
                    Some('<') => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::ShiftLeft, start, token_len);
                        break;
                    }
                    None => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::ShiftLeft, start, token_len);
                        break;
                    }
                    _ => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::Error, start, token_len);
                        break;
                    }
                },
                State::Keyword => match self.buffer.next() {
                    Some('a'..='z' | 'A'..='Z') => {}
                    Some(ch) if ch.is_whitespace() => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(self.get_keyword(start, token_len), start, token_len);
                        break;
                    }
                    None => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(self.get_keyword(start, token_len), start, token_len);
                        break;
                    }
                    _ => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::Error, start, token_len);
                        break;
                    }
                },
                State::ExpectBase => match self.buffer.next() {
                    Some('b') => state = State::BinaryNumber,
                    Some('x') => state = State::HexNumber,
                    _ => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::Error, start, token_len);
                        break;
                    }
                },
                State::DecimalNumber => match self.buffer.next() {
                    Some('0'..='9') => state = State::DecimalNumber,
                    Some(ch)
                        if ch.is_whitespace()
                            || ch == ')'
                            || ch == '('
                            || ch == '>'
                            || ch == '<' =>
                    {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::DecimalNumber, start, token_len);
                        break;
                    }
                    None => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::DecimalNumber, start, token_len);
                        break;
                    }
                    _ => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::Error, start, token_len);
                        break;
                    }
                },
                State::BinaryNumber => match self.buffer.next() {
                    Some('0'..='1') => state = State::BinaryNumber,
                    Some(ch)
                        if ch.is_whitespace()
                            || ch == ')'
                            || ch == '('
                            || ch == '>'
                            || ch == '<' =>
                    {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::BinaryNumber, start, token_len);
                        break;
                    }
                    None => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::BinaryNumber, start, token_len);
                        break;
                    }
                    _ => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::Error, start, token_len);
                        break;
                    }
                },
                State::HexNumber => match self.buffer.next() {
                    Some('0'..='9' | 'a'..='f' | 'A'..='F') => state = State::HexNumber,
                    Some(ch)
                        if ch.is_whitespace()
                            || ch == ')'
                            || ch == '('
                            || ch == '>'
                            || ch == '<' =>
                    {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::HexNumber, start, token_len);
                        break;
                    }
                    None => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::HexNumber, start, token_len);
                        break;
                    }
                    _ => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::new(TokenType::Error, start, token_len);
                        break;
                    }
                },
            }
        }
        result_token
    }

    fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.peek()) && !self.buffer.as_str().is_empty() {
            self.buffer.next();
        }
    }

    pub fn peek(&mut self) -> char {
        self.buffer.clone().next().unwrap_or(EOF_CHAR)
    }

    fn get_keyword(&mut self, start: usize, len: usize) -> TokenType {
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
        println!("{:?}", &self.buffer.as_str());
        if let Some(possible_keyword) = self.buffer.as_str().to_owned().get_mut(start..start+len) {
            return KEYWORDS.get(possible_keyword).cloned().unwrap();
        }
        TokenType::KeywordNotFound

    }
}

impl Token {
    fn new(typ: TokenType, start: usize, length: usize) -> Self {
        Self { typ, start, length }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespaces_only() {
        let mut sc = Scanner::new("  ");
        assert_eq!(sc.next(), None);
    }

    #[test]
    fn test_single_character_token() {
        let mut sc = Scanner::new("(");
        assert_eq!(sc.next().unwrap(), Token::new(TokenType::LeftParen, 0, 1));
    }

    #[test]
    fn test_shift_right() {
        let mut sc = Scanner::new(">>");
        assert_eq!(sc.next().unwrap(), Token::new(TokenType::ShiftRight, 0, 2));
    }

    #[test]
    fn test_few_parens() {
        let mut sc = Scanner::new("()()");
        assert_eq!(sc.next().unwrap(), Token::new(TokenType::LeftParen, 0, 1));
        assert_eq!(sc.next().unwrap(), Token::new(TokenType::RightParen, 1, 2));
        assert_eq!(sc.next().unwrap(), Token::new(TokenType::LeftParen, 2, 3));
        assert_eq!(sc.next().unwrap(), Token::new(TokenType::RightParen, 3, 4));
    }

    #[test]
    fn test_dec_number() {
        let mut sc = Scanner::new("1234");
        assert_eq!(
            sc.next().unwrap(),
            Token::new(TokenType::DecimalNumber, 0, 4)
        );
    }

    #[test]
    fn test_hex_number() {
        let mut sc = Scanner::new("0x1234");
        assert_eq!(sc.next().unwrap(), Token::new(TokenType::HexNumber, 0, 6));
    }

    #[test]
    fn test_bin_number() {
        let mut sc = Scanner::new("0b1010");
        assert_eq!(
            sc.next().unwrap(),
            Token::new(TokenType::BinaryNumber, 0, 6)
        );
    }

    #[test]
    fn test_xor_keyword() {
        let mut sc = Scanner::new("XOR");
        assert_eq!(sc.next().unwrap(), Token::new(TokenType::Xor, 0, 3));
    }

    #[test]
    fn test_shifts() {
        let mut sc = Scanner::new(">><<");
        assert_eq!(sc.next().unwrap(), Token::new(TokenType::ShiftRight, 0, 2));
        assert_eq!(sc.next().unwrap(), Token::new(TokenType::ShiftLeft, 2, 4));
    }

//    #[test]
//    fn test_complete() {
//        let input = "()0x1234 << 10";
//        let mut sc = Scanner::new(input);
//        assert_eq!(sc.next().unwrap(), Token::new(TokenType::LeftParen, 0, 1));
//        assert_eq!(sc.next().unwrap(), Token::new(TokenType::RightParen, 1, 2));
//        assert_eq!(sc.next().unwrap(), Token::new(TokenType::HexNumber, 2, 8));
//        assert_eq!(sc.next().unwrap(), Token::new(TokenType::ShiftLeft, 9, 11));
//        assert_eq!(
//            sc.next().unwrap(),
//            Token::new(TokenType::DecimalNumber, 12, input.len())
//        );
//    }
//
//    #[test]
//    fn test_extended() {
//        let input = "(0b1010 + 0xFF) and (2 OR 0b10) << 12";
//        let mut sc = Scanner::new(input);
//        assert_eq!(sc.next().unwrap(), Token::new(TokenType::LeftParen, 0, 1));
//        assert_eq!(sc.next().unwrap(), Token::new(TokenType::BinaryNumber, 1, 7));
//        assert_eq!(sc.next().unwrap(), Token::new(TokenType::Plus, 8, 9));
//        assert_eq!(sc.next().unwrap(), Token::new(TokenType::HexNumber, 10, 14));
//        assert_eq!(sc.next().unwrap(), Token::new(TokenType::RightParen, 14, 15));
//        assert_eq!(sc.next().unwrap(), Token::new(TokenType::And, 16, 19));
//        assert_eq!(sc.next().unwrap(), Token::new(TokenType::LeftParen, 20, 21));
//        assert_eq!(sc.next().unwrap(), Token::new(TokenType::DecimalNumber, 21, 22));
//        assert_eq!(sc.next().unwrap(), Token::new(TokenType::Or, 23, 25));
//        assert_eq!(sc.next().unwrap(), Token::new(TokenType::BinaryNumber, 26, 30));
//        assert_eq!(sc.next().unwrap(), Token::new(TokenType::RightParen, 30, 31));
//        assert_eq!(sc.next().unwrap(), Token::new(TokenType::ShiftLeft, 32, 34));
//        assert_eq!(
//            sc.next().unwrap(),
//            Token::new(TokenType::DecimalNumber, 35, input.len())
//        );
//    }
}
