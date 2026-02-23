mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add to waitlist");
        }
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // 相对路径：super相当于文件系统中的..
        super::serve_order();
        // 绝对路径
        crate::serve_order();
    }

    fn cook_order() {
        println!("cook order");
    }

    pub struct Breakfast {
        pub toast:String,
        seasonal_fruit:String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast:String::from(toast),
                seasonal_fruit:String::from("apples"),
            }
        }
    }
}

pub fn eat_at_restauranta() {
    let mut meal = back_of_house::Breakfast::summer("wheat");
    meal.toast = String::from("rice");
    println!("meal is {}",meal.toast);
    // meal.seasonal_fruit = String::from("oranges");
}

