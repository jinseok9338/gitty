use std::{fs, io::Read, path::Path, env};

use serde::Deserialize;

use crate::run_cmd;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub git_hub_auth_token: String,
}

impl Settings {
    pub fn new() -> Self {
        //run command to get the directory of ~/.zshrc with run_cmd macro
        //run command to get the directory of ~/.bashrc with run_cmd macro
        let home_dir = match env::var("HOME") {
            Ok(dir) => dir,
            Err(_) => {
              panic!("Unable to get home directory")
            }
        };

        let zshrc_path = Path::new(&home_dir).join(".zshrc");
        let bashrc_path = Path::new(&home_dir).join(".bashrc");

        match Self::get_token_from_file(zshrc_path.to_str().unwrap()) {
            Some(token) => Self {
                git_hub_auth_token: token,
            },
            None => match Self::get_token_from_file(bashrc_path.to_str().unwrap()) {
                Some(token) => Self {
                    git_hub_auth_token: token,
                },
                None =>  {
                    let contents = std::fs::read_to_string("./gitty_config.yml").expect("Unable to read file");
                    let settings: Self = serde_yaml::from_str(&contents).expect("Unable to parse YAML");
                    settings
                },
            },
        }
    }

    // get token from env
    pub fn get_token_from_file(file_path: &str) -> Option<String> {
        let mut file = match fs::File::open(file_path) {
            Ok(file) =>file,
            Err(_) => return None,
        };
    
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(_) => return None,
        };
    
        let token = match contents.lines().find(|line| line.starts_with("GITHUB_ACCESS_TOKEN=")) {
            Some(line) => line,
            None => return None,
        };
    
        let token = token.trim_start_matches("GITHUB_ACCESS_TOKEN=");
        Some(token.to_string())
    }

}
