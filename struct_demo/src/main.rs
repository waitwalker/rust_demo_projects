struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    println!("Hello, world!");

    // 实例化结构体
    let mut user1 = User {
        username: String::from("LiKai"),
        email: String::from("zhang@gmail.com"),
        sign_in_count: 456,
        active: true,
    };

    // 访问结构体
    println!(
        "username:{}, email:{}, sign_in_count:{}, activie:{}",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );

    // 重新赋值,struct 必须可变，才能赋值
    user1.email = String::from("123456@gmail.com");
    println!("username:{}, email:{}, sign_in_count:{}, activie:{}",user1.username,user1.email,user1.sign_in_count,user1.active);
}
