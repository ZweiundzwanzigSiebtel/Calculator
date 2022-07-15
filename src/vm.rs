//use std::thread;
//use std::sync::mpsc::{self, Sender, Receiver};
//
//use crate::parser::*;
//use crate::scanner::*;
//
//struct VM {}
//impl<'b> VM {
//    fn new() -> Self {
//        Self{}
//    }
//
//    fn interpret(&self, tx: Sender<Instruction>, rx: Receiver<Instruction>) {
//        let mut p = Parser::new("");
//        p.parse("1 + 1", tx);
//        let instruction = rx.recv().unwrap();
//
//        match instruction.op.typ {
//            TokenType.:
//            _ => panic!()
//        }
//    }
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_vm() {
//        let (tx, rx) = mpsc::channel();
//        let vm = VM::new();
//
//        let child = thread::spawn(move || {
//            vm.interpret(tx, rx);
//        });
//        let _ = child.join();
//
//    }
//
//
//}
