fn sum_with_loop(arr: &[i64]) -> i64 {
    let mut sum = 0;
    for &item in arr {
        sum += item;
    }
    sum
}

fn sum_with_iter(arr: &[i64]) -> i64 {
    arr.iter().sum()
}

fn main() {
    println!("Hello, world!");

    const ARRAY_SIZE: usize = 1_000_000;
    let array: Vec<i64> = (1..=ARRAY_SIZE as i64).collect();
    let sum1 = sum_with_loop(&array);
    let sum2 = sum_with_iter(&array);
    println!("sum loop : {}", sum1);
    println!("sum iter : {}", sum2);
}
