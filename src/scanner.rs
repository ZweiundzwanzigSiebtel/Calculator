
//#[derive(Debug)]
struct Scanner {
    buffer: String,
    start: usize,
    current: usize,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
struct Token {
    typ: TokenType,
    start: usize,
    length: usize,
    //maybe add line here for when the Interpreter reads from a file.
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

        let iter = self.buffer.chars();
        let mut iter = iter.skip(self.current);

        self.current += 1;
        match iter.next() { //TODO: Remove unwrap here...
            Some('(') => Token::new(TokenType::LeftParen, self.start, self.current),
            Some(')') => Token::new(TokenType::RightParen, self.start, self.current),
            Some('+') => Token::new(TokenType::Plus, self.start, self.current),
            Some('-') => Token::new(TokenType::Minus, self.start, self.current),
            Some('<') => {
                self.current += 1;
                match iter.next() {
                    Some('<') => Token::new(TokenType::ShiftLeft, self.start, self.current),
                    _ => Token::new(TokenType::Error, self.start, self.current),
                }
            },
            Some('>') => {
                self.current += 1;
                match iter.next() {
                    Some('>') => Token::new(TokenType::ShiftRight, self.start, self.current),
                    _ => Token::new(TokenType::Error, self.start, self.current),
                }
            },
            Some('0') => {
                self.current += 1;
                match iter.next() {
                    Some('b') => self.handle_numbers(TokenType::BinaryNumber),
                    Some('x') => self.handle_numbers(TokenType::HexNumber),
                    _ => panic!("should not happen"),
                }
            }
            Some('a'..='z' | 'A'..='Z') => self.handle_keyword(),
            Some('1'..='9') => self.handle_numbers(TokenType::DecimalNumber),
            _ => Token::new(TokenType::NotImplemented, self.start, self.current),
        }
    }

    fn skip_whitespaces(&mut self) {
        while let Some(possible_whitespace) = self.buffer.chars().nth(self.current) {
            if possible_whitespace.is_whitespace() {
                self.current += 1;
            } else {
                break;
            }
        }
    }

    fn handle_numbers(&mut self, typ: TokenType) -> Token {
        self.current += 1;
        println!("self_current is now: {:?}", self.buffer.chars().nth(self.current));
        while let Some(number) = self.buffer.chars().nth(self.current) {
            match typ {
                TokenType::BinaryNumber => {
                    match number {
                        '0'..='1' => self.current += 1,
                        _ => break,
                    }
                }
                TokenType::DecimalNumber => {
                    match number {
                        '0'..='9' | 'a'..='f' | 'A'..='F' => self.current += 1,
                        _ => break,
                    }
                },
                TokenType::HexNumber => {
                    match number {
                        '0'..='9' | 'a'..='f' | 'A'..='F' => self.current += 1,
                        _ => break,
                    }
                },
                _ => panic!("should never be another type except bin, dec or hex!"),
            }
        }
        Token::new(typ, self.start, self.current)
    }

    fn handle_keyword(&mut self) -> Token {
        let iter = self.buffer.chars();
        let mut iter = iter.skip(self.current);
        while let Some(value) = iter.next() {
            self.current += 1;
        }
        Token::new(TokenType::Error, self.start, self.current)
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
    fn test_shift_left() {
        let mut sc = Scanner::new(" <<");
        assert_eq!(sc.next(), Token::new(TokenType::ShiftLeft, 1, 3));
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

//    #[test]
//    fn test_xor_keyword() {
//        let mut sc = Scanner::new("XOR");
//        sc.next();
//    }
//
//    #[test]
//    fn test_complete() {
//        let input = "()0x1234 << 10";
//        let mut sc = Scanner::new(input);
//        assert_eq!(sc.next(), Token::new(TokenType::LeftParen, 0, 1));
//        assert_eq!(sc.next(), Token::new(TokenType::RightParen, 1, 2));
//        assert_eq!(sc.next(), Token::new(TokenType::HexNumber, 2, 8));
//        assert_eq!(sc.next(), Token::new(TokenType::ShiftLeft, 9, 11));
//        assert_eq!(sc.next(), Token::new(TokenType::DecimalNumber, 12, input.len()));
//    }
}
