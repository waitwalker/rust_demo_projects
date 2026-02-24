use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("张三"), 100);
    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let new_scores: HashMap<String, i32> = teams.iter().zip(initial_scores.iter()).collect();
    println!("new scores {:?}", new_scores);
}
