use serde::{Serialize, Deserialize};


#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash,Serialize,Deserialize)]
pub enum Gender {
    Male,
    Female,
    Other,
    Any,
}