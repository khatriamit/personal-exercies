use std::rc::Rc;

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

    println!("{:?}", y);

    let s1 = Rc::new(String::from("Pointer"));
    let s2 = s1.clone();
    let s3 = s2.clone();
    let s4 = s3.clone();

    println!("{s1} {s2} {s3} {s4}");
}
