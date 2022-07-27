use std::str::Chars;

#[derive(Clone)]
pub struct Scanner<'a> {
    buffer: Chars<'a>,
    lookup: &'a str,
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
    Bang,
    TwosComplement,
    Mult,
    Modulo,

    //more character Tokens
    ShiftLeft,
    ShiftRight,

    //Literals
    BinaryNumber(i64),
    DecimalNumber(i64),
    HexNumber(i64),

    //Keywords
    And,
    Or,
    Xor,
    Nor,

    Error(usize, usize, String),
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
            lookup: input, //is just a copy of the buffer, because the chars iterator consumes the values
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
        let mut state = State::Start;

        let token_start = self.buffer.as_str().len();
        loop {
            match state {
                State::Start => {
                    let ch = self.buffer.next();
                    match ch {
                        Some('(') => {
                            return Token::LeftParen;
                        }
                        Some(')') => {
                            return Token::RightParen;
                        }
                        Some('+') => {
                            return Token::Plus;
                        }
                        Some('-') => {
                            return Token::Minus;
                        }
                        Some('&') => {
                            return Token::And;
                        }
                        Some('|') => {
                            return Token::Or;
                        }
                        Some('^') => {
                            return Token::Xor;
                        }
                        Some('!') => {
                            return Token::Bang;
                        }
                        Some('~') => {
                            return Token::TwosComplement;
                        }
                        Some('*') => {
                            return Token::Mult;
                        }
                        Some('%') => {
                            return Token::Modulo;
                        }
                        Some('>') => state = State::ExpectShiftRight,
                        Some('<') => state = State::ExpectShiftLeft,
                        Some('0') => state = State::ExpectBase,
                        Some('1'..='9') => state = State::DecimalNumber,
                        Some('a'..='z' | 'A'..='Z') => {
                            state = State::Keyword;
                        }
                        None => {
                            return Token::Eof;
                        }
                        _ => {
                            let start = self.initial_len - token_start;
                            let token_len = self.initial_len - self.buffer.as_str().len();
                            return Token::Error(start, token_len, self.lookup[start..token_len].to_string());
                        }
                    }
                }
                State::ExpectShiftRight => match self.buffer.next() {
                    Some('>') => {
                        return Token::ShiftRight;
                    }
                    None => {
                        return Token::ShiftRight;
                    }
                    _ => {
                        return Token::ShiftRight;
                    }
                },
                State::ExpectShiftLeft => match self.buffer.next() {
                    Some('<') => {
                        return Token::ShiftLeft;
                    }
                    None => {
                        return Token::ShiftLeft;
                    }
                    _ => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        return Token::Error(start, token_len, self.lookup[start..token_len].to_string());
                    }
                },
                State::Keyword => {
                    let next_char = self.peek_char();
                    match next_char {
                        'a'..='z' | 'A'..='Z' => {
                            self.buffer.next();
                        }
                        ch if is_delimiter(ch) => {
                            let start = self.initial_len - token_start;
                            let token_len = self.initial_len - self.buffer.as_str().len();
                            return self.get_keyword(&self.clone().lookup[start..token_len]);
                        }
                        EOF_CHAR => {
                            let start = self.initial_len - token_start;
                            let token_len = self.initial_len - self.buffer.as_str().len();
                            return self.get_keyword(&self.clone().lookup[start..token_len]);
                        }
                        _ => {
                            let start = self.initial_len - token_start;
                            let token_len = self.initial_len - self.buffer.as_str().len();
                            return Token::Error(start, token_len, self.lookup[start..token_len].to_string());
                        }
                    }
                }
                State::ExpectBase => match self.buffer.next() {
                    Some('b') => state = State::BinaryNumber,
                    Some('x') => state = State::HexNumber,
                    _ => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        return Token::Error(start, token_len, self.lookup[start..token_len].to_string());
                    }
                },
                State::DecimalNumber => match self.peek_char() {
                    '0'..='9' => {
                        state = State::DecimalNumber;
                        self.buffer.next();
                    }
                    ch if is_delimiter(ch) => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        return
                            Token::DecimalNumber(self.lookup[start..token_len].parse().unwrap());
                    }
                    EOF_CHAR => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        return
                            Token::DecimalNumber(self.lookup[start..token_len].parse().unwrap());
                    }
                    _ => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        return Token::Error(start, token_len, self.lookup[start..token_len].to_string());
                    }
                },
                State::BinaryNumber => match self.peek_char() {
                    '0'..='1' => {
                        state = State::BinaryNumber;
                        self.buffer.next();
                    }
                    ch if is_delimiter(ch) => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        return
                            Token::BinaryNumber(i64::from_str_radix(&self.lookup[start + 2..token_len], 2).unwrap());
                    }
                    EOF_CHAR => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        return
                            Token::BinaryNumber(i64::from_str_radix(&self.lookup[start + 2..token_len], 2).unwrap());
                    }
                    _ => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        return Token::Error(start, token_len, self.lookup[start..token_len].to_string());
                    }
                },
                State::HexNumber => match self.peek_char() {
                    '0'..='9' | 'a'..='f' | 'A'..='F' => {
                        state = State::HexNumber;
                        self.buffer.next();
                    }
                    ch if is_delimiter(ch) => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        return
                            Token::HexNumber(i64::from_str_radix(&self.lookup[start + 2..token_len], 16).unwrap());
                    }
                    EOF_CHAR => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        return
                            Token::HexNumber(i64::from_str_radix(&self.lookup[start + 2..token_len], 16).unwrap());
                    }
                    _ => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        return Token::Error(start, token_len, self.lookup[start..token_len].to_string());
                    }
                },
            }
        }
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
            "and" | "AND" | "&" => Token::And,
            "or" | "OR" | "|" => Token::Or,
            "nor" | "NOR" => Token::Nor,
            "xor" | "XOR" | "^" => Token::Xor,
            "mod" | "MOD" | "%" => Token::Modulo,
            _ => Token::KeywordNotFound,
        }
    }
}

