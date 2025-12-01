use std::slice;

unsafe fn dangerous() {
    println!("Unsafe!")
}

unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

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

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    println!("Absolute value of -3 according to C: {}", abs(-3));

    // Accessing or Modifygin a Mutable Static Variable
    println!("value is: {HELLO_WORLD}");


    unsafe {
        // SAFETY: This is only called from a single thread in `main`.
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    //(&mut values[..mid], &mut values[mid..])
    // This fails because we're borrowing from the same slice twice. This is OK
    // in this case because we're borrowing different parts of the slice but the
    // compiler does not know this.

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }

}


#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
