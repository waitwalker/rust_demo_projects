fn main() {
    println!("Hello, world!");
    let today = WeekDay::Sunday;
    // println!("{:?}",today);
    println!("{:?}",WeekDay::Monday);

    let month = Month::Value(String::from("一月"));
    println!("几月：{:?}",month);
    match month {
        Month::Value(value) => {
            println!("当前月份：{:?}",value);
        }
    }
}

#[derive(Debug)]
enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
#[derive(Debug)]
enum Month {
    Value(String)
}