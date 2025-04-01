mod service;
mod models;
mod helper;

fn main() {
    println!("Hello, Pizza!");
    let store = service::Store::new(String::from("first"));
}