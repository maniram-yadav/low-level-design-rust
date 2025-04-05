mod service;
mod models;
mod input ;
use input::Input;

fn main() {
    println!("Hello, Dating App!");
    let data = Input::read();    
    
}