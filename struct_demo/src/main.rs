struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
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
    println!(
        "username:{}, email:{}, sign_in_count:{}, activie:{}",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );

    let mut user2 = build_user(String::from("LiKai"), String::from("zhang@gmail.com"));
    println!(
        "username:{}, email:{}, sign_in_count:{}, activie:{}",
        user2.username, user2.email, user2.sign_in_count, user2.active
    );

    user2.email = String::from("126@gmail.com");
    println!(
        "username:{}, email:{}, sign_in_count:{}, activie:{}",
        user2.username, user2.email, user2.sign_in_count, user2.active
    );

    let mut user3 = build_user_from_user(String::from("zhaz"), user2);
    println!(
        "username:{}, email:{}, sign_in_count:{}, activie:{}",
        user3.username, user3.email, user3.sign_in_count, user3.active
    );

    // tuple color
    let red = Color(255, 0, 0);

    println!("red:{},{},{}", red.0, red.1, red.2);

    let w = 30;
    let h = 50;

    println!("area:{}", area(w, h));

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("area:{:?}", rect);

    let rect2 = Rectangle2 {
        width:30,
        height:60,
    };

    println!("rect2 area:{}",rect2.area());
    println!("rect2 area:{:?}",rect2);


    let rect_a = Rectangle2 {
        width:15,
        height:18,
    };

    let rect_b = Rectangle2 {
        width:20,
        height:20,
    };

    let rect_c = Rectangle2 {
        width:30,
        height:20,
    };

    println!("rect_a can hold rect_b:{}",rect_a.can_hold(&rect_b));
    println!("rect_c can hold rect_a:{}",rect_c.can_hold(&rect_a));

    let rect_d = Rectangle2::new(10,200);
    println!("rect_d:{:?}",rect_d);
}

// 字段名和参数名一样时，可以简写
fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn build_user_from_user(username: String, user: User) -> User {
    User { username, ..user }
}

struct Color(i32, i32, i32);

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle2 {
    width:u32,
    height:u32,
}

impl Rectangle2 {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle2) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn new(width:u32, height:u32) -> Self {
        Self {
            width,
            height,
        }
    }
}
