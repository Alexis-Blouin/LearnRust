mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    // Doesn't work because private
    // front_of_house::hosting::seat_at_table();

    // With use
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye bread
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Doesn't work because private
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

mod customer {
    pub fn eat_at_restaurant() {
        // Doesn't work even with use above since different scope
        // hosting::add_to_waitlist();
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        fn cook_order() {}
        super::deliver_order();
    }
    fn cook_order() {}

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
}
