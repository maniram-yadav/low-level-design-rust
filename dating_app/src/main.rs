mod service;
mod models;
mod input ;
mod login;
mod cli;
mod menu;
mod profile;
mod testdata;

use input::Input;
use cli::Cli;
use testdata::TestData;
use service::DatingService;

fn main() {
    println!("Hello, Dating App!");

    let mut service = DatingService::new();
    TestData::initialize_test_data(&mut service);
    Cli::run_cli(&mut service);
        
}