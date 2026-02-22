fn main() {
    println!("Hello, world!");

    let v4 = IpAddrKind::V4(127, 0, 0, 1);
    let v6 = IpAddrKind::V6;
    route(v4);
    route(v6);
    let v4 = IpAddrKind::V4(127, 0, 0, 1);
    v4.call();

    let some_number = Some(12);
    let some_string = Some("fsdf");
    let absent_number: Option<i32> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(6);
    let sum = x + y.unwrap();

    let coinUsAlabama = Coin::Quarter(UsState::Alabama);
    let value = value_in_cents(coinUsAlabama);
    println!("{:?}",value);
}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6,
}

impl IpAddrKind {
    fn call(&self) -> () {
        println!("{:?}", self);
    }
}

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4(_, _, _, _) => {
            println!("{:?}", ip_kind);
        }
        IpAddrKind::V6 => {
            println!("{:?}", ip_kind);
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state)=>{
            match state {
                UsState::Alaska => {
                    println!("Alaska");
                    25
                },
                UsState::Alabama => {
                    println!("Alabama");
                    25
                },
            }
        }
    }
}
