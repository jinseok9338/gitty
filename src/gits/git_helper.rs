use std::path::PathBuf;

use git2::{ Error, Remote, Repository};

use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
     ClientBuilder,
};
use tokio::spawn;

use super::r#type::Branch;
use reqwest::Result as ReqwestResult;

// impl the debug

#[derive(Debug)]
pub struct GitHelper {}

impl GitHelper {
    pub fn new() -> Self {
        Self {}
    }

    //for cloining the repository
    pub fn clone_repo(&self, url: &str, directory: &PathBuf) -> Result<Repository, Error> {
        // check if the directory is empty
        let repo = if directory.is_dir() {
            // if the directory is not empty then return the error in result enum
            Err(Error::from_str("Directory is not empty"))
        } else {
            // if the directory is empty then clone the repository
            Repository::clone(url, directory)
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

    // return all branches in remote repo with the url provided
    pub async fn remote_branches(&self, url: &str) -> ReqwestResult<Vec<String>> {
        // only get only jinseok/jinseok from string https://github.com/jinseok9338/jinseok9338.git

        // Find the position of the '/' after '.com'
        let start = url.find(".com/").unwrap() + 5;
        // Find the position of the '.git
        let end = url.find(".git").unwrap();
        // Extract the substring between the two positions
        let repo_url = &url[start..end];

        let repo_url = format!("https://api.github.com/repos/{}/branches", repo_url);

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));

        let branches_names = spawn(async move {
            let client = ClientBuilder::new().build().unwrap();
            let response = client.get(&repo_url).headers(headers).send().await;
            let response = response.unwrap();
            let branches_names = response.json::<Vec<Branch>>().await.unwrap();
            let branches_names: Vec<String> = branches_names
                .iter()
                .map(|branch| branch.name.to_string())
                .collect();

            branches_names
        });
        let branches_names = branches_names.await.unwrap();
        Ok(branches_names)
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

    pub fn add_remote_for_fetch(&self, repo: &Repository, url: &str) -> Result<(), Error> {
        let remote_name = "origin";
        let mut remote = repo.remote(remote_name, url)?;
        // fetch from remote
        remote.connect(git2::Direction::Fetch);
        Ok(())
    }

    pub fn create_local_branch(&self, repo: &Repository, branch_name: &str) -> Result<Branch, Error> {
        
        // repo head to repo commit 
        
        let branch = repo.branch(branch_name, &repo.head()?, false)?;
        Ok(branch)
    }


}
