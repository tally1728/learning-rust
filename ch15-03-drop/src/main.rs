fn main() {
    // Listing 15-14: A CustomSmartPointer struct that implements the Drop trait where we would put our
    // cleanup code
    ex_15_14();

    // Listing 15-16: Calling std::mem::drop to explicitly drop a value before it goes out of scope
    ex_15_16();
}

// Listing 15-14: A CustomSmartPointer struct that implements the Drop trait where we would put our
// cleanup code
fn ex_15_14() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// Listing 15-16: Calling std::mem::drop to explicitly drop a value before it goes out of scope
fn ex_15_16() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
