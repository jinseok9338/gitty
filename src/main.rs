mod arguments;
mod consts;
mod gits;
mod setting;

extern crate termion;

use arguments::select::Select;
use color_eyre::eyre::Result;

use gits::{behavior::UserInput, git_work::GitWork};
use tokio::{self};

use crate::{
   
    consts::{CHOOSE_COMMAND, OPTION_MESSAGES, WELCOME_MESSAGE}, arguments::common_trait::{Default, Run},
};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{WELCOME_MESSAGE}");

    // make OPTION_MESSAGES to vec of Userinput

   

    let select = Select::<UserInput>::default(
        CHOOSE_COMMAND,
        None,
        Some(OPTION_MESSAGES.to_vec()),
    );

    let behavior = select.run();
    let behavior = behavior.map_or_else(|_| panic!("Error in selecting the option"), |behavior| behavior);


    println!("You selected: {behavior:?}");

    let mut git_work = GitWork::new(behavior);
    git_work.run().await;
    Ok(())
}
