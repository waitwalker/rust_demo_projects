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

    let mut user2 = build_user(String::from("LiKai"), String::from("zhang@gmail.com"));
    println!("username:{}, email:{}, sign_in_count:{}, activie:{}",user2.username,user2.email,user2.sign_in_count,user2.active);

    user2.email = String::from("126@gmail.com");
    println!("username:{}, email:{}, sign_in_count:{}, activie:{}",user2.username,user2.email,user2.sign_in_count,user2.active);

    let mut user3 = build_user_from_user(String::from("zhaz"),user2);
    println!("username:{}, email:{}, sign_in_count:{}, activie:{}",user3.username,user3.email,user3.sign_in_count,user3.active);
}

// 字段名和参数名一样时，可以简写
fn build_user(username:String,email:String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn build_user_from_user(username:String, user:User) -> User {
    User {
        username,
        ..user
    }
}
