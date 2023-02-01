use std::path::{Path, PathBuf};

use git2::{build::RepoBuilder, BranchType, Error, Remote, RemoteCallbacks, Repository};

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

    pub fn pull_branch(&self, repo: &Repository, branch: &str) -> Result<(), Error> {
        let mut remote = repo.find_remote("origin")?;
        let mut fo = git2::FetchOptions::new();
        let callbacks = RemoteCallbacks::new();
        fo.remote_callbacks(callbacks);
        remote.fetch(
            &[&format!("refs/heads/{branch}:refs/heads/{branch}")],
            Some(&mut fo),
            None,
        )?;
        Ok(())
    }

    pub fn delete_branch(&self, repo: &Repository, branch_name: &str) -> Result<(), Error> {
        let mut branch = repo.find_branch(branch_name, BranchType::Local)?;
        branch.delete()?;

        Ok(())
    }

    //for cloining the repository
    pub fn clone_repo(&self, url: &str, directory: &Path) -> Result<Repository, Error> {
        // if the directory is not empty then return the error in result enum
        if directory.read_dir().unwrap().count() > 0 {
            return Err(Error::from_str("Directory is not empty"));
        }

        let callbacks = RemoteCallbacks::new();
        // set credentials callback here, if necessary TODO need cred when cloning a private repo. This is for later implementation

        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);

        let repo = RepoBuilder::new().fetch_options(fo).clone(url, directory);

        repo
    }

    pub fn repo(&self, directory: &PathBuf) -> Result<Repository, Error> {
        Repository::open(directory)
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

        let repo_url = format!("https://api.github.com/repos/{repo_url}/branches");

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
    pub fn remote<'a>(&'a self, repo: &'a Repository) -> Result<Remote, Error> {
        let remote = repo.find_remote("origin");
        let remote = remote.unwrap();
        let remote = Box::new(remote);
        Ok(*remote)
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
    pub fn _list_differece_branches(
        &self,
        local_branches: &[String],
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
