

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

    pub fn find_best_profile(&self,user:&User) -> Option<User>{
        
        let potential_matches : Vec<&User> = self.users.values()
                .filter(|u| u.id!=user.id)
                .filter(|u| user.declined_profiles.contains(&u.id))
                .filter(|u| user.matched_profiles.contains(&u.id))
                .collect();

        let (mut preferred_accepted, preferred) :
                (Vec<&User>,Vec<&User>) = potential_matches.
                into_iter().partition(|m| {
                    let is_preferred = self.is_preferred_profile(user,m);
                    let has_accepted = m.accepted_profiles.contains(&user.id);
                    is_preferred && has_accepted
                });

        let preferred_clone =  preferred.clone();
        let mut preferred_only:Vec<&User> = preferred.into_iter()
                .filter(|m| self.is_preferred_profile(user,m))
                .collect();
        let mut unpreferred_only:Vec<&User>  = preferred_clone.into_iter()
                .filter(|m| !self.is_preferred_profile(user,m) && 
                        m.accepted_profiles.contains(&user.id))
                .collect();
        
        // Sort each list by mutual interests                
        preferred_accepted.sort_by(|a,b| self.count_mutual_interests(user,b)
                    .cmp(&self.count_mutual_interests(user,a)));

        preferred_only.sort_by(|a,b| self.count_mutual_interests(user,b)
                    .cmp(&self.count_mutual_interests(user,a)));

        unpreferred_only.sort_by(|a,b| self.count_mutual_interests(user,b)
                    .cmp(&self.count_mutual_interests(user,a)));
        
        // Combine list in priority order                        
        let mut ranked_profile: Vec<&User> = Vec::new();
        ranked_profile.extend(preferred_accepted);
        ranked_profile.extend(preferred_only);
        ranked_profile.extend(unpreferred_only);

        ranked_profile.sort_by(|a,b| {
            if a.is_boost_active()|| b.is_boost_active() {
                b.boost_level.cmp(&a.boost_level)
            } else {
                std::cmp::Ordering::Equal
            }}
        );
        ranked_profile.sort_by(|a,b| a.match_count.cmp(&b.match_count));
        ranked_profile.first().map(|u| (*u).clone())

    }

    pub fn accept_profile(&mut self,user_id : Uuid,profile_to_accept_id : Uuid) -> bool {
        if let Some(user) = self.users.get_mut(&user_id) {
                user.accepted_profiles.insert(profile_to_accept_id);
                if let Some(other_user) = self.users.get(&profile_to_accept_id){
                    
                    // Check if there is a match
                    if other_user.accepted_profiles.contains(&user_id) {
                        if let Some(user) = self.users.get_mut(&user_id){
                            user.matched_profiles.insert(profile_to_accept_id);
                            user.match_count += 1;
                        }
                        if let Some(other_user) = self.users.get_mut(&profile_to_accept_id){
                            other_user.matched_profiles.insert(user_id);
                            other_user.match_count += 1;
                        }
                    }
                    return true;
                }
        }
        return false;            
    }

    pub fn decline_profile(&mut self , user_id : Uuid, profile_to_decline_id:Uuid)  {
        if let Some(user) = self.users.get_mut(&user_id) {
            user.declined_profiles.insert(profile_to_decline_id);
        }
    }

    pub fn get_matches(&self,user_id:&Uuid) -> Vec<User> {
        self.users.get(&user_id)
            .map(|user| {
                user.matched_profiles.iter()
                    .filter_map(|id| self.users.get(id))
                    .cloned()
                    .collect()
            }).unwrap_or_default()
    }

    pub fn apply_boost(&mut self,user_id:Uuid,boost_level : u8) {
        if let Some(user) = self.users.get_mut(&user_id) {
            user.boost_level = boost_level;
            user.boost_expiry = Some(if boost_level == 1 {
                    Utc::now() + chrono::Duration::hours(24)
            } else { 
                    Utc::now() + chrono::Duration::hours(48)
            });
        }
    }

    pub fn super_accept_profile(&mut self,user_id:Uuid,profile_to_super_accept_id : Uuid) -> bool {
        
        if let Some(user) = self.users.get_mut(&user_id) {
            if user.used_super_accept {
                return false;
            }
            user.used_super_accept = true;
            return true;
        }

        false
    }


    pub fn get_total_user_count(&self) -> usize {
        self.users.len()
    }

    pub fn get_matched_user_count(&self) -> usize {
        self.users.values().filter(|u| !u.matched_profiles.is_empty()).count()
    }

    // pub fn get_top_users_by_matches(&self,n : usize) -> usize {

    // }

    pub fn get_user_cohort_by_gender(&self) -> HashMap<Gender,usize> {
        let mut cohort = HashMap::new();
        for user in self.users.values() {
            *cohort.entry(user.gender).or_insert(0) += 1;
        }
        cohort
    }

    // pub fn get_user_cohort_by_age(&self) -> HashMap<Gender,usize> {

    // }

    
}