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
    let absent_number:Option<i32> = None;

    let x:i32 = 5;
    let y:Option<i32> = Some(6);
    let sum = x + y.unwrap();
    
}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6,
}

impl IpAddrKind {
    fn call(&self) -> (){
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
