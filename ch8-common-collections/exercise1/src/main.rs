use std::collections::HashMap;

// given a list of integers, use a vector and return the median
// and mode (value that occurs most often)  of the list

fn main() {
    println!("Hello, world!");

    let mut v = vec![1, 2, 3, 4, 5, 5, 5];

    v.sort();

    let len = v.len()/2;

    let mean: Option<&i32> = v.get(len);

    match mean {
        Some(mean) => println!("Mean is: {mean}"),
        None => println!("There is no mean?!?"),
    }

    let mut intmap = HashMap::new();

    for i in v {
        let count = intmap.entry(i).or_insert(0);
        *count += 1;
    }

    let mut mode = -1;

    for (key, _value) in &intmap {
        if *key > mode {
            mode = *key;
        }
    }

    println!("Mode is: {mode}");
}
