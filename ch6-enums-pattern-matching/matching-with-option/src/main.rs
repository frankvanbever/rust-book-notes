
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}

fn main() {

    let five = Some(5);
    let six = plus_one(five);
    println!("{val}", val=six.unwrap_or(0));

    let none = plus_one(None);
    println!("{val}", val=none.unwrap_or(0));
}
