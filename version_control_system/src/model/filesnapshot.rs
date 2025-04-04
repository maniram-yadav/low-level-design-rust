use std::path::{PathBuf};

#[derive(Debug,Clone)]
pub struct FileSnapshot {
    pub path : PathBuf,
    pub content : Vec<u8>,
    pub hash : String,
}