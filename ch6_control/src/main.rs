fn main() {
    println!("Hello, world!");
    let age = 50;
    let msg = if age > 50 {"old"} else {"young"};
    println!("age is {}", age);
    println!("msg is {}", msg);

    let num = 90;
    match num {
        80 => println!("num is 80"),
        90 => println!("num is 90"),
        _ => println!("other num"),
    }

    match num {
        10..=50 => println!("num is 10 to 50"),
        51..=90 => println!("num is 51 to 90"),
        _ => println!("other num"),
    }

    match num {
        10 | 50 | 60 => println!("num is 10, 50, or 60"),
        100 | 300 => println!("num is 100 or 300"),
        _ => println!("other num"), 
    }

    match num {
        x if x < 190 => println!("num is less than 190"),
        x if x > 190 => println!("num is greater than 190"),
        _ => println!("num is greater than 190"),
    }

    let res = match num {
        10 => 100,
        50 => 500,
        60 => 600,
        _ => 0,
    };
    println!("res is {}", res);
}
