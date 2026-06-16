trait Greeter {
    fn greet(&self);
    fn hello() {
        println!("hello");
    }
}

struct Person {
    name: String,
}

impl Greeter for Person {
    fn greet(&self) {
        println!("{:?}", self.name);
    }
}

fn main() {
    println!("Hello, world!");

    let person = Person {
        name: "Tom".to_owned(),
    };
    person.greet();
    Person::hello();
}
