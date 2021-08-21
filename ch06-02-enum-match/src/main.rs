fn main() {
    // match enum variant w/o value
    example1();

    // match enum variant w/ value
    example2();

    // match Option enum variant
    example3();

    // non-exhaustive pattern => Error!!
    //example4();

    // The _ Placeholder
    example5();
}

// match enum variant w/o value
fn example1() {
    let coin = Coin::Penny;
    println!(
        "value of coin {:?} in cents: {}",
        coin,
        coin.value_in_cents()
    );

    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    impl Coin {
        fn value_in_cents(&self) -> u32 {
            match self {
                Coin::Penny => {
                    println!("Lucky penny!");
                    1
                }
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
    }
}

// match enum variant w/ value
fn example2() {
    let coin = Coin::Quarter(UsState::Alaska);
    println!(
        "value of coin {:?} in cents: {}",
        coin,
        coin.value_in_cents()
    );

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState), // Variant w/ Value
    }

    impl Coin {
        fn value_in_cents(&self) -> u32 {
            match self {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
        }
    }
}

// match Option enum variant
fn example3() {
    let five = Some(5);
    println!("plus_one(Some(5)): {:?}", plus_one(five));

    let none = None;
    println!("plus_one(None): {:?}", plus_one(none));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
}

// non-exhaustive pattern
fn example4() {
    let five = Some(5);
    println!("plus_one(Some(5)): {:?}", plus_one(five));

    let none = None;
    println!("plus_one(None): {:?}", plus_one(none));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        // error[E0004]: non-exhaustive patterns: `None` not covered
        match x {
            Some(i) => Some(i + 1),
        }
    }
}

// The _ Placeholder
fn example5() {
    let x = 0u8;
    match x {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
