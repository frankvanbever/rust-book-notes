fn main() {
    println!("Hello, world!");

    another_function();
    argument_fn(5);
    print_labeled_argument(5, 'h');

    //let y = 6; // A statement does not return a value
    let y = {
        let x = 3;
        x+1
    };

    println!("The value of y is: {y}");

    let x = five();

    println!("The value of x is: {x}");

    let a = plus_one(5);
    println!("The value of a is: {a}");
}

fn another_function() {
    println!("Another function.");
}

fn argument_fn(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_argument(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}

fn five() -> i32 {
    5
}


fn plus_one(x: i32) -> i32 {
    x + 1 // putting a semicolon here turns it into a statement
}
