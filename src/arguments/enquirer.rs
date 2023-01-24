use clap::Parser;
use url::Url;

#[derive(Debug, Parser)]
#[clap(name = "Gitty Up", version)]
pub struct Enquirer {
    #[clap(short, long)]
    pub all: bool,
    #[clap(short, long)]
    pub directory: Option<String>,
    #[clap(short, long)]
    pub url: Option<String>,
}

impl Enquirer {
    pub fn transform_enquirer(&self) -> Self {
        let directory = match self.all {
            true => Some(
                std::env::current_dir()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            ),
            false => self.directory.clone(),
        };
        let directory = match directory {
            Some(dir) => Some(dir),
            None => Some(
                std::env::current_dir()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            ),
        };
        let url = match self.all {
            true => None,
            false => self.url.clone(),
        };
        let all = self.all;
        Self {
            all,
            directory,
            url,
        }
    }

    pub fn validate_url(&self) -> bool {
        match Url::parse(self.url.clone().unwrap().as_str()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn validate_directory(&self) -> bool {
        // if directory is not None, then check if it exists
        match std::fs::metadata(self.directory.clone().unwrap().as_str()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
