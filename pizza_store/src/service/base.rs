use crate::models::Base;

impl Base {
    pub fn new(name : String,price : f64) -> Self {
            Self {
                name, price
            }
    }
}


impl OrderItem for Base {
    pub  fn name(&self) -> &str {
        &self.name
    }

    pub fn price(&self) -> f64 {
        self.price
    }
    
}
