mod model;
mod service;

use service::Repository;
use service::Result;
use service::VcsError;
use std::path::{Path};
use std::fs;

fn main() -> Result<()>{
    println!("Version Control System!");
    let temp_dir = tempfile::tempdir()?;
    let repo_path = temp_dir.path();

    let mut repo = Repository::init("my-projeect",repo_path)?;

    let file_path = repo_path.join("file1.txt");
    fs::write(&file_path,b"Hello")?;
    repo.add(&[Path::new("file1.txt")])?;

    repo.commit("Initial commit")?;
    repo.create_branch("feature")?;
    repo.checkout("feature")?;

    fs::write(&file_path,b"Hello\n World")?;
    repo.add(&[Path::new("file1.txt")])?;
    repo.commit("Update greeting")?;

    repo.checkout("master")?;

    match repo.merge("feature") {
        Ok(_) => println!("Merge successfull"),
        Err(VcsError::MergeConflict(conflicts)) => {
            println!("Merge conflicts in : {:?}",conflicts);
        },
        Err(e) => return Err(e),
    }

    Ok(())
}
