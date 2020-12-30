use std::io;

fn main() {
	println!("Convert temperature to Farenheit or Celsius.");
    println!("Is your temp F or C?");

    let mut input = String::new();

    io::stdin()
    	.read_line(&mut input)
    	.expect("Please enter F or C.");

	println!("Enter temperature.");

	let mut temp = String::new();

	io::stdin()
		.read_line(&mut temp)
		.expect("Failed to read.");

	let temp: i32 = temp.trim().parse().expect("Please type a number!");

	match input.trim() {
    	"F" => convert_farenheit(temp),
    	"C" => convert_celcius(temp),
    	_=> println!("Not valid!")
    };

}

fn convert_farenheit(temp: i32) {
	let celsius = (temp - 32) * (5 / 9);

	println!("{} Farenheit is {} Celsius", temp, celsius)
}

fn convert_celcius(temp: i32) {
	let faren = (9 / 5) * temp +32;

	println!("{} Celsius is {} Farenheit", temp, faren)
}
