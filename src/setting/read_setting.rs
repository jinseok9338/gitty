use std::{env, fs, io::Read, path::Path};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub git_hub_auth_token: String,
}

impl Settings {
    pub fn new() -> Self {
     
        let home_dir = env::var("HOME").expect("Unable to get home directory");

        let zshrc_path = Path::new(&home_dir).join(".zshrc");
        let bashrc_path = Path::new(&home_dir).join(".bashrc");

        Self::get_token_from_file(zshrc_path.to_str().unwrap()).map_or_else(
            || {
                Self::get_token_from_file(bashrc_path.to_str().unwrap()).map_or_else(
                    || {
                        let contents = std::fs::read_to_string("./gitty_config.yml")
                            .expect("Unable to read file");
                        let settings: Self =
                            serde_yaml::from_str(&contents).expect("Unable to parse YAML");
                        settings
                    },
                    |token| Self {
                        git_hub_auth_token: token,
                    },
                )
            },
            |token| Self {
                git_hub_auth_token: token,
            },
        )
    }

    // get token from env
    pub fn get_token_from_file(file_path: &str) -> Option<String> {
        let mut file = match fs::File::open(file_path) {
            Ok(file) => file,
            Err(_) => return None,
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(_) => return None,
        };

        let token = match contents
            .lines()
            .find(|line| line.starts_with("GITHUB_ACCESS_TOKEN="))
        {
            Some(line) => line,
            None => return None,
        };

        let token = token.trim_start_matches("GITHUB_ACCESS_TOKEN=");
        Some(token.to_string())
    }
}
