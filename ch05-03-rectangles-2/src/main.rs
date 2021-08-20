fn main() {
    // Method w/ Arg
    example_1_method();

    // Method w/ Arg
    example_2_method_with_arg();

    // Associated Functions
    example_3_associated_function();
}

// Method w/o Arg
fn example_1_method() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
}

// Method w/ Arg
fn example_2_method_with_arg() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// Associated Functions
fn example_3_associated_function() {
    let square = Rectangle::square(3);
    println!("{:?}", square);
}

// Struct Definition
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// method definition
impl Rectangle {
    // Example1: Method w/o Arg
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Example2: Method w/ Arg
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// associated function
impl Rectangle {
    // Example3: Associated Functions
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
