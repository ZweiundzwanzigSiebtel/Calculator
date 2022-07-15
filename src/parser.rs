use std::thread;
use std::sync::mpsc::{self, channel, Sender, Receiver};

use std::fmt;
use crate::scanner::{Scanner, Token, TokenType};

#[derive(Debug)]
pub struct Instruction {
    pub op: Token,
    pub lhs: Token,
    pub rhs: Token,
}

impl Instruction {
    fn new(op: Token, lhs: Token, rhs: Token) -> Self {
        Self {
            op,
            lhs,
            rhs,
        }
    }
}

#[derive(Clone)]
pub struct Parser {
    buffer: String,
}

impl<'a> Parser {
    pub fn new(input: &'a str) -> Self {
        Self {
            buffer: input.to_string(),
        }
    }

    pub fn parse(&mut self, input: &str, tx: Sender<Instruction>) {
        let mut scanner = Scanner::new(input);
        self.parser_worker(&mut scanner, 0, tx);
    }

    fn parser_worker(&mut self, scanner: &mut Scanner, min_bp: u8, tx: Sender<Instruction>) -> Token {
        let handle_yield = |op: Token, lhs: Token, rhs: Token| {
            let data = Instruction::new(op, lhs, rhs);
            tx.send(data).unwrap();
        };

        let next = scanner.next();
        let lhs = match next.typ {
            //next should be a number or a left paren
            TokenType::DecimalNumber | TokenType::BinaryNumber | TokenType::HexNumber => {/*handle_yield(S::Atom(next)); */next},
            TokenType::LeftParen => {
                let lhs = self.parser_worker(scanner, 0, tx.clone());
                assert_eq!(scanner.next().typ, TokenType::RightParen);
                lhs
            },
            //currently only a Minus Token is allowed as prefix
            TokenType::Minus => {
                let ((), r_bp) = self.prefix_binding_power(&next);
                let rhs = self.parser_worker(scanner, r_bp, tx.clone());
                rhs
            },
            _ => panic!("bad token: {:?}", &next),
        };


        loop {
            let op = match scanner.peek() {
                eof if eof.typ == TokenType::Eof => break,
                operator
                    if operator.typ == TokenType::Plus
                        || operator.typ == TokenType::Minus
                            || operator.typ == TokenType::And
                            || operator.typ == TokenType::Or
                            || operator.typ == TokenType::Nor
                            || operator.typ == TokenType::Xor
                            || operator.typ == TokenType::ShiftRight
                            || operator.typ == TokenType::ShiftLeft 
                            || operator.typ == TokenType::LeftParen
                            || operator.typ == TokenType::RightParen => {
                                operator
                            },
                        rest => panic!("bad token {:?}", rest),
            };
            //now compute the binding power of the just fetched operator
            if let Some((l_bp, r_bp)) = self.infix_binding_power(&op) {
                //stop eating more tokens, if the left bp is lower than the min_bp
                if l_bp < min_bp {
                    break;
                }
                scanner.next(); //eat the previous looked at operator (this is safe, because op breaks out of the loop if op.peek() == eof)

                let rhs = self.parser_worker(scanner, r_bp, tx.clone());
                handle_yield(op, lhs.clone(), rhs.clone());

                continue;
            }
            break;
        }
        lhs
    }

    ///returns the precedence of the given `op`.
    ///# Example
    /// ```
    /// assert_eq!(infix_binding_power(Token::new(TokenType::Plus, 0, 1), (11, 12)));
    /// ```
    fn infix_binding_power(&self, op: &Token) -> Option<(u8, u8)> {
        let res = match &op.typ {
            TokenType::Plus | TokenType::Minus => (11, 12), //highest precedence
            TokenType::ShiftRight | TokenType::ShiftLeft => (9, 10),
            TokenType::And => (7, 8),
            TokenType::Xor => (5, 6),
            TokenType::Nor => (3, 4),
            TokenType::Or => (1, 2),
            _ => return None,
        };
        Some(res)
    }

    fn prefix_binding_power(&self, op: &Token) -> ((), u8) {
        match op.typ {
            TokenType::Minus => ((), 13),
            _ => panic!("bad token {:?}", &op),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mpsc() {
        let mut p = Parser::new("(1 + 1) << 5");
        let (tx, rx) = mpsc::channel();
        let child = thread::spawn(move || p.parse(&"(1 + 1) << 5", tx));
        for received in rx {
            println!("Got: {:?}", received);
        }
        println!("finished");
        let _ = child.join();
    }

    #[test]
    fn test_longer_expression() {
        let mut p = Parser::new("");
        let (tx, rx) = mpsc::channel();
        let child = thread::spawn(move || p.parse(&"((1 + 1) << 5) and 0xff", tx));
        for received in rx {
            println!("Got: {:?}", received);
        }
        println!("finished");
        let _ = child.join();

    }

    #[test]
    fn test_nested_expression() {
        let mut p = Parser::new("");
        let (tx, rx) = mpsc::channel();
        let child = thread::spawn(move || {let x = p.parse(&"1 and 2 + 3 and 4 + 5", tx); println!("{:?}", &x)});
        for received in rx {
            println!("Got: {:?}", received);
        }
        println!("finished");
        let _ = child.join();
    }

    #[test]
    fn test_chunks() {
        let mut p = Parser::new("");
        let (tx, rx) = mpsc::channel();
        let child = thread::spawn(move || {
            let x = p.parse(&"1 and 2 + 3 and 4 + 5", tx);
            println!("items >>> {:?}", &x);
        });
        for received in rx {
            println!("Got: {:?}", received);
        }
        println!("finished");
        let _ = child.join();
    }

    #[test]
    fn test_high_precedence_last() {
        let mut p = Parser::new("");
        let (tx, rx) = mpsc::channel();
        let child = thread::spawn(move || {let x = p.parse(&"1 and 2 and 3 and 4 + 5", tx); });
        for received in rx {
            println!("Got: {:?}", received);
        }
        println!("finished");
        let _ = child.join();
    }
    
    fn test_simple() {
        let mut p = Parser::new("1 + 1");
        assert_eq!("(+ DecimalNumber DecimalNumber)", p.parse(&"1 + 1").to_string());
    }

    #[test]
    fn test_parens() {
        let mut p = Parser::new("(1 + 1) << 5");
        println!("total result is >>> {}", p.parse(&"1 + 1 << 5").to_string());
    }
}
