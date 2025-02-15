mod front_of_house;
pub use crate::front_of_house::hosting;

fn deliver_order() {}

mod back_of_house {

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,

    }

    pub enum Appetizer {
        Soup,
        Salad,
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
        // super is like ".." in a filesystem path
        super::deliver_order();
    }

    fn cook_order() {}
}


mod customer {
    // Use creates a "shortcut" for the particular scope
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        // Absolute path
        hosting::add_to_waitlist();
        // Relative path
        //front_of_house::hosting::add_to_waitlist();

        // Order a breakfast in the summer with Rye toast
        let mut meal = super::back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast  = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // this line won't compile because modifyting the seasonal fruit
        // is not allowed.
        // meal.seasonal_fruit = String::from("blueberries");

        // The variants of public enums are all public
        let order1 = super::back_of_house::Appetizer::Soup;
        let order2 = super::back_of_house::Appetizer::Salad;
    }
}
