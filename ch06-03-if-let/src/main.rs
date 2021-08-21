fn main() {
    // if let w/o value
    example1();

    // if let w/ value
    example2();
}

// if let w/o value
fn example1() {
    let v = Some(0u8);

    // match expression
    match v {
        Some(3) => println!("three"),
        _ => println!("Not three"),
    }

    // if let syntax
    if let Some(3) = v {
        println!("three");
    } else {
        println!("Not three");
    }

    // if expression
    if Some(3) == v {
        println!("three");
    } else {
        println!("Not three");
    }
}

// if let w/ value
fn example2() {
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

    let coin = Coin::Quarter(UsState::Alaska);

    // if let w/ value
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    }
}
