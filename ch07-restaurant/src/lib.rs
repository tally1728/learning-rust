// Listing 7-7: Absolute path & Relative path
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

// Listing 7-8: Relative path w/ super
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
