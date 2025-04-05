use std::io;
pub struct Input ;

impl Input {
    pub fn read() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}