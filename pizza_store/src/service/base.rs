use crate::models::Base;
use crate::models::OrderItem;

impl Base {
     fn new(name : String,price : f64) -> Self {
            Self {
                name, price
            }
    }
}


impl OrderItem for Base {
      fn name(&self) -> &str {
        &self.name
    }

     fn price(&self) -> f64 {
        self.price
    }
    
}
