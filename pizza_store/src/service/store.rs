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
    pub fn add_base(&mut self,name : String, price : f64)  {
        self.base_prices.insert(name.clone(),Base::new(name,price));
    }

    pub fn add_topping(&mut self,name : String,price :f64){
        self.topping_prices.insert(name.clone(),Topping::new(name,price));
    }
    
    pub fn add_drink(&mut self,name : String,price :f64){
        self.drink_prices.insert(name.clone(),Drink::new(name,price));
    }
    pub fn add_deal(&mut self,deal : Box<dyn Deal>){
        self.available_deals.push(deal);
    }

    pub fn get_base(&self,name :&str) -> Option<Base>{
        self.base_prices.get(name).cloned()
    }
    
    
    pub fn get_topping(&self,name :&str) -> Option<Topping>{
        self.topping_prices.get(name).cloned()
    }

    pub fn get_drink(&self,name :&str) -> Option<Drink>{
        self.drink_prices.get(name).cloned()
    }

    pub fn available_deals(&self) -> &[Box<dyn Deal>]{
        &self.available_deals
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
}