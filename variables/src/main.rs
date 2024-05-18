
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    mutability();
    constants();
    shadowing();
}

//  variables are immutable by default.
//  Adding `mut` to the variable definition creates a mutatable variable
fn mutability() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

//  Constants are defined with the const keyword.
fn constants() {
    println!("The value of const THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
}


// You can shadow an extisting variable. This is bound to the current scope.
//
fn shadowing() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("Teh value of x is: {x}");

    let spaces = "    ";
    let spaces = spaces.len();

    // You can't change the type of a variable willy nilly
    // let mut spaces = "    ";
    // spaces = spaces.len();

    println!("The value of spaces is: {spaces}");
}
