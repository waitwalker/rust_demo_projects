struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    println!("Hello, world!");

    // 实例化结构体
    let user1 = User {
        username: String::from("LiKai"),
        email: String::from("zhang@gmail.com"),
        sign_in_count: 456,
        active: true,
    };

    // 访问结构体
}
