fn main() {
    println!("Hello, world!");

    // loop循环
    loop {
        println!("Aloop");
        break;
    }

    // while条件循环
    let mut number = 10;
    while number > 0 {
        println!("current value is {}", number);
        if number != 5 {
            println!("not 5");
            break;
        }
        number -= 1;
    }

    // for 循环
    for element in 1..number {
        println!("current value is {}", element);
    }

    for cur in (1..number).rev() {
        println!("reverse current value is {}", cur);
    }
}
