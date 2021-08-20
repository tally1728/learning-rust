fn main() {
    // Example 1: Enum Variants w/o value
    example_1();

    // Example 2: Enum Variants w/ value
    example_2();

    // Example 3: Enum Method
    example_3();

    // Example 4: Option Enum
    example_4();
}

// Example 1: Enum Variants w/o value
fn example_1() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // Enum
    enum IpAddrKind {
        V4,
        V6,
    }

    // Struct with Enum
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
}

// Example 2: Enum Variants w/ value
fn example_2() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Example 3: Enum Method
fn example_3() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("method call of {:?}", self);
    }
}

// Example 4: Option Enum
fn example_4() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<u32> = None;

    // error[E0277]: cannot add `Option<{integer}>` to `{integer}
    //5 + some_number
}
