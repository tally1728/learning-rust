use std::thread;
use std::time::Duration;

fn main() {
    // Listing 16-1: Creating a new thread to print one thing while the main thread prints something
    // else
    //ex_16_1();

    // Listing 16-2: Saving a JoinHandle from thread::spawn to guarantee the thread is run to
    // completion
    //ex_16_2();

    ex_grandchild();

    // Listing 16-5: Using the move keyword to force a closure to take ownership of the values it uses
    ex_16_5();
}

// Listing 16-1: Creating a new thread to print one thing while the main thread prints something
// else
fn ex_16_1() {
    thread::spawn(|| {
        for i in 0..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// Listing 16-2: Saving a JoinHandle from thread::spawn to guarantee the thread is run to
// completion
fn ex_16_2() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 0..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    //handle.join().unwrap();
}

// Listing 16-5: Using the move keyword to force a closure to take ownership of the values it uses
fn ex_16_5() {
    let v = vec![1, 2, 3];

    // error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned
    // by the current function
    //let handle = thread::spawn(|| {
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // error[E0505]: cannot move out of `v` because it is borrowed
    //drop(v);

    handle.join().unwrap();
}

fn ex_grandchild() {
    // Child thread
    let handle = thread::spawn(|| {
        // GrandChild thread WILL BE CANCELLED!
        thread::spawn(|| {
            for i in 0..20 {
                println!("hi number {} from the GrandChild thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 0..10 {
            println!("hi number {} from the Child thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
