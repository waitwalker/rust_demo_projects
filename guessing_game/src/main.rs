use rand::Rng; //里面提供了很多trait
use std::cmp::Ordering;
use std::io; // 使用std中的cmp模块中的Ordering

fn main() {
    println!("猜数！");
    // 整数类型i32,u32,i64
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("神秘数字是：{}", secret_number);

    loop {
        println!("猜测一个数");
        let mut guess = String::new();

        // 这里引用，添加上mute 变成可变的
        io::stdin().read_line(&mut guess).expect("无法读取行");

        // guess允许使用同名的变量，rust允许使用同名的变量，来隐藏旧的变量，隐藏shadow
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的数是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {
                println!("相等");
                break;
            }
        }
    }
}
