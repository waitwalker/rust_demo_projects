
#[derive(Debug)]
enum Color {
    Red,
    Yellow,
    Blue,
    Green,
}

fn print_color(my_color: Color) {
    match my_color {
        Color::Red => println!("current color is Red:{:?}", my_color),
        Color::Yellow => println!("current color is Yellow:{:?}", my_color),
        Color::Blue => println!("current color is Blue:{:?}", my_color),
        _ => println!("other color:{:?}", my_color),
    }
}
fn real_color(color:&Color) {
    match color {
        Color::Red => println!("current color is Red:{:?}", color),
        Color::Blue => println!("current color is Blue:{:?}", color),
        _ => println!("other color is:{:?}", color),
    }
}

impl Color {
    fn color_name(&self) -> &'static str {
        match self {
            Color::Red =>"Red",
            Color::Yellow => "Yellow",
            Color::Blue => "Blue",
            Color::Green => "Green",
        }
    }
}

#[derive(Debug)]
enum BuildingLocation {
    Number(i32),
    Type(String),
    Unknown,
}

impl BuildingLocation {
    fn building_location_name(&self) -> &'static str {
        match self {
            BuildingLocation::Number(num_a) => {
                let num_b = *num_a;
                if num_b > 10 {
                    println!("current num is {}", num_b);
                    "num greater than 10"
                } else {
                    "num less than 10"
                }
            },
            BuildingLocation::Type(type_name)=> {
                if type_name == "" {
                    "type name is empty"
                } else {
                    "type name is {type_name}"
                }
            },
            BuildingLocation::Unknown => "unknown",
        }
    }
}

fn main() {
    println!("Hello, world!");
    print_color(Color::Red);
    print_color(Color::Yellow);
    print_color(Color::Blue);
    print_color(Color::Green);

    let current_color = Color::Red;
    real_color(&current_color);
    println!("current color is:{:?}", current_color);

    println!("current color name is {}", current_color.color_name());

    let building_location = BuildingLocation::Number(20);
    println!("current building location name is {:?}",building_location.building_location_name());
}
