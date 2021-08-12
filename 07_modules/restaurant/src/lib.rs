mod front_of_house; //imports from module

mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
}

fn serve_order() {}

mod back_of_house {
    fn cook_order() {}
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // because Breakfast has a private field, the
    // struct needs to provide a public associated
    // function that constructs an instance of
    // Breakfast, because otherwise there is no way
    // to set the value of the field seasonal_fruit

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // when an enum is public, all variants are public
    pub enum Appetizer {
        Soup,
        Salad
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // we're in a function, which means we've just called the
    // same function twice using absolute & relative paths

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // this next line won't compile because seasonal_fruit is private
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;
}

// now, we'll use the "use" keyword so we don't have to fully qualify everything
use crate::front_of_house::hosting; // can also use a relative path
pub fn eat_at_the_restaurant() {
    hosting::add_to_waitlist();
}

//use self::front_of_house::hosting; // would also have worked

// to avoid requiring the "hosting" namespace, we could also use:
//use self::front_of_house::hosting::add_to_waitlist();

// however, this is not idiomatic because it makes it unclear that
// the function is coming from somewhere else

// we can also re-publish, kind of like es6 modules:
pub use crate::front_of_house::hosting::add_to_waitlist;