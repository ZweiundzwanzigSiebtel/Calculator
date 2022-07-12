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

    fn parse(&mut self, min_bp: Option<u8>) -> S {
        let min_bp = min_bp.unwrap_or(0);

        let mut lhs = match self.scanner.next() {
            tok => S::Atom(tok),
            rest => panic!("bad token {:?}", rest),
        };//must contain a number or a open paren

        loop {
            let op = match self.scanner.next() { //after a number must follow some sort of operator, but just look at it here.
                eof if eof.typ == TokenType::Or => {break},
                operator => operator,
                rest => panic!("bad token {:?}", rest),

            };
            //now compute the binding power of the just fetched operator
            let (l_bp, r_bp) = self.infix_binding_power(&op);

            //stop eating more tokens, if the left bp is lower than the min_bp
            if l_bp < min_bp {
                break;
            }

            //self.scanner.next(); //eat the previous looked at operator

            let rhs = self.parse(Some(r_bp));

            lhs = S::Cons(op, vec![lhs, rhs]);
            println!("lhs {:?}", &lhs);
        }
        lhs
    }

    fn infix_binding_power(&self, op: &Token) -> (u8, u8) {
        match &op.typ {
            TokenType::Plus | TokenType::Minus => (11, 12),
            TokenType::ShiftRight | TokenType::ShiftLeft => (9, 10),
            TokenType::And => (7, 8),
            TokenType::Xor => (5, 6),
            TokenType::Nor => (3, 4),
            TokenType::Or => (1, 2),
            rest => panic!("bad operand: {:?}", rest),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_simple() {
        let mut p = Parser::new("1 + 1");
        println!("{:?}", p.parse(None));
    }

    #[test]
    fn test_more() {
        let mut p = Parser::new("1 << 2 + 3");
        println!("{:?}", p.parse(None));
    }
}
