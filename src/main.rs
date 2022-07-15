use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

use std::thread;
use std::sync::mpsc::{self, channel, Sender, Receiver};

mod scanner;
mod parser;
mod vm;

use scanner::{Scanner, Token};

fn main() -> Result<()> {

    let args: Vec<_> = std::env::args().collect();
    let file_name = args.get(1).unwrap();
    let file_content = std::fs::read_to_string(file_name).unwrap();

    let mut sc = Scanner::new(&file_content);
    let start = std::time::Instant::now();
    while sc.next() != Token::Eof {
        println!("{:?}", sc.next());
    }
    println!("finished in: {:?}", start.elapsed());

    // `()` can be used when n,o completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {

                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
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
