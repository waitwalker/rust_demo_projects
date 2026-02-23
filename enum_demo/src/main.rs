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

    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}",six);

    let none = plus_one(None);
    println!("{:?}",none); 


    let v = 3u8;
    match v {
        0=>println!("v is 0"),
        _=>println!("v is {}",v)
    }

    if let 3 = v {
        println!("v is 3");
    }

    let some = Some(12);
    if let Some(10) = some {
        println!("value is {:?}",some);
    } else {
        println!("the value is {:?}", some);
    }
    
    
}

fn plus_one(x:Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(y) => Some(y + 1)
    }
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
