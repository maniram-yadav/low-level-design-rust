use std::collections::HashMap;
use crate::models::Store;
use crate::models::Base;
use crate::models::PizzaBuilder;
use crate::models::Drink;
use crate::models::Deal;

impl PizzaBuilder {

    pub fn new() -> Self {
    
        Self {
            name:None,
            base:Base,
            toppings:Vec::new(),
        }
    
    }

    pub fn name(mut self,name : String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn price(mut self, base : Base) -> Self {
        self.base = base;
        self
    }

    pub fn add_topping(mut self, topping : Topping) -> Self {
        self.toppings.push(topping);
        self
    }

    pub fn build(self) -> Result<Pizza,String>{
        let name = self.name.ok_or("Pizza must have name");
        let base = self.base.ok_or("Pizza must have price");
        
        Ok(Pizza::new(name,base,self.toppings))
    }
}

