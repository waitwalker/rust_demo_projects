
const MAX_VALUE:u32 = 10_000;
fn main() {
    println!("Hello, world!");
    let mut x = 5;

    println!("The valiue of x is {}", x);

    x = 6;

    println!("The value of x is {}", x);
    // 常量使用const关键字修饰，类型必须声明，必须标注，常量命名规范：全部大小，单词之间使用下划线分割
    const THE_HOUR:u8 = 24;

    println!("The hour is {}",THE_HOUR);

    println!("The max value is {}", MAX_VALUE);
}
