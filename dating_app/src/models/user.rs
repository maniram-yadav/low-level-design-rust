use std::collections::{ HashSet};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use super::Gender;
use super::PartnerPreferences;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub age: u8,
    pub gender: Gender,
    pub interests: HashSet<String>,
    pub partner_prefs: Option<PartnerPreferences>,
    pub accepted_profiles: HashSet<Uuid>,
    pub declined_profiles: HashSet<Uuid>,
    pub matched_profiles: HashSet<Uuid>,
    pub is_admin: bool,
    pub used_super_accept: bool,
    pub boost_level: u8,
    pub boost_expiry: Option<DateTime<Utc>>,
    pub match_count: u32,
}