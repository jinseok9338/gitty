use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Settings {
    pub git_hub_auth_token: Option<String>,
}

impl Settings {
    pub fn new() -> Self {
        //get env
        let var = option_env!("GITHUB_ACCESS_TOKEN");

        var.map_or_else(Self::default, |v| Self {
            git_hub_auth_token: Some(v.to_string()),
        })
    }
}
