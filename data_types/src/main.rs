use std::io;

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess is {guess}");


    // Decimal numbers can use underscore as a separator
    let decimal = 98_222;
    println!("decimal with separator is: {decimal}");

    // Hex is supported
    let hex = 0xff;
    println!("hex number is: {hex}");


    // As is octal
    let octal = 0o77;
    println!("octal number is: {octal}");

    // Also binary
    let binary = 0b1111_0000;
    println!("binary number is: {binary}");

    // and Byte
    let byte = b'A';
    println!("byte is: {byte}");

    // When compiling in debug mode Rust causes programs to panic.
    // let foo: u8 = 0xff;
    // let bar = foo + 1;
    // println!("bar is: {bar}");
    // ^ This code does not compile even, the compiler is smart

    // Most architectures have f64 as default. There's also f32.
    let x = 2.0; // f64 double precision
    let y: f32 = 3.0; //f32 single precision

    println!("x is: {x} -- y is: {y}");

    operations();
    boolean();
    characters();
    tuple_type();
    array_type();
    invalid_array_element_access();
}

fn operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1;

    // remainder
    let remainder = 43 % 5;

    println!("sum: {sum} difference: {difference}");
    println!("product: {product} quotient: {quotient}");
    println!("truncated: {truncated} remainder: {remainder}");
}


fn boolean() {
    let t = true;
    let f : bool = false;

    println!("t is {t} -- f is {f}");
}

fn characters() {
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{c} - {z} - {heart_eyed_cat}");
}


fn tuple_type() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_,y,z) = tup;

    let x = tup.0;

    println!("The value of y is: {y}");
    println!("The value of x is: {x}");
    println!("The value of z is: {z}");
}

fn array_type() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let b = [3,5];

    let foo = a[0];

    println!("foo: {foo}");

    let months = ["January", "February", "March", "April",
    "May", "June", "July", "August", "September", "October",
                  "November", "December"];

    for month in &months {
        println!("{month}");
    }


    let bar = months[11];
    println!("bar is {bar}");
}

fn invalid_array_element_access() {
    let a = [1 ,2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
