#[derive(Debug)]
pub enum Error {
    Invalid(String),
}

#[macro_export]
macro_rules! create_value_type {
    ($name: ident) => {
        #[derive(Debug)]
        pub struct $name<T>(pub Result<T, Error>);
    };
}
/*
#[derive(Debug)] // TODO: take from other
pub enum Value<T> {
    Ok(T),
    Err(Result<T, Error>),
}
*/
