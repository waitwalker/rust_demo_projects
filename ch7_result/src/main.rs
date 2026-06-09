fn divide(a: i32, b: i32) -> Result<f64, String> {
    if b == 0 {
        return Err(String::from("b cannot be zero"));
    }
    let a = a as f64;
    let b = b as f64;
    Ok(a / b)
}

fn find_element(array: &[i32], target: i32) -> Option<usize> {
    for (index, element) in array.iter().enumerate() {
        if (*element) == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    println!("Hello, world!");

    match divide(10, 2) {
        Ok(value) => println!("a / b value is {}", value),
        Err(error) => println!("a / b encounter error: {}", error),
    }

    match divide(10, 0) {
        Ok(value) => println!("a / b value is {}", value),
        Err(error) => println!("a / b encounter error: {}", error),
    }

    let arr = [1,2,3,4,5];
    match find_element(&arr, 4) {
        Some(index) => println!("find element 4 at index {}", index),
        None => println!("cannot find element 4"),
    }

    match find_element(&arr, 8) {
        Some(index) => println!("find element 8 at index {}", index),
        None => println!("cannot find element 8"),
    }

    let vec = vec![1,3,4,5,6];
    vec[12];
}
