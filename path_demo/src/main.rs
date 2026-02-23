use path_demo::color::rgb;
use std::fmt::Result;
use std::io::Result as IoResult;

fn main() {
    println!("Hello, world!");
    rgb::value();
}

fn f1() -> Result {
    Ok(())
}

fn f2() -> IoResult {
    Ok(())
}
