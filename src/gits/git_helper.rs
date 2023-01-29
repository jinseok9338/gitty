use std::{path::PathBuf};

use git2::{BranchType, Branches, Error, Remote, Repository};

use reqwest::{Client, ClientBuilder, header::{HeaderMap, USER_AGENT, HeaderValue}};
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
        let repo = match Repository::clone(url, directory) {
            //if okay then return the repository in result enum
            Ok(repo) => Ok(repo),
            // if the directory is not empty then return the repository in result enum
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

    // return all branches in remote repo with the url provided
    pub async fn  remote_branches(&self, url: &str) -> ReqwestResult<Vec<String>> {
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
            let branches_names:Vec<String> = branches_names
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

    // pub fn pull_branch(&self, branch:&str, repo:Repository){
    //     // pull branch from remote repo to local repo
    //     let mut remote = repo.find_remote("origin").unwrap();
    //     let mut remote_branch = format!("refs/heads/{}", branch);
    //     let mut local_branch = format!("refs/heads/{}", branch);
    //     let mut refspecs = vec![&remote_branch[..], &local_branch[..]];
    //     remote.fetch(&refspecs, None, None).unwrap();
    //     let mut remote_branch = repo.find_reference(&remote_branch).unwrap();
    //     let mut local_branch = repo.find_reference(&local_branch).unwrap();
    //     let mut analysis = repo.merge_analysis(&[&remote_branch]).unwrap();
    //     if analysis.0.is_up_to_date() {
    //         println!("Already up-to-date!");
    //     } else if analysis.0.is_fast_forward() {
    //         println!("Fast-forwarding...");
    //         let mut reference = repo.reference(&local_branch.name().unwrap(), remote_branch.target().unwrap(), true, "Fast-forward").unwrap();
    //         repo.set_head(&reference.name().unwrap()).unwrap();
    //         repo.checkout_head(None).unwrap();
    //     } else {
    //         println!("Merging...");
    //         let mut index = repo.merge_commits(&local_branch.peel_to_commit().unwrap(), &remote_branch.peel_to_commit().unwrap(), None).unwrap();
    //         if index.has_conflicts() {
    //             println!("Conflicts!");
    //         } else {
    //             let mut tree_id = index.write_tree().unwrap();
    //             let mut tree = repo.find_tree(tree_id).unwrap();
    //             let mut sig = repo.signature().unwrap();
    //             let mut parent = repo.head().unwrap();
    //             let mut parent_commit = parent.peel_to_commit().unwrap();
    //             let mut commit_id = repo.commit(Some("HEAD"), &sig, &sig, "Merge", &tree, &[&parent_commit]).unwrap();
    //             let mut reference = repo.reference(&local_branch.name().unwrap(), commit_id, true, "Merge").unwrap();
    //             repo.set_head(&reference.name().unwrap()).unwrap();
    //             repo.checkout_head(None).unwrap();
    //         }
    //     }

    // }
}
