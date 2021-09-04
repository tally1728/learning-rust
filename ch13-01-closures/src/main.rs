fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    // 1st Way: Storing a value by a expensive calculation
    ex13_06::generate_workout(simulated_user_specified_value, simulated_random_number);
    // 2nd Way: Closure
    //ex13_06::generate_workout(simulated_user_specified_value, simulated_random_number);
    // 3rd Way: Memorization with Closure
    ex13_11::generate_workout(simulated_user_specified_value, simulated_random_number);

    // Memorization with Closure
    // A trial: Support Various Types BUT hold a SINGLE value!
    memo_for_various_types_but_single_value::generate_workout(
        simulated_user_specified_value,
        simulated_random_number,
    );
    // 1st Improvement: Hold various values with HashMap!
    improved_memo_for_various_values::generate_workout();
    // 2nd Improvement: Support Various Types!
    improved_memo_for_various_types::generate_workout();

    // Capturing the Environment with Closures
    // Listing 13-12: Example of a closure that refers to a variable in its enclosing scope
    capturing_with_closures();
}

////////////////////////////////////////////////////////////
// 1st Way: Storing a value by a expensive calculation
//
mod ex13_04 {
    use std::thread;
    use std::time::Duration;

    fn simulated_expensive_calculation(intensity: u32) -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    }

    pub fn generate_workout(intensity: u32, random_number: u32) {
        let expensive_result = simulated_expensive_calculation(intensity);

        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_result);

            println!("Next, do {} situps!", expensive_result);
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!("Today, run for {} minutes!", expensive_result);
            }
        }
    }
}

////////////////////////////////////////////////////////////
// 2nd Way: Closure
//
mod ex13_06 {
    use std::thread;
    use std::time::Duration;

    pub fn generate_workout(intensity: u32, random_number: u32) {
        let expensive_closure = |num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        };

        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_closure(intensity));

            println!("Next, do {} situps!", expensive_closure(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!("Today, run for {} minutes!", expensive_closure(intensity));
            }
        }
    }
}

////////////////////////////////////////////////////////////
// 3rd Way: Memorization with Closure
//
mod ex13_11 {
    use std::thread;
    use std::time::Duration;

    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
                Some(v) => v,
            }
        }
    }

    pub fn generate_workout(intensity: u32, random_number: u32) {
        let mut expensive_result = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });

        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_result.value(intensity));

            println!("Next, do {} situps!", expensive_result.value(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_result.value(intensity)
                );
            }
        }
    }
}

////////////////////////////////////////////////////////////
// Memorization with Closure
// A trial: Support Various Types BUT hold a SINGLE value!
//
mod memo_for_various_types_but_single_value {
    use std::thread;
    use std::time::Duration;

    // OK
    struct Fn1<F, T>
    where
        F: Fn() -> T,
    {
        f: F,
    }

    // NG: error[E0392]: parameter `T` is never used
    struct Fn2<F, T>
    where
        F: Fn(T) -> (),
    {
        f: F,
        marker: std::marker::PhantomData<T>,
    }

    struct Cacher<T, V, W>
    where
        T: Fn(&V) -> W,
        W: Clone,
    {
        calculation: T,
        value: Option<W>,
        // otherwise error[E0392]: parameter `V` is never used
        marker: std::marker::PhantomData<V>,
    }

    impl<T, V, W> Cacher<T, V, W>
    where
        T: Fn(&V) -> W,
        W: Clone,
    {
        fn new(calculation: T) -> Cacher<T, V, W> {
            Cacher {
                calculation,
                value: None,
                marker: std::marker::PhantomData,
            }
        }

        fn value(&mut self, arg: &V) -> W {
            match &self.value {
                None => {
                    let w = (self.calculation)(arg);
                    self.value = Some(w.clone());
                    w
                }
                Some(w) => w.clone(),
            }
        }
    }

    pub fn generate_workout(intensity: u32, random_number: u32) {
        let mut expensive_result = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            *num
        });

        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_result.value(&intensity));

            println!("Next, do {} situps!", expensive_result.value(&intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_result.value(&intensity)
                );
            }
        }
    }
}

