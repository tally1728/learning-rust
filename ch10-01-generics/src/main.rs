fn main() {
    // Listing 10-4: Two functions that differ only in their names and the types in their signatures
    example_wo_generics();

    // Listing 10-5: A definition of the largest function that uses generic type parameters but doesn’t
    // compile yet
    example_w_generics();

    // Listing 10-6: A Point<T> struct that holds x and y values of type T
    ex_10_6::example_struct_w_1_type_parameter();

    // Listing 10-8: A Point<T, U> generic over two types so that x and y can be values of different
    // types
    ex_10_8::example_struct_w_2_type_parameters();

    // Listing 10-9: Implementing a method named x on the Point<T> struct that will return a reference
    // to the x field of type T
    ex_10_9::example_struct_method_w_1_type_parameter();

    // Listing 10-10: An impl block that only applies to a struct with a particular concrete type for
    // the generic type parameter T
    ex_10_10::example_struct_method_w_type_parameters();
}

// Listing 10-4: Two functions that differ only in their names and the types in their signatures
fn example_wo_generics() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Listing 10-5: A definition of the largest function that uses generic type parameters but doesn’t
// compile yet
fn example_w_generics() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        // error[E0369]: binary operation `>` cannot be applied to type `T`
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Listing 10-6: A Point<T> struct that holds x and y values of type T
mod ex_10_6 {
    pub fn example_struct_w_1_type_parameter() {
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
        // error[E0308]: mismatched types
        //let wrong_point = Point { x: 1, y: 4.0 };
    }

    struct Point<T> {
        x: T,
        y: T,
    }
}

// Listing 10-8: A Point<T, U> generic over two types so that x and y can be values of different
// types
mod ex_10_8 {
    pub fn example_struct_w_2_type_parameters() {
        let both_integer = Point { x: 5, y: 10 };
        let both_float = Point { x: 1.0, y: 4.0 };
        let integer_and_float = Point { x: 5, y: 4.0 };
    }

    struct Point<T, U> {
        x: T,
        y: U,
    }
}

// Listing 10-9: Implementing a method named x on the Point<T> struct that will return a reference
// to the x field of type T
mod ex_10_9 {
    pub fn example_struct_method_w_1_type_parameter() {
        let p = Point { x: 5, y: 10 };
        println!("p.x = {}", p.x());
    }

    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
}

// Listing 10-10: An impl block that only applies to a struct with a particular concrete type for
// the generic type parameter T
mod ex_10_10 {
    pub fn example_struct_method_w_type_parameters() {
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }

    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }
}
