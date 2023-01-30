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
use gits::git_work::GitWork;
use tokio::{self};

use crate::{
    arguments::{
        common_trait::{Default, Run},
        enquirer::Enquirer,
    },
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
    let mut program = Enquirer::parse();

    //if program.url is not None and is not a valid url, then error
    if program.url.is_some() && !program.validate_url() {
        println!("Error: --url must be a valid url");
        std::process::exit(1);
    }
    //if program.directory is not None and is not a valid directory, then error
    if program.directory.is_some() && !program.validate_directory() {
        println!("Error: --directory must be a valid directory");
        std::process::exit(1);
    }

    if program.directory.is_none() {
        loop {
            let input = Input::default("Enter a directory:", Some(false), None);
            let value = input.run().unwrap();
            program.directory = Some(value);
            //if the value is not a valid directory, then error
            if !program.validate_directory() {
                println!("Error: --directory must be a valid directory");
            } else {
                break;
            }
        }
    }

    if program.url.is_none() {
        loop {
            let input = Input::default("Enter a url:", Some(true), None);
            let value = input.run().unwrap();
            //if the value is not empty, then set it
            if !value.is_empty() {
                program.url = Some(value);
            }
            if !program.validate_url() {
                println!("Error: --url must be a valid url");
            }
            //but if the value is none on purpose break the loop
            else {
                break;
            }
        }
    }

    // do gitty work.

    // if the url is provided then check the repo related to the url and check the branches
    if program.url.is_some() {
        let git_work = GitWork::new();
        let directory = program.directory.unwrap().into();

        git_work
            .gitty_clone_repo(&program.url.unwrap(), &directory)
            .await
            .unwrap();
        // pull the selected branches
    }
}
