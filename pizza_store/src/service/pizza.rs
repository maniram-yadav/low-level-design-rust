use crate::models::OrderItem;
use crate::models::Topping;
use crate::models::Pizza;
use super::Base;

impl Pizza {
    pub fn new(name : String, base : Base,toppings : Vec<Topping>) -> Self {

        Self { 
            name, base, toppings
        }
    }

     fn base(&self) -> &Base {
        &self.base
    }
     fn toppings(&self) -> &[Topping]{
            &self.toppings
    }
}

impl OrderItem for Pizza {
    
     fn name(&self) -> &str {
        &self.name
    }

     fn price(&self) -> f64 {
        self.base.price() + self.toppings.iter().map(|t| t.price()).sum::<f64>()
    }

}