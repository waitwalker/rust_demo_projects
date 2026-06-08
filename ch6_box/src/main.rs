struct Point {
    x: i32,
    y: i32,
}

fn main() {
    println!("Hello, world!");

    
    // 将数据存储在堆上
    let boxed_point = Box::new(Point { x: 10, y: 20 });
    println!("x:{}, y:{}", boxed_point.x, boxed_point.y);

    let mut boxed_data = Box::new(32);
    println!("{}", boxed_data);
    // 解引用获取堆上的数据值
    println!("{}", *boxed_data);
    
    *boxed_data += 100;
    println!("current data:{}", *boxed_data);

    
}
