use std::sync::mpsc::{self, channel, Sender, Receiver};
use std::thread;
use std::fmt;
use crate::scanner::{Scanner, Token};

#[derive(Clone)]
pub struct Parser {
    buffer: String,
    res: Vec<Token>,
}

impl<'a> Parser {
    pub fn new(input: &'a str) -> Self {
        Self {
            buffer: input.to_string(),
            res: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let input = self.buffer.clone();
        let mut scanner = Scanner::new(&input);
        self.parser_worker(&mut scanner, 0)
    }

    fn parser_worker(&mut self, scanner: &mut Scanner, min_bp: u8) -> Vec<Token> {
        let next = scanner.next();
        let mut lhs = match next {
            //next should be a number or a left paren
            Token::DecimalNumber(_) | Token::BinaryNumber(_) | Token::HexNumber(_) => vec![next],
            Token::LeftParen => {
                let lhs = self.parser_worker(scanner, 0);
                assert_eq!(scanner.next(), Token::RightParen);
                lhs
            },
            Token::TwosComplement | Token::Bang => {
                let ((), r_bp) = self.prefix_binding_power(&next);
                let mut rhs = self.parser_worker(scanner, r_bp);
                rhs.append(&mut vec![next]);
                rhs
            },
            _ => panic!("bad token: {:?}", &next),
        };

        loop {
            let op = match scanner.peek() {
                eof if eof == Token::Eof => break,
                operator if operator.is_operator() => operator,
                err => panic!("bad token >>> {:?}", err),
            };
            //now compute the binding power of the just fetched operator
            if let Some((l_bp, r_bp)) = self.infix_binding_power(&op) {
                //stop eating more tokens, if the left bp is lower than the min_bp
                if l_bp < min_bp {
                    break;
                }
                scanner.next(); //eat the previous looked at operator (this is safe, because op breaks out of the loop if op.peek() == eof)

                let rhs = self.parser_worker(scanner, r_bp);
                lhs.append(&mut rhs.clone());
                lhs.append(&mut vec![op.clone()]);
                continue;
            }
            break;
        }
        lhs
    }

    ///returns the precedence of the given `op`.
    ///# Example
    /// ```
    /// assert_eq!(infix_binding_power(Token::new(Token::Plus, 0, 1), (11, 12)));
    /// ```
    fn infix_binding_power(&self, op: &Token) -> Option<(u8, u8)> {
        let res = match &op {
            Token::Modulo => (13, 14),
            Token::Mult => (13, 14),
            Token::Plus | Token::Minus => (11, 12), //highest precedence
            Token::ShiftRight | Token::ShiftLeft => (9, 10),
            Token::And => (7, 8),
            Token::Xor => (5, 6),
            Token::Nor => (3, 4),
            Token::Or => (1, 2),
            _ => return None,
        };
        Some(res)
    }

    fn prefix_binding_power(&self, op: &Token) -> ((), u8) {
        match op {
            Token::Minus | Token::Bang | Token::TwosComplement => ((), 255),
            _ => panic!("bad token {:?}", &op),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bang() {
        let mut p = Parser::new("!5");
        let result = p.parse();
        assert_eq!(vec![Token::DecimalNumber(5), Token::Bang], result);
    }

    #[test]
    fn test_nested() {
        let mut p = Parser::new("1 and 2 + 3 and 4 + 5");
        let actual = p.parse();
        //assert_eq!(vec![Token::DecimalNumber(1), Token::DecimalNumber(2), Token::DecimalNumber(3), Token::Plus, Token::And, Token::DecimalNumber(4), Token::DecimalNumber(5), Token::Plus, Token::And], actual);
        println!("actual: {:?}", &actual);
    }

}
