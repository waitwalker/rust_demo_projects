use std::fs::File;
use std::io::ErrorKind;
fn main() {
    println!("Hello, world!");
    // panic!("crash and burn");
    // let v = vec![1,3,5];
    // println!("value is {}",v[10]);

    let mut s = String::from("hello");

    let data = "initial contents";

    let data_string = data.to_string();

    let mut s = String::from("foo");
    // String可以追加字符串切片
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("hello");
    let s2 = String::from(" world");

    s1.push_str(&s2);

    println!("{}", s1);

    let x1 = String::from("hello");
    let x2 = String::from("world");
    let x3 = x1 + &x2;
    println!("{}", x3);
    // + 类似于 add(selft,s:&str)函数，x1的所有权移到函数内部了
    // println!("{}",x1);
    println!("{}", x2);

    let x8 = format!("{}-{}-{}", "hello", "world", "rust");
    println!("{}", x8);

    let x9 = String::from("你好String");
    println!("length:{}", x9.len());
    // println!("{}",x9[0]);

    for elment in x9.bytes() {
        println!("current byte {}", elment);
    }

    for elment in x9.chars() {
        println!("current char {}", elment);
    }

    let x10 = &x9[0..3];
    println!("{:?}", x10);

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Error creating file: {:?}", e),
    //         },
    //         other_error => panic!("Error opening file: {:?}", other_error),
    //     },
    // };

    // 通过闭包形式处理
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Error creating file :{:?}", error);
    //         })
    //     } else {
    //         panic!("Error opening file :{:?}", error);
    //     }
    // });

    let f = File::open("hello.txt").unwrap();
}
