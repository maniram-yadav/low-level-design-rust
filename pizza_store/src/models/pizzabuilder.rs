use crate::models::Base;
use crate::models::Topping;

pub struct PizzaBuilder {
    name : Option<String>,
    base : Option<Base>,
    toppings : Option<Topping>,
}
