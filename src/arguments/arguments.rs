use clap::{Parser, Subcommand};

#[derive(Default, PartialEq)]
pub enum PositionalArgs {
    Up,
    Log,
    #[default]
    Other,
}

#[derive(Parser, Debug)]
#[command(name = "Gitty")]
#[command(author = "Jinseok seo Jinseok9338@gmail.com")]
#[command(version = "1.0")]
#[command(about = "Gitty tool", long_about = None)]
pub struct Args {
    pub first_arg: Option<String>,

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

    // only for debugging delete when release
    pub fn print_args(&self) {
        println!("{:?}", self);
    }

    pub fn parse_first_args(&self, first_arg: PositionalArgs, restArgs: Args) {
        match first_arg {
            PositionalArgs::Up => {
                // do gitty up task
            }
            PositionalArgs::Log => {
                // do gitty log task
            }
            _ => {
                // throw error
            }
        }
    }
}
