fn main() {
    // infinite loop
    // loop {
    //     println!("Hello, world!");
    // }

    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // Looping Through a Collection
    let a = [10, 20, 30, 40, 50];

    // with while loop
    let mut index = 0;
    while index < 5 {
        println!("a[{}]: {}", index, a[index]);
        index += 1;
    }

    // with for ... in
    for element in a.iter() {
        println!("element in collection: {}", element);
    }

    // for ... in & Range
    for number in (0..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
