use clap::{Parser, Subcommand};

#[derive(PartialEq)]
pub enum PositionalArgs {
    //up
    Up(String),
    Log(String),
}

#[derive(Parser, Debug)]
#[command(name = "Gitty")]
#[command(author = "Jinseok seo Jinseok9338@gmail.com")]
#[command(version = "1.0")]
#[command(about = "Gitty tool", long_about = None)]
pub struct Args {
    pub up: Option<String>,

    /// URL for cloning the repository
    #[arg(short, long)]
    pub url: Option<String>,

    /// Directory to clone the repository
    #[arg(short, long)]
    pub directory: Option<String>,
}

impl Args {
    pub fn new() -> Self {
        Self::parse()
        //if there is no up value return the error
    }

    pub fn print_args(&self) {
        println!("{:?}", self);
    }
}
