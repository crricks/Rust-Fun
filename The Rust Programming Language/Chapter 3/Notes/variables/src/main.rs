fn main() {

	// Must add mut to be able to change variable later
	let mut x = 5;
	println!("The value of x is: {}", x);
	x = 6;
	println!("The value of x is: {}", x);

	// First binds x to 5, then shadows x by using "let x = "
	let x = 5;

	let x = x + 1;

	let x = x * 2;

	println!("The value of x is: {}", x);

	// Possible to change type of variable when calling it again using let
	let spaces = "  "; // Does not work with mut - not allowed to mutate var type
	let spaces = spaces.len();

	// Integers using i or u
	let x: u8 = 127; // u8 - unsigned (only positive) 8-bit
	let x: i8 = -127; // i8 - signed (needs to have sign of neg or pos) 8-bit

	// Tuples - Use destructuring through pattern matching
	let tup = (500, 6.4, 1);

	let (x, y, z) = tup;

	println!("The value of y is: {}", y);

	// Tuples - can access tuple element using period
	let x: (i32, f64, u8) = (500, 6.4, 1);

	let five_hundred = x.0;

	let six_point_four = x.1;

	let one = x.2;

	println!("The vale of x.0 is {}, x.1 is {}, x.2 is {}", five_hundred, six_point_four, one);

	// Arrays have fixed length
	let a: [i32; 5] = [1, 2, 3, 4, 5] // i32 type of each element; 5 elements

	let a: [3; 5]; // 5 elements all set to 3

	let a = [1, 2, 3, 4, 5];

	let first = a[0]; // indexing as usual
	let second = a[1];

}
