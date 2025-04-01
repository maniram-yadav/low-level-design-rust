mod base;
mod topping;
pub mod use base::Base;
pub mod use topping::Topping;

pub struct Pizza {
    name : String ,
    base : Base;
    toppings: Vec<Topping>,
}