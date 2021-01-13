use std::io;

fn main() {
    println!("Enter list of integers.");

    let mut input = String::new();

    let mut v: Vec<i32> = Vec::new();

    while input != "" {
	    io::stdin()
	    	.read_line(&mut input)
	    	.expect("Failed to read.");

	    let n: i32 = input.trim().parse().expect("Please enter number.");

	    v.push(n);
    }

    let mean = mean(&v);
    let median = median(&mut v);
}

fn mean(v: &Vec<i32>) -> i32 {
	let mut total = 0;
	let mut count = 0;

	for i in v {
		total += i;
		count += 1;
	}

	return total / count;
}

fn median(v: &mut Vec<i32>) -> f64 {
	let mut sorted = false;
	let mut temp: i32;

	while !sorted {
		sorted = true;
		for i in 0..v.len() {
			if v[i] < v[i + 1] {
				temp = v[i];
				v[i] = v[i + 1];
				v[i + 1] = temp;
				sorted = false;
			};
		}
	}

	if v.len() % 2 == 0 {
		return v[v.len() / 2].into();
	} else {
		return ((v[v.len() / 2] + v[(v.len() / 2) + 1]) / 2).into();
	}
}
