fn main() {
    println!("Hello, world!");

    let mut s = "hello";
    println!("The value of s is {}", s);
    s = "world";
    println!("The value of s is {}", s);

    let mut str1 = String::from("Hello");
    println!("The value of str1 is {}", str1);
    str1.push_str(", World!");
    println!("The value of str1 is {}", str1);
}
