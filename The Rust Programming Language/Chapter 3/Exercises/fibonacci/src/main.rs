use std::io;

fn main() {
    println!("Generate nth Fibonacci number.");
    println!("Enter a positive integer.");

    let mut n = String::new();

    io::stdin()
    	.read_line(&mut n)
    	.expect("Failed to read.");

    let n: u64 = n.trim().parse().expect("Please enter number.");

    let fib = fib(n);

    println!("The {} Fibonacci number is: {}", n, fib);
}

fn fib(n: u64) -> u64 {
	match n {
		0 => 0,
		1 => 1,
		_ => fib(n - 1) + fib(n - 2),
	}
}
