

use std::collections::{HashMap, HashSet};
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::Gender;
use super::PartnerPreferences;
use super::DatingService;
use super::User;

impl DatingService {
 
    pub fn new() -> Self {
        Self {
            users : HashMap::new(),
            available_interests : vec![
                "Pets".to_string(),
                "Football".to_string(),
                "Movies".to_string(),
                "Books".to_string(),
                "Hiking".to_string(),
                "Cooking".to_string(),
                "Travel".to_string(),
                "Photography".to_string(),
                "Music".to_string(),
                "Gaming".to_string(),
            ],
        }
    }

    pub fn add_user(&mut self,user : User) {
        self.users.insert(user.id,user);
    }
    pub fn get_user(&self,user_id:Uuid) -> Option<&User> {
        self.users.get(&user_id)
    }
    
    pub fn get_user_mut(&mut self,user_id:Uuid) -> Option<&mut User> {
        self.users.get_mut(&user_id)
    }

    pub fn count_mutual_interests(&self,user1:&User,user2:&User) -> usize {
        user1.interests.intersection(&user2.interests).count()
    }

    pub fn is_preferred_profile(&self,user : &User,potential_match:&User) -> bool {
            if let Some(prefs) = &user.partner_prefs {
                if  potential_match.age < prefs.min_age || potential_match.age > prefs.max_age {
                    return false;
                }
                if potential_match.gender !=Gender::Any &&
                     potential_match.gender != prefs.preferred_gender {
                    return false;
                }
                return true;
            } else {
                true
            }
    }


}