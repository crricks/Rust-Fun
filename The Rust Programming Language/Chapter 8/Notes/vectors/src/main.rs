fn main() {
	// vectors only store values of same type
    let v: Vec<i32> = Vec::new();

    // rust can infer type once insert values
    // use macro vec! 
    let v = vec![1, 2, 3];

    // add elements to vector
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // reading contents
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
    	Some(third) => println!("The third element is {}", third),
    	None => println!("There is no third element.")
    }

    // when out of index
    // this crashes program: let does_not_exist = &v[100];
    // this returns None without panicking
    let does_not_exist = v.get(100);

    // iterating over values -- immutable
    let v = vec![100, 32, 57];
    for i in &v {
    	println!("{}", i);
    }

    // iterating and changing -- mutable
    let mut v = vec![100, 32, 57];
    for i in &mut v {
    	*i += 50; // must dereference to get value in i before using
    }

    // when want vector of different types, use enum
    let row = vec![
	SpreadsheetCell::Int(3),
	SpreadsheetCell::Text(String::from("blue")),
	SpreadsheetCell::Float(10.12),
	];
}

// when want vector of different types, use enum
enum SpreadsheetCell {
	Int(i32),
	Float(f64),
	Text(String),
}


