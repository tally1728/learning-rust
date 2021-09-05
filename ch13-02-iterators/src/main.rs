fn main() {
    // Listing 13-14: Using an iterator in a for loop
    ex_13_14();

    // Listing 13-15: Calling the next method on an iterator
    ex_13_15();

    // Listing 13-16: Calling the sum method to get the total of all items in the iterator
    ex_13_16();
    // Listing 13-18: Calling the map method to create a new iterator and then calling the collect
    // method to consume the new iterator and create a vector
    ex_13_18();

    // Listing 13-19: Using the filter method with a closure that captures shoe_size
    filters_by_size();

    // Implement Own Iterator
    // Listing 13-22: Testing the functionality of the next method implementation
    calling_next_directly();
    // Listing 13-23: Using a variety of Iterator trait methods on our Counter iterator
    using_other_iterator_trait_methods();
}

// Listing 13-14: Using an iterator in a for loop
fn ex_13_14() {
    let mut v = vec![1, 2, 3];

    // iterate over immutable references
    let v_iter = v.iter();
    for val in v_iter {
        println!("Got: {}", val);
    }

    // iterate over mutable references
    let v_iter = v.iter_mut();
    for val in v_iter {
        *val += 1;
    }
    println!("{:?}", v);

    // iterator that takes ownership
    let v_iter = v.into_iter();
    for val in v_iter {
        println!("Got: {}", val);
    }
    // error[E0382]: borrow of moved value: `v`
    //println!("{:?}", v);
}

// Listing 13-15: Calling the next method on an iterator
fn ex_13_15() {
    let v = vec![1, 2, 3];
    let mut v_iter = v.iter();

    println!("{:?}", v_iter.next());
    println!("{:?}", v_iter.next());
    println!("{:?}", v_iter.next());
    println!("{:?}", v_iter.next());
}

// Listing 13-16: Calling the sum method to get the total of all items in the iterator
fn ex_13_16() {
    let v = vec![1, 2, 3];

    let v_iter = v.iter();
    let total: i32 = v_iter.sum();
    println!("total: {}", total);
}

// Listing 13-18: Calling the map method to create a new iterator and then calling the collect
// method to consume the new iterator and create a vector
fn ex_13_18() {
    let mut v1 = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    v1.clear();
    println!("v2: {:?}", v2);
}

// Listing 13-19: Using the filter method with a closure that captures shoe_size
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);
    println!("{:?}", in_my_size);
}

////////////////////////////////////////////////////////////
// Implement Own Iterator
//
// Listing 13-20: Defining the Counter struct and a new function that creates instances of Counter
// with an initial value of 0 for count
struct Counter {
    counter: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { counter: 0 }
    }
}

// Listing 13-21: Implementing the Iterator trait on our Counter struct
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;
        if self.counter < 6 {
            Some(self.counter)
        } else {
            None
        }
    }
}

// Listing 13-22: Testing the functionality of the next method implementation
fn calling_next_directly() {
    for v in Counter::new() {
        println!("{:?}", v);
    }
}

// Listing 13-23: Using a variety of Iterator trait methods on our Counter iterator
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1)) // (1, 2), (2, 3), (3, 4), (4, 5)
        .map(|(a, b)| a * b) // 2, 6, 12, 20
        .filter(|x| x % 3 == 0) // 6, 12
        .sum();
    println!("sum: {}", sum);
}
