mod arguments;
mod consts;
mod gits;

extern crate termion;

use arguments::{
    confirm::Confirm, input::Input, multiselect::MultiSelect, secret::Secret, select::Select,
};
use color_eyre::eyre::Result;

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
async fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{WELCOME_MESSAGE}");
    let select = Select::default(
        CHOOSE_COMMAND,
        None,
        Some(OPTION_MESSAGES.iter().map(|&s| s.to_string()).collect()),
    );

    let behavior = select.run().unwrap();

    let behavior = match behavior.as_str() {
        "clone the project" => UserInput::Clone(OPTION_MESSAGES[0].to_string()),
        "sync the existing project with remote repo" => {
            UserInput::Sync(OPTION_MESSAGES[1].to_string())
        }
        "Delete unnecessary branches" => {
            UserInput::Purge(
                //choose the index of 2 of CHOOSE_COMMAND
                OPTION_MESSAGES[2].to_string(),
            )
        }
        _ => panic!("Unexpected variant"),
    };

    println!("You selected: {behavior:?}");

    // this needs url and directory as arguments

    let mut git_work = GitWork::new(behavior);
    git_work.run().await;
    Ok(())
}
