mod front_of_house {
    pub mod hosting {
        pub fn add_to_washlist() {} // use pub mod and pub fn to make child scope's
                                    // function available to parent scope
        // fn seat_at_table() {}
    }
    // mod serving {
    //     fn take_order() {}
    //     fn serve_order() {}
    //     fn take_payment() {}
    // }
}

pub fn eat_at_restaurant() {
    // Absolute path, generally more preferable approach
    crate::front_of_house::hosting::add_to_washlist();

    // Relative path
    front_of_house::hosting::add_to_washlist();
}

fn deliver_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // user `super` to access parent function
    }

    fn cook_order() {}
}

// Structs must declare each of its fields public separately if needed
mod back_house {
    pub struct Breakfast { // struct must be public to be exposed to parent scope
        pub toast: String, // each of its fields that we want to modify must be `pub`
        seasonal_fruit: String,
    }

    impl Breakfast { 
        // public associated function is needed to constuct an instance of Breakfast
        // since Breakfast has one private field (seasonal_fruit)
        pub fn summer(toast: &str) -> Breakfast { 
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
pub fn eat_at_the_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_house::Breakfast::summer("Rye"); // instance must be mutable
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}


// Enums declared as public, have all its variants public automatically
mod back_house_enum {
    pub enum Appetizer {
        Soup, 
        Salad,
    }
}
pub fn eat_at_the_restaurant_enum() {
    let order1 = back_house_enum::Appetizer::Soup;
    let order2 = back_house_enum::Appetizer::Salad;
}


// Bringing path into scope with `use`
mod front_of_house_use {
    pub mod hosting {
        pub fn add_to_washlist() {}
    }
}
// Idiomatic way to bring function into scope (by stopping at hosting)
use crate::front_of_house_use::hosting;
pub fn eat_at_restaurant_use() {
    hosting::add_to_washlist();
}
// However, for structs, enums and others it's idiomatic to specify full path
use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}


// For multiple objects having same name:
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}
fn function2() -> io::Result<()> {
    // --snip--
}

// OR by using `as` syntax to provide alias
use std::fmt::Result;
use std::io::Result as IoResult; 


// This enables to re-export code (making the item available for others to bring in their scope)
pub use crate::front_of_house::hosting;

// Using multiple items with same prefix
use std::{cmp::Ordering, io};
use std::io{self, Write}; // brings std::io and std::io::Write
use std::collections::*; // glob operator