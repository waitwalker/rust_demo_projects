fn main() {
    println!("Hello, world!");

    let num_list = [12,45,67,89,23,34,56,78,90,11];
    let mut max = num_list[0];

    for cur in num_list {
        if cur > max {
            max = cur;
        }
    }

    println!("max num is {}", max);

    let max = largest(&num_list);
    println!("max num is {}", max);
    // 泛型是具体类型或者其他属性的抽象代替，其实就是一种模板，里面有一些占位符，编译器在编译时将占位符替换为具体的类型
}

fn largest(list: &[i32]) -> i32 {
    let mut max = list[0];

    // 结构，模式匹配
    for &cur in list {
        if cur > max {
            max = cur; // 或者 max = *cur; // 解引用
        }
    }
    max
}

struct Point<T,U> {
    x: T,
    y: U,
}

