#[derive(Debug)]
pub enum UserInput {
    Clone(String),
    Sync(String),
    Purge(String),
}
