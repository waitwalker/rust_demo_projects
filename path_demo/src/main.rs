use path_demo::color::rgb;
use std::fmt::Result;
use std::io::Result as IoResult;
use rand::Rng;
use std::hashmap::HashMap;
// 引入嵌套路径
use std::{hash::Hash, cmp::Eq};

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
