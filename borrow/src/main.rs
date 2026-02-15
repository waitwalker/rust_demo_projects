fn main() {
    println!("Hello, world!");

    let s1 = String::from("Hello");
    // 传递的是s1的引用，s1的值不会受到影响，并不是s1的移动
    // 把引用作为函数参数这个行为叫做借用
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    println!("The value of {} in function", s);
    s.len()
}
