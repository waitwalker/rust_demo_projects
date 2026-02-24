fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello");

    let data = "initial contents";

    let data_string = data.to_string();

    let mut s = String::from("foo");
    // String可以追加字符串切片
    s.push_str("bar");
    println!("{}",s);

    let mut s1 = String::from("hello");
    let s2 = String::from(" world");

    s1.push_str(&s2);

    println!("{}",s1);


    
}
