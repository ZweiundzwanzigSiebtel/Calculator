use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

mod scanner;
mod parser;
mod vm;

use vm::VM;

fn main() -> Result<()> {

    let args: Vec<_> = std::env::args().collect();
    if let Some(file_name) = args.get(1) {
        let file_content = std::fs::read_to_string(file_name).unwrap();
    }
    let mut vm = VM::new();

    // `()` can be used when n,o completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {
                let result = vm.run(&line);
                rl.add_history_entry(line.as_str());
                println!("{}", result);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt")
}
