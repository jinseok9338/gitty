use std::error::Error;

use crate::arguments::{
    common_trait::{Default, Run},
    multiselect::MultiSelect,
};

use super::{behavior::UserInput, git_helper::GitHelper};

pub struct GitWork {
    git_helper: GitHelper,
    input: UserInput,
}

impl GitWork {
    pub fn new(input: UserInput) -> Self {
        Self {
            git_helper: GitHelper::new(),
            input,
        }
    }

    pub async fn run(&self) {
        match self.input {
            UserInput::Clone => self.gitty_clone_repo().await.unwrap(),
            UserInput::Sync => self.gitty_sync().unwrap(),
            UserInput::SyncAndDelete => self.gitty_sync_and_delete().unwrap(),
            _ => panic!("Unexpected variant"),
        }
    }

    // gitty up accept url and directory as arguments and clone the repository and all remote branches to local branches
    async fn gitty_clone_repo(&self) -> Result<(), Box<dyn Error>> {
        // wait for remote branches
        let remote_branches = self.git_helper.remote_branches(&url).await;

        let remote_branches = remote_branches.unwrap();

        // spawn multiselect with message choose the branches to pull
        let multiselect = MultiSelect::default(
            "Choose the branches to pull:",
            Some(false),
            Some(remote_branches),
        );
        let selected_branches = multiselect.run().unwrap();
        println!("You chose: {:?} branches", selected_branches);

        let cloned_repo = self.git_helper.clone_repo(&url, &directory);

        let cloned_repo = cloned_repo.unwrap();
        // do a git pull for each branch
        for branch in selected_branches {
            self.git_helper.pull_branch(&cloned_repo, &branch).unwrap();
            println!("Pulling branch: {}", branch);
        }
        Ok(())
    }

    fn gitty_sync(&self) -> Result<(), Box<dyn Error>> {
        todo!("sync the existing project with remote repo")
    }

    fn gitty_sync_and_delete(&self) -> Result<(), Box<dyn Error>> {
        todo!("sync the existing project and delete the unnecessary branches")
    }
}
