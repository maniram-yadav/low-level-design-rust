use super::Base;
use super::PizzaBuilder;
use super::Pizza;
use super::Topping;


impl PizzaBuilder {

    pub fn new() -> Self {
    
        Self {
            name:None,
            base:None,
            toppings:Vec::new(),
        }
    
    }

    pub fn name(mut self,name : String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn base(mut self, base : Base) -> Self {
        self.base = Some(base) ;
        self
    }

    pub fn add_topping(mut self, topping : Topping) -> Self {
        self.toppings.push(topping);
        self
    }

    pub fn build(self) -> Result<Pizza,String>{
        let name = self.name.ok_or("Pizza must have name")?;
        let base = self.base.ok_or("Pizza must have price")?;
        
        Ok(Pizza::new(name,base,self.toppings))
    }
}