impl Token {
    pub fn is_operator(&self) -> bool {
        match self {
            Token::Plus
            | Token::Minus
            | Token::And
            | Token::Or
            | Token::Nor
            | Token::Xor
            | Token::ShiftLeft
            | Token::ShiftRight
            | Token::Bang
            | Token::TwosComplement
            | Token::LeftParen
            | Token::RightParen
            | Token::Modulo
            | Token::Mult => true,
            _ => false,
        }
    }

    pub fn is_operand(&self) -> bool {
        match self {
            Token::DecimalNumber(_) | Token::BinaryNumber(_) | Token::HexNumber(_) => true,
            _ => false,
        }
    }

    pub fn get_value(&self) -> Option<i64> {
        match self {
            Token::DecimalNumber(v) | Token::BinaryNumber(v) | Token::HexNumber(v) => Some(*v),
            _ => None,
        }
    }
}

fn is_delimiter(c: char) -> bool {
    matches!(
        c,
        '('
            | ')'
            | '>'
            | '<'
            | '+'
            | '-'
            | '&'
            | '|'
            | '~'
            | '!'
            | '^'
            | '*'
            | '%'
             //whitespaces:
            | '\u{0009}'   // \t
            | '\u{000A}' // \n
            | '\u{000B}' // vertical tab
            | '\u{000C}' // form feed
            | '\u{000D}' // \r
            | '\u{0020}' // space

            // NEXT LINE from latin1
            | '\u{0085}'

            // Bidi markers
            | '\u{200E}' // LEFT-TO-RIGHT MARK
            | '\u{200F}' // RIGHT-TO-LEFT MARK

            // Dedicated whitespace characters from Unicode
            | '\u{2028}' // LINE SEPARATOR
            | '\u{2029}'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_without_spaces() {
        let mut sc = Scanner::new("1+2+3+4");
        assert_eq!(Token::DecimalNumber(1), sc.next());
        assert_eq!(Token::Plus, sc.next());
        assert_eq!(Token::DecimalNumber(2), sc.next());
        assert_eq!(Token::Plus, sc.next());
        assert_eq!(Token::DecimalNumber(3), sc.next());
        assert_eq!(Token::Plus, sc.next());
        assert_eq!(Token::DecimalNumber(4), sc.next());
    }

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
        assert_eq!(sc.next(), Token::DecimalNumber(i64::from_str_radix(&"1234", 10).unwrap()));
    }

    #[test]
    fn test_hex_number() {
        let mut sc = Scanner::new("0x1234");
        assert_eq!(sc.next(), Token::HexNumber(i64::from_str_radix("1234", 16).unwrap()));
    }

    #[test]
    fn test_bin_number() {
        let mut sc = Scanner::new("0b1010");
        assert_eq!(sc.next(), Token::BinaryNumber(i64::from_str_radix("1010", 2).unwrap()));
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

    #[test]
    fn test_complete() {
        let input = "()0x1234 << 10";
        let mut sc = Scanner::new(input);
        assert_eq!(sc.next(), Token::LeftParen);
        assert_eq!(sc.next(), Token::RightParen);
        assert_eq!(sc.next(), Token::HexNumber(i64::from_str_radix("1234", 16).unwrap()));
        assert_eq!(sc.next(), Token::ShiftLeft);
        assert_eq!(sc.next(), Token::DecimalNumber(10));
    }

    #[test]
    fn test_extended() {
        let input = "(0b1010 + 0xFF) and (2 OR 0b10) << 12";
        let mut sc = Scanner::new(input);
        assert_eq!(sc.next(), Token::LeftParen);
        assert_eq!(sc.next(), Token::BinaryNumber(i64::from_str_radix("1010", 2).unwrap()));
        assert_eq!(sc.next(), Token::Plus);
        assert_eq!(sc.next(), Token::HexNumber(i64::from_str_radix(&"ff", 16).unwrap()));
        assert_eq!(sc.next(), Token::RightParen);
        assert_eq!(sc.next(), Token::And);
        assert_eq!(sc.next(), Token::LeftParen);
        assert_eq!(sc.next(), Token::DecimalNumber(2));
        assert_eq!(sc.next(), Token::Or);
        assert_eq!(sc.next(), Token::BinaryNumber(i64::from_str_radix("10", 2).unwrap()));
        assert_eq!(sc.next(), Token::RightParen);
        assert_eq!(sc.next(), Token::ShiftLeft);
        assert_eq!(sc.next(), Token::DecimalNumber(12));
    }

    #[test]
    fn test_mult_token() {
        let mut sc = Scanner::new("5*3");
        assert_eq!(sc.next(), Token::DecimalNumber(5));
        assert_eq!(sc.next(), Token::Mult);
        assert_eq!(sc.next(), Token::DecimalNumber(3));
    }

    #[test]
    fn test_zero_decimal() {
        let mut sc = Scanner::new("0");
        assert_eq!(sc.next(), Token::DecimalNumber(0));
    }
}
