use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Branch {
    pub name: String,
}
