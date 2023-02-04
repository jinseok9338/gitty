use std::fmt;



#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum UserInput<'a> {
    Clone(&'a str),
    HardSync(&'a str),
    SoftSync(&'a str),
    Purge(&'a str),
}


impl<'a> fmt::Display for UserInput<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserInput::Clone(_) => write!(f, "clone"),
            UserInput::HardSync(_) => write!(f, "hard-sync"),
            UserInput::SoftSync(_) => write!(f, "soft-sync"),
            UserInput::Purge(_) => write!(f, "purge"),
        }
    }
}


