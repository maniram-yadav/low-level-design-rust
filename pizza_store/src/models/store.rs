use std::collections::HashMap;


pub struct Store {
    name : String,
    base_prices : HashMap<String,Base>,
    topping_prices : HashMap<String,Topping>,
    drink_prices : HashMap<String,Drink>,
    available_deals : HashMap<String,Deal>,
}
