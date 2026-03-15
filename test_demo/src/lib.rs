
struct Rectangle {
    width:u32,
    height:u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // #[test]
    // fn test_area() {
    //     let rect = Rectangle {
    //         width: 10,
    //         height: 20,
    //     };
    //     assert_eq!(rect.area(), 200);
    // }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2,2));
    }

    #[test]
    fn add_two_and_three() {
        assert_eq!(5, add_two(2,3));
    }
    
    #[test]
    #[ignore = "先不要测试"]
    fn one_hundred() {
        assert_eq!(100, add_two(5,98));
    }
}

fn add_two(a:i32,b:i32) -> i32 {
    a + b
}

#[cfg(test)]
mod testx {
    use super::*;
    #[test]
    fn test_area() {
        let rect = Rectangle {
            width:10,
            height:30,
        };
        assert_eq!(rect.area(),100,"测试未通过");
    }
}
