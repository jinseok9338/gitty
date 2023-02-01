use std::{error::Error, path::PathBuf};

use reqwest::Url;

use crate::{
    arguments::{
        common_trait::{Default, Run},
        input::Input,
        multiselect::MultiSelect,
    },
    consts::{CHOOSE_BRANCHES, CHOOSE_DELETE_BRANCHES, DEFAULT_BRANCH},
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
            UserInput::Purge(_) => self.purge_branches().unwrap(),
        }
    }

    async fn gitty_clone_repo(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let directory = Input::default("Enter the directory:", None, None)
                .run()
                .unwrap();

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

            match Url::parse(&url) {
                Ok(_) => {
                    self.url = Some(url);
                    break;
                }
                Err(_) => continue,
            }
        }

        let remote_branches = self
            .git_helper
            .remote_branches(&self.url.clone().unwrap())
            .await;

        let remote_branches = remote_branches.unwrap();

        let multiselect = MultiSelect::default(
            &(CHOOSE_BRANCHES.to_owned() + DEFAULT_BRANCH),
            Some(false),
            Some(remote_branches),
        );
        let selected_branches = multiselect.run().unwrap();
        println!("You chose: {selected_branches:?} branches");

        let cloned_repo = self
            .git_helper
            .clone_repo(&self.url.clone().unwrap(), &self.directory.clone().unwrap());

        let cloned_repo = cloned_repo.unwrap();

        for branch in selected_branches {
            self.git_helper.pull_branch(&cloned_repo, &branch).unwrap();
            println!("Pulling branch: {branch:?}");
        }
        Ok(())
    }

    fn gitty_sync(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let directory = Input::default("Enter the directory:", None, None)
                .run()
                .unwrap();

            match PathBuf::from(&directory).exists() {
                true => {
                    self.directory = Some(PathBuf::from(&directory));
                    break;
                }
                false => continue,
            }
        }

        let repo = self
            .git_helper
            .repo(&self.directory.clone().unwrap())
            .unwrap();

        let remote_branches = self.git_helper.remote(&repo).unwrap();

        let remote_branches = self
            .git_helper
            .list_remote_branches(&remote_branches)
            .unwrap();

        let multiselect = MultiSelect::default(CHOOSE_BRANCHES, Some(false), Some(remote_branches))
            .run()
            .unwrap();

        for branch in multiselect {
            self.git_helper.pull_branch(&repo, &branch).unwrap();
            println!("Pulling branch: {branch:?}");
        }

        Ok(())
    }

    fn purge_branches(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let directory = Input::default("Enter the directory:", None, None)
                .run()
                .unwrap();

            match PathBuf::from(&directory).exists() {
                true => {
                    self.directory = Some(PathBuf::from(&directory));
                    break;
                }
                false => continue,
            }
        }

        let repo = self
            .git_helper
            .repo(&self.directory.clone().unwrap())
            .unwrap();

        let local_branches = self.git_helper.list_local_branches(&repo).unwrap();

        let multiselect =
            MultiSelect::default(CHOOSE_DELETE_BRANCHES, Some(false), Some(local_branches))
                .run()
                .unwrap();

        for branch in multiselect {
            self.git_helper.delete_branch(&repo, &branch).unwrap();
            println!("Deleting branch: {branch:?}");
        }
        Ok(())
    }
}
