use std::fmt::Display;
use std::error::Error;

#[derive(Debug)]
struct MyError {
    detail: String,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Custom error: {}", self.detail)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.detail
    }
}

fn func() -> Result<(), MyError> {
    Err(MyError {
        detail: String::from("func error"),
    })
}

fn main() -> Result<(),MyError>{
    println!("Hello, world!");

    match func() {
        Ok(()) => println!("func success"),
        Err(error) => println!("func encounter error: {}", error),
    }

    func()?;
    Ok(())
}
