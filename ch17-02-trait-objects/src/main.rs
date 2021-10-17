fn main() {
    // Example of Trait Object
    ex_trait_obj();

    // Example of Non Object Safe
    ex_non_object_safe_tait();
}

// Definition of the Draw trait
trait Draw {
    fn draw(&self);
}

// Definition of the Screen struct with a components field holding a vector of trait objects that
// implement the Draw trait
struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for i in self.components.iter() {
            i.draw();
        }
    }
}

// A Button struct that implements the Draw trait
#[derive(Clone)]
struct Button {
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button: {}", self.label);
    }
}

// A SelectBox struct that implements the Draw trait
struct SelectBox {
    label: String,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox: {}", self.label);
    }
}

fn ex_trait_obj() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                label: String::from("SB1"),
            }),
            Box::new(Button {
                label: String::from("Btn1"),
            }),
        ],
    };

    screen.run();
}

////////////////////////////////////////////////////////////
// Example of Non Object Safe
//
trait NonSafeTrait {
    fn mycopy(&self) -> Self;
    fn id<U>(&self, x: U) -> U;
}

//struct ScreenNG {
//    // error[E0038]: the trait `NonSafeTrait` cannot be made into an object
//    components: Vec<Box<dyn NonSafeTrait>>,
//}
//
//impl ScreenNG {
//    fn run(&self) {
//        for i in self.components.iter() {
//            i.id(1);
//        }
//    }
//}

impl NonSafeTrait for Button {
    fn mycopy(&self) -> Self {
        println!("Button.mycopy(): {}", self.label);
        self.clone()
    }

    fn id<U>(&self, x: U) -> U {
        println!("Button.Id(): {}", self.label);
        x
    }
}

fn ex_non_object_safe_tait() {
    let button = Button {
        label: String::from("Btn"),
    };

    button.mycopy();
    button.id(1);
    button.id(String::from("hello"));
}
