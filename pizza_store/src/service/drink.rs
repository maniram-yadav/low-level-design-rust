use crate::models::Drink;
use crate::models::OrderItem;

impl Drink {
    pub fn new(name : String, price : f64) -> Self {
        Self { name, price }
    }
}

impl OrderItem for Drink {
     fn name(&self) -> &str {
        &self.name
    }

     fn price(&self) -> f64 {
        self.price
    }
}