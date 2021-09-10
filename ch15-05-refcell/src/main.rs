use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    ex_refcell();
    ex_rc_refcell();

    ex_15_24();
}

fn ex_refcell() {
    let r = RefCell::new(1);
    println!("{:?}", r);

    // RefMut Instant
    *r.borrow_mut() += 1;

    // Ref Instant
    println!("{:?}", r.borrow());

    // some Refs
    {
        // Ref-1
        let b1 = r.borrow();
        println!("b1: {:?}", b1);

        // Ref-2
        let b2 = r.borrow();
        println!("b2: {:?}", b2);
    }

    // Ref & RefMut
    {
        // Ref
        let b = r.borrow();
        println!("b: {:?}", b);

        // RefMut
        // => thread 'main' panicked at 'already borrowed: BorrowMutError', src/main.rs:24:19
        //let b_mut = r.borrow_mut();
        //println!("{:?}", b_mut);
    }

    // some RefMuts
    {
        // RefMut-1
        let b_mut_1 = r.borrow_mut();
        println!("b_mut_1: {:?}", b_mut_1);

        // RefMut-2
        // => thread 'main' panicked at 'already borrowed: BorrowMutError', src/main.rs:49:25
        //let b_mut_2 = r.borrow_mut();
        //println!("b_mut_2: {:?}", b_mut_2);
    }
}

fn ex_rc_refcell() {
    let r1: Rc<RefCell<i32>> = Rc::new(RefCell::new(1));
    println!("r1: {:?}", r1);

    let r2 = Rc::clone(&r1);
    println!("r2: {:?}", r2);

    // get RefMut from r2
    *r2.borrow_mut() += 1;

    println!("r1: {:?}", r1);
    println!("r2: {:?}", r2);
}

////////////////////////////////////////////////////////////
// Example - 1: Mock Object
//
// Listing 15-20: A library to keep track of how close a value is to a maximum value and warn when the value is at certain levels
trait Messenger {
    // Immutable method
    fn send(&self, msg: &str);
}

// I want to test this struct!
struct LimitTracker<'a, T>
where
    T: Messenger,
{
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

// Listing 15-22: Using RefCell<T> to mutate an inner value while the outer value is considered immutable
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    // Mock Object
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

////////////////////////////////////////////////////////////
// Example - 2: Cons List with Rc<RefCeLL<T>>
//
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

// Listing 15-24: Using Rc<RefCell<i32>> to create a List that we can mutate
fn ex_15_24() {
    let value = Rc::new(RefCell::new(1));

    let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));
    let b = List::Cons(Rc::new(RefCell::new(2)), Rc::clone(&a));
    let c = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);

    *value.borrow_mut() += 1;

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
}
