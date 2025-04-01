mod service;
mod models;

fn main() {

    println!("Hello, Pizza!");
    let store = service::Store::new("first");
}