use std::{fmt::format, ops::Add};

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

trait Overview {
    fn overview(&self) -> String {
        String::from("This is rust course")
    }
}

struct Course {
    headline: String,
    author: String,
}

impl Drop for Course {
    fn drop(&mut self) {
        println!("Dropping: {}", self.author);
    }
}

struct AnotherCourse {
    headline: String,
    author: String,
}

impl Overview for Course {
    fn overview(&self) -> String {
        format!("{} {}", self.author, self.headline)
    }
}

impl Overview for AnotherCourse {
    fn overview(&self) -> String {
        format!("{} {}", self.author, self.headline)
    }
}
#[derive(Debug)]
struct Points<T> {
    x: T,
    y: T,
}

impl<T> Add for Points<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Points {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
fn main() {
    let p1 = Point { x: 1, y: 3 };
    println!("{:?}", p1);
    let p2 = Point { x: 1.2, y: 4.5 };
    println!("{:?}", p2);
    let p3 = Point { x: "a", y: "b" };
    println!("{:?}", p3);

    let course1 = Course {
        headline: String::from("Headline 1"),
        author: String::from("Alex"),
    };
    println!("{}", course1.overview());
    let course2 = AnotherCourse {
        headline: String::from("Headline 2"),
        author: String::from("Bob"),
    };
    println!("{}", course2.overview());
    call_overview(&course1);
    call_overview(&course2);
    call_overview_simplified(&course2);

    drop(course1);

    let coord_1 = Points { x: 5.0, y: 5.0 };
    let coord_2 = Points { x: 1.0, y: 5.0 };
    let sum = coord_1 + coord_2;
    println!("The operator overloading example {:?}", sum)
}

fn call_overview(item: &impl Overview) {
    println!("Overview: {}", item.overview())
}

fn call_overview_simplified<T: Overview>(item: &T) {
    println!("Overview: {}", item.overview())
}
/*
    ******************* Multiple Trait Bound *******************
    fn overview(item1: &impl Overview, item2: &impl Overview)
    fn overview<T: Overview)(item1: &T, item2: &T)

    fn overview(item1: &impl Overview+ AnotherTrait)
    fn overview<T:Overview+ AnotherTrait>(item1: &T, item2:&T)
*/
