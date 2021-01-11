use std::collections::HashMap;

fn main() {
	// creating new hash map - stores data on heap
	// all keys must have same type and all values must have same type
	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);

	// turn vector of tuples into hash map
	let teams = vec![String::from("Blue"), String::from("Yellow")];
	let initial_scores = vec![10, 50];

	let mut scores: HashMap<_, _> = // rust will infer types from data so can use _
		teams.into_iter().zip(initial_scores.into_iter()).collect();

	// for Copy types, values copied
	// for owned values, values moved and hash map will be owner
	let field_name = String::from("Favorite color");
	let field_value = String::from("Blue");

	let mut map = HashMap::new();
	map.insert(field_name, field_value);
	// field_name and field_value are invalid at this point

	// accessing value
	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);

	let team_name = String::from("Blue");
	let score = scores.get(&team_name);

	// iterate over key/value pair
	for (key, value) in &scores {
		println!("{}: {}", key, value);
	}

	// overwriting value - just use insert
	scores.insert(String::from("Blue"), 25);
	println!("{:?}", scores);

	// only insert value if key has no value - use or_insert
	let mut scores = HashMap::new();
	scores.insert(String::from("Blue"), 10);

	scores.entry(String::from("Yellow")).or_insert(50);
	scores.entry(String::from("Blue")).or_insert(50);

	println!("{:?}", scores);

	
}