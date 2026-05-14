fn main() {
    println!("Hello, world!");
    let tup = (0,"hi",3.4);
    let str = "zhangsan";

    println!("tup elements {} {} {}",tup.0,tup.1,tup.2);
    println!("str is {str}");

    let mut tup2 = (100,30.0,"ok");
    println!("tup2 elements {} {} {}", tup2.0, tup2.1, tup2.2);
    tup2.1 = 80.0;
    println!("tup2 elements {} {} {}", tup2.0, tup2.1, tup2.2);

    let arr:[i32;4] = [1,2,3,4];
    println!("arr elements {} {} {} {}", arr[0], arr[1], arr[2], arr[3]);
    
    for elment in arr {
        println!("current element {elment}");
    }

    let mut arr2 = [5,6,7,8];
    arr2[1] = 90;
    for elment in arr2 {
        println!("arr2 current element {elment}");
    }

}
