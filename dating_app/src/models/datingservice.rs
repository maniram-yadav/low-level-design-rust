use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use super::User;

#[derive(Debug,Clone)]
pub struct DatingService {
    pub users : HashMap<Uuid,User>,
    pub available_interests : Vec<String>,
}