fn main() {
    // inconsistency between String & Index
    first_word_example_1();

    // examples of slice
    string_slice();
    array_slice();

    // prevent inconsistency between String & Slice
    first_word_example_2();
}

////////////////////////////////////////////////////////////
// inconsistency between String & Index
fn first_word_example_1() {
    let mut s = String::from("hello world");
    println!("s: {}", s);

    let word = first_word(&s);

    s.clear();

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    println!("cleared s: {}", s);
    println!("word: {}", word);
}

// return an index of the first space
fn first_word(s: &String) -> usize {
    // make an array as bytes from String
    let bytes = s.as_bytes();

    // add an index to iterator
    for (i, &item) in bytes.iter().enumerate() {
        // if a space is found, the index of the space is returned
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

////////////////////////////////////////////////////////////
// Slices
fn string_slice() {
    let s = String::from("hello world");

    // Slice (type: &str)
    let e = &s[1..2];
    let hello = &s[..5];
    let world = &s[6..];
    let all = &s[..];

    println!("s: {}", s);
    println!("slice &s[1..2]: {}", e);
    println!("slice &s[..5]: {}", hello);
    println!("slice &s[6..]: {}", world);
    println!("slice &s[..]: {}", all);

    if s == all {
        println!("String == Whole Slice");
    }
}

fn array_slice() {
    let a = [1, 2, 3, 4, 5];

    // Slice (type: &[i32])
    let slice: &[i32] = &a[0..1];

    println!("length of array slice: {}", slice.len());

    if a == &a[..] {
        println!("Array == Whole Slice");
    }
}

////////////////////////////////////////////////////////////
// prevent inconsistency between String & Slice
fn first_word_example_2() {
    let mut s = String::from("hello world");
    println!("s: {}", s);

    // Occur of Immutable Borrow as Slice (type: &str)
    let word = first_word_2(&s);

    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    s.clear();

    // Use of Immutable Borrow as Slice (type: &str)
    println!("word: {}", word);
}

// return an index of the first space
//fn first_word_2(s: &String) -> &str {
fn first_word_2(s: &str) -> &str {
    // make an array as bytes from String
    let bytes = s.as_bytes();

    // add an index to iterator
    for (i, &item) in bytes.iter().enumerate() {
        // if a space is found, the index of the space is returned
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
