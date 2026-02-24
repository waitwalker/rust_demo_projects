use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("张三"), 100);
    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut new_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("new scores {:?}", new_scores);
    let team_name = String::from("Blue");

    let score = new_scores.get(&team_name);

    if let Some(x) = score {
        println!("score: {}", x);
    } else {
        println!("score: {}", 0);
    }

    for (key, value) in &new_scores {
        println!("{}: {}", key, value);
    }

    let mut class: HashMap<String, String> = HashMap::new();
    class.insert(String::from("张三"), String::from("100"));
    class.insert(String::from("李四"), String::from("90"));
    class.insert(String::from("王五"), String::from("80"));
    println!("{:?}", class);

    class.insert(String::from("张三"), String::from("200"));
    for (key, value) in &class {
        println!("key {}: value{}", key, value);
    }

    class
        .entry(String::from("张三"))
        .or_insert(String::from("12000"));
    class.entry(String::from("忘记")).or_insert(String::from("09876"));

    for (key, value) in &class {
        println!("key {}: value {}", key, value);
    }
}
