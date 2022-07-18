mod scanner;
mod parser;
mod vm;
mod editor;

use vm::VM;
use editor::*;

fn main() {

    let args: Vec<_> = std::env::args().collect();
    if let Some(file_name) = args.get(1) {
        let file_content = std::fs::read_to_string(file_name).unwrap();
    }
    let mut vm = VM::new();

    // `()` can be used when n,o completer is required
    let mut editor = Editor::new();
    loop {
        match editor.read_line() {
            Ok(line) => {
                let result = vm.run(&line);
                println!("{}", result);
            }
            err => panic!("{:?}", err),
        }
    }
}
