use crate::models::Topping;
use crate::models::Drink;
use crate::models::Deal;
use crate::models::OrderItem;

impl Topping {
    pub  fn new(name : String, price : f64) -> Self {
        Self  {
                name, price
        }
    }
}


impl OrderItem for Topping {
     fn name(&self) -> &str {
           &self.name 
    }
    
     fn price(&self) -> f64 {
        self.price
 }
}
