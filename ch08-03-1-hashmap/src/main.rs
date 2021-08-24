use std::collections::HashMap;

fn main() {
    // Listing 8-20
    // Creating a new hash map and inserting some keys and values
    example_generate_and_insert();

    // Listing 8-21
    // Creating a new hash map from 2 vectors w/ zip() & collect()
    example_iter_zip_collect();

    // Listing 8-23: get method & for loop
    example_read();

    // Listing 8-24: Update (Overwrite)
    example_update_overwrite();

    // Listing 8-25: Update (No Overwrite)
    example_update_no_overwrite();

    // Listing 8-26: Updating a Value Based on the Old Value
    example_update_based_on_old();
}

// Listing 8-20
// Creating a new hash map and inserting some keys and values
fn example_generate_and_insert() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);
}

// Listing 8-21
// Creating a new hash map from 2 vectors w/ zip() & collect()
fn example_iter_zip_collect() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // Struct std::vec::Vec
    // pub fn iter(&self) -> std::slice::Iter<'_, T>
    //
    // Trait std::iter::Iterator
    // fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter>
    // ‘Zips up’ two iterators into a single iterator of pairs.
    // fn collect<B>(self) -> B
    // Transforms an iterator into a collection.
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);
}

// Listing 8-23: get method & for loop
fn example_read() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // Option<&V>
    println!("key: {}, value: {:?}", &team_name, score);

    for (key, value) in &scores {
        println!("key: {}, value: {:?}", key, value);
    }
}

// Listing 8-24: Update (Overwrite)
fn example_update_overwrite() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    println!("{:?}", scores);

    scores.insert(String::from("Blue"), 20);
    println!("{:?}", scores);
}

// Listing 8-25: Update (No Overwrite)
fn example_update_no_overwrite() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    println!("{:?}", scores);

    // Empty => Insert
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("{:?}", scores);

    // Not Empty => No Overwrite
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
}

// Listing 8-26: Updating a Value Based on the Old Value
fn example_update_based_on_old() {
    let text = "hello world wonderful world";

    let mut word_counter = HashMap::new();

    for word in text.split_whitespace() {
        // get a mutable reference
        let count = word_counter.entry(word).or_insert(0);
        // dereference
        *count += 1;
    }

    println!("{:?}", word_counter);
}
