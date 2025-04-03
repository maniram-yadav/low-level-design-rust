mod service;
mod models;
mod helper;
use service::Store;
use service::Base;
use service::Topping;
use service::Pizza;
use service::Order;
use service::PizzaBuilder;

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

    let pizza1 = PizzaBuilder::new()
            .name("Meat Lovers".to_string())
            .base(store.get_base("Regular Crust").expect("Drink not found"))
            .add_topping(store.get_topping("Cheese").expect("Drink not found"))            
            .add_topping(store.get_topping("Pepperoni").expect("Drink not found"))
            .add_topping(store.get_topping("Sausage").expect("Drink not found"))
            .build()
            .unwrap();

        let pizza2 = PizzaBuilder::new()
            .name("Veggie".to_string())
            .base(store.get_base("Thin Crust").expect("Drink not found"))
            .add_topping(store.get_topping("Cheese").expect("Drink not found"))
            .add_topping(store.get_topping("Mushrooms").expect("Drink not found"))
            .add_topping(store.get_topping("Onions").expect("Drink not found"))
            .build()
            .unwrap();

        order.add_item(Box::new(pizza1));
        order.add_item(Box::new(pizza2));
        order.add_item(Box::new(store.get_drink("Soda").expect("Drink not found")));
        order.add_item(Box::new(store.get_drink("Beer").expect("Drink not found")));

        order.apply_deal(&BuyOneGetOnefree).unwrap();
        order.apply_deal(&MostExpensive).unwrap();
        order.apply_deal(&FreeDrinkWithPizza).unwrap();

        let total = order.calculate_total();
        println!("Total {}",total)
}