fn main() {
    // Lifetime & Borrowing
    // ex_10_17();
    ex_10_17_b();
    test_1();

    // Lifetime Annotations in Function Signatures
    ex_10_22();
    ex_10_23();
    // ex_10_24();
}

////////////////////////////////////////////////////////////
// Lifetime & Borrowing
//
// Listing 10-17: An attempt to use a reference whose value has gone out of scope
//fn ex_10_17() {
//    let r;
//
//    {
//        let x = 5;
//        // error[E0597]: `x` does not live long enough
//        r = &x; // Borrowed!
//    } // Lifetime ends!
//
//    println!("{}", r); // Borrow used here!
//}

fn ex_10_17_b() {
    let r;

    {
        let x = 5;
        r = &x; // Borrowed!
        println!("{}", r); // Borrow ends here!
    } // Lifetime ends!
}

// Why is this OK?
fn test_1() {
    let r1: &i32;

    {
        let r2: &i32 = &5;
        r1 = r2;
    }

    println!("{}", r1);
}

////////////////////////////////////////////////////////////
// Lifetime Annotations in Function Signatures
//
// Listing 10-22: The longest function definition specifying that all the references in the signature must have the same lifetime 'a
fn ex_10_22() {
    let string1 = "abcd";
    let string2 = "xyz";

    let result = longest(string1, string2);
    println!("The longest string is {}", result);
}

// 3 Cases
// Case 1: fn longest<'a>(x: &'a str, y: &str) -> &'a str
// Case 2: fn longest<'a>(x: &str, y: &'a str) -> &'a str
// Case 3
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Listing 10-23: Using the longest function with references to String values that have different concrete lifetimes
fn ex_10_23() {
    let string1 = String::from("abcd");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str()); // Borrowed!
        println!("The longest string is {}", result); // Borrow ends here! => OK!
    } // Lifetime of string2 ends! => Lifetime 'a ends!
} // Lifetime of string1 ends!

// Listing 10-24: Attempting to use result after string2 has gone out of scope
//fn ex_10_24() {
//    let string1 = String::from("long string is long");
//    let result;
//    {
//        let string2 = String::from("xyz");
//        // error[E0597]: `string2` does not live long enough
//        result = longest(string1.as_str(), string2.as_str()); // Borrowed!
//    } // Lifetime of string2 ends! => Lifetime 'a ends!
//    println!("The longest string is {}", result); // Borrow ends here! => NG!
//} // Lifetime of string1 ends!

// Lifetime Annotations: 2 Inputs & 1 Output
fn lifetime_anotations_1<'a>(x: &'a str, y: &str) -> &'a str {
    println!("y: {}", y);
    x
}

// Lifetime Annotations: 2 Inputs & 1 Output
//fn lifetime_anotations_2<'a>(x: &'a str, y: &str) -> &'a str {
//    println!("x: {}", x);
//    // error[E0621]: explicit lifetime required in the type of `y`
//    y
//}

// Lifetime Annotations: 2 Input & 1 Output
//fn lifetime_anotations_3<'a>(x: &str, y: &str) -> &'a str {
//    println!("x: {}", x);
//    println!("y: {}", y);
//    let s = String::from("really long string");
//    // error[E0515]: cannot return reference to local variable `s`
//    &s
//}

////////////////////////////////////////////////////////////
// Lifetime Annotations in Struct Definitions
//
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn ex_10_25() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

////////////////////////////////////////////////////////////
// Lifetime Elision
//

// Listing 10-26: A function we defined in Listing 4-9 that compiled without lifetime annotations, even though the parameter and return type are references
// fn first_word<'a>(s: &'a str) -> &'a str
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

////////////////////////////////////////////////////////////
// Generic Type Parameters, Trait Bounds, and Lifetimes Together
//
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
