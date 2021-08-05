fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);
    let x = x * 2;
    println!("The value of x is: {}", x);

    // OK
    let spaces = "   ";
    let spaces = spaces.len();
    println!("length of spaces is: {}", spaces);

    // NG
    // let mut spaces = "   ";
    // spaces = spaces.len(); // error[E0308]: mismatched types
}
