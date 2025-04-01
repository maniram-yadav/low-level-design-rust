use std::collections::HashMap;
mod super::models::Base;
mod super::models::Topping;
mod super::models::Drink;
mod super::models::Deal;


pub struct Store {
    name : String,
    base_prices : HashMap<String,Base>,
    topping_prices : HashMap<String,Topping>,
    drink_prices : HashMap<String,Drink>,
    available_deals : HashMap<String,Deal>,
}

impl Store {
    pub fn new(name : String) -> Self {
        Self {
            name m,
            base_prices : HashMap::new(),
            topping_prices : HashMap::new(),
            drink_prices : HashMap::new(),
            available_deals : HashMap::new(),
        }
    }
}