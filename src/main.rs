mod app;
mod arguments;
mod consts;
mod gits;
mod logs;

extern crate termion;

use arguments::{
    confirm::Confirm, input::Input, multiselect::MultiSelect, secret::Secret, select::Select,
};
use clap::Parser;
use gits::{behavior::UserInput, git_work::GitWork};
use tokio::{self};

use crate::{
    arguments::common_trait::{Default, Run},
    consts::{CHOOSE_COMMAND, OPTION_MESSAGES, WELCOME_MESSAGE},
};

#[derive(Debug, Parser)]
enum EnquirerSubcommand {
    Confirm(Confirm),
    Input(Input),
    Secret(Secret),
    MultiSelect(MultiSelect),
    Select(Select),
}

#[tokio::main]
async fn main() {
    println!("{}", WELCOME_MESSAGE);
    let select = Select::default(
        CHOOSE_COMMAND,
        None,
        Some(OPTION_MESSAGES.iter().map(|&s| s.to_string()).collect()),
    );

    let behavior = select.run().unwrap();

    let behavior = match behavior.as_str() {
        "clone the project" => UserInput::Clone("clone the project".to_string()),
        "sync the existing project with remote repo" => {
            UserInput::Sync("sync the existing project with remote repo".to_string())
        }
        "sync the existing project and delete the unnecessary branches" => {
            UserInput::SyncAndDelete(
                "sync the existing project and delete the unnecessary branches".to_string(),
            )
        }
        _ => panic!("Unexpected variant"),
    };

    println!("You selected: {:?}", behavior);

    // this needs url and directory as arguments

    let mut git_work = GitWork::new(behavior);
    git_work.run().await
}
