fn main() {
    let t = (12, "eggs");
    let b = Box::new(t);
    println!("{:?}", b);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("{:?}", y)
}
