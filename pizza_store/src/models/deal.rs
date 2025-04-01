use  super::OrderItem;

pub trait Deal {
    fn apply_discount(&self,items:&[Box<dyn OrderItem>]) -> f64;
    fn description(&self) -> &str;
}