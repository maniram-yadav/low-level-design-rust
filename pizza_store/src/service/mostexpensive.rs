use crate::models::Pizza;
use crate::models::OrderItem;
use crate::models::Deal;

pub struct BuyOneGetOnefree;

impl Deal for BuyOneGetOnefree {
    
    fn apply_discount(&self,items: &[Box<dyn OrderItem>]) -> f64 {
        
        return 0.0;
    }

     fn description(&self) -> &'static str {
         "Most Expensive Topping Free on each Pizza"
    }
}