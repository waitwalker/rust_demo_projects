fn main() {
    println!("Hello, world!");
    // let v:Vec<i32> = Vec::new();
    // let v = vec![1,3,5];
    // println!("v is {:?}", v);

    let mut v = Vec::new();
    v.push(1);
    v.push(3);
    v.push(5);

    println!("v is {:?}", v);

    let v0 = &v[0];
    println!("v0 is {}", v0);

    let v1 = v.get(1);
    match v1 {
        Some(x) => println!("v1 is {}", x),
        _ => println!("v1 is not 1"),
    }

    for element in &v {
        println!("current element is {}", element);
    }

    for index in 0..v.len() {
        let current_elment = &v[index];
        println!("current element is {}", current_elment);
    }



}
