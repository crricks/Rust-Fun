fn main() {
	// using tuples to calculate area - better since passing one argument
    let rect1 = (30, 50);

    println!(
    	"The area of the rectangle is {} square pixels.",
    	area(rect1)
    );

	// even better option using struct - can name parameters
	let rect2 = Rectangle {
		width: 30,
		height: 50,
	};

	println!("rect2 is {:?}", rect2); // :? or :#? debug output format

	println!(
		"The area of the rectangle is {} square pixels.",
		better_area(&rect2)
		);

	// using methods
	let rect3 = Rectangle {
		width: 30,
		height: 50,
	};

	println!(
		"The area of the rectangle is {} square pixels.",
		rect3.area()
		);

	let rect4 = Rectangle {
		width: 10,
		height: 20,
	};

	// immutable borrow of rect4 as parameter
	println!("Can rect3 hold rect4? {}", rect3.can_hold(&rect4));

	// calling associated function
	let sq = Rectangle::square(3);
}

// not so good since don't know what dimensions are
fn area(dimensions: (u32, u32)) -> u32 {
	dimensions.0 * dimensions.1
}

#[derive(Debug)] // added functionality for debugging
struct Rectangle {
	width: u32,
	height: u32,
}

// implementation block to define all methods
impl Rectangle {
	fn area(&self) -> u32 { // create method
		self.width * self.height
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}

	// associated function
	fn square(size: u32) -> Rectangle {
		Rectangle {
			width: size,
			height: size,
		}
	}
}

fn better_area(rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height
}
