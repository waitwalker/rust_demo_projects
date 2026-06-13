struct MyString<'a> {
    text:&'a str, // 不要这么写，纯粹演示时用， 直接用String
}

impl<'a> MyString<'a> {
    fn get_length(&self) -> usize {
        self.text.len()
    }

    fn modify_data(&mut self) {
        self.text = "world";
    }
}

struct StringHolder {
    data:String,
}

impl StringHolder {
    fn get_length(&self) -> usize {
        self.data.len()
    }

    fn get_reference<'a>(&'a self) -> &'a String {
        &self.data
    }

    fn get_ref(&self) -> &String {
        &self.data
    }
}

fn main() {
    println!("Hello, world!");
    let str1 = String::from("hello");
    let mut x = MyString {
        text:str1.as_str(),
    };
    x.modify_data();
    println!("{}",x.text);

    let mut holder = StringHolder {
        data:String::from("Hello"),
    };

    println!("{}",holder.get_length());
    println!("{}",holder.get_reference());
    println!("{}",holder.get_ref());

}
