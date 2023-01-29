use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Commit {
    sha: String,
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct Branch {
    pub name: String,
    commit: Commit,
    protected: bool,
}