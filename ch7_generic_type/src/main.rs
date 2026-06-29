#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    fn get_coordinates(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
}

fn main() {
    println!("Hello, world!");

    let p1 = Point {
        x: String::from("10"),
        y: String::from("20"),
    };
    println!("{:?}", p1);

    let p2: Point<f64> = Point { x: 100.2, y: 12.6 };

    println!("{:?}", p2);

    println!("{:#?}", swap(0, 1));

    let str1 = swap("a", "b");
    println!("{:?}", str1);

    let i32_point = Point::new(10, 20);
    let f64_point = Point::new(2.0,2.0);
    let (x1, y1) = i32_point.get_coordinates();
    let (x2, y2) = f64_point.get_coordinates();
    println!("{:?}", (x1, y1));
    println!("{:?}", (x2, y2));
    
}

fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}
