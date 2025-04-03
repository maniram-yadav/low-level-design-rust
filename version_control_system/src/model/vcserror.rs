use thiserror::Error;
use std::io;

#[derive(Error,Debug)]
pub enum VcsError {

    #[error("IO errror : {0}")]
    Io(#[from] io::Error),
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("No changes staged for commit")]
    NoChangesStaged,
    #[error("Branch already exists : {0}")]
    BranchExists(String),
    #[error("Branch not found : {0}")]
    BranchNotFound(String),
    #[error("Cannot merge branch with itself")]
    MergeWithSelf,
    #[error("Commit not found : {0}")]
    CommitNotFound(String),
    #[error("Merge conflic in files : {0:?}")]
    MergeConflict(Vec<String>),
}