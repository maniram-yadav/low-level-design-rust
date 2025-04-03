use super::Repository;
use super::Commit;
use super::FileSnapshot;
use super::Branch;
use super::VcsError;
use std::path::{PathBuf,Path};
use std::collections::{HashMap,HashSet};
use std::fs;
use std::io;
use super::Result;

impl Repository {

    pub fn init(name : &str,path : &Path) -> Result<Self> {
            
        let root_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        let vcs_dir = root_path.join(".vcs");
        fs::create_dir_all(vcs_dir.join("objects"))?;
        fs::create_dir_all(vcs_dir.join("refs"))?;

        let mut branches = HashMap::new();
        let master = Branch::new("master");
        branches.insert("master".to_string(),master);

        Ok( Self {
            name:name.to_string(),
            branches,
            head : "master".to_string(),
            staging_area : HashSet::new(),
            root_path
        })
    }

    pub fn add(&mut self,paths : &[&Path]) -> Result<()> {
        for path in paths {
            let abs_path = self.root_path.join(path);
            if !abs_path.exists() {
                return Err(VcsError::FileNotFound(path.to_string_lossy().to_string()));
            }
            self.staging_area.insert(path.to_path_buf());
            
        }
        Ok(())
    }
}