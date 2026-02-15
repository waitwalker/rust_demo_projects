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

    let s4 = String::from("Hello, World!");
    // s4移动到函数里面了，调用函数后s4就失效了
    ownership(s4);
    // println!("The value of s4 is {}", s4);

    let x = 8;
    // 传入函数内部的是x的一个副本，x的值不会受到影响，都是存储在栈上
    make_copy(x);
    println!("The value of x is {}", x);

    let s5 = gives_ownership();

    println!("The value of s5 is {}", s5);

    let s6 = String::from("s6");
    println!("The value of s6 is {}", s6);

    let s7 = takes_and_gives_back(s6);
    println!("The value of s7 is {}", s7);
    // println!("The value of s6 is {}", s6);

    let s8 = String::from("s8");
    println!("The value of s8 is {}", s8);
    let(s9,len) = calculate_length(s8);
    println!("The value of s9 is {}", s9);
    println!("The value of len is {}", len);
    

}

fn ownership(some_string: String) {
    println!("The vaule of some_string is {}", some_string);
}

fn make_copy(some_number:i32) {
    println!("The value of some_number is {}", some_number);
}

fn gives_ownership() -> String {
    let some_string = String::from("sx");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s:String)->(String,usize) {
    let length = s.len();
    (s, length)
}
