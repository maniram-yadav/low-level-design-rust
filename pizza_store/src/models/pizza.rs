
use super::Base;
use super::Topping;

pub struct Pizza {
    name : String ,
    base : Base,
    toppings: Vec<Topping>,
}