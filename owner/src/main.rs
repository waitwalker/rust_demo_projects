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
    // 当变量走出作用域时，Rust会自动调用drop函数，释放变量占用的内存
    let x = 5;
    let y = x;
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);

    let s1 = String::from("hello");
    // s1移动后，赋值给s2就失效了，s1就不能再使用了，此时s2是有效的
    let s2 = s1;
    // println!("The value of s1 is {}", s1);
    println!("The value of s2 is {}", s2);

    // 克隆操作会把Stack上的指针、长度、容量，以及Heap上的值都会复制一份
    let s3 = s2.clone();
    println!("The value of s2 is {}", s2);
    println!("The value of s3 is {}", s3);



}
