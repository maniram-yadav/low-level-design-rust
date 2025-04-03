use super::FileSnapshot;
use sha1::{Sha1,Digest};
use std::path::{PathBuf};

impl FileSnapshot {
    pub fn new(path : PathBuf,content : Vec<u8>) -> Self{
        let mut hasher = Sha1::new();
        hasher.update(&content);
        let hash = format!("{:x}",hasher.finalize());
        Self { path,content,hash }
    }
}