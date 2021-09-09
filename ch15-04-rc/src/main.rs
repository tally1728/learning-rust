fn main() {
    // Listing 15-18: A definition of List that uses Rc<T>
    ex_15_18();

    // Listing 15-19: Printing the reference count
    ex_15_19();

    try_derefmut();
}

use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

// Listing 15-18: A definition of List that uses Rc<T>
fn ex_15_18() {
    let a: Rc<List> = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    let b: List = List::Cons(3, Rc::clone(&a));
    let c: List = List::Cons(4, Rc::clone(&a));

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
}

// Listing 15-19: Printing the reference count
fn ex_15_19() {
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = List::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = List::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn try_derefmut() {
    let mut a: Rc<i32> = Rc::new(1);
    println!("a: {:?}", a);

    // error[E0594]: cannot assign to data in an `Rc`
    //*a += 1;
}
