
fn main() {
    //println!("Hello, world!");

    //let mut v: Vec<i32> = Vec::new();
    //v.push(5);
    //v.push(6);
    //v.push(7);
    //v.push(8);

    //let v2 = vec![1, 2, 3];

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // This fails at runtime
    //let _does_not_exist = &v[100];
    //let _does_not_exist2 = v.get(100);


    // This is not allowed because the push might cause the memory
    // to reallocate. The push creates a mutable borrow after the
    // immutable borrow to first. This is not allowed.
    //let first = &v[0];
    //v.push(6);

    //println!("The first element is: {first}");

    // Iterating over the Values in a Vector

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }

    // Using an Enum to Store Multiple Types

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // When a vector goes out of scope it is automatically dropped.
    // This means that the contents of the vector also get dropped.
    // The borrow checker ensures that any references to vector
    // contents are only used when the vector itself is valid
}
