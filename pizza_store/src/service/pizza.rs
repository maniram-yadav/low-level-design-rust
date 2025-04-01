use std::collections::HashMap;
use crate::models::Store;
use crate::models::OrderItem;
use crate::models::Topping;
use crate::models::Base;
use super::Base;

impl Pizza {
    pub fn new(name : String, base : Base,toppings : Vec<Topping>) -> Self {

        Self { 
            name, base, toppings
        }
    }

    pub fn base(&self) -> &Base {
        &self.base
    }
    pub fn toppings(&self) -> &[Topping]{
            &self.toppings
    }
}

impl OrderItem for Pizza {
    
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn price(&self) -> f64 {
        self.base.price() + self.toppings.iter().map(|t| t.price()).sum::<f64>()
    }

}