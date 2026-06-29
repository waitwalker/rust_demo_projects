trait Overview {
    fn overview(&self) -> String {
        String::from("Course ")
    }
}

trait Another {
    fn hello(&self) {
        println!("welcome to hell")
    }
}

struct Course {
    headline:String,
    author:String,
}

struct AnotherCourse {
    headline:String,
    author:String,
}

impl Overview for Course {

}

impl Overview for AnotherCourse {}

impl Another for Course {}

fn call_overview(item: & impl Overview) {
    println!("{}",item.overview())
}

fn call_overview_x<T:Overview>(item: &T) {
    println!("{}",item.overview())
}

fn call_course(item: & (impl Overview + Another)) {
    println!("{}",item.overview());
    item.hello();
}

fn main() {
    println!("Hello, world!");
    let c1 = Course {
        headline:String::from("Rust"),
        author:String::from("Rustacean"),
    };
    call_overview(&c1);

    let c2 = AnotherCourse {
        headline:String::from("S"),
        author:String::from("D"),
    };
    call_overview_x::<AnotherCourse>(&c2);
    call_course(&c2);
}
