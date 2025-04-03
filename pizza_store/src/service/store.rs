use std::collections::HashMap;
use crate::models::Store;
use super::Base;
use crate::models::Topping;
use crate::models::Drink;
use crate::models::Deal;

 impl Store {
    pub fn new(name : String) -> Self {
        Self {
            name ,
            base_prices : HashMap::<String,Base>::new(),
            topping_prices : HashMap::<String,Topping>::new(),
            drink_prices : HashMap::<String,Drink>::new(),
            available_deals : Vec::new(),
        }
    }
}