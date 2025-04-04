use serde::{Serialize, Deserialize};
use super::Gender;
use std::collections::HashMap;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PartnerPreferences {
    pub min_age: u8,
    pub max_age: u8,
    pub preferred_gender: Gender,
    pub strict_mode: bool,
    pub lenient_prefs: HashMap<String,bool>,
}