use crate::models::Pizza;
use crate::models::OrderItem;
use crate::helper::asany::AsAny;
use crate::models::Deal;
pub struct FreeDrinkWithPizza;

impl Deal for FreeDrinkWithPizza {
    
    fn apply_discount(&self,items: &[Box<dyn OrderItem>]) -> f64 {
        
        let has_pizza = items.iter().any(|item| item.as_any().
            downcast_ref::<Pizza>().is_some());

        if !has_pizza {
            return 0.0;
        }    
        return 1.1;

    }

     fn description(&self) -> &'static str {
        "Free Drink with Pizza (cheapest free)"
    }
}