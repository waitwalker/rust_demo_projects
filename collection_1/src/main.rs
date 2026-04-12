use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");

    let mut vector: Vec<&str> = Vec::new();
    vector.push("你好1");
    vector.push("世界");
    vector.push("Hello");
    vector.push("World!");
    vector.push("Rust");
    println!(
        "vector:{:?}, capacity:{:?}, length:{:?}",
        vector,
        vector.capacity(),
        vector.len()
    );
    vector.remove(2);
    println!("vector:{:?}",vector);
    println!("contains:{:?}",vector.contains(&"世界"));

    // HashMap
    let mut map:HashMap<&str,u32> = HashMap::new();
    map.insert("James", 98);
    map.insert("Durant", 99);
    map.insert("Jordan", 100);
    println!("map:{:?}",map);
    match map.get(&"James") {
        Some(value)=>{
            println!("James score:{:?}",value)
        }
        None => {
            println!("James is not in range")
        }
    }

    for (key,value) in map {
        println!("key:{:?}, value:{:?}",key,value)
    }

    // HashSet
    let mut set:HashSet<&str> = HashSet::new();
    set.insert("James");
    set.insert("Durant");
    set.insert("Jordan");
    set.insert("Denk");
    set.insert("James");
    println!("set:{:?}",set);

    let removed = set.remove("Dd");
    println!("set removed:{:?}, removed:{:?}",set, removed);


    for value in set {
        println!("current value: {:?}",value)
    }
}
