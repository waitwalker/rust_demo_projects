fn main() {
    println!("Hello, world!");
    sum();
}

fn sum() {
    let a = 1;
    let b = 3;

    println!("The sum of a and b is {}", a + b);

    let c = sumAdd(100, 300);
    println!("The sum function value c is {}", c);

    // 块，里面加上分号，就没有返回值了
    let y = {
        let m = 10;
        m + 8
    };

    println!("The value of y is {}", y);

    /**
     * 
     * 
     * 
    */
}

fn sumAdd(x: i64, y: i64) -> i64 {
    return x + y;
}
