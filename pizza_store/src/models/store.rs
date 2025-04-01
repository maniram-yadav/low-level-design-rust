use std::collections::HashMap;

 use  super::Base;
 use  super::Topping;
 use  super::Drink;
 use  super::Deal;

pub struct Store {
   pub name : String,
   pub base_prices : HashMap<String,Base>,
   pub topping_prices : HashMap<String,Topping>,
   pub drink_prices : HashMap<String,Drink>,
   pub available_deals : Vec<Box<dyn Deal>>,
}
