fn main() {
    println!("Hello, world!");

    let mut s1 = String::from("hello world");
    let word = first_world(&s1);
    println!("The value of word is {}", word);

    let hello = &s1[0..5];
    
    let world = &s1[6..];

    println!("The value of hello is {}", hello);
    println!("The value of world is {}", world);
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
