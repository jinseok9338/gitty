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
