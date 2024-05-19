fn main() {
    if_expressions();
    multiple_conditions();
    if_in_let();
    // infinite_loop();
    return_from_loop();

    loop_label();
    conditional_while_loop();
    conditional_for_loop();
    countdown();
}

fn if_expressions() {
    let number = 7;

    println!("--- if Expressions ---");

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition as false");
    }
}

// Only a bool can be used as the result of a condition in an if epxression
// Other languages will attempt to convert non-boolean values to true, however
// this is not the case for Rust. I like this, explicit is better than implicit
// fn failing_if_expressions() {
//     let number = 3;

//     if number {
//         println!("number was three");
//     }
// }

fn multiple_conditions() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_in_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

// fn if_in_let_diff_type() {
//     let condition = true;
//     let number = if condition { 5 } else { "six" };

//     println!("The value of number is : {number}");
// }
//
// ⬆️ The values in the if condition need to all be of the same type. If they're
// not the compiler will complain

//⬇️ This function is OK, but I want to be able to add functions after this one
// so it's commented out.
// fn infinite_loop() {
//     loop {
//         println!("again!");
//     }
// }

fn return_from_loop() {
    let mut  counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

fn conditional_while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF");
}

fn conditional_for_loop() {
    let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }
    for element in a {
        println!("the value is: {element}");
    }
}

fn countdown() {
    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");
}
