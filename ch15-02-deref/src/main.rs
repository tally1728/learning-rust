fn main() {
    // Listing 15-6: Using the dereference operator to follow a reference to an i32 value
    ex_15_6();

    // Listing 15-7: Using the dereference operator on a Box<i32>
    ex_15_7();

    // Listing 15-9: Attempting to use MyBox<T> in the same way we used references and Box<T>
    ex_15_9();

    // Listing 15-12: Calling hello with a reference to a MyBox<String> value, which works because of deref coercion
    ex_15_12();

    // Use DerefMut trait to mutate the inner value
    ex_derefmut();
}

// Listing 15-6: Using the dereference operator to follow a reference to an i32 value
fn ex_15_6() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // error[E0277]: can't compare `{integer}` with `&{integer}`
    //assert_eq!(5, y);
    assert_eq!(5, *y);
}

// Listing 15-7: Using the dereference operator on a Box<i32>
fn ex_15_7() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    // error[E0277]: can't compare `{integer}` with `Box<{integer}>`
    //assert_eq!(5, y);
    assert_eq!(5, *y);

    // try * operator
    let mut y = Box::new(x);
    *y += 1;
    println!("y: {}", y);
}

// Listing 15-9: Attempting to use MyBox<T> in the same way we used references and Box<T>
fn ex_15_9() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::{Deref, DerefMut};

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

// Listing 15-12: Calling hello with a reference to a MyBox<String> value, which works because of deref coercion
fn ex_15_12() {
    let m = MyBox::new(String::from("Rust"));
    // &MyBox<String> => &String => &str
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// Use DerefMut trait to mutate the inner value
fn ex_derefmut() {
    let mut b = MyBox::new(1);

    println!("box: {}", *b);

    *b += 1;

    println!("box: {}", *b);
}
