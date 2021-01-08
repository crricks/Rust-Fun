#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// defining module in library
// can have defs for other items such as mod, structs, enums, fnsm etc
// all items are private by default - child can see parent, but parent can't see child
mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist() {}

		fn seat_at_table() {}
	}

	mod serving {
		fn take_order() {}

		fn serve_order() {}

		fn take_paymen() {}
	}
}

mod back_of_house {
	fn fix_incorrect_order() {
		cook_order();
		super::serve_order();
	}

	fn cook_order() {}

	pub struct Breakfast {
		pub toast: String, // toast public
		seasonal_fruit: String, // fruit private
	}

	impl Breakfast {
		pub fn summer(toast: &str) -> Breakfast {
			Breakfast {
				toast: String::from(toast),
				seasonal_fruit: String::from("peaches"),
			}
		}
	}

	// create enum with all public variants
	pub enum Appetizer {
		Soup,
		Salad,
	}
}

// define absolute path so can use front_of_house easily
use crate::front_of_house::hosting;

// can also use relative
use self::front_of_house::hosting;

// if want external code to call, add pub
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
	// Absolute path
	crate::front_of_house::hosting::add_to_waitlist();

	// Relative path
	front_of_house::hosting::add_to_waitlist();

	// order break and choose toast since public
	let mut meal = back_of_house::Breakfast::summer("Rye");
	// chage mind about toast
	meal.toast = String::from("Wheat");
	println!("I'd like {} toast please", meal.toast);

	let order1 = back_of_house::Appetizer::Soup;
	let order2 = back_of_house::Appetizer::Salad;

	// equivalent to 65 and 68
	hosting::add_to_waitlist();

}