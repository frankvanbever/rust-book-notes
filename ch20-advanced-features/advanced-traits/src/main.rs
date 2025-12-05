use std::ops::Add;
use std::fmt;

// pub trait Iterator {
//     type Item; // This is a placeholder

//     fn next(&mut self) -> Option<Self::Item>;
// }

// impl Iterator for Counter {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         // -- snip --
//     }
// }

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

// Using Newtype
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let one_meter = Meters(1);
    let five_millimeter = Millimeters(5);

    let result = five_millimeter + one_meter;

    println!("result is {}", result.0);

    let person = Human;

    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("A baby dog is called a {}", Dog::baby_name());

    // Animal doesn't have a self parameter
    //println!("A baby dog is called a {}", Animal::baby_name());

    // So the path needs to be fully qualified
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // Fully qualified syntax is
    //     <Type as Trait>::function(receiver_if_method, next_arg, ...);

    let printpoint = Point { x: 1, y: 2 };
    printpoint.outline_print();


    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
