use std::slice;

fn main() {
    // Unsafe1: Raw Pointers
    ex_unsafe1_raw_pointers();

    // Unsafe2: Calling an Unsafe Function or Method
    ex_unsafe2_unsafe_func();

    // Unsafe3: Static Variable
    ex_unsafe3_static_var();

    // Ex: split_at_mut()
    ex_split_at_mut();

    // Ex: Using extern Functions to Call External Code
    ex_extern_functions();
}

// Unsafe1: Raw Pointers
fn ex_unsafe1_raw_pointers() {
    let mut n = 5;

    let p1 = &n as *const i32;
    let p2 = &mut n as *mut i32;

    unsafe {
        println!("p1: {}", *p1);
        println!("p2: {}", *p2);

        *p2 += 1;

        println!("p1: {}", *p1);
        println!("p2: {}", *p2);
    }
}

// Unsafe2: Calling an Unsafe Function or Method
fn ex_unsafe2_unsafe_func() {
    // error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
    //dangerous();

    unsafe {
        dangerous();
    }
}

unsafe fn dangerous() {}

// Unsafe3: Static Variable
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn ex_unsafe3_static_var() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// Ex: split_at_mut()
fn ex_split_at_mut() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let (l, r) = split_at_mut(&mut v, 3);

    for (i, x) in l.iter_mut().enumerate() {
        println!("left[{}]: {}", i, x);
    }
    for (i, x) in r.iter_mut().enumerate() {
        println!("right[{}]: {}", i, x);
    }
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

// Ex: Using extern Functions to Call External Code
fn ex_extern_functions() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

extern "C" {
    fn abs(x: i32) -> i32;
}
