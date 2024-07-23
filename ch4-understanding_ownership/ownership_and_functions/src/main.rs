fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s);
    // s's value moves into the function and is no longer valid here

    println!("{s}");

    let x = 5; // x comes into scope
    makes_copy(x);
    // x would move into the function, but i32 is Copy, so it's OK to still
    // use x afterward
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
//  memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
