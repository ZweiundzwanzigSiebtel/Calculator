mod scanner;
mod parser;
mod vm;
mod editor;

use vm::VM;
use editor::*;

fn main() {
    let mut vm = VM::new();
    let args: Vec<_> = std::env::args().collect();
    if let Some(file_name) = args.get(1) {
        let file_content = std::fs::read_to_string(file_name).unwrap();
        let result = vm.run(&file_content);
        println!("result: {:?}", result);
    } else {
        let editor = Editor::new();
        loop {
            match editor.read_line() {
                Ok(line) if line == "exit" || line == "quit" => {
                    break;
                }
                Ok(line) if line == "copy bin" => {
                    //TODO vm.result.as_bin() to clipboard;
                }
                Ok(line) if line == "copy dec" => {
                    //TODO vm.result.as_dec() to clipboard;
                }
                Ok(line) if line == "copy hex" => {
                    //TODO vm.result.as_hex() to clipboard;
                }
                Ok(line) => {
                    let result = vm.run(&line);
                    println!("{}", &result);
                    println!("0x{:x}", &result);
                    println!("0b{:b}", &result);
                }
                err => panic!("{:?}", err),
            }
        }
    }
}
