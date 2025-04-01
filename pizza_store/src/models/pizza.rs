
use super::Base;
use super::Topping;

pub struct Pizza {
  pub  name : String ,
  pub  base : Base,
  pub  toppings: Vec<Topping>,
}