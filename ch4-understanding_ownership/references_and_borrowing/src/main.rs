fn main() {
    reference_borrowing();
    // modify_borrow(); // --> you're not allowed to modify a borrowed variable
    // References are immutable by default
    mutable_reference();
    // two_mutable_references();
    scope_and_references();
    // combining_mutable_and_immutable();
    // dangling_references();
}

fn reference_borrowing() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    // use a reference as the argument

    println!("The length of '{}' is {}.", s1, len);
}

// calculate_length takes a &String, a string reference
fn calculate_length(s: &String) -> usize {
    s.len()
}
// This is an example of reference borrowing.



// fn modify_borrow() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }


fn mutable_reference() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("s is {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn two_mutable_references() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s; // cannot do a second mutable borrow

//     println!("{}, {}", r1, r2);
// }


fn scope_and_references() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("r1 is {r1}");
    }

    let r2 = &mut s;

    println!("r2 is {r2}");
}

// fn combining_mutable_and_immutable() {
//     let mut s = String::from("hello");

//     let r1 = &s;
//     let r2 = &s;
//     let r3 = &mut s;

//     println!("{r1}, {r2} and {r3}");
// }
// You're not allowed to combine mutable and immutable
// references

// fn dangling_references() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
