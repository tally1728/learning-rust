fn main() {
    ////////////////////////////////////////////////////////
    // a reference cycle
    // Cons(i32, RefCell<Rc<List>>)
    ex_15_26::ex();
    // Cons(i32, Rc<RefCell<List>>)
    ex_15_26_2nd::ex();

    ////////////////////////////////////////////////////////
    // a Tree Data Structure with a weak reference to its parent node
    // Listing 15-27: Creating a leaf node with no children and a branch node with leaf as one of its
    // children
    ex_15_27::ex();

    // Listing 15-28: A leaf node with a weak reference to its parent node branch
    tree_w_weak::ex_15_28();
    // Listing 15-29: Creating branch in an inner scope and examining strong and weak reference counts
    tree_w_weak::ex_15_29();

    // Trial: a Tree Data Structure with a 'reference' to its parent node
    tree_w_box_ref::ex();
}

////////////////////////////////////////////////////////
// a reference cycle
//
// Listing 15-25: A cons list definition that holds a RefCell<T> so we can modify what a Cons
// variant is referring to
// Listing 15-26: Creating a reference cycle of two List values pointing to each other
mod ex_15_26 {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                List::Cons(_, next) => Some(next),
                List::Nil => None,
            }
        }
    }

    impl Drop for List {
        fn drop(&mut self) {
            println!("Dropping: {:?}", self);
        }
    }

    pub fn ex() {
        println!("### Listing 15-26 ###");

        // create a list 'a': (1, Nil)
        let a = Rc::new(List::Cons(1, RefCell::new(Rc::new(List::Nil))));
        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        // create a list 'b' connecting 'a': (2, a) = (2, (1, Nil))
        let b = Rc::new(List::Cons(2, RefCell::new(Rc::clone(&a))));
        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        // connecting 'a' to 'b' => a Reference Cycle!
        // a = (1, b) = (1, (2, a))
        //   = (1, (2, (1, b))) = (1, (2, (1, (2, a)))) = ...
        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("a rc count after changing a = {}", Rc::strong_count(&a));
        println!("b rc count after changing a = {}\n", Rc::strong_count(&b));
        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        //println!("a next item = {:?}", a.tail());

        // a & b have not dropped!!
    }
}

// Cons(i32, Rc<RefCell<List>>)
mod ex_15_26_2nd {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<RefCell<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&Rc<RefCell<List>>> {
            match self {
                List::Cons(_, next) => Some(next),
                List::Nil => None,
            }
        }

        fn tail_mut(&mut self) -> Option<&mut Rc<RefCell<List>>> {
            match self {
                List::Cons(_, next) => Some(next),
                List::Nil => None,
            }
        }
    }

    impl Drop for List {
        fn drop(&mut self) {
            println!("Dropping: {:?}", self);
        }
    }

    pub fn ex() {
        println!("### Listing 15-26 (Rc<RefCell<List>>) ###");

        // create a list 'a': (1, Nil)
        let a = Rc::new(RefCell::new(List::Cons(
            1,
            Rc::new(RefCell::new(List::Nil)),
        )));
        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.borrow().tail());

        // create a list 'b' connecting 'a': (2, a) = (2, (1, Nil))
        let b = Rc::new(RefCell::new(List::Cons(2, Rc::clone(&a))));
        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.borrow().tail());

        // connecting 'a' to 'b' => a Reference Cycle!
        // a = (1, b) = (1, (2, a))
        //   = (1, (2, (1, b))) = (1, (2, (1, (2, a)))) = ...
        // Method-1: List::tail_mut() method
        if let Some(link) = a.borrow_mut().tail_mut() {
            *link = Rc::clone(&b);
        }
        // Method-2: direct
        if let List::Cons(_, ref mut link) = *a.borrow_mut() {
            *link = Rc::clone(&b);
        }

        // increment a value of 'a'
        if let List::Cons(ref mut v, _) = *a.borrow_mut() {
            *v += 1;
        }

        println!("a rc count after changing a = {}", Rc::strong_count(&a));
        println!("b rc count after changing a = {}\n", Rc::strong_count(&b));
        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        //println!("a next item = {:?}", a.borrow().tail());

        // a & b have not dropped!!
    }
}

////////////////////////////////////////////////////////
// a Tree Data Structure with a weak reference to its parent node
//
// Listing 15-27: Creating a leaf node with no children and a branch node with leaf as one of its
// children
mod ex_15_27 {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
    }

    pub fn ex() {
        println!("### Listing 15-27 ###");

        let leaf = Rc::new(Node {
            value: 3,
            children: RefCell::new(Vec::new()),
        });
        println!("leaf: {:?}", leaf);

        let branch = Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        };
        println!("branch: {:?}\n", branch);
    }
}

mod tree_w_weak {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    // Listing 15-28: A leaf node with a weak reference to its parent node branch
    pub fn ex_15_28() {
        println!("### Listing 15-28 ###");

        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        });
        println!("leaf: {:?}", leaf);
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        println!("branch: {:?}", branch);

        // a weak reference to its parent node branch
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!("leaf: {:?}", leaf);
        println!("leaf parent = {:?}\n", leaf.parent.borrow().upgrade());
    }

    // Listing 15-29: Creating branch in an inner scope and examining strong and weak reference counts
    pub fn ex_15_29() {
        println!("### Listing 15-29 ###");

        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        });
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
}

// Trial: a Tree Data Structure with a 'reference' to its parent node
mod tree_w_box_ref {
    #[derive(Debug)]
    struct Node<'a> {
        value: i32,
        parent: Option<&'a Box<Node<'a>>>,
        children: Vec<Box<Node<'a>>>,
    }

    pub fn ex() {
        println!("### Listing 15-28 ###");

        let leaf = Box::new(Node {
            value: 3,
            parent: None,
            children: Vec::new(),
        });
        println!("leaf: {:?}", leaf);

        let branch = Box::new(Node {
            value: 5,
            parent: None,
            children: vec![leaf],
        });
        println!("branch: {:?}", branch);

        // error[E0499]: cannot borrow `branch.children` as mutable more than once at a time
        //branch.children[0].parent = Some(&mut branch);
    }
}
