mod app;
mod arguments;
mod consts;
mod gits;
mod logs;
mod sys_work;

extern crate termion;


use arguments::{
    confirm::Confirm, input::Input, multiselect::MultiSelect, secret::Secret, select::Select,
};
use clap::Parser;


use crate::arguments::enquirer::Enquirer;




#[derive(Debug, Parser)]
enum EnquirerSubcommand {
    Confirm(Confirm),
    Input(Input),
    Secret(Secret),
    MultiSelect(MultiSelect),
    Select(Select),
}

fn main() {
    let program = Enquirer::parse();
    //if program.all exists and program.directory or program.url is not None, then error
    if program.all && (program.directory.is_some() || program.url.is_some()) {
        println!("Error: --all cannot be used with --directory or --url");
        std::process::exit(1);
    }
    //if program.all is false and program.directory or program.url is None, then error
    if !program.all && (program.directory.is_none() && program.url.is_none()) {
        println!("Error: --directory or --url must be used");
        std::process::exit(1);
    }
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


    let program = program.transform_enquirer();
  
    //print values 
    println!("{:?}", program);

}
