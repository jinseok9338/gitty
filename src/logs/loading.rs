use std::{iter::Cycle, time};

pub struct Loading {
    message: String,
    spinner_chars: Vec<&'static str>,
    pub spinner_interval: time::Duration,
}

impl Loading {
    // show loading message with dot indicator in terminal
    pub fn spinner(&self) -> Cycle<std::slice::Iter<'_, &str>> {
        self.spinner_chars.iter().cycle()
    }

    pub fn new(message: String) -> Self {
        Self {
            message,
            spinner_chars: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            spinner_interval: time::Duration::from_millis(100),
        }
    }
}
