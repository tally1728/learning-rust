fn main() {
    // Listing 15-1: Storing an i32 value on the heap using a box
    ex_15_1();

    // Listing 15-5: Definition of List that uses Box<T> in order to have a known size
    ex_15_5();

    // cons w/ Struct & Option
    cons_struct();
}

// Listing 15-1: Storing an i32 value on the heap using a box
fn ex_15_1() {
    let b = Box::new(5);
    println!("b = {}", b);
}

// Listing 15-5: Definition of List that uses Box<T> in order to have a known size
fn ex_15_5() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    println!("{:?}", list);

    // read values
    let mut cell = &list;
    loop {
        if let List::Cons(v, next) = cell {
            println!("val: {}", v);
            cell = next;
        } else {
            break;
        }
    }
}

// if w/o Box, error[E0072]: recursive type `List` has infinite size
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// cons w/ Struct & Option
fn cons_struct() {
    let list = Cell {
        next: Some(Box::new(Cell {
            next: Some(Box::new(Cell { next: None, val: 3 })),
            val: 2,
        })),
        val: 1,
    };

    println!("{:?}", list);

    // read values
    let mut cell = &list;
    loop {
        println!("val: {}", cell.val);

        // implementation-1: 6 lines
        //cell = match cell.next {
        //    Some(ref next) => next,
        //    None => {
        //        break;
        //    }
        //};

        // implementation-2: 5 lines
        //cell = if let Some(ref next) = cell.next {
        //    next
        //} else {
        //    break;
        //}

        // implementation-3: 4 lines
        if cell.next.is_none() {
            break;
        }
        cell = cell.next.as_ref().unwrap();
    }
}

#[derive(Debug)]
struct Cell {
    next: Option<Box<Cell>>,
    val: i32,
}
