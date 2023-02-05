use std::{error::Error, path::PathBuf};

use git2::Direction;
use reqwest::Url;

use crate::{
    arguments::{
        common_trait::{Default, Run},
        confirm::Confirm,
        input::Input,
        multiselect::MultiSelect,
    },
    consts::{CHOOSE_BRANCHES, CHOOSE_DELETE_BRANCHES, DEFAULT_BRANCH},
};

use super::{behavior::UserInput, git_helper::GitHelper};

pub struct GitWork<'a> {
    git_helper: GitHelper,
    input: UserInput<'a>,

    url: Option<String>,
    directory: Option<PathBuf>,
}

impl<'a> GitWork<'a> {
    pub const fn new(input: UserInput<'a>) -> Self {
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

            UserInput::HardSync(_) => self.gitty_sync_hard(),
            UserInput::SoftSync(_) => self.gitty_sync_soft(), // change it to soft Sync
            UserInput::Diff(_) => self.purge_only_difference(),
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

        let cloned_repo = match cloned_repo {
            Ok(repo) => repo,
            // have different error behavior for different errors
            Err(err) => {
                match err.code() {
                    git2::ErrorCode::GenericError => panic!("This is generic Error {err:?}"),
                    git2::ErrorCode::NotFound =>  panic!("The cloned repo not found"),
                    git2::ErrorCode::Exists => panic!("The repo already exists and is not an empty directory choose different directory to clone your project"),
                    git2::ErrorCode::Auth => panic!("Authentication error, need to provide authentication credentials {err:?}"),
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
        println!("🚜 The work is done 🚜");
        Ok(())
    }

    fn gitty_sync_hard(&mut self) {
        loop {
            let directory = Input::default("Enter the directory:", None, None)
                .run()
                .unwrap();

            if PathBuf::from(&directory).exists() {
                self.directory = Some(PathBuf::from(&directory));
                break;
            }
        }

        //Confirm warning message: "This action will fetch and prune all remote branches are you sure you want to continue?"
        // if yes then continue else return

        let confirm = Confirm::default("This action will fetch and prune all remote branches are you sure you want to continue?", None, None);

        match confirm.run() {
            Ok(true) => (),
            Ok(false) => return,
            Err(err) => panic!("Unable to confirm: {err:?}"),
        };

        let repo = GitHelper::repo(&self.directory.clone().unwrap()).unwrap();

        match GitHelper::fetch_all_and_prune(&repo) {
            Ok(_) => println!("Fetching all and pruning"),
            Err(err) => panic!("Unable to fetch all and prune: {err:?}"),
        }

        let mut remote = GitHelper::remote(&repo);
        match remote.connect(Direction::Fetch) {
            Ok(_) => (),
            Err(err) => panic!("Unable to connect to remote: {err:?}"),
        }

        let remote_branches = match GitHelper::list_remote_branches(&remote) {
            Ok(branches) => branches,
            Err(err) => panic!("Unable to list remote branches: {err:?}"),
        };

        let multiselect =
            match MultiSelect::default(CHOOSE_BRANCHES, Some(false), Some(remote_branches)).run() {
                Ok(branches) => branches,
                Err(err) => panic!("Unable to select branches: {err:?}"),
            };

        let confirm = Confirm::default(
            "This action will delete except the current branch and pull from the fresh branches",
            None,
            None,
        );

        match confirm.run() {
            Ok(true) => (),
            Ok(false) => return,
            Err(err) => panic!("Unable to confirm: {err:?}"),
        };

        let current_branch =
            GitHelper::current_branch(&repo).expect("Unable to get current branch");

        //delete the branches except the current_branch before pulling

        for branch in multiselect
            .clone()
            .into_iter()
            .filter(|b| b != &current_branch)
        {
            match GitHelper::delete_branch(&repo, &branch) {
                Ok(_) => println!("Deleting branch: {branch:?}"),
                Err(err) => panic!("Unable to delete branch: {err:?}"),
            }
        }

        for branch in multiselect {
            match GitHelper::pull_branch(&repo, &branch) {
                Ok(_) => println!("Pulling branch: {branch:?}"),
                Err(err) => panic!("Unable to pull branch: {err:?}"),
            }
        }
        println!("🚜 The work is done 🚜");
    }

    fn gitty_sync_soft(&mut self) {
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

        match GitHelper::fetch_all(&repo) {
            Ok(_) => println!("Fetching all"),
            Err(err) => panic!("Unable to fetch all: {err:?}"),
        }

        let mut remote = GitHelper::remote(&repo);
        match remote.connect(Direction::Fetch) {
            Ok(_) => (),
            Err(err) => panic!("Unable to connect to remote: {err:?}"),
        }

        let remote_branches = match GitHelper::list_remote_branches(&remote) {
            Ok(branches) => branches,
            Err(err) => panic!("Unable to list remote branches: {err:?}"),
        };

        let multiselect =
            match MultiSelect::default(CHOOSE_BRANCHES, Some(false), Some(remote_branches)).run() {
                Ok(branches) => branches,
                Err(err) => panic!("Unable to select branches: {err:?}"),
            };

        let confirm = Confirm::default("This action will fail if the branches that you want to pull from conflicts with the branch. You want to continue?", None, None);

        match confirm.run() {
            Ok(true) => (),
            Ok(false) => return,
            Err(err) => panic!("Unable to confirm: {err:?}"),
        };

        for branch in multiselect {
            match GitHelper::pull_branch(&repo, &branch) {
                Ok(_) => println!("Pulling branch: {branch:?}"),
                Err(err) => panic!("Unable to pull branch: {err:?}"),
            }
        }
        println!("🚜 The work is done 🚜");
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

        let current_branch =
            GitHelper::current_branch(&repo).expect("Unable to get current branch");

        let local_branches: Vec<String> = local_branches
            .into_iter()
            .filter(|b| b != &current_branch)
            .collect();

        let multiselect =
            MultiSelect::default(CHOOSE_DELETE_BRANCHES, Some(false), Some(local_branches))
                .run()
                .unwrap();

        let confirm = Confirm::default(
            "This action will delete the branches you choose. You want to continue?",
            None,
            None,
        );

        match confirm.run() {
            Ok(true) => (),
            Ok(false) => return,
            Err(err) => panic!("Unable to confirm: {err:?}"),
        };

        for branch in multiselect {
            GitHelper::delete_branch(&repo, &branch).unwrap();
            println!("Deleting branch: {branch:?}");
        }
        println!("🚜 The work is done 🚜");
    }

    fn purge_only_difference(&mut self) {
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

        let current_branch =
            GitHelper::current_branch(&repo).expect("Unable to get current branch");

   

        //get the difference between local and remote
        let mut remote = GitHelper::remote(&repo);
        match remote.connect(Direction::Fetch) {
            Ok(_) => (),
            Err(err) => panic!("Unable to connect to remote: {err:?}"),
        }
        // get the list of remote branches
        let remote_branches = match GitHelper::list_remote_branches(&remote) {
            Ok(branches) => branches,
            Err(err) => panic!("Unable to list remote branches: {err:?}"),
        };
        // filter out remote branches from the local branches
        let local_branches: Vec<String> = local_branches
            .into_iter()
            .filter(|b| !remote_branches.contains(b) && b != &current_branch)
            .collect();
        let multiselect =
            match MultiSelect::default(CHOOSE_DELETE_BRANCHES, Some(false), Some(local_branches))
                .run()
            {
                Ok(branches) => {
                    if branches.is_empty() {
                        // exit the program if there is no branches to delete
                        println!("👋There is no branches difference to delete Bye Bye👋");
                        return;
                    }
                    branches
                }
                Err(err) => panic!("Unable to select branches: {err:?}"),
            };

        let confirm = Confirm::default(
            "This action will delete the branches you choose. You want to continue?",
            None,
            None,
        );

        match confirm.run() {
            Ok(true) => (),
            Ok(false) => return,
            Err(err) => panic!("Unable to confirm: {err:?}"),
        };

        for branch in multiselect {
            GitHelper::delete_branch(&repo, &branch).unwrap();
            println!("Deleting branch: {branch:?}");
        }
        println!("🚜 The work is done 🚜");
    }
}
