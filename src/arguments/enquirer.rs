use clap::Parser;
use url::Url;

#[derive(Debug, Parser, Default)]
#[clap(name = "Gitty Up", version)]
pub struct Enquirer {
    #[clap(short, long)]
    pub directory: Option<String>,
    #[clap(short, long)]
    pub url: Option<String>,
}

impl Enquirer {
    // need to consider the fact that the url can be empty on purpose
    pub fn validate_url(&self) -> bool {
        if self.url.is_some() {
            match Url::parse(self.url.clone().unwrap().as_str()) {
                Ok(_) => true,
                Err(_) => false,
            }
        } else {
            true
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
