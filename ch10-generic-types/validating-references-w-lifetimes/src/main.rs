
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn main() {

    //let r;

    //{
    //     let x = 5;
    //     r = &x;
    // }

    //println!("r: {r}");

    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());

    }

    //let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}
