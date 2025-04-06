use std::io::{self, Write}; 
pub struct Input ;

impl Input {
    pub fn read() -> String {
        let mut input = String::new();
        std::io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}