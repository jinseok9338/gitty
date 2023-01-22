use std::path::PathBuf;

use git2::{BranchType, Branches, Error, Remote, Repository};

pub struct GitWork {}

impl GitWork {
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
        let branches = remote.unwrap();
        let branches = Box::new(branches);
        Ok(branches)
    }

    //return all local branches in repository as Branches enum
    pub fn get_local_branches<'a>(
        &'a self,
        repo: &'a Repository,
        filter: Option<BranchType>,
    ) -> Result<Box<Branches>, Error> {
        let branches = repo.branches(filter);
        let branches = branches.unwrap();
        let branches = Box::new(branches);
        Ok(branches)
    }
}
