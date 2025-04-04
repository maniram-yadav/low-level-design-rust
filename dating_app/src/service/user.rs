
use std::collections::{ HashSet};
use chrono::{ Utc};
use uuid::Uuid;

use super::User;
use super::Gender;

impl User {
    fn new(name: String, age: u8, gender: Gender) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            age,
            gender,
            interests: HashSet::new(),
            partner_prefs: None,
            accepted_profiles: HashSet::new(),
            declined_profiles: HashSet::new(),
            matched_profiles: HashSet::new(),
            is_admin: false,
            used_super_accept: false,
            boost_level: 0,
            boost_expiry: None,
            match_count: 0,
        }
    }

    fn is_boost_active(&self) -> bool {
        self.boost_expiry.map_or(false,|expiry| expiry > Utc::now())
    }
}