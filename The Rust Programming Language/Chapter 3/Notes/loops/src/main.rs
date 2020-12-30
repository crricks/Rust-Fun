fn main() {
	let mut counter = 0;

	// Var result holds value returned from loop
	let result = loop {
		counter += 1;

		if counter == 10 {
			break counter * 2; // This is cool - specify value returned at break
		}
	};

	println!("The result is {}", result);

	// While loops - standard fare
	let mut number = 3;

	while number != 0 {
		println!("{}!", number);

		number -= 1;
	}

	println!("LIFTOFF!!!");

	// For loops
	let a = [10, 20, 30, 40, 50];

	for element in a.iter() {
		println!("the value is: {}", element);
	}

	// Using range
	for number in (1..4).rev() {
		println!("{}!", number);
	}

	println!("LIFTOFF!");
}
