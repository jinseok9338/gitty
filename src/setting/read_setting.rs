use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub git_hub_auth_token: String,
}

impl Settings {
    pub fn new() -> Self {
        let contents = std::fs::read_to_string("./gitty_config.yml").expect("Unable to read file");
        let settings: Self = serde_yaml::from_str(&contents).expect("Unable to parse YAML");
        settings
    }
}
