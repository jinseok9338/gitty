pub trait Run<T, E> {
    fn run(&self) -> Result<T, E>;
}

pub trait Default<T> {
    fn default(message: &str, can_be_nullable: Option<bool>, items: Option<Vec<T>>) -> Self;
}

pub trait ToString<T> {
    fn to_string(&self) -> String;
}
