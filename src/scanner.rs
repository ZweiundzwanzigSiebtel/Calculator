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
    Bang,
    TwosComplement,
    Mult,
    Modulo,

    //more character Tokens
    ShiftLeft,
    ShiftRight,

    //Literals
    BinaryNumber(u64),
    DecimalNumber(u64),
    HexNumber(u64),

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
            lookup: input.to_string(), //is just a copy of the buffer, because the chars iterator consumes the values
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
                        Some('&') => {
                            result_token = Token::And;
                            break;
                        }
                        Some('|') => {
                            result_token = Token::Or;
                            break;
                        }
                        Some('^') => {
                            result_token = Token::Xor;
                            break;
                        }
                        Some('!') => {
                            result_token = Token::Bang;
                            break;
                        }
                        Some('~') => {
                            result_token = Token::TwosComplement;
                            break;
                        }
                        Some('*') => {
                            result_token = Token::Mult;
                            break;
                        }
                        Some('%') => {
                            result_token = Token::Modulo;
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
                            result_token = Token::Error(start, token_len, self.lookup[start..token_len].to_string());
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
                        result_token = Token::Error(start, token_len, self.lookup[start..token_len].to_string());
                        break;
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
                            result_token = Token::Error(start, token_len, self.lookup[start..token_len].to_string());
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
                        result_token = Token::Error(start, token_len, self.lookup[start..token_len].to_string());
                        break;
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
                        result_token =
                            Token::DecimalNumber(self.lookup[start..token_len].parse().unwrap());
                        break;
                    }
                    EOF_CHAR => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token =
                            Token::DecimalNumber(self.lookup[start..token_len].parse().unwrap());
                        break;
                    }
                    _ => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::Error(start, token_len, self.lookup[start..token_len].to_string());
                        break;
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
                        result_token =
                            Token::BinaryNumber(str_to_dec(&self.lookup[start + 2..token_len], 2));
                        break;
                    }
                    EOF_CHAR => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token =
                            Token::BinaryNumber(str_to_dec(&self.lookup[start + 2..token_len], 2));
                        break;
                    }
                    _ => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::Error(start, token_len, self.lookup[start..token_len].to_string());
                        break;
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
                        result_token =
                            Token::HexNumber(str_to_dec(&self.lookup[start + 2..token_len], 16));
                        break;
                    }
                    EOF_CHAR => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token =
                            Token::HexNumber(str_to_dec(&self.lookup[start + 2..token_len], 16));
                        break;
                    }
                    _ => {
                        let start = self.initial_len - token_start;
                        let token_len = self.initial_len - self.buffer.as_str().len();
                        result_token = Token::Error(start, token_len, self.lookup[start..token_len].to_string());
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

    pub fn get_value(&self) -> Option<u64> {
        match self {
            Token::DecimalNumber(v) | Token::BinaryNumber(v) | Token::HexNumber(v) => Some(*v),
            _ => None,
        }
    }
}

/// calculates the decimal value of the input of base `base`.
///
/// # Example:
/// assert_eq!(str_to_dec("1234", 10), 1234);
/// assert_eq!(str_to_dec("1010", 2), 10);
/// assert_eq!(str_to_dec("ff", 16), 255);
fn str_to_dec(value: &str, base: u64) -> u64 {
    let mut iter = value.chars();
    let mut result: u64 = 0;
    let max_pow = value.len();
    for power in (0..max_pow).rev() {
        if let Some(ch) = iter.next() {
            match ch {
                c @ '0'..='9' => {
                    result += (c.to_digit(10).unwrap() as u64) * base.pow(power.try_into().unwrap())
                }
                'a' | 'A' => result += 10_u64 * base.pow(power.try_into().unwrap()),
                'b' | 'B' => result += 11_u64 * base.pow(power.try_into().unwrap()),
                'c' | 'C' => result += 12_u64 * base.pow(power.try_into().unwrap()),
                'd' | 'D' => result += 13_u64 * base.pow(power.try_into().unwrap()),
                'e' | 'E' => result += 14_u64 * base.pow(power.try_into().unwrap()),
                'f' | 'F' => result += 15_u64 * base.pow(power.try_into().unwrap()),
                _ => panic!("didn't expect that"),
            }
        }
    }
    result
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
    fn test_string_to_hex() {
        let value = "AA";
        assert_eq!(str_to_dec(&value, 16), 170);
        let v2 = "A0";
        assert_eq!(str_to_dec(&v2, 16), 160);
        let v3 = "0";
        assert_eq!(str_to_dec(&v3, 16), 0);
        let v4 = "FFFFFFFF";
        assert_eq!(str_to_dec(&v4, 16), 4294967295);
        let v5 = "C0FFE";
        assert_eq!(str_to_dec(&v5, 16), 790526);
        let v6 = "a0F3";
        assert_eq!(str_to_dec(&v6, 16), 41203);
        let v7 = "aaBBccDD";
        assert_eq!(str_to_dec(&v7, 16), 2864434397);
        let v8 = "0000";
        assert_eq!(str_to_dec(&v8, 16), 0);
    }

    #[test]
    fn test_bin_from_string() {
        let value = "1010";
        assert_eq!(str_to_dec(&value, 2), 10);
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
        assert_eq!(sc.next(), Token::DecimalNumber(str_to_dec("1234", 10)));
    }

    #[test]
    fn test_hex_number() {
        let mut sc = Scanner::new("0x1234");
        assert_eq!(sc.next(), Token::HexNumber(str_to_dec("1234", 16)));
    }

    #[test]
    fn test_bin_number() {
        let mut sc = Scanner::new("0b1010");
        assert_eq!(sc.next(), Token::BinaryNumber(str_to_dec("1010", 2)));
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
        assert_eq!(sc.next(), Token::HexNumber(str_to_dec("1234", 16)));
        assert_eq!(sc.next(), Token::ShiftLeft);
        assert_eq!(sc.next(), Token::DecimalNumber(10));
    }

    #[test]
    fn test_extended() {
        let input = "(0b1010 + 0xFF) and (2 OR 0b10) << 12";
        let mut sc = Scanner::new(input);
        assert_eq!(sc.next(), Token::LeftParen);
        assert_eq!(sc.next(), Token::BinaryNumber(str_to_dec("1010", 2)));
        assert_eq!(sc.next(), Token::Plus);
        assert_eq!(sc.next(), Token::HexNumber(str_to_dec(&"ff", 16)));
        assert_eq!(sc.next(), Token::RightParen);
        assert_eq!(sc.next(), Token::And);
        assert_eq!(sc.next(), Token::LeftParen);
        assert_eq!(sc.next(), Token::DecimalNumber(2));
        assert_eq!(sc.next(), Token::Or);
        assert_eq!(sc.next(), Token::BinaryNumber(str_to_dec("10", 2)));
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
