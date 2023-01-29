// this is the trait that gets implemented throughout the arguments module

use std::error::Error;

pub trait Run<T>{
    fn run<'a>(&self) -> Result<T, Box<dyn Error>>;
}

pub trait Default {
    fn default(message:&str, can_be_nullable:Option<bool>) -> Self;
}
