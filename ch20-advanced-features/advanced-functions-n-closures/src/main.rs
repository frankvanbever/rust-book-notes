
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// fn returns_closure() -> impl Fn(i32) -> i32 {
//     |x| x + 1
// }

// fn returns_initialized_closure(init: i32) -> impl Fn(i32) -> i32 {
//     move |x| x + init
// }

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32>{
    Box::new(move |x| x + init)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");

    let list_of_numbers = vec![1, 2, 3];
    // let list_of_strings: Vec<String> =
    //     list_of_numbers.iter().map(|i| i.to_string()).collect();

    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();

    println!("List of strings: {:?}", list_of_strings);

    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    println!("List of statuses: {:?}", list_of_statuses);

    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
    for handler in handlers {
        let output = handler(5);
        println!("{output}");
    }
}
