
pub trait OrderItem {
    fn name(&self) -> &str;
    fn price(&self) -> f64;
}