use std::collections::{HashMap,HashSet};
use std::path::PathBuf;
use super::Branch;

pub struct Repository {
    pub name : String,
    pub branches : HashMap<String,Branch>,
    pub head : String,
    pub staging_area : HashSet<PathBuf>,
    pub root_path : PathBuf,
    
}