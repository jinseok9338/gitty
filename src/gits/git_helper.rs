use std::path::{Path, PathBuf};

use git2::{BranchType, Error, Remote, RemoteCallbacks, Repository};

use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
    ClientBuilder, Url,
};
use tokio::spawn;

use crate::{consts::PROPER_URL_WARNING, run_cmd, setting::read_setting::Settings};

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

    pub fn fetch_all(repo: &Repository) -> Result<(), git2::Error> {
        let mut remote = repo.find_remote("origin")?;
        let mut fo = git2::FetchOptions::new();
        let callbacks = RemoteCallbacks::new();
        fo.remote_callbacks(callbacks);
        remote.fetch(&["refs/heads/*:refs/heads/*"], Some(&mut fo), None)?;
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

    pub fn current_branch(repo: &Repository) -> Result<String, Error> {
        let head = repo.head()?;
        let branch = head.shorthand().unwrap();
        Ok(branch.to_string())
    }

    pub fn delete_branch(repo: &Repository, branch_name: &str) -> Result<(), Error> {
        let mut branch = repo.find_branch(branch_name, BranchType::Local)?;
        branch.delete()?;

        Ok(())
    }

    fn change_url(url: &str, access_token: &str) -> String {
        let parts: Vec<&str> = url.split('/').collect();
        let username = parts[3];
        let project = parts[4].trim_end_matches(".git");
        format!(
            "https://{}:{}@github.com/{}/{}.git",
            access_token, "x-oauth-basic", username, project
        )
    }

    pub fn clone_repo(url: &str, directory: &Path) -> Result<Repository, git2::Error> {
        let url = Url::parse(url).unwrap();
        let binding = url.to_string();
        let project_name = binding
            .split('/')
            .last()
            .unwrap()
            .split('.')
            .next()
            .unwrap();
        let settings = Settings::new();

        let directory = directory.join(project_name);

        let url = match settings.git_hub_auth_token {
            Some(token) => Self::change_url(&binding, &token),
            None => binding,
        };

        let command = format!("git clone {} {}", url, directory.display());
        match run_cmd!(command) {
            Ok(_) => {}
            Err(err) => panic!("Error while cloning repo: {err:?}"),
        }

        Self::repo(&directory)
    }

    pub fn repo(directory: &PathBuf) -> Result<Repository, Error> {
        Repository::open(directory)
    }

    pub async fn remote_branches(&self, url: &str) -> ReqwestResult<Vec<String>> {
        let start = url.find(".com/").expect(PROPER_URL_WARNING) + 5;

        let end = url.find(".git").expect(PROPER_URL_WARNING);

        let repo = &url[start..end];
        let repo_url = format!("https://api.github.com/repos/{repo}/branches?per_page=100");

        let branches_names = spawn(async move {
            let settings = Settings::new();

            let mut branches: Vec<String> = vec![];
            let mut page = 1;

            loop {
                let repo_url = repo_url.clone();

                let repo_url = format!("{repo_url}&page={page}");
                let client = ClientBuilder::new().build().unwrap();
                let mut headers = HeaderMap::new();
                headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
                let response = match settings.git_hub_auth_token {
                    Some(ref token) => {
                        client
                            .get(&repo_url)
                            .headers(headers)
                            .header("Accept", "application/vnd.github+json")
                            .header("Authorization", format!("Bearer {}", token))
                            .header("X-GitHub-Api-Version", "2022-11-28")
                            .send()
                            .await
                    }
                    None => {
                        client
                            .get(&repo_url)
                            .headers(headers)
                            .header("Accept", "application/vnd.github+json")
                            .header("X-GitHub-Api-Version", "2022-11-28")
                            .send()
                            .await
                    }
                };
                let response = response.unwrap();

                let response = response.json::<Vec<Branch>>().await;

                match response {
                    Ok(branches_names) => {
                        if branches_names.is_empty() {
                            break;
                        }
                        let branches_names: Vec<String> = branches_names
                            .iter()
                            .map(|branch| branch.name.to_string())
                            .collect();
                        branches.extend(branches_names);
                    }
                    Err(err) => panic!("Error while fecching branches names: {err:?} "),
                }
                page += 1;
            }

            branches
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
