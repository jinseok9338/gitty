use std::{error::Error, path::PathBuf};

use reqwest::Url;

use crate::{
    arguments::{
        common_trait::{Default, Run},
        input::Input,
        multiselect::MultiSelect,
    },
    consts::CHOOSE_BRANCHES,
};

use super::{behavior::UserInput, git_helper::GitHelper};

pub struct GitWork {
    git_helper: GitHelper,
    input: UserInput,
    url: Option<String>,
    directory: Option<PathBuf>,
}

impl GitWork {
    pub fn new(input: UserInput) -> Self {
        Self {
            git_helper: GitHelper::new(),
            input,
            url: None,
            directory: None,
        }
    }

    pub async fn run(&mut self) {
        match self.input {
            UserInput::Clone(_) => self.gitty_clone_repo().await.unwrap(),
            UserInput::Sync(_) => self.gitty_sync().unwrap(),
            UserInput::SyncAndDelete(_) => self.gitty_sync_and_delete().unwrap(),
        }
    }

    // gitty up accept url and directory as arguments and clone the repository and all remote branches to local branches
    async fn gitty_clone_repo(&mut self) -> Result<(), Box<dyn Error>> {
        // get the url

        loop {
            let directory = Input::default("Enter the directory:", None, None)
                .run()
                .unwrap();
            // if the url is valid then break the loop with match
            match PathBuf::from(&directory).exists() {
                true => {
                    self.directory = Some(PathBuf::from(&directory));
                    break;
                }
                false => continue,
            }
        }

        loop {
            let url = Input::default("Enter the url of the repository:", None, None)
                .run()
                .unwrap();
            // if the url is valid then break the loop
            match Url::parse(&url) {
                Ok(_) => {
                    self.url = Some(url);
                    break;
                }
                Err(_) => continue,
            }
        }

        // wait for remote branches
        let remote_branches = self
            .git_helper
            .remote_branches(&self.url.clone().unwrap())
            .await;

        let remote_branches = remote_branches.unwrap();

        // spawn multiselect with message choose the branches to pull
        let multiselect = MultiSelect::default(CHOOSE_BRANCHES, Some(false), Some(remote_branches));
        let selected_branches = multiselect.run().unwrap();
        println!("You chose: {:?} branches", selected_branches);

        let cloned_repo = self
            .git_helper
            .clone_repo(&self.url.clone().unwrap(), &self.directory.clone().unwrap());

        let cloned_repo = cloned_repo.unwrap();
        // do a git pull for each branch
        for branch in selected_branches {
            self.git_helper.pull_branch(&cloned_repo, &branch).unwrap();
            println!("Pulling branch: {:?}", branch);
        }
        Ok(())
    }

    fn gitty_sync(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let directory = Input::default("Enter the directory:", None, None)
                .run()
                .unwrap();
            // if the url is valid then break the loop with match
            match PathBuf::from(&directory).exists() {
                true => {
                    self.directory = Some(PathBuf::from(&directory));
                    break;
                }
                false => continue,
            }
        }
        // with the directory get the repository
        let repo = self
            .git_helper
            .repo(&self.directory.clone().unwrap())
            .unwrap();
        // get the remote branches
        let remote_branches = self.git_helper.remote(&repo).unwrap();
        // get branch lists from the remote
        let remote_branches = self
            .git_helper
            .list_remote_branches(&remote_branches)
            .unwrap();

        // spawn multiselect with message choose the branches to pull
        let multiselect = MultiSelect::default(CHOOSE_BRANCHES, Some(false), Some(remote_branches))
            .run()
            .unwrap();
        // do a git pull for each branch
        for branch in multiselect {
            self.git_helper.pull_branch(&repo, &branch).unwrap();
            println!("Pulling branch: {:?}", branch);
        }

        Ok(())
    }

    fn gitty_sync_and_delete(&self) -> Result<(), Box<dyn Error>> {
        todo!("sync the existing project and delete the unnecessary branches")
    }
}
