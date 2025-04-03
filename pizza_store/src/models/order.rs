use super::Store;
use super::OrderItem;
use super::Deal;

pub struct Order<'a>{
    pub store : &'a Store,
    pub items:Vec<Box<dyn OrderItem>>,
    pub applied_deals :Vec<&'a dyn Deal>,
    pub total_discount:f64,
}