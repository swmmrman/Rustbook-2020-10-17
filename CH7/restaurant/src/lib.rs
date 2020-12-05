mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
    mod serving {
        fn take_order() {}
        fn take_payment() {}
    }
}

fn serve_order() {}
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
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // Order with Rye
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change mind to Wheat
    meal.toast = String::from("Wheat");
    println!("I'd link {} toast please", meal.toast);
    // Next line will not compile, due to accesing a private field.
    // meal.seasonal_fruit = String::from("blueberries");
}
