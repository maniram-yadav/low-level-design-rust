use crate::models::Pizza;
use crate::models::OrderItem;
use crate::models::Deal;
use crate::helper::asany::AsAny;

pub struct BuyOneGetOnefree;

impl Deal for BuyOneGetOnefree {
    
    fn apply_discount(&self,items: &[Box<dyn OrderItem>]) -> f64 {
        
        let pizzas : Vec<&Pizza> = items.iter()
            .filter_map(|item| item.as_any().downcast_ref::<Pizza>())
            .collect();

        if pizzas.len() < 2 {
            return 0.0;
        }

        pizzas.iter()
            .map(|p| p.price())
            .min_by(|a,b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)

    }

     fn description(&self) -> &'static str {
        "Buy One get One Free (cheapest free)"
    }
}