mod front_of_house;

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
    pub enum Appetizer {
        Soup,
        Salad,
    }
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    //Appetizers
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    // Order with Rye
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change mind to Wheat
    meal.toast = String::from("Wheat");
    println!("I'd link {} toast please", meal.toast);
    // Next line will not compile, due to accesing a private field.
    // meal.seasonal_fruit = String::from("blueberries");
}
