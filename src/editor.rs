use std::io::{self, Write};

pub struct Editor {}
impl Editor {
    pub fn new() -> Self {
        Self{}
    }

    pub fn read_line(&self) -> io::Result<String> {
        let mut buffer = String::new();
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer)?;
        
        Ok(buffer.trim().to_string())
    }
}
