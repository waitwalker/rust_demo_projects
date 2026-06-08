#[derive(Debug,Clone,Copy)]
struct Book{
    page:i32,
    rating:f64,
}

fn main() {
    println!("Hello, world!");
    let x = vec![1, 2, 3];
    let y = x.clone();
    println!("{:?}", y);
    println!("{:?}", x);

    let x = "ss".to_string();
    // 所有权move 转移
    let y = x.clone();
    println!("y:{:?}",y);
    println!("x:{:?}",x);

    let b1 = Book{
        page:100,
        rating:4.5,
    };

    let b2 = b1.clone();
    println!("{:?}", b2);
    // 所有权转移了，不能操作
    println!("{:?}", b1);


}
