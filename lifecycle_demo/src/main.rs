fn main() {
    println!("Hello, world!");

    {
        let x = 5;
        let r;
        {
            
            r = &x;
        }
        println!("r: {}", r);
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(&string1.as_str(), string2);
    println!("The longest string is {}", result);
    
}

// 生命周期的标注
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longestX<'a>(x:&'a str, y:&'a str) -> String {
    let result = String::from("value");
    result
}

// 生命周期标注以' 开头，标注的位置在引用符号&后面，使用空格将标注和引用类型分开
// 使用空格将标注和引用类型分开
fn add<'a>(str1:&'a str, str2:&'a str) {

}

fn name<'a>(str:&'a str) -> &'a str {
    str + "hello".to_string()
}

fn area<'a>(str:&'a str) {}

fn minus<'a>(num1: &'a mut i32, num2:&'a mut i32) -> &'a mut i32 {
    num1 + num2
}

struct ImportantExcerpt<'a> {
    part:&'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32{
        3
    }

    fn announce_and_return_part(&self, announcement:&str) -> &str {
        println!("Announcement! {}", announcement);
        self.part
    }
}
