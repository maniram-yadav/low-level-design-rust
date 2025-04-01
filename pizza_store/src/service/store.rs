use std::collections::HashMap;
use crate::models::Store;
mod super::models::Base;
mod super::models::Topping;
mod super::models::Drink;
mod deal::Deal;

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