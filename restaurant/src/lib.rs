mod back_of_house;
mod front_of_house;
use crate::{
    back_of_house::{Appetizer, Breakfast},
    front_of_house::hosting,
};

pub fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();

    // Relative path
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = Breakfast {
        toast: String::from(r#"Rye"#),
        seasonal_fruits: String::from(r#"peaches"#),
    };
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!(r#"I'd like {} toast please"#, meal.toast);

    let _order1 = Appetizer::Soup;
    let _order2 = Appetizer::Salad;
}
