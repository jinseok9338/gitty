use std::{
    f32::consts::E,
    path::{Path, PathBuf},
};

use git2::{build::RepoBuilder, BranchType, Error, Remote, RemoteCallbacks, Repository};

use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
    ClientBuilder,
};
use tokio::spawn;

use crate::{consts::PROPER_URL_WARNING, setting::read_setting::Settings};

use super::r#type::Branch;
use reqwest::Result as ReqwestResult;

#[derive(Debug)]
pub struct GitHelper {}

impl GitHelper {
    pub const fn new() -> Self {
        Self {}
    }

    pub fn fetch_all_and_prune(repo: &Repository) -> Result<(), git2::Error> {
        let mut remote = repo.find_remote("origin")?;
        let mut fo = git2::FetchOptions::new();
        let callbacks = RemoteCallbacks::new();
        fo.remote_callbacks(callbacks);
        remote.fetch(&["refs/heads/*:refs/heads/*"], Some(&mut fo), None)?;
        let callbacks = RemoteCallbacks::new();
        remote.prune(Some(callbacks))?;
        Ok(())
    }

    pub fn pull_branch(repo: &Repository, branch: &str) -> Result<(), Error> {
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

    pub fn delete_branch(repo: &Repository, branch_name: &str) -> Result<(), Error> {
        let mut branch = repo.find_branch(branch_name, BranchType::Local)?;
        branch.delete()?;

        Ok(())
    }

    pub fn clone_repo(url: &str, directory: &Path) -> Result<Repository, git2::Error> {
        let project_name = url.split('/').last().unwrap().split('.').next().unwrap();

        let directory = directory.join(project_name);

        let callbacks = RemoteCallbacks::new();

        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);

        let repo = RepoBuilder::new().fetch_options(fo).clone(url, &directory);

        repo
    }

    pub fn repo(directory: &PathBuf) -> Result<Repository, Error> {
        Repository::open(directory)
    }

    pub async fn remote_branches(&self, url: &str) -> ReqwestResult<Vec<String>> {
        let start = url.find(".com/").expect(PROPER_URL_WARNING) + 5;

        let end = url.find(".git").expect(PROPER_URL_WARNING);

        let repo_url = &url[start..end];

        let repo_url = format!("https://api.github.com/repos/{repo_url}/branches");

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));

        let branches_names = spawn(async move {
            let client = ClientBuilder::new().build().unwrap();
            let settings = Settings::new();
            let response = client
                .get(&repo_url)
                .headers(headers)
                .header("Accept", "application/vnd.github+json")
                .header(
                    "Authorization",
                    format!("Bearer {}", settings.git_hub_auth_token),
                )
                .header("X-GitHub-Api-Version", "2022-11-28")
                .send()
                .await;
            let response = response.unwrap();
            let response = response.json::<Vec<Branch>>().await;

            let branch_names = match response {
                Ok(branches_names) => branches_names,
                Err(err) => panic!("Error while fecching branches names: {:?} ", err),
            };

            let branches_names: Vec<String> = branch_names
                .iter()
                .map(|branch| branch.name.to_string())
                .collect();

            branches_names
        });
        let branches_names = branches_names.await.unwrap();
        Ok(branches_names)
    }

    pub fn remote(repo: &Repository) -> Remote {
        let remote = repo.find_remote("origin");
        let remote = remote.unwrap();
        let remote = Box::new(remote);
        *remote
    }

    pub fn list_remote_branches(remote: &Remote) -> Result<Vec<String>, Error> {
        let branches = remote.list()?;
        let remote_branches = branches
            .iter()
            .filter(|branch| branch.name().starts_with("refs/heads/"))
            .map(|branch| branch.name().replace("refs/heads/", ""))
            .collect();

        Ok(remote_branches)
    }

    pub fn list_local_branches(repo: &Repository) -> Result<Vec<String>, Error> {
        // list only local branches that doesn't contain remote branches
        let branches = repo.branches(Some(BranchType::Local))?;
        let local_branches = branches
            .map(|branch| branch.unwrap().0.name().unwrap().unwrap().to_string())
            .collect();
        Ok(local_branches)
    }

    pub fn _list_differece_branches(
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
