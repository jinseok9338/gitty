mod arguments;
mod gits;
mod sys_work;
mod logs;

use crate::sys_work::sys_work::SysWork;
use arguments::arguments::{Args, PositionalArgs};

fn main() {
    let args = Args::new();
    //allow positional arguments

    let sys_work = SysWork {};
    //call syswork struct and get the current directory
    // if no up value is provided, return the error

    // let directory = sys_work.currnet_dir(&args.directory).unwrap();
    // let gits = gits::gits::GitWork {};
}
