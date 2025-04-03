use crate::models::Base;
use crate::models::Topping;

pub struct PizzaBuilder {
  pub  name : Option<String>,
  pub  base : Option<Base>,
   pub toppings : Option<Topping>,
}
