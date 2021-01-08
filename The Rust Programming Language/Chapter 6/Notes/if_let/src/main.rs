fn main() {
	// match that only cares about executing code when value is Some(3)
    let some_u8_value = Some(0u8)
    match some_u8_value {
    	Some(3) => println!("three"),
    	_ => (),
    }

    // equivalent to by using if let
    if let Some(3) = some_u8_value {
    	println!("three");
    }
}