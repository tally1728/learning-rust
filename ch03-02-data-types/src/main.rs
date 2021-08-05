fn main() {
    // Integer Types
    let x = 111_222_333;
    println!("111_222_333 (Dicimal): {}", x);
    let x = 0xff_ff;
    println!("0xff_ff (Hex): {}", x);
    let x = 0o77;
    println!("0o77 (Octal): {}", x);
    let x = 0b1111_1111;
    println!("0b1111_1111 (Binary): {}", x);
    let x = b'1';
    println!("b'1' (Byte): {}", x);

    // Floating-Point Types
    let x = 2.0;
    println!("2.0 (Floating-Point): {}", x);

    // Numeric Operations
    let x = 2 + 5;
    println!("2 + 5 = {}", x);
    let x = 2 - 5;
    println!("2 - 5 = {}", x);
    let x = 2 * 5;
    println!("2 * 5 = {}", x);
    let x = 5 / 2;
    println!("5 / 2 = {}", x);
    let x = 5 % 2;
    println!("5 % 2 = {}", x);
    let x = 5.0 / 2.0;
    println!("5.0 / 2.0 = {}", x);

    // Character Type
    let x = 'X';
    println!("'X': {}", x);

    // Tuple Type
    let tup: (i32, f64, char) = (500, 6.4, 'A');
    let (x, y, z) = tup; // destructure
    println!("1st element of Tuple (500, 6.4, 'A'): {}", x);
    println!("2nd element of Tuple (500, 6.4, 'A'): {}", y);
    println!("3rd element of Tuple (500, 6.4, 'A'): {}", z);
    // index
    println!("Tuple.0: {}", tup.0);
    println!("Tuple.1: {}", tup.1);
    println!("Tuple.2: {}", tup.2);

    // Array Type
    let a = [0, 1, 2];
    println!("a[0]: {}", a[0]);
    println!("a[2]: {}", a[2]);
}
