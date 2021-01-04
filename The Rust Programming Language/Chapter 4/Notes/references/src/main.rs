fn main() {
    
    // borrow using & in parameters
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // & - reference to s1

    println!("The length of '{}' is {}.", s1, len);

    // use mut to change - only one mutable reference for piece of data per scope
    let mut s = String::from("hello");

    change(&mut s);

    // must create new scope for second ref
    let mut s = String::from("hello");

    {
    	let r1 = &mut s;
    } // r1 goes out of scope

    let r2 = &mut s;
}

fn calculate_length(s: &String) -> usize {
	s.len()
} // don't drop s, because don't have ownership


fn change(some_string: &mut String) {
	some_string.push_str(", world");
}