fn main() {
    println!("Hello, world!");
    let name:String = String::from("Value C++");
    let course = "Rust".to_owned();
    let new_name = name.replace("C++", "CPP");

    println!("name:{name}, course:{course}, new_name:{new_name}");

    let rust: &str = "Rust";
    let rust = "\x52\x75\x73\x74";
    println!("{rust}");
}

struct Person<'a> {
    name:&'a str,
    color:String,
    age:u8,
}
