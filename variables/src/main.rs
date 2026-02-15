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

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 5;
    let quotient = 20.0 / 4.0;

    println!("The sum is {}", sum);
    println!("The difference is {}", difference);
    println!("The product is {}", product);
    println!("The quotient is {}", quotient);

    let t = true;

    println!("The value of t is {}", t);

    let a: (i32, f64, char) = (500, 6.4, 'a');
    println!("The value of a is {:?}", a);

    // 解构
    let (x, y, z) = a;
    println!("The value of x is {}, y is {}, z is {}", x, y, z);

    let a1 = a.0;
    let a2 = a.1;
    let a3 = a.2;
    println!("The value of a1 is {}, a2 is {}, a3 is {}", a1, a2, a3);

    let array1: [String; 5] = [
        "abcd".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
        "e".to_string(),
    ];
    println!("The value of array1 is {:?}", array1);

    // 声明三个元素都是3的数组
    let array2: [i32; 3] = [3, 3, 3];
    println!("The value of array2 is {:?}", array2);

    // 遍历数组
    // 访问数组元素
    for i in 0..array1.len() {
        println!("element: {}", array1[i]);
    }

    for element in array1.iter() {
        println!("current element:{}",element);
    }


}
