use std::io::{Stdin, StdinLock, Stdout, StdoutLock, Write};
use termion::color;
use termion::raw::RawTerminal;

pub struct App<'a> {
    stdout: &'a mut RawTerminal<StdoutLock<'a>>,
    stdin: &'a StdinLock<'a>,
}

impl<'a> App<'a> {
    pub fn new(stdout: &'a mut RawTerminal<StdoutLock<'a>>, stdin: &'a StdinLock) -> Self {
        Self { stdout, stdin }
    }

    pub fn show_welcome_message(&mut self) {
        write!(
            self.stdout,
            "{}{}{}yo, 'q' will exit.{}{}",
            termion::clear::All,
            termion::cursor::Goto(5, 5),
            termion::style::Bold,
            termion::style::Reset,
            termion::cursor::Goto(20, 10)
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }
}
