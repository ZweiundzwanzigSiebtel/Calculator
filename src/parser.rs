use std::thread;
use std::sync::mpsc::{self, channel, Sender, Receiver};

use std::fmt;
use crate::scanner::{Scanner, Token, TokenType};

#[derive(Clone)]
struct Parser {
    buffer: String,
}

#[derive(Debug, Clone)]
enum S {
    Atom(Token),
    Cons(Token, Vec<S>),
}

impl<'a> Parser {
    fn new(input: &'a str) -> Self {
        Self {
            buffer: input.to_string(),
        }
    }

    pub fn parse(&mut self, input: &str, tx: Sender<S>) -> S {
        let mut scanner = Scanner::new(input);
        self.parser_worker(&mut scanner, 0, tx)
    }

    fn parser_worker(&mut self, scanner: &mut Scanner, min_bp: u8, tx: Sender<S>) -> S {
        let handle_yield = |data: S| {
            tx.send(data).unwrap();
        };

        let next = scanner.next();
        let mut lhs = match next.typ {
            //next should be a number or a left paren
            TokenType::DecimalNumber | TokenType::BinaryNumber | TokenType::HexNumber => S::Atom(next),
            TokenType::LeftParen => {
                let lhs = self.parser_worker(scanner, 0, tx.clone());
                assert_eq!(scanner.next().typ, TokenType::RightParen);
                lhs
            },
            //currently only a Minus Token is allowed as prefix
            TokenType::Minus => {
                let ((), r_bp) = self.prefix_binding_power(&next);
                let rhs = self.parser_worker(scanner, r_bp, tx.clone());
                let test = S::Cons(next, vec![rhs]);
                println!("inside minus: {}", test.to_string());
                test
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


                handle_yield(rhs.clone());
                handle_yield(S::Atom(op));
                handle_yield(lhs.clone());
                lhs = S::Cons(op, vec![lhs, rhs]);
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

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Atom(i) => write!(f, "{}", i),
            S::Cons(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
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
            println!("Got: {}", received);
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
            println!("Got: {}", received);
        }
        println!("finished");
        let _ = child.join();

    }
}
