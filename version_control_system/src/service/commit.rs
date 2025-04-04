use super::Commit;
use super::FileSnapshot;
use std::collections::HashMap;
use chrono::{Utc};
use sha1::{Sha1,Digest};
use::std::path::{PathBuf};

impl Commit {


    pub fn new(parent:Option<Commit>,message:String,files:HashMap<PathBuf,FileSnapshot>) -> Self {
        let timestamp = Utc::now();
        let mut hasher = Sha1::new();
        hasher.update(timestamp.to_rfc3339().as_bytes());
        hasher.update(message.as_bytes());

        if let Some(p) = &parent {
            hasher.update(p.id.as_bytes());
        }

        let id = format!("{:x}",hasher.finalize())[..7].to_string();

        Self {
            id,
            message : message.to_string(),
            timestamp,
            parent : parent.map(Box::new),
            files
        }
    }
}