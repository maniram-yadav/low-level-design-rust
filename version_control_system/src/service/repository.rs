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

    pub fn commit(&mut self,message : &str ) -> Result<&Commit> {

        if self.staging_area.is_empty(){
            return Err(VcsError::NoChangesStaged);
        }

        let mut files = HashMap::new();
        
        for path in &self.staging_area {
            let abs_path = self.root_path.join(path);
            let content = fs::read(&abs_path)?;
            files.insert(path.clone(),FileSnapshot::new(path.clone(),content));
        }

        let parent = self.branches[&self.head].latest_commit().cloned();
        let commit = Commit::new(parent,message.to_string(),files);
        // let current_branch =  self.branches.get_mut(&self.head).unwrap();
        // current_branch.add_commit(commit);
        
        self.branches.get_mut(&self.head).unwrap().add_commit(commit);
        
        self.staging_area.clear();
        let latest_commit = self.branches[&self.head].latest_commit().unwrap();
        self.persist_commit(latest_commit)?;

        Ok(self.branches[&self.head].latest_commit().unwrap())
    }

    fn persist_commit(&self,commit :&Commit) -> Result<()> {
        let commit_path = self.root_path.join(".vcs/objects").join(&commit.id);
        fs::write(commit_path,format!("Commit {}\nMessage : {}",commit.id,commit.message))?;
        Ok(())
    }
    pub fn checkout(&mut self,reference : &str) -> Result<()> {

        if self.branches.contains_key(reference) {
                self.head = reference.to_string();
                if let Some(commit) = self.branches[reference].latest_commit(){
                    return self.restore_files(commit);
                }
                return Ok(());
        }

        let commit = self.find_commit(reference)?;
        return self.restore_files(commit);
        
    }

    fn restore_files(&self,commit : &Commit) -> Result<()>{

            for entry in fs::read_dir(&self.root_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.file_name().unwrap() == ".vcs" {
                    continue
                }
                if path.is_dir() {
                    fs::remove_dir_all(&path)?;
                } else {
                    fs::remove_file(&path)?;
                }
            }

        for (path,file) in &commit.files {

            let abs_path = self.root_path.join(path);
            if let Some(parent) = abs_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            fs::write(abs_path,&file.content)?;
        }
        Ok(())
    }

    fn find_commit(&self,partial_hash:&str) -> Result<&Commit> {

        for branch in self.branches.values() {
            for commit in &branch.commits {
                if commit.id.starts_with(partial_hash) {
                    return Ok(commit);
                }
            }
        }
        return Err(VcsError::CommitNotFound(partial_hash.to_string()));
    }

    pub fn create_branch(&mut self,name : &str) -> Result<()> {
        if self.branches.contains_key(name) {
            return Err(VcsError::BranchExists(name.to_string()));
        }
        let mut new_branch = Branch::new(name);
        if let Some(commit) = self.current_branch().latest_commit().cloned() {
            new_branch.add_commit(commit);
        }
        self.branches.insert(name.to_string(),new_branch);
        Ok(())
    }

    fn current_branch(&mut self) -> &mut Branch {
        self.branches.get_mut(&self.head).unwrap()
    }


}