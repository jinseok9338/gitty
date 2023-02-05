use core::time;
use std::thread;

use indicatif::{ProgressBar, ProgressStyle};

pub fn pretty_print_loading(msg:&str){
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}%").expect("Unable to set template")
            .progress_chars("#>-"),
    );
    pb.set_message("Waiting for response");
    pb.set_position(0);
}
