mod app;
mod arguments;
mod consts;
mod gits;
mod logs;

extern crate termion;

use std::path::PathBuf;

use arguments::{
    confirm::Confirm, input::Input, multiselect::MultiSelect, secret::Secret, select::Select,
};
use clap::Parser;
use gits::{behavior::UserInput, git_work::GitWork};
use tokio::{self};

use crate::{
    arguments::{
        common_trait::{Default, Run},
        enquirer::Enquirer,
    },
    consts::OPTION_MESSAGES,
    gits::git_helper::GitHelper,
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
    let select = Select::default(
        "Choose the command you want to execute:",
        None,
        Some(OPTION_MESSAGES.iter().map(|&s| s.to_string()).collect()),
    );

    let behavior = select.run().unwrap();

    let behavior = match behavior.as_str() {
        "clone the project" => UserInput::Clone,
        "sync the existing project with remote repo" => UserInput::Sync,
        "sync the existing project and delete the unnecessary branches" => UserInput::SyncAndDelete,
        _ => panic!("Unexpected variant"),
    };

    println!("You selected: {:?}", behavior);

    let git_work = GitWork::new(behavior);
    git_work.run();
}
