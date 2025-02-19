fn main() {

    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    let s = "Initial contents".to_string();

    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    println!("{hello}");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    // Concatenation with the + Operator or the format! Macro

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2; // s1 has been moved and is invalid
    // fn add(self, s: &str) -> String

    let s_1 = String::from("tic");
    let s_2 = String::from("tac");
    let s_3 = String::from("toe");

    //let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{s_1}-{s_2}-{s_3}");

    println!("{s}");

    //let s1 = String::from("hello");
    //let h = s1[0];
    // Rust strings don't support indexing


}
