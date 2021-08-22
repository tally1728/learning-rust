// Ch.07.3 Paths - Listing 7-7: Absolute path & Relative path
mod listing_7_7 {
    mod front_of_house {

        pub mod hosting {
            pub fn add_to_waitlist() {}

            fn seat_at_table() {}
        }

        mod serving {
            fn cook_order() {}

            fn take_order() {}

            fn serve_order() {}

            fn take_payment() {}
        }
    }

    pub fn eat_at_restaurant() {
        // Absolute path
        crate::listing_7_7::front_of_house::hosting::add_to_waitlist();

        // Relative path
        front_of_house::hosting::add_to_waitlist();
    }
}

// Ch.07.3 Paths - Listing 7-8: Relative path w/ super
mod listing_7_8 {
    fn serve_order() {}

    mod back_of_house {
        fn fix_incorrect_order() {
            cook_order();
            super::serve_order();
        }
        fn cook_order() {}
    }
}

// Ch.07.3 Paths - Listing 7-9: pub struct
mod listing_7_9 {
    mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }
    }

    pub fn eat_at_restaurant() {
        // Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");

        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");

        println!("I'd like {} toast please", meal.toast);

        // error[E0616]: field `seasonal_fruit` of struct `Breakfast` is private
        //meal.seasonal_fruit = String::from("blueberries");
    }
}

// Ch.07.3 Paths - Listing 7-10: pub enum
mod listing_7_10 {
    mod back_of_house {
        pub enum Appetizer {
            Soup,
            Salad,
        }
    }

    pub fn eat_at_restaurant() {
        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
}

// Ch.07.4 use - Listing 7-11 & 7-12: use
mod listing_7_11 {
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    // Absolute path
    //use crate::listing_7_11::front_of_house::hosting;
    // Relative path
    use front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
