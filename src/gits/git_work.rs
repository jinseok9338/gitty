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
    pub const fn new(input: UserInput) -> Self {
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
            UserInput::Sync(_) => self.gitty_sync(),
            UserInput::Purge(_) => self.purge_branches(),
        }
    }

    async fn gitty_clone_repo(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let directory = Input::default("Enter the directory:", None, None)
                .run()
                .unwrap();

            if PathBuf::from(&directory).exists() {
                self.directory = Some(PathBuf::from(&directory));
                break;
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

        let remote_branches = match remote_branches {
            Ok(branches) => branches,
            Err(err) => {
                panic!("{}", err)
            }
        };

        let multiselect = MultiSelect::default(
            &(CHOOSE_BRANCHES.to_owned() + DEFAULT_BRANCH),
            Some(false),
            Some(remote_branches),
        );
        let selected_branches = multiselect.run().unwrap();
        println!("You chose: {selected_branches:?} branches");

        let cloned_repo =
            GitHelper::clone_repo(&self.url.clone().unwrap(), &self.directory.clone().unwrap());

        //
        let cloned_repo = match cloned_repo {
            Ok(repo) => repo,
            // have different error behavior for different errors
            Err(err) => {
                match err.code() {
                    git2::ErrorCode::GenericError => panic!("This is generic Error"),
                    git2::ErrorCode::NotFound =>  panic!("The cloned repo not found"),
                    git2::ErrorCode::Exists => panic!("The repo already exists and is not an empty directory choose different directory to clone your project"),
                    _ => panic!("Unexpected error: {err:?}"),
                }
            },
        };

        for branch in selected_branches {
            match GitHelper::pull_branch(&cloned_repo, &branch) {
                Ok(_) => println!("Pulling branch: {branch:?}"),
                Err(err) => panic!("Unable to pull branch: {err:?}"),
            };
        }
        Ok(())
    }

    fn gitty_sync(&mut self) {
        loop {
            let directory = Input::default("Enter the directory:", None, None)
                .run()
                .unwrap();

            if PathBuf::from(&directory).exists() {
                self.directory = Some(PathBuf::from(&directory));
                break;
            }
        }

        let repo = GitHelper::repo(&self.directory.clone().unwrap()).unwrap();

        let remote_branches = GitHelper::remote(&repo);

        let remote_branches = GitHelper::list_remote_branches(&remote_branches).unwrap();

        let multiselect = MultiSelect::default(CHOOSE_BRANCHES, Some(false), Some(remote_branches))
            .run()
            .unwrap();

        for branch in multiselect {
            GitHelper::pull_branch(&repo, &branch).unwrap();
            println!("Pulling branch: {branch:?}");
        }
    }

    fn purge_branches(&mut self) {
        loop {
            let directory = Input::default("Enter the directory:", None, None)
                .run()
                .unwrap();

            if PathBuf::from(&directory).exists() {
                self.directory = Some(PathBuf::from(&directory));
                break;
            }
        }

        let repo = GitHelper::repo(&self.directory.clone().unwrap()).unwrap();

        let local_branches = GitHelper::list_local_branches(&repo).unwrap();

        let multiselect =
            MultiSelect::default(CHOOSE_DELETE_BRANCHES, Some(false), Some(local_branches))
                .run()
                .unwrap();

        for branch in multiselect {
            GitHelper::delete_branch(&repo, &branch).unwrap();
            println!("Deleting branch: {branch:?}");
        }
    }
}
