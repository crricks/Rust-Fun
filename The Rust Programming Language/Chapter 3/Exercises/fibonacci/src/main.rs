use std::io;

fn main() {
    println!("Generate nth Fibonacci number.");
    println!("Enter n")

    let n = String::new();

    io::std()
    	.read_line(&n)
    	.expect("Failed to read.")

    let n: u32 = n.trim().parse().expect("Please enter number.")

    
}
