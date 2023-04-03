struct User {
    active: bool,
    username: String,
    sign_in_count: u32,
}

// Tuple struct
#[derive(Debug)]
struct Coordinates(i32, i32, i32);

// Unit struct
struct UnitStruct;

struct Square {
    width: u32,
    height: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn find_width(&self) -> u32 {
        self.width
    }

    fn change_width(&mut self, new_width: u32, new_height: u32) {
        self.width = new_width;
        self.height = new_height;
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("Tyler"),
        sign_in_count: 0,
    };

    println!(
        "Hello my username is {} and my status is {} with sign in count {}",
        user1.username, user1.active, user1.sign_in_count
    );

    let user2 = build_user(String::from("Alex"));

    println!(
        "Hello my username is {} and my status is {} with sign in count {}",
        user2.username, user2.active, user2.sign_in_count
    );
    let coords = Coordinates(1, 2, 3);
    println!("The coordinate is {:?}", coords);

    let mut square1 = Square {
        width: 10,
        height: 10,
    };
    println!(
        "The square's width is {} & height is {}",
        square1.width, square1.height
    );

    println!("The area of square is {}", square1.area());
    println!("The width of square is {}", square1.find_width());
    square1.change_width(11, 11);
    println!("The width of square is {}", square1.find_width());
}

fn build_user(username: String) -> User {
    User {
        username,
        active: true,
        sign_in_count: 1,
    }
}
