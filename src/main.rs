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

use crate::arguments::{enquirer::Enquirer, common_trait::{Run, Default}};

#[derive(Debug, Parser)]
enum EnquirerSubcommand {
    Confirm(Confirm),
    Input(Input),
    Secret(Secret),
    MultiSelect(MultiSelect),
    Select(Select),
}

fn main() {
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

  
    // loop until the condition is met

        if program.directory.is_none() {
            loop{
                let input = Input::default("Enter a directory:",Some(false));
                let value = input.run().unwrap();
                program.directory = Some(value);
                //if the value is not a valid directory, then error
                if !program.validate_directory() {
                    println!("Error: --directory must be a valid directory");
                }
                else{
                    break;
                }
            }
        }

        if program.url.is_none() {
            loop{
                let input = Input::default("Enter a url:",Some(true));
                let value = input.run().unwrap();
                //if the value is not empty, then set it
                if !value.is_empty(){
                    program.url = Some(value);
                }
                if !program.validate_url() {
                    println!("Error: --url must be a valid url");
                }
                //but if the value is none on purpose break the loop
                else{
                    break;
                }
            }
        }

        // do gitty work.
        
 


    //print values
    println!("{:?}", program);
}
