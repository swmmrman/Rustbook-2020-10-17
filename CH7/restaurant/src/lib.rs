mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
    mod serving {
        fn take_order() {}
        fn take_payment() {}
    }
}
pub fn eat_at_restaurant() {
    // Absolute version
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative version
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
}
