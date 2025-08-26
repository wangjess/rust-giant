mod front_of_house;

mod customer {
    use crate::front_of_house::hosting;
    use crate::back_of_house;

    pub fn eat_at_restaurant() {
        // Absolute path
        //crate::front_of_house::hosting::add_to_waitlist();

        // Relative path
        //front_of_house::hosting::add_to_waitlist();

        // Works because we used use above to bring hosting module into scope
        hosting::add_to_waitlist();

        // Order a breakfast in the summer with Rye toast.
        let mut meal = back_of_house::Breakfast::summer("Rye_Toast");

        // Change our mind about what bread we want.
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // This next line won't compile because we aren't allowed to
        // modify the seasonal_fruit as it's not public.
        //meal.seasonal_fruit = String::from("strawberries");

        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
}


fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub enum Appetizer {
        Soup,
        Salad,
    }

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