fn main() {
    ex_enum();
    ex_at_bindings();
}

// enum
fn ex_enum() {
    // owned value => moved
    let o = Some(vec![0, 1, 2]);
    if let Some(v) = o {
        println!("some: {:?}", v);
    }

    // owned value (ref)
    let o = Some(vec![0, 1, 2]);
    if let Some(ref v) = o {
        println!("some: {:?}", v);
    }
    println!("Some(Vec): {:?}", o);

    // reference
    let o = Some(vec![0, 1, 2]);
    if let Some(v) = &o {
        println!("some: {:?}", v);
    }
    println!("Some(Vec): {:?}", o);

    // reference (Dereference with &)
    // error[E0507]: cannot move out of a shared reference
    // because Vec<i32> does not implement the Copy trait
    //let o = Some(vec![0, 1, 2]);
    //if let &Some(v) = &o {
    //    println!("some: {:?}", v);
    //}
    //println!("Some(Vec): {:?}", o);

    // It works for i32 which implements the Copy trait
    let o = Some(1);
    if let &Some(v) = &o {
        println!("some: {:?}", v);
    }
    println!("Some(Vec): {:?}", o);
}

// @ Bindings
fn ex_at_bindings() {
    // basic
    match 2 {
        x @ 0..=3 => {
            println!("{}", x);
        }
        _ => {}
    }

    // enum
    match Some(2) {
        Some(x @ 0..=3) => {
            println!("Some({})", x);
        }
        _ => {}
    }

    // struct
    struct Hello {
        x: i32,
    }

    match { Hello { x: 1 } } {
        Hello { x: a @ 0..=3 } => {
            println!("Hello: {}", a);
        }
        Hello { x: a } if (4..=7).contains(&a) => {
            println!("Hello: {}", a);
        }
        _ => {}
    }
}
