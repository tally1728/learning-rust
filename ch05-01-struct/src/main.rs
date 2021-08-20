fn main() {
    // Define a struct
    // Instantiate
    // Field Access
    example_1();

    // the Field Init Shorthand & Struct Update Syntax
    example_2();

    // Tuple Struct
    example_3_tuple_struct();
}

// Define a struct
// Instantiate
// Field Access
fn example_1() {
    let mut user1 = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1.email: {}", user1.email);

    user1.email = String::from("anotheremail@example.com");
    println!("user1.email: {}", user1.email);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// the Field Init Shorthand & Struct Update Syntax
fn example_2() {
    let user1 = build_user(
        String::from("someusername123"),
        String::from("someone@example.com"),
    );

    let user2 = User {
        username: String::from("another@example.com"),
        email: String::from("anotherusername567"),
        ..user1 // Struct Update Syntax
    };
}

fn build_user(username: String, email: String) -> User {
    User {
        username, // Field Init Shorthand
        email,    // Field Init Shorthand
        sign_in_count: 1,
        active: true,
    }
}

// Tuple Struct
fn example_3_tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // error[E0369]: binary operation `==` cannot be applied to type `Color`
    //println!("black: Color == origin: Point? {}", black == origin);
}

struct Color(u32, u32, u32);
struct Point(u32, u32, u32);
