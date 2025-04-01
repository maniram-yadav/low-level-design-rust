use std::collections::HashMap;
use crate::models::Store;
use crate::models::Base;
use crate::models::Topping;
use crate::models::Drink;
use crate::models::Deal;

impl Pizza {
    pub fn new(name : String, base Base,toppings : Vec<Topping>) -> Self {

        Self { 
            name, base, topping
        }
    }

    pub fn base(&self) -> &Base {
        &self.base
    }
}