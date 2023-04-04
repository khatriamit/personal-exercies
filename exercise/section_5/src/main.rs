enum Pets {
    Dog,
    Cat,
    Fish,
}

impl Pets {
    fn what_am_i(self) -> &'static str {
        match self {
            Pets::Cat => "I am a cat",
            Pets::Dog => "I am a dog",
            Pets::Fish => "I am a fish",
        }
    }
}

enum IpAddrKing {
    V4(String),
    V6,
}

struct IpAddr {
    kind: IpAddrKing,
    address: String,
}

fn main() {
    let dog = Pets::Dog;
    let cat = Pets::Cat;
    let fish = Pets::Fish;
    println!("{}", dog.what_am_i());
    println!("{}", cat.what_am_i());
    println!("{}", fish.what_am_i());

    let home = IpAddr {
        kind: IpAddrKing::V4(String::from("V4")),
        address: "127.0.0.1".to_owned(),
    };

    let office = IpAddr {
        kind: IpAddrKing::V6,
        address: "127.0.0.1".to_owned(),
    };

    let some_number = Some(5);
    let some_string = Some("a string");
    let nothing: Option<i32> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(5);
    let sum = x + y.expect("Some error");
}
