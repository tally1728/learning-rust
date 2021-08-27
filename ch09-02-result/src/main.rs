use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // Listing 9-5: Handling different kinds of errors in different ways
    file_open();

    // Listing 9-5 w/ unwrap_or_else() method
    file_open_unwrap_or_else();

    //file_open_unwrap();
    //file_open_expect();

    // Propagating Errors
    propagating_errors();
}

// Listing 9-5: Handling different kinds of errors in different ways
fn file_open() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(file) => file,
            Err(error) => {
                panic!("Tried to create file but there was a problem: {:?}", error);
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };
}

// Listing 9-5 w/ unwrap_or_else() method
fn file_open_unwrap_or_else() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn file_open_unwrap() {
    let f = File::open("hello.txt").unwrap();
}

fn file_open_expect() {
    let f = File::open("hello-no-existes.txt").expect("Failed to open hello.txt");
}

// Propagating Errors
fn propagating_errors() {
    read_username_from_file_1().unwrap();
    read_username_from_file_2().unwrap();
    read_username_from_file_3().unwrap();
}

// Listing 9-6: A function that returns errors to the calling code using match
fn read_username_from_file_1() -> Result<String, io::Error> {
    let f = File::open("username.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(error) => {
            return Err(error);
        }
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => Err(error),
    }
}

// Listing 9-7: A function that returns errors to the calling code using the ? operator
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Listing 9-8: Chaining method calls after the ? operator
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
