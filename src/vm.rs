use crate::scanner::Token;
use crate::parser::Parser;

#[derive(Debug, Clone)]
pub struct VM {
    parse_expression: Vec<Token>,
    stack: Vec<u64>,
    result: u64,
}

impl VM {
    pub fn new() -> Self {
        Self {
            parse_expression: Vec::new(),
            stack: Vec::new(),
            result: 0,
        }
    }

    pub fn run(&mut self, input: &str) -> u64 {
        let mut parser = Parser::new(&input);
        self.parse_expression.append(&mut parser.parse());
        for item in &self.parse_expression {
            match item {
                Token::BinaryNumber(x) | Token::DecimalNumber(x) | Token::HexNumber(x) => self.stack.push(*x),
                op if op.is_operator() => {
                    let lhs = self.stack.pop().unwrap();
                    let rhs = self.stack.pop().unwrap();
                    let result = self.clone().apply_operator(op, lhs, rhs);
                    self.stack.push(result);
                },
                err => panic!("err: {:?}", err),

            }
        }
        self.stack.pop().unwrap()
    }

    fn apply_operator(&mut self, operator: &Token, rhs: u64, lhs: u64) -> u64 {
        match operator {
            Token::Plus => lhs + rhs,
            Token::Minus => lhs - rhs,
            Token::And => lhs & rhs,
            Token::Or => lhs | rhs,
            Token::Nor => !(lhs | rhs),
            Token::Xor => lhs ^ rhs,
            Token::ShiftLeft => lhs << rhs,
            Token::ShiftRight => lhs >> rhs,
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
    fn test_unary_expr() {
        let mut vm = VM::new();
        assert_eq!(-1 + 2, vm.run("-1 + 2"));
    }
}
