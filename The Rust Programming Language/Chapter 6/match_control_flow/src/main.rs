enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState), // can include another enum value
}

#[derive(Debug)]
enum UsState {
	Alabama,
	Alaska,
	Etc,
}

// matching 
fn main() {
	value_in_cents(Coin::Quarter(UsState::Alaska));

	// matching with option T
	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);
}

fn value_in_cents(coin: Coin) -> u8 {
	match coin {
		Coin::Penny => {
			println!("Lucky penny!");
			1
		}
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("State quarter from {:?}!", state);
			25
		}
	}
}

// matching with option T
fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i + 1),
	}
}