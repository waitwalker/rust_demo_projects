fn main() {
    println!("Hello, world!");

    let mut s1 = String::from("Hello");
    // 传递的是s1的引用，s1的值不会受到影响，并不是s1的移动
    // 把引用作为函数参数这个行为叫做借用
    let len = calculate_length(&mut s1);
    println!("The length of '{}' is {}", s1, len);

    let mut sx = String::from("Hello");
    let s2 = &mut sx;       
    println!("The value of s2 is {}", s2);

    // let s3 = &mut sx;
    // println!("The value of s3 is {}", s3);

    let mut s3 = String::from("What is this?");
    {
        let s4 = &mut s3;
        println!("The value of s4 is {}", s4);
    }

    let s5 = &mut s3;
    println!("The value of s5 is {}", s5);

    let s7 = dangle();
    println!("The value of s7 is {}", s7);
}

fn calculate_length(s: &mut String) -> usize {
    println!("The value of {} in function", s);
    s.push_str(" World!");
    s.len()
}

fn dangle() -> &String {
    let s = String::from("Hello");
    &s
}
