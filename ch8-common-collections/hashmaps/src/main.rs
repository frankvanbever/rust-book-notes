use std::collections::HashMap;
fn main() {

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //let team_name = String::from("Blue");
    //let score = scores.get(&team_name).copied().unwrap_or(0);


    for (key, value) in &scores {
        println!("{key} scored {value}");
    }

    // Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    //println!("{field_name}");
    // Doing this is invalid because the values have been moved
    // into the hashmap. This would not be an issue if we
    // inserted references but that raises other problems with
    // the validity of the lifetime.

    // Updating
    // Overwriting
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    // Adding only if a key isn't present

    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Green")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    // Updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
