use std::fmt;
use crate::scanner::{Scanner, Token, TokenType};

struct Parser<'a> {
    buffer: String,
    scanner: Scanner<'a>,
}

#[derive(Debug)]
enum S {
    Atom(Token),
    Cons(Token, Vec<S>),
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            buffer: input.to_string(),
            scanner: Scanner::new(&input),
        }
    }

    pub fn parse(&mut self) -> S {
        self.parser_worker(0)
    }

    fn parser_worker(&mut self, min_bp: u8) -> S {
        let next = self.scanner.next();
        let mut lhs = match next.typ {
            //TODO must contain a number or an open paren
            TokenType::DecimalNumber | TokenType::BinaryNumber | TokenType::HexNumber => S::Atom(next),
            TokenType::LeftParen => {
                let lhs = self.parser_worker(0);
                assert_eq!(self.scanner.next().typ, TokenType::RightParen);
                lhs
            },
            TokenType::And | TokenType::Or | TokenType::Xor | TokenType::Nor | TokenType::ShiftLeft | TokenType::ShiftRight => {
                let ((), r_bp) = self.prefix_binding_power(&next);
                let rhs = self.parser_worker(r_bp);
                S::Cons(next, vec![rhs])
            },
            rest => panic!("bad token: {:?}", rest),
        };

        loop {
            let op = match self.scanner.peek() { //after a number must follow some sort of operator, but just look at it here.
                eof if eof.typ == TokenType::Eof => break,
                operator
                    if operator.typ == TokenType::Plus
                        || operator.typ == TokenType::Minus
                        || operator.typ == TokenType::And
                        || operator.typ == TokenType::Or
                        || operator.typ == TokenType::Nor
                        || operator.typ == TokenType::Xor
                        || operator.typ == TokenType::ShiftRight
                        || operator.typ == TokenType::ShiftLeft => {
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
                self.scanner.next(); //eat the previous looked at operator or else implement a

                let rhs = self.parser_worker(r_bp);

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
        todo!()
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
    fn test_simple() {
        let mut p = Parser::new("1 + 1");
        println!("{:?}", p.parse());
    }

    #[test]
    fn test_more() {
        let mut p = Parser::new("1 << 2 + 3");
        println!("{}", p.parse().to_string());
    }
}
