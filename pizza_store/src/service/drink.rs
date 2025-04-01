use std::collections::HashMap;
use crate::models::Store;
use crate::models::Base;
use crate::models::Topping;
use crate::models::Drink;
use crate::models::Deal;

impl Drink {
    pub fn new(name : String, price : f64) -> Self {
        Self { name, price }
    }
}

impl OrderItem for Drink {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn price(&self) -> f64 {
        self.price
    }
}