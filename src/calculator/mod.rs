mod parser;

#[allow(dead_code)]
pub struct Operator<'a,T> {
    symbol : &'a str,
    order : usize,
    f : fn(T, T) -> T
}

/// Evaluates an expression and returns a `Result<T, &str>` where `T` is the output type and `&str` is an error string.
#[allow(dead_code)]
pub fn evaluate<'a,T>(expression : &str, operators : &[Operator<'a,T>]) -> Result<T,&'a str> {

    Err("Not implemented.")
}