fn main() {
    // Listing 8-1 & 8-2: generate new vectors
    example_new_vectors();

    // Listing 8-3: push vectors
    example_push();

    // Listing 8-5: access elements
    example_read();

    // Listing 8-6: access out of bounds
    example_access_out_of_bounds();

    // Listing 8-7: borrow checker
    example_borrow_checker();

    // Listing 8-8 & 8-9: for loop
    example_for_loop();

    // Listing 8-10: vector of enum
    example_enum_vector();
}

// Listing 8-1 & 8-2: generate new vectors
fn example_new_vectors() {
    let v1: Vec<i32> = Vec::new();
    println!("v1: {:?}", v1);

    let v2 = vec![1, 2, 3];
    println!("v2: {:?}", v2);
}

// Listing 8-3: push vectors
fn example_push() {
    // w/o type annotation!!
    let mut v = Vec::new();
    println!("v: {:?}", v);

    // infer type from values
    v.push(1);
    println!("v: {:?}", v);

    v.push(2);
    println!("v: {:?}", v);
}

// Listing 8-5: access elements
fn example_read() {
    let v = vec![1, 2, 3, 4, 5];

    let e = &v[1]; // &i32
    println!("&v[1]: {}", e);

    let e = v.get(2); // Option<&i32>
    println!("v.get(2): {:?}", e);
}

// Listing 8-6: access out of bounds
fn example_access_out_of_bounds() {
    let v = vec![1, 2, 3, 4, 5];

    // runtime error: index out of bound
    //let e = &v[100];
    //println!("&v[100]: {}", e);

    let e = v.get(100); // Option<&i32>
    println!("v.get(100): {:?}", e);
}

// Listing 8-7: borrow checker
fn example_borrow_checker() {
    let mut v = vec![1, 2, 3, 4, 5];
    println!("v: {:?}", v);

    // immutable borrow occurs here
    let e = &v[0];

    // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    //v.push(6);

    // immutable borrow later used here
    println!("&v[0]: {}", e);
}

// Listing 8-8 & 8-9: for loop
fn example_for_loop() {
    let mut v = vec![100, 32, 57];

    // immutable
    for e in &v {
        println!("{}", e);
    }

    // mutable
    for e in &mut v {
        *e += 10; // dereference with * operator
    }
    println!("v: {:?}", v);
}

// Listing 8-10: vector of enum
fn example_enum_vector() {
    #[derive(Debug)]
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
    println!("row: {:?}", row);
}
