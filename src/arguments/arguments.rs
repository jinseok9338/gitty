use clap::{Parser, Subcommand};

#[derive(PartialEq)]
pub enum PositionalArgs {
    //up
    Up,
    Log,
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

    pub fn print_args(&self) {
        println!("{:?}", self);
    }

    pub fn parse_first_args(&self,first_arg:PositionalArgs, restArgs:Args) {
        match first_arg {
            PositionalArgs::Up => {
                if self.first_arg.is_some() {
                    // do gitty up task
                    // if no argument is provided, return sync local repo with remote repo 

                    //if restArgs.url.is_some() && restArgs.directory.is_some() {} // do gitty up task with url and directory

                    // if only restArgs.url is provided  // clone the git repo and sync with remote repo at the current directory

                    // if only restArgs.directory is provided // sync local repo with remote repo if there is git repo in the directory

                }
            }
            PositionalArgs::Log => {
                if self.first_arg.is_some() {
                  // do gitty log task
                }
            }
        }
    }
}
