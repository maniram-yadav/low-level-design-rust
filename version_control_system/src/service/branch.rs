use super::Commit;
use super::Branch;

impl Branch {

    pub fn new(name:&str) -> Self{
        Self {
            name:name.to_string(),
            commits:Vec::new()
        }
    }

    pub fn latest_commit(&self) -> Option<&Commit>{
        self.commits.last()
    }

    pub fn add_commit(&mut self,commit :Commit) {
        self.commits.push(commit);
    }

}