#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
fn main() {
    let p1 = Point { x: 1, y: 3 };
    println!("{:?}", p1);
    let p2 = Point { x: 1.2, y: 4.5 };
    println!("{:?}", p2);
    let p3 = Point { x: "a", y: "b" };
    println!("{:?}", p3);
}
