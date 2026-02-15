fn main() {
    println!("Hello, world!");

    let s1 = String::from("hello world");
    let word = first_world(&s1);
    println!("The value of word is {}", word);

    let hello = &s1[0..5];

    let world = &s1[6..];

    println!("The value of hello is {}", hello);
    println!("The value of world is {}", world);

    let my_string = "hello world";

    let space = slice(my_string);
    // space.clear();
    println!("The value of space is {}", space);

}

fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &element) in bytes.iter().enumerate() {
        if element == b' ' {
            return i;
        }
    }
    s.len()
}

fn slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}
