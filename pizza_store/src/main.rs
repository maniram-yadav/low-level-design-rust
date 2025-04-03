mod service;
mod models;
mod helper;
use service::Store;
use service::Base;
use service::Topping;
use service::Pizza;

use service::BuyOneGetOnefree;
use service::FreeDrinkWithPizza;
use service::MostExpensive;


fn main() {
    println!("Hello, Pizza!");
    let mut store = service::Store::new(String::from("Luigi Pizza"));
    store.add_base("Regular Crust".to_string(),5.8);
    store.add_base("Thin Crust".to_string(),4.8);
    store.add_base("Deep Crust".to_string(),7.99);
    
    store.add_topping("Cheese".to_string(),1.50);
    store.add_topping("Pepperoni".to_string(), 2.00);
    store.add_topping("Mushrooms".to_string(), 1.00);
    store.add_topping("Onions".to_string(), 0.75);
    store.add_topping("Sausage".to_string(), 2.25);

    store.add_drink("Soda".to_string(),1.99);
    store.add_drink("Beer".to_string(), 3.99);
    store.add_drink("Water".to_string(), 0.99);

    store.add_deal(Box::new(BuyOneGetOnefree));
    store.add_deal(Box::new(FreeDrinkWithPizza));
    store.add_deal(Box::new(MostExpensive));

    let mut order = Order::new(&store);

    
}