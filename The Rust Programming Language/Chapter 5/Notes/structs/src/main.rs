fn main() {
	// create instance with key: value pairs
    let mut user1 = User {
    	email: String::from("someone@example.com"),
    	username: String::from("someusername123"),
    	active: true,
    	sign_in_count: 1,
    };

    // dot notation
    user1.email = String::from("anotheremail@example.com");

    // these two equivalent - remaining fields have same as user1
    let user2 = User {
    	email: String::from("another@example.com"),
    	username: String::from("anotherusername567"),
    	active: user1.active,
    	sign_in_count: user1.sign_in_count,
    };

    let user2 = User {
    	email: String::from("another@example.com"),
    	username: String::from("anotherusername567"),
    	..user1
    };

    // tuple struct
    struct Color(i32, i32, i32);

    let black = Color(0, 0, 0);

}

// making a struct - similar to Object/dictionary
struct User {
	// fields
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

fn build_user(email: String, username: String) -> User {
	User {
		email, // if parameter and field have same name, can use this shorthand
		username,
		active: true,
		sign_in_count: 1,
	}
}
