use crate::models::Store;
use crate::models::OrderItem;
use crate::models::Deal;

use super::Order;
impl<'a> Order<'a> {
    pub fn new(store : &'a Store) -> Self{
        Self {
            store,
            items:Vec::new(),
            applied_deals : Vec::new(),
            total_discount:0.0,
        }
    }

    pub fn add_item(&mut self,item : Box<dyn OrderItem>){
        self.items.push(item);
    }

    pub fn apply_deal(&mut self,deal : &'a dyn Deal) -> Result<(),String>{

        if !self.store.available_deals.iter().any(|d| {
            d.description() == deal.description()
        }) {
            return Err("deal not available to this store".to_string());
        }
        self.applied_deals.push(deal);
        Ok(())
    }

    pub fn calculate_total(&mut self) -> f64 {
        let subtotal = self.items.iter().map(|item| item.price()).sum::<f64>();
        self.total_discount = self.applied_deals.iter()
                .map(|deal| deal.apply_discount(&self.items))
                .sum::<f64>();
        subtotal - self.total_discount
    }

    pub fn items(&self) -> &[Box<dyn OrderItem>] {
        &self.items
    }

    pub fn applied_deals(&self) -> &[&dyn Deal] {
            &self.applied_deals
    }

    pub fn total_apply_discount(&self) -> f64 {
        self.total_discount
    }
    pub fn store(&self) -> &Store {
        self.store
    }
    
}
