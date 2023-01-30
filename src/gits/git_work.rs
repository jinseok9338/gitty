use std::{error::Error, path::PathBuf};

use crate::{
    arguments::{
        common_trait::{Default, Run},
        multiselect::MultiSelect,
    },
    logs::loading::Loading,
};

use super::git_helper::GitHelper;

pub struct GitWork {
    git_helper: GitHelper,
}

impl GitWork {
    pub fn new() -> Self {
        Self {
            git_helper: GitHelper::new(),
        }
    }

    // gitty up accept url and directory as arguments and clone the repository and all remote branches to local branches
    pub async fn gitty_clone_repo(
        &self,
        url: &str,
        directory: &PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let spinner = Loading::new("Waiting for remote branches".to_string());
        // wait for remote branches
        let remote_branches = self.git_helper.remote_branches(&url).await;

        loop {
            match remote_branches {
                Ok(_) => break,
                Err(_) => {
                    print!("\r{} ", spinner.spinner().next().unwrap());
                    std::thread::sleep(spinner.spinner_interval);
                }
            }
        }
        let remote_branches = remote_branches.unwrap();

        // spawn multiselect with message choose the branches to pull
        let multiselect = MultiSelect::default(
            "Choose the branches to pull:",
            Some(false),
            Some(remote_branches),
        );
        let selected_branches = multiselect.run().unwrap();
        println!("You chose: {:?} branches", selected_branches);

        let cloned_repo = self.git_helper.clone_repo(&url, &directory).unwrap();
        println!("cloned_repo: {:?}", cloned_repo.path());

        // for each branch in selected_branches, create a local branch and pull the remote branch and sync the branches
        for branch in selected_branches {
            let local_branch = self
                .git_helper
                .create_local_branch(&cloned_repo, &branch)
                .unwrap();
            println!("local_branch: {:?}", local_branch.name().unwrap());

            let remote_branch = self
                .git_helper
                .remote_branch(&cloned_repo, &branch)
                .unwrap();
            println!("remote_branch: {:?}", remote_branch.name().unwrap());

            let sync_branch = self
                .git_helper
                .sync_branch(&cloned_repo, &local_branch, &remote_branch)
                .unwrap();
            println!("sync_branch: {:?}", sync_branch.name().unwrap());
        }

        Ok(())
    }
}
