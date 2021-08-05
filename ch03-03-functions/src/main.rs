fn main() {
    another_function();

    // Function w/ Arguments
    function_with_argument(2);
    function_with_2_arguments(2, 3);

    // Statements & Expressions
    block_expression();
    if_expression();

    // Function w/ Return
    let x = five();
    println!("The value of x is: {}", x);
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}

// Function w/ Arguments
fn function_with_argument(x: i32) {
    println!("The value of x is: {}", x);
}

fn function_with_2_arguments(x: i32, y: i32) {
    println!("x + y = {}", x + y);
}

// Statements & Expressions
fn block_expression() {
    let x = {
        // Block Expression
        let x = 1;
        x + 1 // Don't put ';'!
    };

    println!("block_expression: {}", x);
}

fn if_expression() {
    let x = 1;
    let x = if x == 5 { 10 } else { 15 };

    println!("if_expression: {}", x);
}

// Function w/ Return
fn five() -> i32 {
    5 // Don't put ';'!
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
