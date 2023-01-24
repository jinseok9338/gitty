use std::{error::Error, path::PathBuf};

use super::git_helper::GitHelper;

struct GitWork {
    git_helper: GitHelper,
}

impl GitWork {
    pub fn new() -> Self {
        Self {
            git_helper: GitHelper::new(),
        }
    }

    // gitty up accept url and directory as arguments and clone the repository and all remote branches to local branches
    pub fn gitty_up(&self, url: &str, directory: &PathBuf) -> Result<(), Box<dyn Error>> {
        //check if the folder is empty
        Ok(())
    }
}
