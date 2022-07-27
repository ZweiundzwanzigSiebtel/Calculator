use crate::scanner::Token;
use crate::parser::Parser;

#[derive(Debug, Clone)]
pub struct VM {
    parse_expression: Vec<Token>,
    stack: Vec<i64>,
    result: i64,
    previous_result: Option<i64>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            parse_expression: Vec::new(),
            stack: Vec::new(),
            result: 0,
            previous_result: None,
        }
    }

    pub fn run(&mut self, input: &str) -> i64 {
        let mut parser = Parser::new(&input);
        self.parse_expression.append(&mut parser.parse());
        for item in &self.parse_expression {
            match item {
                Token::BinaryNumber(x) | Token::DecimalNumber(x) | Token::HexNumber(x) => self.stack.push(*x),
                Token::PreviousResult => self.stack.push(self.previous_result.expect("previous result")),
                op if op.is_operator() => {
                    let result;
                    if op == &Token::Bang || op == &Token::TwosComplement {
                        let val = self.stack.pop().unwrap();
                        result = self.clone().apply_operator(op, val, 0);
                    } else {
                        let lhs = self.stack.pop().unwrap();
                        let rhs = self.stack.pop().unwrap();
                        result = self.clone().apply_operator(op, lhs, rhs);
                    }
                    self.stack.push(result);
                },
                err => panic!("err: {:?}", err),

            }
        }
        self.result = self.stack.pop().unwrap();
        self.previous_result = Some(self.result);
        self.result
    }

    fn apply_operator(&mut self, operator: &Token, rhs: i64, lhs: i64) -> i64 {
        match operator {
            Token::Plus => lhs + rhs,
            Token::Minus => lhs - rhs,
            Token::And => lhs & rhs,
            Token::Or => lhs | rhs,
            Token::Nor => !(lhs | rhs),
            Token::Xor => lhs ^ rhs,
            Token::ShiftLeft => lhs << rhs,
            Token::ShiftRight => lhs >> rhs,
            Token::Bang => !rhs,
            Token::TwosComplement => (!rhs) + 1,
            Token::Mult => lhs * rhs,
            Token::Modulo => lhs % rhs,
            err => panic!("unexpected operator: {:?}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_1_plus_1() {
        let mut vm = VM::new();
        assert_eq!(1+1, vm.run("1+1"));
    }


    #[test]
    fn test_nested() {
        let mut vm = VM::new();
        assert_eq!((1&(2+3)&(4+5)), vm.run("1 and 2 + 3 and 4 + 5"));
    }

    #[test]
    fn test_extended() {
        let mut vm = VM::new();
        assert_eq!(1+2+3+4+5, vm.run("1 + 2 + 3 + 4 + 5"));
    }

    #[test]
    fn test_expression() {
        let mut vm = VM::new();
        assert_eq!(0b01 << 2, vm.run("0b01 << 2"));
    }

    #[test]
    fn test_expression_2() {
        let mut vm = VM::new();
        assert_eq!(1 + 2 + 3 - 4, vm.run("1+2+3-4"));
    }

    #[test]
    fn test_hex_expr() {
        let mut vm = VM::new();
        assert_eq!(0xff & 0xf1, vm.run("0xff&0xf1"));
    }

    #[test]
    fn test_unary() {
        let mut vm = VM::new();
        assert_eq!(!1_i64, vm.run("!1"));
    }

    #[test]
    fn test_twos_complement() {
        let mut vm = VM::new();
        assert_eq!(!(1_i64)+1, vm.run("~1"));
    }

    #[test]
    fn test_mult() {
        let mut vm = VM::new();
        assert_eq!(5*3, vm.run("5*3"));
    }

    #[test]
    fn test_precedence() {
        let mut vm = VM::new();
        assert_eq!((5&5)+(15&7), vm.run("(5 & 5)+(15 and 7)"));
    }

    #[test]
    fn test_zeros() {
        let mut vm = VM::new();
        println!("vm runs on 0: {:?}", vm.run("0"));
        assert_eq!(0, vm.run("0"));
    }

    #[test]
    fn test_negative() {
        let mut vm = VM::new();
        assert_eq!(1-2, vm.run("1-2"));
        assert_eq!(-1*2, vm.run("-1*2"));
    }
}
