fn main() {
    another_function(5, 6);

    let y = 6; // Statement - does not return value

    let y = { // Expression - does not have semicolon ending
    	let x = 3;
    	x + 1
    };

    println!("The value of y is now: {}", y);

    // Business as usual
    let x = five();

    println!("The value of x is now: {}", x);
}

 // Functions start with fn - doesn't matter what order defined
fn another_function(x: i32, y: i32) {
	println!("The value of x is: {}", x);
	println!("The value of y is: {}", y);
}

// Declare type of value to be returned with ->
fn five() -> i32 {
	5 // No semicolon because want to return value
}