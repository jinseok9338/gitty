// this is the trait that gets implemented throughout the arguments module

use std::error::Error;

pub trait Run<T,E> {
    fn run<'a>(&self) -> Result<T, E>;
}

pub trait Default {
    fn default(message: &str, can_be_nullable: Option<bool>, items: Option<Vec<String>>) -> Self;
}
