fn main() {
    println!("Hello, world!");
}

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
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let rect = Rectangle {
            width: 10,
            height: 20,
        };
        assert_eq!(rect.area(), 200);
    }
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
