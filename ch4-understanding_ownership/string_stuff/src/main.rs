fn main() {
    string_stuff();
    interacting_with_move();
    interatcting_with_clone();
}

fn string_stuff() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); //
}



fn interacting_with_move() {
    let x = 5;
    let _y = x;

    let s1 = String::from("hello");
    let _s2 = s1;

    // println!("{s1}, world"); //This is not allowed because s1 was moved to s2
    // i.e. the reference was invalidated
}

fn interatcting_with_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}
