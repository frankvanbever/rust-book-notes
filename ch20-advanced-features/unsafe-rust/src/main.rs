
unsafe fn dangerous() {
    println!("Unsafe!")
}

fn main() {

    // Dereferencing a Raw Pointer
    let mut num = 5;
    let r1 = &raw const num;
    let r2 = &raw mut num;

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    // calling an unsafe function or method
    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 3, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
