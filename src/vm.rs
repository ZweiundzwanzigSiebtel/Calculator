use crate::scanner::{Scanner, Token};
use crate::parser::{Parser};

#[derive(Debug, Clone)]
struct VM {
    stack: Vec<Token>,
    pending_operators: Vec<Token>,
    result: u32,
    sp: u32,
    bp: u32,
}

impl VM {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            pending_operators: Vec::new(),
            result: 0,
            sp: 0,
            bp: 0,
        }
    }

    fn run(&mut self, input: &str) {
        let mut parser = Parser::new(&input);
        self.stack.append(&mut parser.parse());
        for item in &self.stack {
            println!("item: {:?}", &item);
        }

        while let Some(item) = self.stack.pop() {
            self.sp += 1;
            self.handle_operator(item);
        }
    }

    fn handle_plus(&mut self) {
    }

    fn handle_minus(&mut self) {
        unimplemented!();
    }

    fn handle_and(&mut self) {
    }

    fn handle_or(&mut self) {
        unimplemented!();
    }

    fn handle_nor(&mut self) {
        unimplemented!();
    }

    fn handle_xor(&mut self) {
        unimplemented!();
    }

    ///Saves the current operator in `self.pending_operators` and calls the function that handles
    ///the given operator.
    fn handle_operator(&mut self, token: Token) {
        self.pending_operators.push(token.clone()); //saves the pending operator
        match token {
            Token::Plus => self.handle_plus(),
            Token::Minus => self.handle_minus(),
            Token::And => self.handle_and(),
            Token::Or => self.handle_or(),
            Token::Nor => self.handle_nor(),
            Token::Xor => self.handle_xor(),
            num if token.is_operand() => {},
            not_expected_token => panic!("should not happen >>> {:?}", not_expected_token),
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stack() {
        let mut vm = VM::new();
        vm.run("1 and 2 + 3 and 4 + 5");
        assert_eq!(vm.result, 1);
    }
}
