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

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", six);
    println!("{:?}", none);

    what_pet("Dog");
    what_pet("Cat");
    what_pet("Cow");

    let dog1 = Some(Pets::Dog);
    if let Some(Pets::Dog) = dog1 {
        println!("The animal is a DOG!")
    } else {
        println!("Not a DOG")
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn what_pet(input: &str) {
    match input {
        "Dog" => println!("I have a dog!"),
        "Cat" => println!("I have a cat!"),
        "Fish" => println!("I have a fish!"),
        _ => println!("I have no clue what pet you have"),
    }
}
