// make enum for each behavior ->     "clone the project", "sync the existing project with remote repo", "sync the existing project and delete the unnecessary branches",

#[derive(Debug)]
pub enum UserInput {
    Clone(String),
    Sync(String),
    Purge(String),
}
