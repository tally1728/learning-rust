use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

fn main() {
    // Listing 16-12: Exploring the API of Mutex<T> in a single-threaded context for simplicity
    ex_16_12();

    // Listing 16-15: Using an Arc<T> to wrap the Mutex<T> to be able to share ownership across
    // multiple threads
    ex_16_15();
    // Atomic types
    ex_16_15_w_atomic();

    //ex_deadlock();
    // Aboid Deadlock 1: Lock Ordering
    ex_avoid_deadlock_1();
    // Aboid Deadlock 2: try_lock()
    ex_avoid_deadlock_2();
}

// Listing 16-12: Exploring the API of Mutex<T> in a single-threaded context for simplicity
fn ex_16_12() {
    let m = Mutex::new(5);

    {
        let mut mg = m.lock().unwrap();
        *mg += 1;
    }

    println!("m = {:?}", m);
}

// Listing 16-15: Using an Arc<T> to wrap the Mutex<T> to be able to share ownership across
// multiple threads
fn ex_16_15() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// Atomic types
fn ex_16_15_w_atomic() {
    let counter = Arc::new(AtomicI32::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            counter.fetch_add(1, Ordering::SeqCst);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", counter.load(Ordering::SeqCst));
}

// 0:00 threadA has locked Mutex1
// 0:01 threadB has locked mutex2
// 0:02 threadA tries to lock mutex2...
// 0:03 threadB tries to lock mutex1...
// ... Deadlock!!!
fn ex_deadlock() {
    // 2 mutexes
    let mutex1 = Arc::new(Mutex::new(String::new()));
    let mutex2 = Arc::new(Mutex::new(String::new()));
    let mut handles = vec![];

    // clone mutexes
    let mutex1_for_a = Arc::clone(&mutex1);
    let mutex2_for_a = Arc::clone(&mutex2);
    let mutex1_for_b = Arc::clone(&mutex1);
    let mutex2_for_b = Arc::clone(&mutex2);

    // sub thread A
    handles.push(thread::spawn(move || {
        let mut lock1 = mutex1_for_a.lock().unwrap();
        *lock1 = String::from("Thread A");
        println!("Thread A has locked Mutex1");

        thread::sleep(time::Duration::from_secs(2));

        println!("Thread A tries to lock Mutex2...");
        let mut lock2 = mutex2_for_a.lock().unwrap();
        *lock2 = String::from("Thread A");
        println!("Thread A has locked Mutex2");
    }));

    thread::sleep(time::Duration::from_secs(1));

    // sub thread B
    handles.push(thread::spawn(move || {
        let mut lock1 = mutex2_for_b.lock().unwrap();
        *lock1 = String::from("Thread B");
        println!("Thread B has locked Mutex2");

        thread::sleep(time::Duration::from_secs(2));

        println!("Thread B tries to lock Mutex1...");
        let mut lock2 = mutex1_for_b.lock().unwrap();
        *lock2 = String::from("Thread B");
        println!("Thread B has locked Mutex1");
    }));

    for handle in handles {
        handle.join().unwrap();
    }

    println!("mutex1: {}", *mutex1.lock().unwrap());
    println!("mutex2: {}", *mutex2.lock().unwrap());
}

// Aboid Deadlock 1: Lock Ordering
// 0:00 threadA has locked Mutex1
// 0:01 threadB tries to lock Mutex1...
// 0:02 threadA has locked Mutex2
// 0:02 threadB has locked Mutex1
// 0:04 threadB has locked Mutex2
fn ex_avoid_deadlock_1() {
    println!("Ex: Avoid Deadlock 1: Lock Ordering");

    // 2 mutexes
    let mutex1 = Arc::new(Mutex::new(String::new()));
    let mutex2 = Arc::new(Mutex::new(String::new()));
    let mut handles = vec![];

    // clone mutexes
    let mutex1_for_a = Arc::clone(&mutex1);
    let mutex2_for_a = Arc::clone(&mutex2);
    let mutex1_for_b = Arc::clone(&mutex1);
    let mutex2_for_b = Arc::clone(&mutex2);

    // sub thread A
    handles.push(thread::spawn(move || {
        println!("Thread A tries to lock Mutex1...");
        let mut lock1 = mutex1_for_a.lock().unwrap();
        *lock1 = String::from("Thread A");
        println!("Thread A has locked Mutex1");

        thread::sleep(time::Duration::from_secs(2));

        println!("Thread A tries to lock Mutex2...");
        let mut lock2 = mutex2_for_a.lock().unwrap();
        *lock2 = String::from("Thread A");
        println!("Thread A has locked Mutex2");
    }));

    thread::sleep(time::Duration::from_secs(1));

    // sub thread B
    handles.push(thread::spawn(move || {
        println!("Thread B tries to lock Mutex1...");
        let mut lock1 = mutex1_for_b.lock().unwrap();
        *lock1 = String::from("Thread B");
        println!("Thread B has locked Mutex1");

        thread::sleep(time::Duration::from_secs(2));

        println!("Thread B tries to lock Mutex2...");
        let mut lock2 = mutex2_for_b.lock().unwrap();
        *lock2 = String::from("Thread B");
        println!("Thread B has locked Mutex2");
    }));

    for handle in handles {
        handle.join().unwrap();
    }

    println!("mutex1: {}", *mutex1.lock().unwrap());
    println!("mutex2: {}\n", *mutex2.lock().unwrap());
}

// Aboid Deadlock 2: try_lock()
// 0:00 threadA has locked Mutex1
// 0:01 threadB has locked Mutex2
// 0:02 threadA tries to lock Mutex2...
// 0:03 threadB TRIES to lock Mutex1... failed
//      threadB has UNLOCKED Mutex2
//      threadA has locked Mutex2
//      threadA has unlocked Mutex1 & Mutex2
// 0:04 threadB has locked Mutex2 again
// 0:06 threadB TRIES to lock Mutex1... ok
fn ex_avoid_deadlock_2() {
    println!("Ex: Avoid Deadlock 2: try_lock()");

    // 2 mutexes
    let mutex1 = Arc::new(Mutex::new(String::new()));
    let mutex2 = Arc::new(Mutex::new(String::new()));
    let mut handles = vec![];

    // clone mutexes
    let mutex1_for_a = Arc::clone(&mutex1);
    let mutex2_for_a = Arc::clone(&mutex2);
    let mutex1_for_b = Arc::clone(&mutex1);
    let mutex2_for_b = Arc::clone(&mutex2);

    // sub thread A
    handles.push(thread::spawn(move || {
        let mut lock1 = mutex1_for_a.lock().unwrap();
        *lock1 = String::from("Thread A");
        println!("Thread A has locked Mutex1");

        thread::sleep(time::Duration::from_secs(2));

        println!("Thread A tries to lock Mutex2...");
        let mut lock2 = mutex2_for_a.lock().unwrap();
        *lock2 = String::from("Thread A");
        println!("Thread A has locked Mutex2");

        println!("Thread A has unlocked Mutex1 & Mutex2");
    }));

    thread::sleep(time::Duration::from_secs(1));

    // sub thread B
    handles.push(thread::spawn(move || loop {
        let mut lock1 = mutex2_for_b.lock().unwrap();
        *lock1 = String::from("Thread B");
        println!("Thread B has locked Mutex2");

        thread::sleep(time::Duration::from_secs(2));

        print!("Thread B tries to lock Mutex1... ");
        if let Ok(mut lock2) = mutex1_for_b.try_lock() {
            *lock2 = String::from("Thread B");
            println!("ok");
            break;
        } else {
            println!("failed");
            drop(lock1);
            println!("Thread B has unlocked Mutex2");
            thread::sleep(time::Duration::from_secs(1));
        }
    }));

    for handle in handles {
        handle.join().unwrap();
    }

    println!("mutex1: {}", *mutex1.lock().unwrap());
    println!("mutex2: {}", *mutex2.lock().unwrap());
}
