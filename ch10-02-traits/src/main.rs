use std::fmt::Display;

fn main() {
    // Listing 10-13: Implementing the Summary trait on the NewsArticle and Tweet types
    listing_10_13::example_1_structs_implementing_a_trait();

    // Listing 10-14: Definition of a Summary trait with a default implementation of the summarize method
    listing_10_14::example_2_structs_implementing_a_trait();

    // Listing 10-15: A working definition of the largest function that works on any generic type that implements the PartialOrd and Copy traits
    example_largest();

    // Listing 10-16: Conditionally implement methods on a generic type depending on trait bounds
    example_conditionally_impl();
}

// Listing 10-13: Implementing the Summary trait on the NewsArticle and Tweet types
mod listing_10_13 {
    pub fn example_1_structs_implementing_a_trait() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());
    }

    trait Summary {
        fn summarize(&self) -> String;
    }

    struct NewsArticle {
        headline: String,
        location: String,
        author: String,
        content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    struct Tweet {
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
}

// Listing 10-14: Definition of a Summary trait with a default implementation of the summarize method
mod listing_10_14 {
    pub fn example_2_structs_implementing_a_trait() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("summarize_author(): {}", tweet.summarize_author());
        println!("1 new tweet: {}", tweet.summarize());

        // Traits as Arguments
        notify(&tweet);

        // Trait as Return
        returns_summarizable();
    }

    trait Summary {
        fn summarize_author(&self) -> String;

        // default implementation refers another method
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    struct Tweet {
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    ////////////////////////////////////////////////////////
    // Traits as Arguments
    //
    // Trait Bound Syntax
    //fn notify<T: Summary>(item: &T) {
    //    println!("Breaking news! {}", item.summarize());
    //}

    // impl Trait Syntax
    //fn notify(item: &impl Summary) {
    //    println!("Breaking news! {}", item.summarize());
    //}

    // Trait Bound Syntax with where Clauses
    fn notify<T>(item: &T)
    where
        T: Summary,
    {
        println!("Breaking news! {}", item.summarize());
    }

    ////////////////////////////////////////////////////////
    // Trait as Return
    //
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}

// Listing 10-15: A working definition of the largest function that works on any generic type that implements the PartialOrd and Copy traits
fn example_largest() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let result = largest_ref(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
    let result = largest_ref(&char_list);
    println!("The largest char is {}", result);
}

// w/ Copy Trait
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// w/o Copy Trait
fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Listing 10-16: Conditionally implement methods on a generic type depending on trait bounds
fn example_conditionally_impl() {
    let pair = Pair::new(1, 2);
    pair.cmp_display();
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