////////////////////////////////////////////////////////////
// Memorization with Closure
// 1st Improvement: Hold various values with HashMap!
//
mod improved_memo_for_various_values {
    use std::collections::HashMap;
    use std::thread;
    use std::time::Duration;

    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        values: HashMap<u32, u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                values: HashMap::new(),
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.values.get(&arg) {
                None => {
                    let w = (self.calculation)(arg);
                    self.values.insert(arg, w);
                    w
                }
                Some(w) => *w,
            }
        }
    }

    pub fn generate_workout() {
        let mut c = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num + 1
        });

        println!("c(1): {}", c.value(1));
        println!("c(1): {}", c.value(1));
        println!("c(2): {}", c.value(2));
        println!("c(1): {}", c.value(1));
        println!("c(2): {}", c.value(2));
        println!("c(3): {}", c.value(3));
    }
}

////////////////////////////////////////////////////////////
// Memorization with Closure
// 2nd Improvement: Support Various Types!
//
mod improved_memo_for_various_types {
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::thread;
    use std::time::Duration;

    struct Cacher<T, V, W>
    where
        T: Fn(&V) -> W,
        V: Eq + Hash + Clone,
        W: Clone,
    {
        calculation: T,
        values: HashMap<V, W>,
    }

    impl<T, V, W> Cacher<T, V, W>
    where
        T: Fn(&V) -> W,
        V: Eq + Hash + Clone,
        W: Clone,
    {
        fn new(calculation: T) -> Cacher<T, V, W> {
            Cacher {
                calculation,
                values: HashMap::new(),
            }
        }

        fn value(&mut self, arg: &V) -> W {
            match self.values.get(arg) {
                None => {
                    let w = (self.calculation)(arg);
                    self.values.insert(arg.clone(), w.clone());
                    w
                }
                Some(w) => w.clone(),
            }
        }
    }

    pub fn generate_workout() {
        // Fn(&i32) -> String
        let mut c = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            format!("Hello!: {}", num)
        });

        println!("c(1): {}", c.value(&1));
        println!("c(1): {}", c.value(&1));
        println!("c(2): {}", c.value(&2));
        println!("c(1): {}", c.value(&1));
        println!("c(2): {}", c.value(&2));
        println!("c(3): {}", c.value(&3));

        // Fn(&String) -> usize
        let mut c = Cacher::new(|str: &String| -> usize {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            str.len()
        });

        println!("c(a): {}", c.value(&String::from("a")));
        println!("c(a): {}", c.value(&String::from("a")));
        println!("c(ab): {}", c.value(&String::from("ab")));
        println!("c(a): {}", c.value(&String::from("a")));
        println!("c(ab): {}", c.value(&String::from("ab")));
        println!("c(abc): {}", c.value(&String::from("abc")));
    }
}

////////////////////////////////////////////////////////////
// Capturing the Environment with Closures
//
// Listing 13-12: Example of a closure that refers to a variable in its enclosing scope
fn capturing_with_closures() {
    let mut v = vec![1, 2, 3];

    // Fn
    let equal_to_v = |z| z == v;
    println!("equal_to_x(vec![1]): {}", equal_to_v(vec![1]));

    // FnMut
    let mut push_v = |i| v.push(i);
    push_v(4);
    println!("after push_v(4): {:?}", v);

    // FnOnce w/ move keyword
    let mut move_v = move |i| v.push(i);
    //println!("before move_v(5): {:?}", v);
    move_v(5);
    //println!("after move_v(5): {:?}", v);

    // FnOnce w/o move keyword
    let v = vec![1, 2, 3];
    let move2 = || {
        // Consuming v!
        let w = v;
        println!("w: {:?}", w);
    };
    // error[E0382]: borrow of moved value: `v`
    //println!("v: {:?}", v);
    move2();
    // error[E0382]: use of moved value: `move2`
    //move2();
}
