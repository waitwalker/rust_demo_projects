use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut scores:HashMap<String,i32> = HashMap::new();
    scores.insert(String::from("张三"),100);
    println!("{:?}",scores);
}
