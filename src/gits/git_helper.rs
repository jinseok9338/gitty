use std::path::PathBuf;

use git2::{BranchType, Branches, Error, Remote, Repository};

pub struct GitHelper {}

impl GitHelper {
    pub fn new() -> Self {
        Self {}
    }

    //for cloining the repository
    pub fn clone_repo(&self, url: &str, directory: &PathBuf) -> Result<Repository, Error> {
        let repo = match Repository::clone(url, directory) {
            //if okay then return the repository in result enum
            Ok(repo) => Ok(repo),
            // if the repository is already cloned then return the repository in result enum
            Err(e) if e.code() == git2::ErrorCode::Exists => {
                let repo = Repository::open(directory)?;
                Ok(repo)
            }
            // if the repository is not cloned then return the error in result enum
            Err(e) => Err(e),
        };
        return repo;
    }

    // fetch all remote branches
    pub fn fetch_all(&self, repo: &Repository) -> Result<(), Error> {
        let mut remote = repo.find_remote("origin")?;
        remote.fetch(&["refs/heads/*:refs/heads/*"], None, None)?;
        Ok(())
    }

    // fetch a specific remote branch
    pub fn fetch_branch(&self, repo: &Repository, branch: &str) -> Result<(), Error> {
        let mut remote = repo.find_remote("origin")?;
        remote.fetch(
            &[&format!("refs/heads/{}:refs/heads/{}", branch, branch)],
            None,
            None,
        )?;
        Ok(())
    }

    // return all branches in remote repository
    pub fn remote<'a>(&'a self, repo: &'a Repository) -> Result<Box<Remote>, Error> {
        let remote = repo.find_remote("origin");
        let remote = remote.unwrap();
        let remote = Box::new(remote);
        Ok(remote)
    }

    //list all remote branes in remote repo
    pub fn list_remote_branches(&self, remote: &Remote) -> Result<Vec<String>, Error> {
        let branches = remote.list()?;
        let remote_branches = branches
            .iter()
            .filter(|branch| branch.name().starts_with("refs/heads/"))
            .map(|branch| branch.name().replace("refs/heads/", ""))
            .collect();

        Ok(remote_branches)
    }

    pub fn list_local_branches(&self, repo: &Repository) -> Result<Vec<String>, Error> {
        let branches = repo.branches(None)?;
        let local_branches = branches
            .map(|branch| branch.unwrap().0.name().unwrap().unwrap().to_string())
            .collect();
        Ok(local_branches)
    }

    // list remote branches that are not in local repository
    pub fn list_differece_branches(
        &self,
        local_branches: &Vec<String>,
        remote_branches: &Vec<String>,
    ) -> Vec<String> {
        let mut difference_branches = Vec::new();
        for remote_branch in remote_branches {
            if !local_branches.contains(remote_branch) {
                difference_branches.push(remote_branch.to_string());
            }
        }
        difference_branches
    }
}
