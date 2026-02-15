const MAX_VALUE: u32 = 10_000;
// Rust是静态编译语言，在编译时必须知道所有变量的类型
fn main() {
    println!("Hello, world!");
    let mut x = 5;

    println!("The valiue of x is {}", x);

    x = 6;

    println!("The value of x is {}", x);
    // 常量使用const关键字修饰，类型必须声明，必须标注，常量命名规范：全部大小，单词之间使用下划线分割
    const THE_HOUR: u8 = 24;

    println!("The hour is {}", THE_HOUR);

    println!("The max value is {}", MAX_VALUE);

    let y = 5;
    println!("The value of y is {}", y);
    // shadowing 隐藏
    let y = y + 1;
    println!("The value of y is {}", y);

    let y = y * 2;
    println!("The value of y is {}", y);

    let spaces = "   ";
    println!("The value of spaces is {}", spaces);
    let spaces = spaces.len() + 1;
    println!("The value of spaces is {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The guess is {}", guess);
}
