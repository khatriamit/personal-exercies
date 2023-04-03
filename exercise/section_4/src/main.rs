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
    println!("The coordinate is {:?}", coords)
}

fn build_user(username: String) -> User {
    User {
        username,
        active: true,
        sign_in_count: 1,
    }
}
