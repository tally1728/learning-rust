fn main() {
    // Stack
    assign_to_another_on_stack_1();
    assign_to_another_on_stack_2();

    // Heap
    mutable_string();
    assign_to_another_2();
    clone_string();

    // Function & Ownership
    function_and_ownership_1();
    function_and_ownership_2();
}

////////////////////////////////////////////////////////////
// Stack
fn assign_to_another_on_stack_1() {
    let x = 0;
    let mut y = x; // copied! -> 2 Values

    y += 1;

    println!("x = {}", x); // 0
    println!("y = {}", y); // 1
}

fn assign_to_another_on_stack_2() {
    let x = [0];
    let mut y = x; // copied! -> 2 Arrays

    y[0] = 1;

    println!("x[0] = {}", x[0]); // 0
    println!("y[0] = {}", y[0]); // 1
}

////////////////////////////////////////////////////////////
// Heap

// 可変な String 型（Heap に確保）の例
fn mutable_string() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn assign_to_another_2() {
    let s1 = String::from("hello");
    let s2 = s1; // NOT shallow copy, BUT Moved!

    // error[E0382]: borrow of moved value: `s1`
    //println!("s1 = {}", s1);

    // ok
    println!("s2 = {}", s2);
}

fn clone_string() {
    let s1 = String::from("hello");

    let mut s2 = s1.clone(); // deep copy
    s2.push_str(", world!");

    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
}

////////////////////////////////////////////////////////////
// Function & Ownership

fn function_and_ownership_1() {
    let s1 = String::from("hello");

    let s2 = takes_and_gives_back(s1); // Moved!

    // error[E0382]: borrow of moved value: `s1`
    //println!("s1 = {}", s1);

    println!("s2 = {}", s2);
}

fn takes_and_gives_back(s: String) -> String {
    println!("takes_and_gives_back fn: {}", s);

    s
}

fn function_and_ownership_2() {
    let s = String::from("hello");

    // gives & takes ownership... Inefficient way!
    let (s, len) = calculate_length(s);

    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
