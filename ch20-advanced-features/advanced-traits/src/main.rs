
pub trait Iterator {
    type Item; // This is a placeholder

    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // -- snip --
    }
}


fn main() {
    println!("Hello, world!");
}
