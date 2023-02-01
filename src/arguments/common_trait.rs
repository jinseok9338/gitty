pub trait Run<T, E> {
    fn run(&self) -> Result<T, E>;
}

pub trait Default {
    fn default(message: &str, can_be_nullable: Option<bool>, items: Option<Vec<String>>) -> Self;
}
