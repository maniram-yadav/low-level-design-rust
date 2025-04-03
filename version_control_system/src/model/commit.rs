use std::path::{Path,PathBuf};
use chrono::{DateTime,Utc};
use super::FileSnapshot;
use std::collections::{HashMap};

#[derive(Debug,Clone)]
pub struct Commit {
    pub id : String,
    pub message : String,
    pub timestamp : DateTime<Utc>,
    pub parent : Option<Box<Commit>>,
    pub files : HashMap<PathBuf,FileSnapshot>,
}