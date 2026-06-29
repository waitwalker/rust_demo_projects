#[derive(Debug, Clone)]
enum Race {
    White,
    Yellow,
    Black,
}

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    race: Race,
}

impl PartialEq for Race {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Race::White, Race::White) => true,
            (Race::Yellow, Race::Yellow) => true,
            (Race::Black, Race::Black) => true,
            _ => false,
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name && self.race == other.race
    }
}

fn main() {
    println!("Hello, world!");
    let user = User {
        id: 3,
        name: String::from("T99"),
        race: Race::White,
    };
    println!("{:#?}", user);

    let user2 = user.clone();
    println!("{:#?}", user2);

    println!("user == user2:{}", user == user2);

    // let user3 = user;
    // println!("{:#?}", user3);
}
