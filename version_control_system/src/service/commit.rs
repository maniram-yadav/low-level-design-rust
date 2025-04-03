use super::Commit;
use super::FileSnapshot;
use std::collections::HashMap;
use chrono::{DateTime,Utc};

impl Commit {


    pub fn new(parent:Option<Commit>,message:String,files:HashMap<String,FileSnapshot>) -> Self {
        let timestamp = Utc::new();
        let mut hasher = Sha1::new();
        hasher.update(timestamp.to_rfc3339(),as_bytes());
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