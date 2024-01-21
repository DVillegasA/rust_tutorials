mod front_of_house;

mod back_of_house;

fn deliver_order() {}

// By adding pub to this use, code that uses this package can access hosting
// and it's functions with restaurant::hosting instead of restaurant::front_of_house::hosting
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    // Order at restaurant in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about the bread we would like
    meal.toast = String::from("Wheat");
    print!("I'd like {} toast please!", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}