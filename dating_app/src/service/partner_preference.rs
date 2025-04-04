use super::PartnerPreferences;
use super::Gender;
use std::collections::HashMap;

impl PartnerPreferences {
    pub fn new(min_age: u8, max_age: u8, preferred_gender: Gender, strict_mode: bool)-> Self{

        Self {
            min_age,
            max_age,
            preferred_gender,
            strict_mode,
            lenient_prefs : HashMap::new(),
        }
    }
}