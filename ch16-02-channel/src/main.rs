use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Listing 16-8: Receiving the value “hi” in the main thread and printing it
    ex_16_8();

    // Listing 16-10: Sending multiple messages and pausing between each
    ex_16_10();

    // Listing 16-11: Sending multiple messages from multiple producers
    ex_16_11();
}

// Listing 16-8: Receiving the value “hi” in the main thread and printing it
fn ex_16_8() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();

        // error[E0382]: borrow of moved value: `val`
        //println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// Listing 16-10: Sending multiple messages and pausing between each
fn ex_16_10() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

// Listing 16-11: Sending multiple messages from multiple producers
fn ex_16_11() {
    let (tx1, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx1);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
