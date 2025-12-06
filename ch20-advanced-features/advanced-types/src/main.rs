


fn main() {

    // Creating Type Sysnonyms with Type Aliases
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // The main use case for Type Synonyms is to prevent lengthy types
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    // fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    //     // --snip
    // }

    // fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    //     let foo: Box<dyn Fn() + Send + 'static>;
    // }

    impl<T> Option<T> {
        pub fn unwrap(self) -> T {
            match self {
                Some(val) => val

            }
        }
    }
}
