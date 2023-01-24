mod arguments;
mod gits;
mod logs;
mod sys_work;
mod app;
mod consts;

extern crate termion;



use arguments::{multiselect::MultiSelect, secret::Secret, confirm::Confirm, input::Input, select::Select};
use clap::Parser;
use dialoguer::console::set_colors_enabled;



#[derive(Debug, Parser)]
#[clap(name = "enquirer", version)]
struct Enquirer {
    #[clap(subcommand)]
    cmd: EnquirerSubcommand,

    /// Disable colors in the prompt
    #[clap(long)]
    no_color: bool,
}

#[derive(Debug, Parser)]
enum EnquirerSubcommand {
    Confirm(Confirm),
    Input(Input),
    Secret(Secret),
    MultiSelect(MultiSelect),
    Select(Select),
   
}


fn main() {
        // TODO: Specify height for selection prompts (like fzf)
        let program = Enquirer::parse();
        set_colors_enabled(!program.no_color);
        match program.cmd {
            EnquirerSubcommand::Confirm(x) => x.run(),
            EnquirerSubcommand::Input(x) => x.run(),
            EnquirerSubcommand::Secret(x) => x.run(),
            EnquirerSubcommand::MultiSelect(x) => x.run(),
            EnquirerSubcommand::Select(x) => x.run(),
        }
        .unwrap();

}







