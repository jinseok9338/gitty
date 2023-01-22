mod arguments;
mod gits;
mod sys_work;

use crate::sys_work::sys_work::SysWork;
use arguments::arguments::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    let sys_work = SysWork {};
    //call syswork struct and get the current directory

    let directory = sys_work.currnet_dir(&args.directory).unwrap();
    println!("The directory chosen is: {}", directory.display());
}
